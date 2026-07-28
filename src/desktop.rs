use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct DesktopApp {
    pub name: String,
    pub exec: String,
    pub icon_name: Option<String>,
}

/// Strips desktop-entry-spec field codes (%f, %F, %u, %U, %i, %c, %k, %%, ...)
/// from an `Exec=` value, since we're not passing any files/urls to it.
fn strip_field_codes(exec: &str) -> String {
    let mut out = String::with_capacity(exec.len());
    let mut chars = exec.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '%' {
            match chars.next() {
                Some('%') => out.push('%'),
                Some(_) => {} // drop %f %F %u %U %d %D %n %N %i %c %k etc.
                None => out.push('%'),
            }
        } else {
            out.push(c);
        }
    }
    out.trim().to_string()
}

fn parse_desktop_file(path: &Path) -> Option<DesktopApp> {
    let content = fs::read_to_string(path).ok()?;

    let mut in_main_group = false;
    let mut name = None;
    let mut exec = None;
    let mut icon = None;
    let mut no_display = false;
    let mut hidden = false;
    let mut is_application = true;

    for line in content.lines() {
        let line = line.trim();

        if line.starts_with('[') {
            in_main_group = line == "[Desktop Entry]";
            continue;
        }

        if !in_main_group || line.is_empty() || line.starts_with('#') {
            continue;
        }

        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim();
        let value = value.trim();

        match key {
            "Name" => name = Some(value.to_string()),
            "Exec" => exec = Some(strip_field_codes(value)),
            "Icon" => icon = Some(value.to_string()),
            "NoDisplay" => no_display = value.eq_ignore_ascii_case("true"),
            "Hidden" => hidden = value.eq_ignore_ascii_case("true"),
            "Type" => is_application = value == "Application",
            _ => {}
        }
    }

    if no_display || hidden || !is_application {
        return None;
    }

    Some(DesktopApp {
        name: name?,
        exec: exec?,
        icon_name: icon,
    })
}

fn walk_desktop_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        // Avoid following symlinked directories to keep this simple and loop-free.
        let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);

        if is_dir {
            walk_desktop_files(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "desktop") {
            out.push(path);
        }
    }
}

fn application_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    if let Some(data_home) = dirs::data_dir() {
        dirs.push(data_home.join("applications"));
    }

    match std::env::var("XDG_DATA_DIRS") {
        Ok(value) if !value.is_empty() => {
            for entry in value.split(':') {
                dirs.push(PathBuf::from(entry).join("applications"));
            }
        }
        _ => {
            dirs.push(PathBuf::from("/usr/local/share/applications"));
            dirs.push(PathBuf::from("/usr/share/applications"));
        }
    }

    dirs
}

/// Scans standard XDG application directories for `.desktop` files and
/// returns the ones meant to be shown to a user (skips `NoDisplay`/`Hidden`
/// entries and non-Application types).
pub fn scan_applications() -> Vec<DesktopApp> {
    let mut files = Vec::new();
    for dir in application_dirs() {
        walk_desktop_files(&dir, &mut files);
    }

    let mut apps: Vec<DesktopApp> = files.iter().filter_map(|f| parse_desktop_file(f)).collect();

    apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    apps.dedup_by(|a, b| a.name == b.name && a.exec == b.exec);

    apps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_field_codes() {
        assert_eq!(strip_field_codes("libreoffice %U"), "libreoffice");
        assert_eq!(strip_field_codes("firefox %u"), "firefox");
        assert_eq!(
            strip_field_codes("myapp --flag %f --other"),
            "myapp --flag  --other"
        );
        assert_eq!(strip_field_codes("echo 100%%"), "echo 100%");
        assert_eq!(strip_field_codes("plain-command"), "plain-command");
    }

    #[test]
    fn ignores_desktop_action_sections() {
        let dir = std::env::temp_dir().join("gmrun-rejuvenated-test-desktop-action");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("libreoffice-test.desktop");
        std::fs::write(
            &path,
            "[Desktop Entry]\n\
             Name=LibreOffice\n\
             Exec=libreoffice %U\n\
             Icon=libreoffice-startcenter\n\
             Type=Application\n\
             NoDisplay=false\n\
             \n\
             [Desktop Action Writer]\n\
             Name=Writer\n\
             Exec=libreoffice --writer\n",
        )
        .unwrap();

        let app = parse_desktop_file(&path).expect("should parse main entry");
        assert_eq!(app.name, "LibreOffice");
        assert_eq!(app.exec, "libreoffice");
        assert_eq!(app.icon_name.as_deref(), Some("libreoffice-startcenter"));

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn skips_no_display_and_hidden_entries() {
        let dir = std::env::temp_dir().join("gmrun-rejuvenated-test-hidden");
        std::fs::create_dir_all(&dir).unwrap();

        let hidden_path = dir.join("hidden.desktop");
        std::fs::write(
            &hidden_path,
            "[Desktop Entry]\nName=Hidden\nExec=hidden\nNoDisplay=true\n",
        )
        .unwrap();
        assert!(parse_desktop_file(&hidden_path).is_none());

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn scans_real_system_applications() {
        // Sanity check against whatever .desktop files actually exist in
        // this environment (e.g. /usr/share/applications).
        let apps = scan_applications();
        assert!(
            !apps.is_empty(),
            "expected to find at least one .desktop entry on this system"
        );
        assert!(apps
            .iter()
            .all(|a| !a.name.is_empty() && !a.exec.is_empty()));
    }
}

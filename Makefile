PREFIX ?= /usr/local
BINDIR ?= $(PREFIX)/bin
DATADIR ?= $(PREFIX)/share
DESTDIR ?=

INSTALL ?= install
INSTALL_PROGRAM ?= $(INSTALL) -Dm755
INSTALL_DATA ?= $(INSTALL) -Dm644

.PHONY: all build check test clean install uninstall

all: build

build:
	cargo build --release

check: test

test:
	cargo test --release

install: build
	$(INSTALL_PROGRAM) target/release/d77run $(DESTDIR)$(BINDIR)/d77run
	$(INSTALL_DATA) assets/d77run.desktop $(DESTDIR)$(DATADIR)/applications/d77run.desktop
	$(INSTALL_DATA) assets/d77run-icon.svg $(DESTDIR)$(DATADIR)/icons/hicolor/scalable/apps/d77run.svg
	$(INSTALL_DATA) assets/d77run-icon-16.png $(DESTDIR)$(DATADIR)/icons/hicolor/16x16/apps/d77run.png
	$(INSTALL_DATA) assets/d77run-icon-32.png $(DESTDIR)$(DATADIR)/icons/hicolor/32x32/apps/d77run.png
	$(INSTALL_DATA) assets/d77run-icon-192.png $(DESTDIR)$(DATADIR)/icons/hicolor/192x192/apps/d77run.png
	$(INSTALL_DATA) assets/d77run-icon-512.png $(DESTDIR)$(DATADIR)/icons/hicolor/512x512/apps/d77run.png
	$(INSTALL_DATA) LICENSE $(DESTDIR)$(DATADIR)/licenses/d77run/LICENSE
	@if [ -z "$(DESTDIR)" ] && command -v gtk-update-icon-cache >/dev/null 2>&1; then \
		gtk-update-icon-cache -f -t $(DATADIR)/icons/hicolor 2>/dev/null || true; \
	fi

uninstall:
	rm -f $(DESTDIR)$(BINDIR)/d77run
	rm -f $(DESTDIR)$(DATADIR)/applications/d77run.desktop
	rm -f $(DESTDIR)$(DATADIR)/icons/hicolor/scalable/apps/d77run.svg
	rm -f $(DESTDIR)$(DATADIR)/icons/hicolor/16x16/apps/d77run.png
	rm -f $(DESTDIR)$(DATADIR)/icons/hicolor/32x32/apps/d77run.png
	rm -f $(DESTDIR)$(DATADIR)/icons/hicolor/192x192/apps/d77run.png
	rm -f $(DESTDIR)$(DATADIR)/icons/hicolor/512x512/apps/d77run.png
	rm -rf $(DESTDIR)$(DATADIR)/licenses/d77run
	@if [ -z "$(DESTDIR)" ] && command -v gtk-update-icon-cache >/dev/null 2>&1; then \
		gtk-update-icon-cache -f -t $(DATADIR)/icons/hicolor 2>/dev/null || true; \
	fi

clean:
	cargo clean

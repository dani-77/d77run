use crate::config::error::ConfigError;
use iced::Color;
use std::{fmt::Display, num::ParseIntError};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct D77Color {
    color: Color,
}

impl Eq for D77Color {}

impl D77Color {
    pub(crate) const DEFAULT_BACKGROUND: D77Color = D77Color {
        color: Color {
            r: 0.08235294,
            g: 0.08235294,
            b: 0.08235294,
            a: 1.0,
        },
    };

    pub(crate) const DEFAULT_SCROLL: D77Color = D77Color {
        color: Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 0.2,
        },
    };

    pub(crate) const DEFAULT_SCROLLER: D77Color = D77Color {
        color: Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 0.4,
        },
    };

    pub(crate) const DEFAULT_BORDER: D77Color = D77Color {
        color: Color {
            r: 0.3647059,
            g: 0.36862746,
            b: 0.44705883,
            a: 1.0,
        },
    };

    pub(crate) const DEFAULT_TEXT: D77Color = D77Color {
        color: Color {
            r: 0.25490198,
            g: 0.25490198,
            b: 0.25490198,
            a: 1.0,
        },
    };

    pub(crate) const TRANSPARENT: D77Color = D77Color {
        color: Color::TRANSPARENT,
    };

    pub(crate) const WHITE: D77Color = D77Color {
        color: Color::WHITE,
    };

    pub(crate) const RED: D77Color = D77Color {
        color: Color {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        },
    };

    pub(crate) fn from(hex_color: &str) -> Result<Self, ConfigError> {
        let r = if let Some(red) = hex_color.get(1..3) {
            D77Color::f32_from_str_hex(red)
                .map_err(|_err| ConfigError::ParseColor(hex_color.to_string()))?
        } else {
            0.0
        };

        let g = if let Some(green) = hex_color.get(3..5) {
            D77Color::f32_from_str_hex(green)
                .map_err(|_err| ConfigError::ParseColor(hex_color.to_string()))?
        } else {
            0.0
        };

        let b = if let Some(blue) = hex_color.get(5..7) {
            D77Color::f32_from_str_hex(blue)
                .map_err(|_err| ConfigError::ParseColor(hex_color.to_string()))?
        } else {
            0.0
        };

        let a = if let Some(opacity) = hex_color.get(7..9) {
            D77Color::f32_from_str_hex(opacity)
                .map_err(|_err| ConfigError::ParseColor(hex_color.to_string()))?
        } else {
            1.0
        };

        Ok(D77Color {
            color: Color { r, g, b, a },
        })
    }

    fn f32_from_str_hex(hex_color: &str) -> Result<f32, ParseIntError> {
        u32::from_str_radix(hex_color, 16).map(|value| value as f32 / 255.0)
    }
}

impl Display for D77Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = (self.color.r * 255.0) as u32;
        let g = (self.color.g * 255.0) as u32;
        let b = (self.color.b * 255.0) as u32;
        let a = (self.color.a * 255.0) as u32;

        let r = to_lower_gex_with_leading_zero(r);
        let g = to_lower_gex_with_leading_zero(g);
        let b = to_lower_gex_with_leading_zero(b);
        let a = to_lower_gex_with_leading_zero(a);
        write!(f, "#{}{}{}{}", r, g, b, a)
    }
}

fn to_lower_gex_with_leading_zero(value: u32) -> String {
    let val = format!("{:x}", value);
    if val.len() == 1 {
        format!("0{}", val)
    } else {
        val
    }
}

impl From<D77Color> for Color {
    fn from(color: D77Color) -> Self {
        Color {
            r: color.color.r,
            g: color.color.g,
            b: color.color.b,
            a: color.color.a,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::config::color::D77Color;
    use iced_core::Color;

    #[test]
    fn should_get_color_from_str() {
        let hex_color = "#ff00ff";

        let color = D77Color::from(hex_color);

        assert_eq!(
            D77Color {
                color: Color {
                    r: 1.0,
                    g: 0.0,
                    b: 1.0,
                    a: 1.0,
                }
            },
            color.unwrap()
        );
    }

    #[test]
    fn should_get_color_from_with_opacity() {
        let hex_color = "#CCFF00CC";

        let color = D77Color::from(hex_color);

        assert_eq!(
            D77Color {
                color: Color {
                    r: 0.8,
                    g: 1.0,
                    b: 0.0,
                    a: 0.8,
                }
            },
            color.unwrap()
        );
    }

    #[test]
    fn should_get_red_value_and_default() {
        let hex_color = "#CC";

        let color = D77Color::from(hex_color);

        assert_eq!(
            D77Color {
                color: Color {
                    r: 0.8,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                }
            },
            color.unwrap()
        );
    }

    #[test]
    fn parse_error() {
        let hex_color = "#II";

        let color = D77Color::from(hex_color);

        assert!(color.is_err());
    }
}

use crate::app::style::Scale;
use crate::config::color::D77Color;
use crate::config::padding::D77Padding;
use iced::alignment::{Horizontal, Vertical};
use iced::Length;

#[derive(Debug, PartialEq, Clone)]
pub struct IconStyle {
    // Style
    pub background: D77Color,
    pub border_radius: f32,
    pub border_width: f32,
    pub color: D77Color,
    pub border_color: D77Color,

    // Layout
    pub padding: D77Padding,
    pub width: Length,
    pub height: Length,
    pub align_x: Horizontal,
    pub align_y: Vertical,
    pub icon_size: u16,
}

impl Scale for IconStyle {
    fn scale(mut self, scale: f32) -> Self {
        self.height = self.height.scale(scale);
        self.width = self.width.scale(scale);
        self.padding = self.padding.scale(scale);
        self.border_width = self.border_width.scale(scale);
        self.icon_size = self.icon_size.scale(scale);
        self
    }
}

impl Eq for IconStyle {}

impl Default for IconStyle {
    fn default() -> Self {
        IconStyle {
            // Style
            background: D77Color::DEFAULT_BACKGROUND,
            border_radius: 0.0,
            border_width: 0.0,
            border_color: D77Color::TRANSPARENT,
            color: D77Color::DEFAULT_TEXT,

            // Layout
            padding: D77Padding {
                top: 3,
                right: 6,
                bottom: 3,
                left: 0,
            },
            width: Length::Shrink,
            height: Length::Shrink,
            align_x: Horizontal::Center,
            align_y: Vertical::Center,
            icon_size: 22,
        }
    }
}

impl IconStyle {
    pub(crate) fn category_default() -> Self {
        Self {
            icon_size: 12,
            padding: D77Padding {
                top: 10,
                right: 6,
                bottom: 0,
                left: 0,
            },
            ..Default::default()
        }
    }
}

use crate::app::style::Scale;
use crate::config::color::D77Color;
use crate::config::padding::D77Padding;
use iced::alignment::{Horizontal, Vertical};
use iced::Length;
use iced_core::border::Radius;
use iced_core::{Background, Border, Color};
use iced_style::text_input::{Appearance, StyleSheet};

#[derive(Debug, PartialEq)]
pub struct SearchInputStyles {
    // Style
    pub background: D77Color,
    pub border_radius: f32,
    pub border_width: f32,
    pub border_color: D77Color,
    pub placeholder_color: D77Color,
    pub value_color: D77Color,
    pub selection_color: D77Color,
    pub text_width: Length,

    // Layout
    pub font_size: u16,
    pub width: Length,
    pub height: Length,
    pub align_x: Horizontal,
    pub align_y: Vertical,
    pub padding: D77Padding,
}

impl Scale for SearchInputStyles {
    fn scale(mut self, scale: f32) -> Self {
        self.height = self.height.scale(scale);
        self.width = self.width.scale(scale);
        self.padding = self.padding.scale(scale);
        self.padding = self.padding.scale(scale);
        self.padding = self.padding.scale(scale);
        self
    }
}

impl Eq for SearchInputStyles {}

impl StyleSheet for &SearchInputStyles {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> Appearance {
        Appearance {
            background: Background::Color(self.background.into()),
            border: Border {
                color: self.border_color.into(),
                width: self.border_width,
                radius: Radius::from(self.border_radius),
            },
            icon_color: Default::default(),
        }
    }

    fn focused(&self, style: &Self::Style) -> Appearance {
        self.active(style)
    }

    fn placeholder_color(&self, _: &Self::Style) -> Color {
        self.placeholder_color.into()
    }

    fn value_color(&self, _: &Self::Style) -> Color {
        self.value_color.into()
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        Color::TRANSPARENT
    }

    fn selection_color(&self, _: &Self::Style) -> Color {
        self.selection_color.into()
    }

    fn disabled(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            background: Background::Color(self.background.into()),
            border: Border {
                color: self.border_color.into(),
                width: self.border_width,
                radius: Radius::from(self.border_radius),
            },
            icon_color: Default::default(),
        }
    }
}

impl Default for SearchInputStyles {
    fn default() -> Self {
        SearchInputStyles {
            border_radius: 0.0,
            border_width: 0.0,
            border_color: D77Color::TRANSPARENT,
            background: D77Color::WHITE,
            placeholder_color: D77Color::DEFAULT_TEXT,
            value_color: D77Color::DEFAULT_TEXT,
            selection_color: D77Color::DEFAULT_BORDER,
            text_width: Length::Fill,
            font_size: 14,
            width: Length::Fill,
            height: Length::Fill,
            align_x: Horizontal::Left,
            align_y: Vertical::Center,
            padding: D77Padding {
                top: 0,
                right: 5,
                bottom: 0,
                left: 5,
            },
        }
    }
}

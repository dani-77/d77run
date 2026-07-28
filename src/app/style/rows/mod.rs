use crate::app::style::Scale;
use crate::config::color::D77Color;
use crate::config::padding::D77Padding;
use generic::GenericContainerStyle;
use iced::alignment::{Horizontal, Vertical};
use iced::Length;
use iced_core::border::Radius;
use iced_core::{Background, Border};
use iced_style::container::{Appearance, StyleSheet};
use icon::IconStyle;

pub mod button;
pub mod generic;
pub mod icon;

#[derive(Debug, PartialEq, Clone)]
pub struct RowStyles {
    // Layout
    pub padding: D77Padding,
    pub width: Length,
    pub height: Length,
    pub spacing: u16,
    pub align_x: Horizontal,
    pub align_y: Vertical,

    // Style
    pub background: D77Color,
    pub border_radius: f32,
    pub border_width: f32,
    pub color: D77Color,
    pub border_color: D77Color,
    pub hide_description: bool,
    pub hide_category_icon: bool,

    // Children
    pub title: GenericContainerStyle,
    pub description: GenericContainerStyle,
    pub icon: IconStyle,
    pub category_icon: IconStyle,
}

impl Scale for RowStyles {
    fn scale(mut self, scale: f32) -> Self {
        self.height = self.height.scale(scale);
        self.width = self.width.scale(scale);
        self.spacing = self.spacing.scale(scale);
        self.border_width = self.border_width.scale(scale);
        self.title = self.title.scale(scale);
        self.description = self.description.scale(scale);
        self.icon = self.icon.scale(scale);
        self.category_icon = self.category_icon.scale(scale);
        self
    }
}
impl StyleSheet for &RowStyles {
    type Style = iced::Theme;

    fn appearance(&self, _: &Self::Style) -> Appearance {
        Appearance {
            text_color: Some(self.color.into()),
            background: Some(Background::Color(self.background.into())),
            border: Border {
                color: self.border_color.into(),
                width: self.border_width,
                radius: Radius::from(self.border_radius),
            },
            shadow: Default::default(),
        }
    }
}

impl Default for RowStyles {
    fn default() -> Self {
        RowStyles {
            width: Length::Fill,
            height: Length::Shrink,
            background: D77Color::DEFAULT_BACKGROUND,
            color: D77Color::DEFAULT_TEXT,
            border_radius: 0.0,
            border_width: 0.0,
            padding: D77Padding::from(5),
            align_x: Horizontal::Right,
            align_y: Vertical::Bottom,
            border_color: D77Color::RED,
            hide_description: false,
            hide_category_icon: false,
            title: GenericContainerStyle::default(),
            description: GenericContainerStyle::description_default(),
            icon: Default::default(),
            category_icon: IconStyle::category_default(),
            spacing: 2,
        }
    }
}

impl RowStyles {
    pub fn default_selected() -> Self {
        Self {
            color: D77Color::WHITE,
            title: GenericContainerStyle {
                color: D77Color::WHITE,
                ..Default::default()
            },
            description: GenericContainerStyle {
                color: D77Color::WHITE,
                ..GenericContainerStyle::description_default()
            },
            ..Default::default()
        }
    }
}
impl Eq for RowStyles {}

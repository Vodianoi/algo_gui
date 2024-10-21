// General theme settings

use console_engine::Color;

pub struct Theme {
    pub color: Color,
    pub color_hover: Color,
    pub color_selected: Color,
    pub color_selected_hover: Color,
    pub bg_color: Color,
}

impl Theme {
    pub fn new(
        color: Color,
        color_hover: Color,
        color_selected: Color,
        color_selected_hover: Color,
        bg_color: Color,
    ) -> Theme {
        Theme {
            color,
            color_hover,
            color_selected,
            color_selected_hover,
            bg_color,
        }
    }
}

pub fn default_theme() -> Theme {
    Theme::new(
        Color::White,
        Color::Black,
        Color::Black,
        Color::White,
        Color::Black,
    )
}

use iced::{
    Background, Border, Color, Shadow, Vector,
    border::Radius,
    color,
    overlay::menu,
    widget::{button, checkbox, pick_list},
};

// rosé pine.

pub const BASE: Color = color!(0x191724);
pub const SURFACE: Color = color!(0x1f1d2e);
pub const TEXT: Color = color!(0xe0def4);
pub const SUBTLE: Color = color!(0x908caa);
pub const LOVE: Color = color!(0xeb6f92);
pub const FOAM: Color = color!(0x9ccfd8);

pub fn pick_list_style(status: pick_list::Status) -> pick_list::Style {
    pick_list::Style {
        text_color: TEXT,
        placeholder_color: SUBTLE,
        handle_color: LOVE,
        background: Background::Color(match status {
            pick_list::Status::Hovered => SURFACE,
            _ => BASE,
        }),
        border: Border {
            color: LOVE,
            width: 1.0,
            radius: Radius::new(10.0),
        },
    }
}

pub fn menu_style() -> menu::Style {
    menu::Style {
        text_color: TEXT,
        background: Background::Color(SURFACE),
        border: Border {
            color: BASE,
            width: 1.0,
            radius: Radius::new(5.0),
        },
        selected_text_color: LOVE,
        selected_background: Background::Color(BASE),
    }
}

pub fn checkbox_style(status: checkbox::Status) -> checkbox::Style {
    checkbox::Style {
        background: Background::Color(match status {
            checkbox::Status::Active { is_checked } if is_checked => FOAM,
            checkbox::Status::Hovered { is_checked } if is_checked => FOAM,
            checkbox::Status::Disabled { is_checked: _ } => BASE,
            _ => LOVE,
        }),
        icon_color: BASE,
        border: Border {
            color: BASE,
            width: 0.0,
            radius: Radius::new(10.0),
        },
        text_color: None,
    }
}

pub fn button_style(status: button::Status) -> button::Style {
    button::Style {
        background: Some(Background::Color(LOVE)),
        text_color: match status {
            button::Status::Pressed => TEXT,
            _ => BASE,
        },
        border: Border {
            color: BASE,
            width: 0.0,
            radius: Radius::new(10.0),
        },
        shadow: Shadow {
            color: LOVE.scale_alpha(0.1),
            blur_radius: match status {
                button::Status::Hovered => 15.0,
                _ => 0.0,
            },
            offset: Vector::new(0.0, 0.0),
        },
    }
}

use color_hex::color_from_hex;
use tmux_interface::Colour;

pub const STYLE_DEFAULT: &str = "default";
pub const VARIABLE_USER: &str = "user";

const fn c(hex: [u8; 3]) -> Colour {
    let r = hex[0] as u32 * 16 * 16 * 16 * 16;
    let g = hex[1] as u32 * 16 * 16;
    let b = hex[2] as u32;
    Colour::HEX(r + g + b)
}

pub const C_WHITE: Colour = c(color_from_hex!("#cccccc"));
pub const C_BLACK: Colour = c(color_from_hex!("#202328"));
pub const C_RED: Colour = c(color_from_hex!("#cc0000"));
pub const C_GREEN: Colour = c(color_from_hex!("#039300"));
pub const C_BLUE: Colour = c(color_from_hex!("#51afef"));
pub const C_YELLOW: Colour = c(color_from_hex!("#fed73b"));
pub const C_PURPLE: Colour = c(color_from_hex!("#c678dd"));

pub const C_DEFAULT_BG: Colour = C_BLACK;
pub const C_DEFAULT_FG: Colour = C_WHITE;

// Special chars
pub const RIGHT_TRIANGLE: &str = "\u{e0b0}";
pub const LOWER_LEFT_TRIANGLE: &str = "\u{e0ba}";
pub const UPPER_LEFT_TRIANGLE: &str = "\u{e0bc}";
pub const SLASH: &str = "\u{e0bb}";

pub const LINUX_ICON: &str = "\u{f17c}";
pub const APPLE_ICON: &str = "\u{f179}";

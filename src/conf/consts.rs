use color_hex::color_from_hex;
use tmux_interface::Colour;

const fn c(hex: [u8; 3]) -> Colour {
    let r = hex[0] as u32 * 16 * 16;
    let g = hex[1] as u32 * 16;
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

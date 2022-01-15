pub type Color = u32;

pub const fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}


pub const fn from_u8_rgba(r: u8, g: u8, b: u8, a: u8) -> u32 {
    let (r, g, b, a) = (r as u32, g as u32, b as u32, a as u32);
    (a << 24) | (r << 16) | (g << 8) | b 
}

pub const fn from_rgba_u8(color: u32) -> (u8, u8, u8, u8) {
    (
        ((color >> 16)&255) as u8,
        ((color >> 8)&255) as u8,
        (color&255) as u8,
        ((color >> 24)&255) as u8
    )
}



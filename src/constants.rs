pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 600;

pub const MAP_W: usize = 24;
pub const MAP_H: usize = 24;

// Tiles
pub const TILE_EMPTY: i32 = 0;
pub const TILE_WALL:  i32 = 1; // fuego
pub const TILE_EXIT:  i32 = 9; // meta

#[inline]
pub const fn rgb(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

#[inline]
pub fn shade(color: u32, factor: f64) -> u32 {
    let r = ((color >> 16) & 0xFF) as f64;
    let g = ((color >> 8) & 0xFF) as f64;
    let b = ( color        & 0xFF) as f64;
    rgb((r * factor) as u8, (g * factor) as u8, (b * factor) as u8)
}

#[inline]
pub fn alpha_blend(bg: u32, fg: u32, a: u8) -> u32 {
    if a == 255 { return fg; }
    if a == 0   { return bg; }
    let ar = a as u32;
    let br = 255 - ar;
    let fr = (fg >> 16) & 0xFF; let fg_g = (fg >> 8) & 0xFF; let fb = fg & 0xFF;
    let br_r = (bg >> 16) & 0xFF; let br_g = (bg >> 8) & 0xFF; let bb = bg & 0xFF;
    let r = (fr*ar + br_r*br) / 255;
    let g = (fg_g*ar + br_g*br) / 255;
    let b = (fb*ar + bb*br) / 255;
    (r << 16) | (g << 8) | b
}

pub const TILE_HAZARD: i32 = 7;        // “lava” que hace daño al pisarla
pub const PLAYER_MAX_HP: i32 = 100;    // vida máxima

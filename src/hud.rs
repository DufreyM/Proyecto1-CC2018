use crate::constants::{rgb, WIDTH, HEIGHT, MAP_W, MAP_H};
use crate::world::WorldMap;

#[inline]
pub fn put_pixel(buf: &mut [u32], x: usize, y: usize, c: u32) {
    if x < WIDTH && y < HEIGHT { buf[y * WIDTH + x] = c; }
}

pub fn draw_minimap(buf: &mut [u32], map: &WorldMap, px: f64, py: f64, dx: f64, dy: f64) {
    const SCALE: usize = 4;
    let off_x = 10usize;
    let off_y = 10usize;

    for my in 0..MAP_H {
        for mx in 0..MAP_W {
            let cell = map[my][mx];
            let base = match cell {
                0 => rgb(28,28,36),
                9 => rgb(255,180,80),
                _ => rgb(90,90,110),
            };
            for pyx in 0..SCALE {
                for pxx in 0..SCALE {
                    put_pixel(buf, off_x + mx*SCALE + pxx, off_y + my*SCALE + pyx, base);
                }
            }
        }
    }

    // jugador
    let ux = off_x + (px as usize) * SCALE;
    let uy = off_y + (py as usize) * SCALE;
    for dy2 in 0..2 { for dx2 in 0..2 {
        put_pixel(buf, ux+dx2, uy+dy2, rgb(255,255,255));
    }}

    // dirección
    let fx = (px + dx * 0.8) as usize * SCALE + off_x;
    let fy = (py + dy * 0.8) as usize * SCALE + off_y;
    super::render::line(buf, ux, uy, fx, fy, rgb(255,255,255));
}

pub fn draw_health_bar(buf: &mut [u32], hp: i32, max_hp: i32) {
    let w = 200usize;
    let h = 12usize;
    let x0 = 10usize;
    let y0 = HEIGHT - h - 10;
    // fondo
    for y in 0..h { for x in 0..w { put_pixel(buf, x0+x, y0+y, rgb(28,28,36)); } }
    // relleno
    let pct = (hp.max(0) as f64) / (max_hp.max(1) as f64);
    let fill = (pct * w as f64) as usize;
    let color = if pct > 0.5 { rgb(70, 220, 90) } else if pct > 0.25 { rgb(255, 200, 50) } else { rgb(230, 60, 60) };
    for y in 0..h { for x in 0..fill { put_pixel(buf, x0+x, y0+y, color); } }
    // borde
    let c = rgb(255,255,255);
    for x in 0..w { put_pixel(buf, x0+x, y0, c); put_pixel(buf, x0+x, y0+h-1, c); }
    for y in 0..h { put_pixel(buf, x0, y0+y, c); put_pixel(buf, x0+w-1, y0+y, c); }
}

pub fn glyph5x7(ch: char) -> [u8; 7] {
    // Cada byte es una fila (5 bits útiles: MSB a la izquierda). 1 = píxel encendido.
    match ch.to_ascii_uppercase() {
        'A' => [0x0E,0x11,0x11,0x1F,0x11,0x11,0x11],
        'B' => [0x1E,0x11,0x11,0x1E,0x11,0x11,0x1E],
        'C' => [0x0E,0x11,0x10,0x10,0x11,0x0E,0x00],
        'D' => [0x1E,0x11,0x11,0x11,0x11,0x1E,0x00],
        'E' => [0x1F,0x10,0x1E,0x10,0x10,0x1F,0x00],
        'F' => [0x1F,0x10,0x1E,0x10,0x10,0x10,0x00],
        'G' => [0x0E,0x11,0x10,0x17,0x11,0x0F,0x00],
        'H' => [0x11,0x11,0x11,0x1F,0x11,0x11,0x11],
        'I' => [0x0E,0x04,0x04,0x04,0x04,0x0E,0x00],
        'J' => [0x07,0x01,0x01,0x01,0x11,0x0E,0x00],
        'K' => [0x11,0x12,0x14,0x18,0x14,0x12,0x11],
        'L' => [0x10,0x10,0x10,0x10,0x10,0x1F,0x00],
        'M' => [0x11,0x1B,0x15,0x11,0x11,0x11,0x11],
        'N' => [0x11,0x19,0x15,0x13,0x11,0x11,0x00],
        'O' => [0x0E,0x11,0x11,0x11,0x11,0x0E,0x00],
        'P' => [0x1E,0x11,0x11,0x1E,0x10,0x10,0x10],
        'Q' => [0x0E,0x11,0x11,0x11,0x15,0x0E,0x01],
        'R' => [0x1E,0x11,0x11,0x1E,0x14,0x12,0x11],
        'S' => [0x0F,0x10,0x0E,0x01,0x01,0x1E,0x00],
        'T' => [0x1F,0x04,0x04,0x04,0x04,0x04,0x00],
        'U' => [0x11,0x11,0x11,0x11,0x11,0x0E,0x00],
        'V' => [0x11,0x11,0x11,0x11,0x0A,0x04,0x00],
        'W' => [0x11,0x11,0x11,0x15,0x15,0x0A,0x00],
        'X' => [0x11,0x11,0x0A,0x04,0x0A,0x11,0x11],
        'Y' => [0x11,0x11,0x0A,0x04,0x04,0x04,0x00],
        'Z' => [0x1F,0x01,0x02,0x04,0x08,0x10,0x1F],
        ' ' => [0x00,0x00,0x00,0x00,0x00,0x00,0x00],
        _   => [0x1F,0x1F,0x1F,0x1F,0x1F,0x1F,0x00], // bloque si falta el glifo
    }
}


fn pset(buf: &mut [u32], x: i32, y: i32, color: u32) {
    if x >= 0 && y >= 0 && (x as usize) < WIDTH && (y as usize) < HEIGHT {
        buf[y as usize * WIDTH + x as usize] = color;
    }
}

fn draw_char(buf: &mut [u32], ch: char, x: i32, y: i32, scale: usize, color: u32) {
    let g = glyph5x7(ch.to_ascii_uppercase());
    for (ry, row) in g.iter().enumerate() {
        for rx in 0..5 {
            if ((row >> (4 - rx)) & 1) == 1 {
                for sy in 0..scale {
                    for sx in 0..scale {
                        pset(buf,
                            x + (rx as i32 * scale as i32) + sx as i32,
                            y + (ry as i32 * scale as i32) + sy as i32,
                            color);
                    }
                }
            }
        }
    }
}

pub fn draw_text(buf: &mut [u32], text: &str, x: i32, y: i32, scale: usize, color: u32) {
    let mut cx = x;
    for ch in text.chars() {
        draw_char(buf, ch, cx, y, scale, color);
        cx += ((5 + 1) * scale) as i32; // 1 columna de espacio
    }
}

pub fn draw_text_centered(buf: &mut [u32], text: &str, y: usize, scale: usize, color: u32) {
    let n = text.chars().count();
    let cw = (5 + 1) * scale;
    let total_w = n * cw - scale; // sin el último espacio
    let start_x = (WIDTH.saturating_sub(total_w)) / 2;
    draw_text(buf, text, start_x as i32, y as i32, scale, color);
}

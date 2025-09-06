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
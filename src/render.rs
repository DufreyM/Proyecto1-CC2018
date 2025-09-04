use crate::constants::{WIDTH, HEIGHT, rgb, shade, TILE_EXIT};
use crate::textures::{TextureSet, Texture}; // <-- agrega Texture
use crate::world::WorldMap;
use crate::sprites::SpriteManager;
use std::f64::consts::PI; // <-- para ángulos

/// Dibuja cielo desde textura (top) y piso “lava” (bottom).
/// `dir_x/dir_y` controlan el desplazamiento horizontal del cielo (parallax).
pub fn clear_bg(buf: &mut [u32], sky: &Texture, dir_x: f64, dir_y: f64) {
    let half = HEIGHT / 2;

    // --- CIELO: mapeo panorámico simple con desplazamiento por ángulo ---
    // Ángulo de la dirección de vista -> offset horizontal
    let angle = dir_y.atan2(dir_x); // [-PI, PI]
    let u_off = angle / (2.0 * PI); // [-0.5, 0.5]
    for y in 0..half {
        let v = y as f64 / (half as f64); // [0..1)
        let row = y * WIDTH;
        for x in 0..WIDTH {
            let u = ((x as f64 / WIDTH as f64) + u_off).fract();
            let uu = if u < 0.0 { u + 1.0 } else { u };
            let color = sky.sample(uu, v);
            buf[row + x] = color;
        }
    }

    // --- PISO: degradé “lava” cálida ---
    for y in half..HEIGHT {
        let t = (y - half) as f64 / half as f64;
        let r = (100.0 + 100.0*t) as u8;
        let g = (40.0  +  40.0*t) as u8;
        let b = (30.0  +  10.0*t) as u8;
        let color = rgb(r,g,b);
        let row = y * WIDTH;
        for x in 0..WIDTH { buf[row + x] = color; }
    }
}


pub fn line(buf: &mut [u32], x0: usize, y0: usize, x1: usize, y1: usize, color: u32) {
    let mut x0 = x0 as i32;
    let mut y0 = y0 as i32;
    let x1 = x1 as i32;
    let y1 = y1 as i32;
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    loop {
        if x0>=0 && x0 < WIDTH as i32 && y0>=0 && y0 < HEIGHT as i32 {
            buf[y0 as usize * WIDTH + x0 as usize] = color;
        }
        if x0 == x1 && y0 == y1 { break; }
        let e2 = 2 * err;
        if e2 >= dy { err += dy; x0 += sx; }
        if e2 <= dx { err += dx; y0 += sy; }
    }
}

/// Render principal con raycasting + texturas + “linterna”.
///
/// `light_radius`: radio de luz (en tiles). `ambient`: mínimo de luz [0..1].
pub fn raycast(
    buf: &mut [u32],
    zbuf: &mut [f64],
    map: &WorldMap,
    tex: &TextureSet,
    px: f64, py: f64,
    dir_x: f64, dir_y: f64,
    plane_x: f64, plane_y: f64,
    light_radius: f64,
    ambient: f64,
) {
    for x in 0..WIDTH {
        let camera_x = 2.0 * (x as f64) / (WIDTH as f64) - 1.0;
        let ray_dir_x = dir_x + plane_x * camera_x;
        let ray_dir_y = dir_y + plane_y * camera_x;

        let mut map_x = px as i32;
        let mut map_y = py as i32;

        let delta_dist_x = if ray_dir_x == 0.0 { f64::INFINITY } else { (1.0 / ray_dir_x).abs() };
        let delta_dist_y = if ray_dir_y == 0.0 { f64::INFINITY } else { (1.0 / ray_dir_y).abs() };

        let (step_x, mut side_dist_x) = if ray_dir_x < 0.0 {
            (-1, (px - map_x as f64) * delta_dist_x)
        } else {
            ( 1, (map_x as f64 + 1.0 - px) * delta_dist_x)
        };
        let (step_y, mut side_dist_y) = if ray_dir_y < 0.0 {
            (-1, (py - map_y as f64) * delta_dist_y)
        } else {
            ( 1, (map_y as f64 + 1.0 - py) * delta_dist_y)
        };

        let mut hit_tile = 0;
        let mut side = 0; // 0 = x, 1 = y
        while hit_tile == 0 {
            if side_dist_x < side_dist_y {
                side_dist_x += delta_dist_x; map_x += step_x; side = 0;
            } else {
                side_dist_y += delta_dist_y; map_y += step_y; side = 1;
            }
            let cell = map[map_y as usize][map_x as usize];
            if cell != 0 && cell != TILE_EXIT { hit_tile = cell; }
        }

        let perp_dist = if side == 0 {
            ((map_x as f64 - px) + (1 - step_x) as f64 / 2.0) / ray_dir_x
        } else {
            ((map_y as f64 - py) + (1 - step_y) as f64 / 2.0) / ray_dir_y
        };

        let line_h = ((HEIGHT as f64) / perp_dist) as i32;
        let mut draw_start = -line_h / 2 + HEIGHT as i32 / 2;
        if draw_start < 0 { draw_start = 0; }
        let mut draw_end = line_h / 2 + HEIGHT as i32 / 2;
        if draw_end >= HEIGHT as i32 { draw_end = HEIGHT as i32 - 1; }

        // Coordenada “u” de la pared para texturizar
        let wall_x = if side == 0 {
            py + perp_dist * ray_dir_y
        } else {
            px + perp_dist * ray_dir_x
        }.fract();
        let mut tex_x = (wall_x * tex.wall_fire_a.w as f64) as usize;
        // Corrige orientación
        if side == 0 && ray_dir_x > 0.0 { tex_x = tex.wall_fire_a.w - tex_x - 1; }
        if side == 1 && ray_dir_y < 0.0 { tex_x = tex.wall_fire_a.w - tex_x - 1; }

        // Selección de textura (variedad, pero siempre fuego)
        let use_b = ((map_x + map_y) & 1) == 0;
        let wall_tex = if use_b { &tex.wall_fire_b } else { &tex.wall_fire_a };

        // Paso vertical en V
        let step = wall_tex.h as f64 / line_h.max(1) as f64;
        let mut tex_pos = (draw_start as f64 - HEIGHT as f64 / 2.0 + line_h as f64 / 2.0) * step;

        // Iluminación (“linterna”): distancia radial + sombra de pared lateral
        let dist = perp_dist.max(0.0001);
        let mut base_light = ((light_radius - dist) / light_radius).clamp(0.0, 1.0);
        base_light = base_light.max(ambient); // piso mínimo de luz
        if side == 1 { base_light *= 0.7; }   // sombra lateral
        
        zbuf[x] = perp_dist.max(0.0001);

        for y in draw_start as usize..=draw_end as usize {
            let tex_y = tex_pos as usize & (wall_tex.h - 1);
            tex_pos += step;
            let c = wall_tex.data[tex_y * wall_tex.w + tex_x];
            // aplica luz
            let lit = shade(c, base_light as f64);
            buf[y * WIDTH + x] = lit;
        }
    }
}

pub fn draw_sprites(
    buf: &mut [u32],
    zbuf: &[f64],
    px: f64, py: f64,
    dir_x: f64, dir_y: f64,
    plane_x: f64, plane_y: f64,
    sprites: &SpriteManager,
    ambient: f64,
) {
    use crate::constants::{shade, alpha_blend};

    for s in &sprites.list {
        let sx = s.x - px;
        let sy = s.y - py;

        let inv_det = 1.0 / (plane_x * dir_y - dir_x * plane_y);
        let transform_x = inv_det * ( dir_y * sx - dir_x * sy);
        let transform_y = inv_det * (-plane_y * sx + plane_x * sy);

        if transform_y <= 0.01 { continue; } // detrás del player

        let sprite_screen_x = ((WIDTH as f64) / 2.0 * (1.0 + transform_x / transform_y)) as i32;

        // Tamaño en pantalla
        let sprite_h = (HEIGHT as f64 / transform_y) as i32;
        let mut draw_start_y = -sprite_h / 2 + (HEIGHT as i32)/2;
        let mut draw_end_y   =  sprite_h / 2 + (HEIGHT as i32)/2;
        draw_start_y = draw_start_y.clamp(0, HEIGHT as i32 - 1);
        draw_end_y   = draw_end_y.clamp(0, HEIGHT as i32 - 1);

        let sprite_w = (HEIGHT as f64 / transform_y) as i32;
        let mut draw_start_x = -sprite_w / 2 + sprite_screen_x;
        let mut draw_end_x   =  sprite_w / 2 + sprite_screen_x;
        draw_start_x = draw_start_x.clamp(0, WIDTH as i32 - 1);
        draw_end_x   = draw_end_x.clamp(0, WIDTH as i32 - 1);

        let frame = s.current();
        let dist = (sx*sx + sy*sy).sqrt();
        let mut light = (1.0 / (1.0 + 0.18*dist)).clamp(ambient, 1.0); // luz simple

        for stripe in draw_start_x as usize ..= draw_end_x as usize {
            // Oclusión con paredes
            if transform_y >= zbuf[stripe] { continue; }

            let tex_x = (((stripe as i32 - (-sprite_w/2 + sprite_screen_x)) as f64)
                        * frame.w as f64 / sprite_w as f64) as usize;

            for y in draw_start_y as usize ..= draw_end_y as usize {
                let d = (y as i32 * 256 - (HEIGHT as i32)*128 + sprite_h*128) as f64;
                let tex_y = ((d * frame.h as f64) / (sprite_h as f64 * 256.0)) as usize;

                if tex_x >= frame.w || tex_y >= frame.h { continue; }
                let (rgb_color, alpha) = frame.sample(tex_x as f64 / frame.w as f64,
                                                      tex_y as f64 / frame.h as f64);
                if alpha == 0 { continue; }

                let lit = shade(rgb_color, light);
                let idx = y * WIDTH + stripe;
                let bg  = buf[idx];
                buf[idx] = alpha_blend(bg, lit, alpha);
            }
        }
    }
}
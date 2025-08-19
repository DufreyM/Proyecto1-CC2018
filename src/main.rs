use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

const MAP_W: usize = 24;
const MAP_H: usize = 24;

/// Mapa: 0 = vacío; 1..=4 = tipos de pared (colores poké).
/// Puedes “dibujar” un nivel con pasillos y una salida (celda 9) más adelante.
const WORLD_MAP: [[i32; MAP_W]; MAP_H] = [
    [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,3,3,3,0,0,0,0,0,2,2,2,0,0,0,0,4,4,4,0,0,0,1],
    [1,0,3,0,3,0,0,0,0,0,2,0,2,0,0,0,0,4,0,4,0,0,0,1],
    [1,0,3,3,3,0,0,0,0,0,2,2,2,0,0,0,0,4,4,4,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,1,1,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,1,0,1,0,0,0,0,0,1,0,1,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,1,0,1,0,0,0,0,0,1,0,1,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,1,1,1,0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,9,1], // 9: "salida" (win)
    [1,0,0,0,4,0,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,2,2,2,0,0,0,0,3,3,3,0,0,0,0,0,4,4,4,0,0,0,1],
    [1,0,2,0,2,0,0,0,0,3,0,3,0,0,0,0,0,4,0,4,0,0,0,1],
    [1,0,2,2,2,0,0,0,0,3,3,3,0,0,0,0,0,4,4,4,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
    [1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
];

#[derive(Copy, Clone, PartialEq)]
enum GameState { Menu, Playing, Win }

fn main() {
    let mut window = Window::new(
        "Poke Raycaster - FPS: 0 | Enter para iniciar",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: false,
            scale: minifb::Scale::X1,
            ..WindowOptions::default()
        },
    ).unwrap();

    // Intento de ~60 FPS
    window.limit_update_rate(Some(Duration::from_micros(16_667)));

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    // Posición y cámara inicial (estilo lodev)
    let mut pos_x: f64 = 12.0;
    let mut pos_y: f64 = 12.0;
    let mut dir_x: f64 = -1.0;
    let mut dir_y: f64 = 0.0;
    let mut plane_x: f64 = 0.0;
    let mut plane_y: f64 = 0.66;

    let mut last = Instant::now();
    let mut fps_timer = Instant::now();
    let mut frames = 0u32;
    let mut fps = 0u32;

    let mut state = GameState::Menu;

    while window.is_open() {
        let now = Instant::now();
        let dt = (now - last).as_secs_f64();
        last = now;

        // Input global
        if window.is_key_down(Key::Escape) { break; }

        match state {
            GameState::Menu => {
                draw_menu(&mut buffer);
                if window.is_key_down(Key::Enter) {
                    state = GameState::Playing;
                }
            }
            GameState::Playing => {
                // Movimiento
                let move_speed = 4.0 * dt;   // unidades/seg
                let rot_speed  = 2.5 * dt;   // rad/seg

                // W/S adelantar/retroceder
                if window.is_key_down(Key::W) {
                    try_move(move_speed, &mut pos_x, &mut pos_y, dir_x, dir_y);
                }
                if window.is_key_down(Key::S) {
                    try_move(-move_speed, &mut pos_x, &mut pos_y, dir_x, dir_y);
                }

                // A/D strafe
                let perp_x = -dir_y;
                let perp_y =  dir_x;
                if window.is_key_down(Key::A) {
                    try_move(move_speed, &mut pos_x, &mut pos_y, perp_x, perp_y);
                }
                if window.is_key_down(Key::D) {
                    try_move(move_speed, &mut pos_x, &mut pos_y, -perp_x, -perp_y);
                }

                // Rotación con flechas
                if window.is_key_down(Key::Left) {
                    rotate(-rot_speed, &mut dir_x, &mut dir_y, &mut plane_x, &mut plane_y);
                }
                if window.is_key_down(Key::Right) {
                    rotate( rot_speed, &mut dir_x, &mut dir_y, &mut plane_x, &mut plane_y);
                }

                // Render
                clear_bg(&mut buffer);
                raycast(&mut buffer, pos_x, pos_y, dir_x, dir_y, plane_x, plane_y);
                draw_minimap(&mut buffer, pos_x, pos_y, dir_x, dir_y);

                // Condición de victoria: llegar cerca de la celda (11, 23) marcada con 9
                let goal_x = 23.5;
                let goal_y = 11.5;
                let dx = pos_x - goal_x;
                let dy = pos_y - goal_y;
                if (dx*dx + dy*dy).sqrt() < 0.6 {
                    state = GameState::Win;
                }
            }
            GameState::Win => {
                draw_win(&mut buffer);
                if window.is_key_down(Key::Enter) {
                    // Reiniciar
                    pos_x = 12.0; pos_y = 12.0;
                    dir_x = -1.0; dir_y = 0.0;
                    plane_x = 0.0; plane_y = 0.66;
                    state = GameState::Playing;
                }
            }
        }

        // FPS en el título
        frames += 1;
        if fps_timer.elapsed() >= Duration::from_secs(1) {
            fps = frames;
            frames = 0;
            fps_timer = Instant::now();
        }
        let title = match state {
            GameState::Menu => format!("Poke Raycaster - FPS: {fps} | Enter para iniciar"),
            GameState::Playing => format!("Poke Raycaster - FPS: {fps} | W/A/S/D moverte, ←→ girar"),
            GameState::Win => format!("Poke Raycaster - FPS: {fps} | ¡Ganaste! Enter para reiniciar"),
        };
        window.set_title(&title);

        // Mostrar frame
        if window.update_with_buffer(&buffer, WIDTH, HEIGHT).is_err() {
            break;
        }
    }
}

fn rotate(angle: f64, dir_x: &mut f64, dir_y: &mut f64, plane_x: &mut f64, plane_y: &mut f64) {
    let old_dir_x = *dir_x;
    *dir_x =  *dir_x * angle.cos() - *dir_y * angle.sin();
    *dir_y =  old_dir_x * angle.sin() + *dir_y * angle.cos();

    let old_plane_x = *plane_x;
    *plane_x = *plane_x * angle.cos() - *plane_y * angle.sin();
    *plane_y =  old_plane_x * angle.sin() + *plane_y * angle.cos();
}

fn try_move(speed: f64, pos_x: &mut f64, pos_y: &mut f64, dir_x: f64, dir_y: f64) {
    let nx = *pos_x + dir_x * speed;
    let ny = *pos_y + dir_y * speed;
    // Colisión por eje para suavidad
    if WORLD_MAP[*pos_y as usize][nx as usize] == 0 {
        *pos_x = nx;
    }
    if WORLD_MAP[ny as usize][*pos_x as usize] == 0 {
        *pos_y = ny;
    }
}

fn clear_bg(buf: &mut [u32]) {
    // Cielo y piso simples
    let half = HEIGHT / 2;
    for y in 0..HEIGHT {
        let color = if y < half { rgb(120, 160, 255) } else { rgb(60, 60, 70) };
        let row = y * WIDTH;
        for x in 0..WIDTH {
            buf[row + x] = color;
        }
    }
}

fn raycast(
    buf: &mut [u32],
    pos_x: f64, pos_y: f64,
    dir_x: f64, dir_y: f64,
    plane_x: f64, plane_y: f64,
) {
    for x in 0..WIDTH {
        let camera_x = 2.0 * (x as f64) / (WIDTH as f64) - 1.0;
        let ray_dir_x = dir_x + plane_x * camera_x;
        let ray_dir_y = dir_y + plane_y * camera_x;

        let mut map_x = pos_x as i32;
        let mut map_y = pos_y as i32;

        let delta_dist_x = if ray_dir_x == 0.0 { f64::INFINITY } else { (1.0 / ray_dir_x).abs() };
        let delta_dist_y = if ray_dir_y == 0.0 { f64::INFINITY } else { (1.0 / ray_dir_y).abs() };

        let (mut step_x, mut side_dist_x) = if ray_dir_x < 0.0 {
            (-1, (pos_x - map_x as f64) * delta_dist_x)
        } else {
            (1, (map_x as f64 + 1.0 - pos_x) * delta_dist_x)
        };

        let (mut step_y, mut side_dist_y) = if ray_dir_y < 0.0 {
            (-1, (pos_y - map_y as f64) * delta_dist_y)
        } else {
            (1, (map_y as f64 + 1.0 - pos_y) * delta_dist_y)
        };

        let mut hit = 0;
        let mut side = 0; // 0 = x, 1 = y

        while hit == 0 {
            if side_dist_x < side_dist_y {
                side_dist_x += delta_dist_x;
                map_x += step_x;
                side = 0;
            } else {
                side_dist_y += delta_dist_y;
                map_y += step_y;
                side = 1;
            }
            let cell = WORLD_MAP[map_y as usize][map_x as usize];
            if cell > 0 && cell != 9 { // 9 es “salida”, no cuenta como pared
                hit = cell;
            }
        }

        // Distancia perpendicular para evitar efecto ojo de pez
        let perp_wall_dist = if side == 0 {
            ((map_x as f64 - pos_x) + (1 - step_x) as f64 / 2.0) / ray_dir_x
        } else {
            ((map_y as f64 - pos_y) + (1 - step_y) as f64 / 2.0) / ray_dir_y
        };

        let line_height = ((HEIGHT as f64) / perp_wall_dist) as i32;
        let mut draw_start = -line_height / 2 + HEIGHT as i32 / 2;
        if draw_start < 0 { draw_start = 0; }
        let mut draw_end = line_height / 2 + HEIGHT as i32 / 2;
        if draw_end >= HEIGHT as i32 { draw_end = HEIGHT as i32 - 1; }

        let mut color = match hit {
            1 => rgb(228, 62, 62),   // rojo (fuego)
            2 => rgb(63, 122, 255),  // azul (agua)
            3 => rgb(84, 176, 79),   // verde (planta)
            4 => rgb(247, 206, 70),  // amarillo (eléctrico)
            _ => rgb(200, 200, 200),
        };
        // Sombreado en paredes “laterales” para dar profundidad
        if side == 1 { color = shade(color, 0.6); }

        // Dibujar la columna
        for y in draw_start as usize..=draw_end as usize {
            let idx = y * WIDTH + x;
            buf[idx] = color;
        }
    }
}

fn draw_minimap(buf: &mut [u32], pos_x: f64, pos_y: f64, dir_x: f64, dir_y: f64) {
    const SCALE: usize = 4; // 1 celda = 4x4 píxeles
    let off_x = 10usize;
    let off_y = 10usize;

    // Fondo
    for my in 0..MAP_H {
        for mx in 0..MAP_W {
            let cell = WORLD_MAP[my][mx];
            let base = if cell == 0 { rgb(28, 28, 36) }
            else if cell == 9 { rgb(255, 180, 80) }  // objetivo
            else { rgb(90, 90, 110) };
            for py in 0..SCALE {
                for px in 0..SCALE {
                    let x = off_x + mx * SCALE + px;
                    let y = off_y + my * SCALE + py;
                    if x < WIDTH && y < HEIGHT {
                        buf[y * WIDTH + x] = base;
                    }
                }
            }
        }
    }

    // Jugador
    let px = off_x + (pos_x as usize) * SCALE;
    let py = off_y + (pos_y as usize) * SCALE;
    for dy in 0..2 {
        for dx in 0..2 {
            let x = px + dx;
            let y = py + dy;
            if x < WIDTH && y < HEIGHT {
                buf[y * WIDTH + x] = rgb(255, 255, 255);
            }
        }
    }
    // Dirección (flecha corta)
    let fx = (pos_x + dir_x * 0.8) as usize * SCALE + off_x;
    let fy = (pos_y + dir_y * 0.8) as usize * SCALE + off_y;
    line(buf, px, py, fx, fy, rgb(255, 255, 255));
}

/* === Utilidades de dibujo === */

fn rgb(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

fn shade(color: u32, factor: f64) -> u32 {
    let r = ((color >> 16) & 0xFF) as f64;
    let g = ((color >> 8) & 0xFF) as f64;
    let b = (color & 0xFF) as f64;
    rgb((r * factor) as u8, (g * factor) as u8, (b * factor) as u8)
}

fn draw_menu(buf: &mut [u32]) {
    // Fondo “pokéball” simple
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let cx = x as i32 - (WIDTH as i32 / 2);
            let cy = y as i32 - (HEIGHT as i32 / 2);
            let r2 = cx*cx + cy*cy;
            let radius2 = ((HEIGHT as i32 / 3) * (HEIGHT as i32 / 3)) as i32;
            let color = if r2 < radius2 {
                if y < HEIGHT/2 { rgb(220, 60, 60) } else { rgb(245, 245, 245) }
            } else {
                rgb(15, 15, 20)
            };
            buf[y * WIDTH + x] = color;
        }
    }
    // “Cinta” negra
    let band_h = 10usize;
    let mid = HEIGHT/2;
    for y in (mid - band_h)..(mid + band_h) {
        let row = y * WIDTH;
        for x in 0..WIDTH { buf[row + x] = rgb(10, 10, 12); }
    }
}

fn draw_win(buf: &mut [u32]) {
    for p in buf.iter_mut() { *p = rgb(30, 200, 120); }
    // un borde simple
    for x in 0..WIDTH {
        buf[x] = rgb(255, 255, 255);
        buf[(HEIGHT-1)*WIDTH + x] = rgb(255, 255, 255);
    }
    for y in 0..HEIGHT {
        buf[y*WIDTH] = rgb(255, 255, 255);
        buf[y*WIDTH + (WIDTH-1)] = rgb(255, 255, 255);
    }
}

fn line(buf: &mut [u32], x0: usize, y0: usize, x1: usize, y1: usize, color: u32) {
    // Bresenham sencillo
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
        if x0 >= 0 && (x0 as usize) < WIDTH && y0 >= 0 && (y0 as usize) < HEIGHT {
            buf[y0 as usize * WIDTH + x0 as usize] = color;
        }
        if x0 == x1 && y0 == y1 { break; }
        let e2 = 2 * err;
        if e2 >= dy { err += dy; x0 += sx; }
        if e2 <= dx { err += dx; y0 += sy; }
    }
}

mod constants;
mod world;
mod player;
mod textures;
mod render;
mod hud;

use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};

use constants::{WIDTH, HEIGHT, rgb, TILE_EXIT};
use player::Player;
use world::{gym_fuego};
use textures::TextureSet;

#[derive(Copy, Clone, PartialEq)]
enum GameState { Menu, Playing, Win }

fn main() {
    let mut window = Window::new(
        "Gimnasio Fuego - FPS: 0 | Enter para iniciar",
        WIDTH, HEIGHT,
        WindowOptions { resize: false, scale: minifb::Scale::X1, ..WindowOptions::default() }
    ).unwrap();

    window.limit_update_rate(Some(Duration::from_micros(16_667)));

    let mut buffer = vec![0u32; WIDTH * HEIGHT];
    let mut state = GameState::Menu;

    // Mundo “Fuego”
    let world_map = gym_fuego();

    // Jugador
    let mut p = Player::new();

    // Texturas (se cargan o hacen fallback procedural)
    let textures = TextureSet::load();

    let mut last = Instant::now();
    let mut fps_timer = Instant::now();
    let mut frames = 0u32;
    let mut fps = 0u32;

    while window.is_open() {
        let now = Instant::now();
        let dt = (now - last).as_secs_f64();
        last = now;

        if window.is_key_down(Key::Escape) { break; }

        match state {
            GameState::Menu => {
                draw_menu(&mut buffer);
                if window.is_key_down(Key::Enter) { state = GameState::Playing; }
            }
            GameState::Playing => {
                let move_speed = 4.0 * dt;
                let rot_speed = 2.8 * dt;

                // Movimiento
                if window.is_key_down(Key::W) { p.try_move(move_speed,  p.dir_x,  p.dir_y, &world_map); }
                if window.is_key_down(Key::S) { p.try_move(-move_speed, p.dir_x,  p.dir_y, &world_map); }
                // Strafe
                let perp_x = -p.dir_y; let perp_y = p.dir_x;
                if window.is_key_down(Key::A) { p.try_move(move_speed,  perp_x,  perp_y, &world_map); }
                if window.is_key_down(Key::D) { p.try_move(move_speed, -perp_x, -perp_y, &world_map); }

                // Rotación
                if window.is_key_down(Key::Left)  { p.rotate(-rot_speed); }
                if window.is_key_down(Key::Right) { p.rotate( rot_speed); }

                // Render
                render::clear_bg(&mut buffer);
                // Linterna: radio 8 tiles, luz ambiente 0.15
                render::raycast(&mut buffer, &world_map, &textures, p.x, p.y, p.dir_x, p.dir_y, p.plane_x, p.plane_y, 8.0, 0.15);
                hud::draw_minimap(&mut buffer, &world_map, p.x, p.y, p.dir_x, p.dir_y);

                // Win si pisas la casilla de salida
                let tx = p.x as usize; let ty = p.y as usize;
                if world_map[ty][tx] == TILE_EXIT { state = GameState::Win; }
            }
            GameState::Win => {
                draw_win(&mut buffer);
                if window.is_key_down(Key::Enter) {
                    p = Player::new();
                    state = GameState::Playing;
                }
            }
        }

        // FPS en título
        frames += 1;
        if fps_timer.elapsed() >= Duration::from_secs(1) {
            fps = frames; frames = 0; fps_timer = Instant::now();
        }
        let title = match state {
            GameState::Menu => format!("Gimnasio Fuego - FPS: {fps} | Enter para iniciar"),
            GameState::Playing => format!("Gimnasio Fuego - FPS: {fps} | W/A/S/D moverte, ←→ girar"),
            GameState::Win => format!("Gimnasio Fuego - FPS: {fps} | ¡Ganaste! Enter para reiniciar"),
        };
        window.set_title(&title);

        if window.update_with_buffer(&mut buffer, WIDTH, HEIGHT).is_err() { break; }
    }
}

fn draw_menu(buf: &mut [u32]) {
    // Pokéball simple en fondo oscuro
    use constants::{rgb, WIDTH, HEIGHT};
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let cx = x as i32 - (WIDTH as i32 / 2);
            let cy = y as i32 - (HEIGHT as i32 / 2);
            let r2 = cx*cx + cy*cy;
            let radius2 = (HEIGHT as i32 / 3).pow(2);
            let color = if r2 < radius2 {
                if y < HEIGHT/2 { rgb(220,60,60) } else { rgb(245,245,245) }
            } else { rgb(15,15,20) };
            buf[y * WIDTH + x] = color;
        }
    }
    // cinta negra
    let band_h = 10usize; let mid = HEIGHT/2;
    for y in (mid - band_h)..(mid + band_h) {
        let row = y * WIDTH;
        for x in 0..WIDTH { buf[row + x] = rgb(10,10,12); }
    }
}

fn draw_win(buf: &mut [u32]) {
    use constants::{rgb, WIDTH, HEIGHT};
    for p in buf.iter_mut() { *p = rgb(30,200,120); }
    for x in 0..WIDTH {
        buf[x] = rgb(255,255,255);
        buf[(HEIGHT-1)*WIDTH + x] = rgb(255,255,255);
    }
    for y in 0..HEIGHT {
        buf[y*WIDTH] = rgb(255,255,255);
        buf[y*WIDTH + (WIDTH-1)] = rgb(255,255,255);
    }
}

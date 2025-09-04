mod constants;
mod world;
mod player;
mod textures;
mod render;
mod hud;
mod audio;
mod sprites; // <-- NUEVO

use minifb::{Key, Window, WindowOptions, MouseMode};
use std::time::{Duration, Instant};

use constants::{WIDTH, HEIGHT, TILE_EXIT};
use player::Player;
use world::gym_fuego;
use textures::TextureSet;
use audio::Audio;
use sprites::SpriteManager; // <-- NUEVO

#[derive(Copy, Clone, PartialEq)]
enum GameState { Menu, Playing, Win }
const MOUSE_SENS: f64 = 0.004;

fn main() {
    let mut window = Window::new(
        "Gimnasio Fuego - FPS: 0 | Enter para iniciar",
        WIDTH, HEIGHT,
        WindowOptions { resize: false, scale: minifb::Scale::X1, ..WindowOptions::default() }
    ).unwrap();

    window.limit_update_rate(Some(Duration::from_micros(16_667)));

    let mut buffer = vec![0u32; WIDTH * HEIGHT];
    let mut zbuffer = vec![0.0f64; WIDTH];        // <-- NUEVO
    let mut state = GameState::Menu;

    let world_map = gym_fuego();
    let mut p = Player::new();
    let textures = TextureSet::load();

    let mut audio = Audio::new();
    let mut step_timer: f64 = 0.0;
    let mut last_mouse_x: Option<f32> = None;

    // === SPRITES ===
    let mut sprites = SpriteManager::new_fire_gym(); // pokébola cerca de la meta

    let mut last = Instant::now();
    let mut fps_timer = Instant::now();
    let mut frames = 0u32;
    let mut fps = 0u32;

    while window.is_open() {
        let now = Instant::now();
        let dt = (now - last).as_secs_f64();
        last = now;

        if window.is_key_down(Key::Escape) { break; }

        // Mouse horizontal
        if let Some((mx, _)) = window.get_mouse_pos(MouseMode::Pass) {
            if let Some(prev_x) = last_mouse_x {
                p.rotate((mx - prev_x) as f64 * MOUSE_SENS);
            }
            last_mouse_x = Some(mx);
        }

        match state {
            GameState::Menu => {
                draw_menu(&mut buffer);
                if window.is_key_down(Key::Enter) { state = GameState::Playing; }
            }
            GameState::Playing => {
                let move_speed = 4.0 * dt;
                let rot_speed  = 2.8 * dt;

                // Movimiento + SFX pasos
                let mut moving = false;
                if window.is_key_down(Key::W) { p.try_move(move_speed,  p.dir_x,  p.dir_y, &world_map); moving = true; }
                if window.is_key_down(Key::S) { p.try_move(-move_speed, p.dir_x,  p.dir_y, &world_map); moving = true; }
                let px = -p.dir_y; let py = p.dir_x;
                if window.is_key_down(Key::A) { p.try_move(move_speed,  px,  py, &world_map); moving = true; }
                if window.is_key_down(Key::D) { p.try_move(move_speed, -px, -py, &world_map); moving = true; }

                if window.is_key_down(Key::Left)  { p.rotate(-rot_speed); }
                if window.is_key_down(Key::Right) { p.rotate( rot_speed); }

                if moving {
                    step_timer += dt;
                    if step_timer > 0.38 { audio.play_step(); step_timer = 0.0; }
                } else { step_timer = 0.0; }

                // Update sprites
                sprites.update(dt);

                // Render paredes + zbuffer
                render::clear_bg(&mut buffer, &textures.sky, p.dir_x, p.dir_y);
                render::raycast(&mut buffer, &mut zbuffer, &world_map, &textures,
                                p.x, p.y, p.dir_x, p.dir_y, p.plane_x, p.plane_y,
                                8.0, 0.15);

                // Render sprites (con oclusión)
                render::draw_sprites(&mut buffer, &zbuffer,
                                     p.x, p.y, p.dir_x, p.dir_y, p.plane_x, p.plane_y,
                                     &sprites, 0.20);

                // Minimap
                hud::draw_minimap(&mut buffer, &world_map, p.x, p.y, p.dir_x, p.dir_y);

                // Win
                let tx = p.x as usize; let ty = p.y as usize;
                if world_map[ty][tx] == TILE_EXIT {
                    audio.play_win();
                    state = GameState::Win;
                }
            }
            GameState::Win => {
                draw_win(&mut buffer);
                if window.is_key_down(Key::Enter) { p = Player::new(); state = GameState::Playing; }
            }
        }

        // FPS
        frames += 1;
        if fps_timer.elapsed() >= Duration::from_secs(1) { fps = frames; frames = 0; fps_timer = Instant::now(); }
        let title = match state {
            GameState::Menu => format!("Gimnasio Fuego - FPS: {fps} | Enter para iniciar"),
            GameState::Playing => format!("Gimnasio Fuego - FPS: {fps} | Mouse rotación, W/A/S/D moverte"),
            GameState::Win => format!("Gimnasio Fuego - FPS: {fps} | ¡Ganaste! Enter para reiniciar"),
        };
        window.set_title(&title);

        if window.update_with_buffer(&mut buffer, WIDTH, HEIGHT).is_err() { break; }
    }
}

// draw_menu / draw_win quedan igual

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

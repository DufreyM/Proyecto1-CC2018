mod constants;
mod world;
mod player;
mod textures;
mod render;
mod hud;
mod audio;
mod sprites;

use minifb::{Key, Window, WindowOptions, MouseMode};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use constants::{WIDTH, HEIGHT, MAP_W, MAP_H, TILE_EXIT, TILE_HAZARD, PLAYER_MAX_HP};
use player::Player;
use world::gym_fuego;
use textures::TextureSet;
use audio::Audio;
use sprites::SpriteManager;

use crate::world::gym_agua;

#[derive(Copy, Clone, PartialEq)]
enum GameState { Menu, Playing, Win, Dead }

// NEW: opciones de nivel (puedes renombrar y luego mapearlos a distintos mapas)
const LEVELS: &[&str] = &["FUEGO EASY", "LEVEL RANDOM"];

const MOUSE_SENS: f64 = 0.004;

fn main() {
    let mut window = Window::new(
        "Gimnasio Fuego - FPS: 0 | Enter para iniciar",
        WIDTH, HEIGHT,
        WindowOptions { resize: false, scale: minifb::Scale::X1, ..WindowOptions::default() }
    ).unwrap();

    window.limit_update_rate(Some(Duration::from_micros(16_667)));

    let mut buffer = vec![0u32; WIDTH * HEIGHT];
    let mut zbuffer = vec![0.0f64; WIDTH];
    let mut state = GameState::Menu;

    // NEW: world_map ahora es mutable porque lo cambiaremos según el nivel
    let mut world_map = gym_fuego();
    let mut p = Player::new();
    let mut textures = TextureSet::load();


    // Audio (con toggle mute)
    let mut audio = Audio::new();
    let mut muted = false;
    let mut prev_m_down = false;

    // Timers varios
    let mut step_timer: f64 = 0.0;
    let mut last_mouse_x: Option<f32> = None;

    // Sprites
    let mut sprites = SpriteManager::new_fire_gym();

    // Vida/daño
    let mut damage_flash: f64 = 0.0; // 0..0.5s para overlay
    let mut hazard_tick: f64 = 0.0;  // ticks de lava

    // Overlay de lava (animación)
    let mut lava_phase: f64 = 0.0;

    // FPS
    let mut last = Instant::now();
    let mut fps_timer = Instant::now();
    let mut frames = 0u32;
    let mut fps = 0u32;

    // NEW: estado del menú (selección + debounce)
    let mut selected_level: usize = 0;
    let mut prev_up = false;
    let mut prev_down = false;
    let mut current_level: usize = 0; // para título dinámico y assets por nivel


    while window.is_open() {
        let now = Instant::now();
        let dt = (now - last).as_secs_f64();
        last = now;

         if window.is_key_down(Key::Escape) {
        break; // salir del juego
    }
       if window.is_key_down(Key::Enter) {
    current_level = selected_level;
    world_map = build_level(current_level); // genera el mapa según nivel
    p = Player::new();

    // Sprites por nivel (por ahora usamos los de fuego como default)
    sprites = SpriteManager::new_fire_gym();

    // (Opcional) texturas por nivel:
    // Si luego implementas TextureSet::load_water(), descomenta:
    if current_level == 1 {
        // textures = TextureSet::load_water();
    } else {
        textures = TextureSet::load();
    }

    state = GameState::Playing;
}

        // Mouse horizontal
        if let Some((mx, _)) = window.get_mouse_pos(MouseMode::Pass) {
            if let Some(prev_x) = last_mouse_x {
                p.rotate((mx - prev_x) as f64 * MOUSE_SENS);
            }
            last_mouse_x = Some(mx);
        }

        // Ticks jugador/efectos
        p.tick(dt);
        if damage_flash > 0.0 { damage_flash -= dt; }
        lava_phase += dt;

        // Toggle MUTE (tecla M, con debounce)
        let m_down = window.is_key_down(Key::M);
        if m_down && !prev_m_down {
            muted = audio.toggle_muted();
        }
        prev_m_down = m_down;

        match state {
            GameState::Menu => {
                // NEW: dibuja el menú con niveles
                draw_menu_levels(&mut buffer, selected_level, LEVELS);

                // Navegación ↑ / ↓ con debounce
                let up = window.is_key_down(Key::Up);
                let down = window.is_key_down(Key::Down);

                if up && !prev_up {
                    selected_level = if selected_level == 0 {
                        LEVELS.len() - 1
                    } else {
                        selected_level - 1
                    };
                }
                if down && !prev_down {
                    selected_level = (selected_level + 1) % LEVELS.len();
                }
                prev_up = up;
                prev_down = down;

                // Enter: cargar nivel seleccionado
                // Enter: cargar nivel seleccionado
if window.is_key_down(Key::Enter) {
    current_level = selected_level;

    // Mapa por nivel (AGUA ya crea un layout random en build_level)
    world_map = build_level(current_level);

    // Reset jugador/sprites
    p = Player::new();
    sprites = SpriteManager::new_fire_gym(); // cámbialo si tienes sprites por nivel

    // Texturas por nivel (azules para AGUA)
    textures = if current_level == 1 {
        // Requiere textures::TextureSet::load_water()
        TextureSet::load_water()
    } else {
        TextureSet::load()
    };

    state = GameState::Playing;
}

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

                // Daño por hazard (lava)
                let tx = p.x as usize; let ty = p.y as usize;
                if world_map[ty][tx] == TILE_HAZARD {
                    hazard_tick += dt;
                    if hazard_tick >= 0.5 {
                        p.damage(12);
                        damage_flash = 0.5;
                        hazard_tick = 0.0;
                    }
                } else {
                    hazard_tick = 0.0;
                }

                // Muerte
                if p.hp <= 0 {
                    state = GameState::Dead;
                }

                // Sprites
                sprites.update(dt);

                // Render 3D
                render::clear_bg(&mut buffer, &textures.sky, p.dir_x, p.dir_y);
                render::floorcast(&mut buffer, &world_map,
                  p.x, p.y, p.dir_x, p.dir_y, p.plane_x, p.plane_y,
                  lava_phase);
                render::raycast(&mut buffer, &mut zbuffer, &world_map, &textures,
                                p.x, p.y, p.dir_x, p.dir_y, p.plane_x, p.plane_y,
                                8.0, 0.15);

                render::draw_sprites(&mut buffer, &zbuffer,
                                     p.x, p.y, p.dir_x, p.dir_y, p.plane_x, p.plane_y,
                                     &sprites, 0.20);

                // HUD
                hud::draw_minimap(&mut buffer, &world_map, p.x, p.y, p.dir_x, p.dir_y);
                hud::draw_health_bar(&mut buffer, p.hp, PLAYER_MAX_HP);
                render::draw_damage_overlay(&mut buffer, (damage_flash / 0.5) as f32);

                // Win
                if world_map[ty][tx] == TILE_EXIT {
                    audio.play_win();
                    state = GameState::Win;
                }
            }
            GameState::Win => {
                draw_win(&mut buffer);
                if window.is_key_down(Key::Enter) {
                    p = Player::new();
                    state = GameState::Playing;
                }
            }
            GameState::Dead => {
                draw_dead(&mut buffer);
                if window.is_key_down(Key::Enter) {
                    p = Player::new();
                    state = GameState::Playing;
                }
            }
        }

        // FPS + estado de mute en el título
        frames += 1;
        if fps_timer.elapsed() >= Duration::from_secs(1) { fps = frames; frames = 0; fps_timer = Instant::now(); }
        let mute_tag = if muted { " [MUTE]" } else { "" };
let title = match state {
    GameState::Menu => {
        // Muestra el nivel seleccionado en el título
        let sel = LEVELS[selected_level];
        format!("Gimnasio {sel} - FPS: {fps}{mute_tag} | ↑/↓ elegir nivel • Enter jugar")
    }
    GameState::Playing => {
        let name = LEVELS[current_level];
        format!("Gimnasio {name} - FPS: {fps}{mute_tag} | Mouse rotación, W/A/S/D moverte")
    }
    GameState::Win => {
        let name = LEVELS[current_level];
        format!("Gimnasio {name} - FPS: {fps}{mute_tag} | ¡Ganaste! Enter para reiniciar")
    }
    GameState::Dead => {
        let name = LEVELS[current_level];
        format!("Gimnasio {name} - FPS: {fps}{mute_tag} | ¡Derrotado! Enter para reintentar")
    }
};
        window.set_title(&title);

        if window.update_with_buffer(&mut buffer, WIDTH, HEIGHT).is_err() { break; }
    }
}

// ========= Helpers =========

// NEW: construye el mapa según el índice del nivel.
// Por ahora todos usan el mismo; cuando tengas más, cambia aquí.
fn build_level(idx: usize) -> [[i32; MAP_W]; MAP_H] {
    match idx {
        // FUEGO
        0 => gym_fuego(),

        // AGUA (random seed)
        1 => {
            let seed = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64;
            gym_agua(seed)
        }

        // PLANTA (placeholder: reusa fuego hasta que tengas su mapa)
        2 => gym_fuego(),

        _ => gym_fuego(),
    }
}

#[inline]
fn lerp_u8(a: u8, b: u8, t: f32) -> u8 {
    let t = t.clamp(0.0, 1.0);
    (a as f32 + (b as f32 - a as f32) * t).round().clamp(0.0, 255.0) as u8
}

#[inline]
fn blend_rgb(bg: u32, fg: u32, alpha: f32) -> u32 {
    use constants::rgb;
    let a = alpha.clamp(0.0, 1.0);
    let inv = 1.0 - a;
    let br = ((bg >> 16) & 0xFF) as f32;
    let bgc= ((bg >>  8) & 0xFF) as f32;
    let bb = ( bg        & 0xFF) as f32;
    let fr = ((fg >> 16) & 0xFF) as f32;
    let fgc= ((fg >>  8) & 0xFF) as f32;
    let fb = ( fg        & 0xFF) as f32;
    rgb((br*inv + fr*a) as u8, (bgc*inv + fgc*a) as u8, (bb*inv + fb*a) as u8)
}

fn draw_menu_levels(buf: &mut [u32], selected: usize, options: &[&str]) {
    use constants::{rgb, WIDTH, HEIGHT};

    // ------------------ Fondo: LAVA procedural ------------------
    let w = WIDTH as i32; let h = HEIGHT as i32;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let fx = x as f32 / WIDTH as f32;
            let fy = y as f32 / HEIGHT as f32;

            // Ondas entrecruzadas (estático; si quieres animar, suma una fase global)
            let s1 = (fx * 11.0 + (fy * 4.0).sin() * 0.7).sin();
            let s2 = (fy * 9.0  + (fx * 5.0).cos() * 0.6).cos();
            let s3 = ((fx * 3.0 + fy * 6.0).sin()) * 0.5;
            let v = ((s1 + s2 + s3) * 0.28 + 0.55).clamp(0.0, 1.0);

            let deep  = (110u8,  20u8,  10u8); // rojo oscuro
            let mid   = (210u8,  60u8,  20u8); // naranja
            let light = (255u8, 200u8,  60u8); // amarillo

            // 2 pasos de paleta para más contraste
            let (r, g, b) = if v < 0.6 {
                let t = (v / 0.6).clamp(0.0, 1.0);
                (lerp_u8(deep.0, mid.0, t), lerp_u8(deep.1, mid.1, t), lerp_u8(deep.2, mid.2, t))
            } else {
                let t = ((v - 0.6) / 0.4).clamp(0.0, 1.0);
                (lerp_u8(mid.0, light.0, t), lerp_u8(mid.1, light.1, t), lerp_u8(mid.2, light.2, t))
            };

            // Vignette (oscurece bordes para que el texto destaque)
            let cx = x as i32 - (w / 2);
            let cy = y as i32 - (h / 2);
            let d  = ((cx*cx + cy*cy) as f32).sqrt() / ((w.min(h)) as f32 / 2.0);
            let dark = (d * 0.55).clamp(0.0, 0.55); // 0..0.55
            let base = rgb(r, g, b);
            let final_px = blend_rgb(base, rgb(8, 6, 6), dark);

            buf[y * WIDTH + x] = final_px;
        }
    }

    // ------------------ Panel oscuro para texto ------------------
    let title_scale = (HEIGHT / 70).max(3);
    let opt_scale   = (HEIGHT / 90).max(3);

    let total = options.len() as i32;
    let start_y = (HEIGHT as i32 / 2) - ((total * (8*opt_scale as i32)) / 2);
    let pad_y = 10i32.max((6 * opt_scale as i32) / 5);
    let panel_y0 = (start_y - pad_y).clamp(0, HEIGHT as i32 - 1) as usize;
    let panel_y1 = (start_y + total * (8 * opt_scale as i32) + pad_y)
        .clamp(0, HEIGHT as i32 - 1) as usize;
    let panel_x0 = (WIDTH as i32 / 8) as usize;
    let panel_x1 = (WIDTH as i32 * 7 / 8) as usize;

    for y in panel_y0..panel_y1 {
        let row = y * WIDTH;
        for x in panel_x0..panel_x1 {
            let bg = buf[row + x];
            buf[row + x] = blend_rgb(bg, rgb(0, 0, 0), 0.40); // 40% negro encima
        }
    }

    // ------------------ Título con sombra ------------------
    let title_y = HEIGHT/6;
    hud::draw_text_centered(buf, "SELECCIONA NIVEL", title_y + 2, title_scale, rgb(0,0,0)); // sombra
    hud::draw_text_centered(buf, "SELECCIONA NIVEL", title_y,     title_scale, rgb(255,255,255));

    // ------------------ Opciones (sombra + resaltado) ------------------
    for (i, name) in options.iter().enumerate() {
        let y = (start_y + i as i32 * (8 * opt_scale as i32)) as usize;
        let (col, col_shadow) = if i == selected {
            (rgb(255,230,120), rgb(20,12,0)) // texto dorado + sombra cálida
        } else {
            (rgb(235,235,235), rgb(0,0,0))   // blanco suave + sombra negra
        };

        // Sombra 1px hacia abajo
        hud::draw_text_centered(buf, &format!("{}",
            if i == selected { format!("> {} <", name) } else { name.to_string() }
        ), y + 2, opt_scale, col_shadow);

        // Texto principal
        hud::draw_text_centered(buf, &format!("{}",
            if i == selected { format!("> {} <", name) } else { name.to_string() }
        ), y, opt_scale, col);
    }

    // ------------------ Pie de ayuda (sombra + texto) ------------------
    let hint_scale = (HEIGHT / 110).max(2);
    let hint_y = (HEIGHT*5)/6;
    hud::draw_text_centered(buf, "↑/↓ ELEGIR  •  ENTER JUGAR  •  ESC SALIR", hint_y + 2, hint_scale, rgb(0,0,0));
    hud::draw_text_centered(buf, "↑/↓ ELEGIR  •  ENTER JUGAR  •  ESC SALIR", hint_y,     hint_scale, rgb(245,245,245));
}

// ======= Pantallas de victoria y derrota ya existentes =======

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
    let scale = (HEIGHT / 80).max(3);
    let y_main = HEIGHT/2 - (7*scale)/2;
    hud::draw_text_centered(buf, "GANASTE", y_main, scale, rgb(0,0,0));
    let y_sub = y_main + (7*scale) + (6*scale/5);
    hud::draw_text_centered(buf, "ENTER PARA REINICIAR", y_sub, scale.saturating_sub(1).max(2), rgb(0,0,0));
}

fn draw_dead(buf: &mut [u32]) {
    use constants::{rgb, WIDTH, HEIGHT};
    for px in buf.iter_mut() { *px = rgb(80, 0, 0); }
    for x in 0..WIDTH {
        buf[x] = rgb(255,255,255);
        buf[(HEIGHT-1)*WIDTH + x] = rgb(255,255,255);
    }
    for y in 0..HEIGHT {
        buf[y*WIDTH] = rgb(255,255,255);
        buf[y*WIDTH + (WIDTH-1)] = rgb(255,255,255);
    }
    let scale = (HEIGHT / 80).max(3);
    let y_main = HEIGHT/2 - (7*scale)/2;
    hud::draw_text_centered(buf, "PERDISTE BUH", y_main, scale, rgb(255,255,255));
}

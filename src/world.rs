use crate::constants::{MAP_H, MAP_W, TILE_EMPTY, TILE_EXIT, TILE_WALL, TILE_HAZARD};

pub type WorldMap = [[i32; MAP_W]; MAP_H];

pub fn gym_fuego() -> WorldMap {
    let mut m = [[TILE_WALL; MAP_W]; MAP_H];

    let w = MAP_W as i32;
    let h = MAP_H as i32;

    // ===================== 1) Interior vacío (borde = pared) =====================
    for y in 1..(MAP_H - 1) {
        for x in 1..(MAP_W - 1) {
            m[y][x] = TILE_EMPTY;
        }
    }

    // ===================== 2) Pasillos serpenteantes (paredes verticales con huecos alternados) =====================
    // Cada 3 columnas coloco una pared vertical, abriendo un hueco que alterna en filas.
    // Esto obliga a zig-zaguear y evita "caminar recto".
    let col_start = 4;
    let col_end = (w - 5).max(col_start + 1);
    let gap_band_top = 3i32;             // margen superior donde podrían ir huecos
    let gap_band_bot = (h - 4).max(4);   // margen inferior

    let mut toggle = false;
    let gap_step = 4.max((h / 6) as i32); // distancia vertical entre huecos
    for cx in (col_start..col_end).step_by(3) {
        for y in 2..(MAP_H - 2) {
            m[y][cx as usize] = TILE_WALL;
        }
        // abrir 2 huecos por columna, alternando su posición vertical
        let mut gy = if toggle { gap_band_top + 2 } else { gap_band_top + gap_step / 2 };
        for _ in 0..2 {
            let gyi = gy.clamp(2, MAP_H as i32 - 3) as usize;
            m[gyi][cx as usize] = TILE_EMPTY;
            m[gyi + 1][cx as usize] = TILE_EMPTY; // hueco de 2 celdas para que sea “usable”
            gy += gap_step;
        }
        toggle = !toggle;
    }

    // ===================== 3) Charcos/rieles de lava alternados en los pasillos =====================
    // En cada corredor entre muros verticales, coloco lava como damero pero dejando "islas" seguras.
    for y in 2..(MAP_H - 2) {
        for x in 2..(MAP_W - 2) {
            let vx = x as i32;
            // Evita poner lava sobre paredes; usa alternancia para crear patrón.
            if m[y][x] == TILE_EMPTY && ((vx + y as i32) % 3 == 0) {
                // deja “pisos” seguros: cada 5 casillas horizontales, una queda libre
                if (vx % 5) != 0 {
                    m[y][x] = TILE_HAZARD;
                }
            }
        }
    }

    // ===================== 4) Puentes seguros mínimos =====================
    // Añade pequeñas líneas horizontales de 2 celdas vacías cada cierto tramo para que siempre haya ruta.
    for (i, by) in (3..(MAP_H - 3)).step_by(5).enumerate() {
        let bx = (4 + (i as i32 * 6)).min(MAP_W as i32 - 6) as usize;
        m[by][bx] = TILE_EMPTY;
        m[by][bx + 1] = TILE_EMPTY;
    }

    // ===================== 5) Antesala de la meta con foso de lava =====================
    // Un rectángulo pequeño con un único cuello de botella para entrar a la sala final.
    let goal_room_w = 6i32;
    let goal_room_h = 5i32;
    let room_x0 = (w - goal_room_w - 2).max(6);
    let room_y0 = (h / 2 - goal_room_h / 2).clamp(3, h - goal_room_h - 3);

    // Paredes de la sala final
    for x in room_x0..(room_x0 + goal_room_w) {
        m[room_y0 as usize][x as usize] = TILE_WALL;
        m[(room_y0 + goal_room_h - 1) as usize][x as usize] = TILE_WALL;
    }
    for y in room_y0..(room_y0 + goal_room_h) {
        m[y as usize][room_x0 as usize] = TILE_WALL;
        m[y as usize][(room_x0 + goal_room_w - 1) as usize] = TILE_WALL;
    }

    // Foso de lava frente a la puerta (pared izquierda de la sala)
    let moat_y = (room_y0 + goal_room_h / 2) as usize;
    for x in (room_x0 - 4)..(room_x0 - 1) {
        let xi = x as usize;
        m[moat_y - 1][xi] = TILE_HAZARD;
        m[moat_y][xi] = TILE_HAZARD;
        m[moat_y + 1][xi] = TILE_HAZARD;
    }
    // Un único "puente" de entrada al centro del foso
    m[moat_y][(room_x0 - 2) as usize] = TILE_EMPTY;

    // “Puerta” a la sala final: abre un hueco exacto en la pared izquierda
    m[moat_y][room_x0 as usize] = TILE_EMPTY;

    // ===================== 6) Colocación de la meta y entorno castigador =====================
    // Meta centrada en la sala y rodeada de lava (menos la casilla justo anterior)
    let exit_x = (room_x0 + goal_room_w / 2) as usize;
    let exit_y = (room_y0 + goal_room_h / 2) as usize;

    // Limpia el interior de la sala final (por si algún patrón anterior la tocó)
    for y in (room_y0 + 1) as usize..((room_y0 + goal_room_h - 1) as usize) {
        for x in (room_x0 + 1) as usize..((room_x0 + goal_room_w - 1) as usize) {
            m[y][x] = TILE_EMPTY;
        }
    }

    // Rodea la meta con lava, dejando una celda de acceso desde la “puerta”
    for dy in [-1i32, 0, 1] {
        for dx in [-1i32, 0, 1] {
            if dy == 0 && dx == 0 { continue; }
            let ny = (exit_y as i32 + dy) as usize;
            let nx = (exit_x as i32 + dx) as usize;
            m[ny][nx] = TILE_HAZARD;
        }
    }
    // Canal de entrada (limpio) alineado con la puerta
    m[exit_y][exit_x - 1] = TILE_EMPTY;

    // Meta: aquí debes dibujar la pokebola en tu render justo en este bloque
    m[exit_y][exit_x] = TILE_EXIT;

    m
}

#[inline]
pub fn is_passable(tile: i32) -> bool {
    // La lava es transitable (hace daño), paredes no. La salida también lo es.
    tile == TILE_EMPTY || tile == TILE_EXIT || tile == TILE_HAZARD
}

#[inline]
fn rng_next(s: &mut u64) -> u64 {
    // xorshift64* (rápido, sin dependencias)
    *s ^= *s >> 12;
    *s ^= *s << 25;
    *s ^= *s >> 27;
    s.wrapping_mul(2685821657736338717)
}
#[inline]
fn rand_range(s: &mut u64, max: i32) -> i32 {
    (rng_next(s) % (max as u64)) as i32
}
#[inline]
fn chance(s: &mut u64, num: u32, den: u32) -> bool {
    (rng_next(s) % (den as u64)) < (num as u64)
}

pub fn gym_agua(seed: u64) -> WorldMap {
    let mut m = [[TILE_WALL; MAP_W]; MAP_H];

    let w = MAP_W as i32;
    let h = MAP_H as i32;

    // 1) Interior vacío (dejamos borde como pared)
    for y in 1..(MAP_H - 1) {
        for x in 1..(MAP_W - 1) {
            m[y][x] = TILE_EMPTY;
        }
    }

    // RNG (evita 0)
    let mut s = if seed == 0 { 0xA2C5_9D3F_F00D_BABE } else { seed } | 1;

    // 2) Pasillo seguro (garantiza ruta de izquierda a derecha)
    let rpath_y = 2 + rand_range(&mut s, h - 4).clamp(0, h - 5);
    for x in 2..(w - 6) {
        m[rpath_y as usize][x as usize] = TILE_EMPTY;
        if chance(&mut s, 1, 3) && rpath_y + 1 < h - 2 {
            m[(rpath_y + 1) as usize][x as usize] = TILE_EMPTY; // un poquito de grosor
        }
    }

    // 3) Columnas de pared aleatorias con huecos; abrir sobre el pasillo seguro
    for cx in 4..(w - 4) {
        if chance(&mut s, 1, 3) { // ~33% columnas
            for y in 2..(h - 2) {
                m[y as usize][cx as usize] = TILE_WALL;
            }
            // 2–3 huecos random
            let gaps = 2 + rand_range(&mut s, 2); // 2 o 3
            for _ in 0..gaps {
                let gy = 2 + rand_range(&mut s, h - 4);
                m[gy as usize][cx as usize] = TILE_EMPTY;
                if gy + 1 < h - 2 { m[(gy + 1) as usize][cx as usize] = TILE_EMPTY; }
            }
            // Asegurar paso por el rpath_y
            m[rpath_y as usize][cx as usize] = TILE_EMPTY;
            if rpath_y + 1 < h - 2 { m[(rpath_y + 1) as usize][cx as usize] = TILE_EMPTY; }
        }
    }

    // 4) “Corrientes” y charcos de agua (hazard) – azul
    for y in 2..(MAP_H - 2) {
        for x in 2..(MAP_W - 2) {
            if m[y][x] == TILE_EMPTY
                && y != rpath_y as usize
                && ((x + y) % 5 != 0)            // deja islitas
                && chance(&mut s, 1, 4)          // ~25% prob.
            {
                m[y][x] = TILE_HAZARD;           // agua que te daña
            }
        }
    }
    // Corrientes horizontales extra + puentes
    for _ in 0..3 {
        let yy = 3 + rand_range(&mut s, h - 6);
        for x in 4..(w - 4) {
            if m[yy as usize][x as usize] == TILE_EMPTY && chance(&mut s, 3, 4) {
                m[yy as usize][x as usize] = TILE_HAZARD;
            }
        }
        // puentes
        for _ in 0..2 {
            let bx = 4 + rand_range(&mut s, w - 8);
            m[yy as usize][bx as usize] = TILE_EMPTY;
            if bx + 1 < w - 2 { m[yy as usize][(bx + 1) as usize] = TILE_EMPTY; }
        }
    }

    // 5) Sala final a la derecha con foso de agua alineado al pasillo
    let goal_room_w = 6i32;
    let goal_room_h = 5i32;
    let room_x0 = (w - goal_room_w - 2).max(6);
    let room_y0 = (rpath_y - goal_room_h / 2).clamp(3, h - goal_room_h - 3);

    for x in room_x0..(room_x0 + goal_room_w) {
        m[room_y0 as usize][x as usize] = TILE_WALL;
        m[(room_y0 + goal_room_h - 1) as usize][x as usize] = TILE_WALL;
    }
    for y in room_y0..(room_y0 + goal_room_h) {
        m[y as usize][room_x0 as usize] = TILE_WALL;
        m[y as usize][(room_x0 + goal_room_w - 1) as usize] = TILE_WALL;
    }

    // Foso de agua frente a la puerta, con un puente alineado al pasillo
    let moat_y = rpath_y as usize;
    for x in (room_x0 - 4)..(room_x0 - 1) {
        let xi = x as usize;
        for dy in -1..=1 {
            let yy = (moat_y as i32 + dy) as usize;
            m[yy][xi] = TILE_HAZARD;
        }
    }
    m[moat_y][(room_x0 - 2) as usize] = TILE_EMPTY;  // puente
    m[moat_y][room_x0 as usize] = TILE_EMPTY;        // puerta

    // Limpia interior
    for y in (room_y0 + 1) as usize..((room_y0 + goal_room_h - 1) as usize) {
        for x in (room_x0 + 1) as usize..((room_x0 + goal_room_w - 1) as usize) {
            m[y][x] = TILE_EMPTY;
        }
    }

    // 6) Salida en islita de agua (ligeramente aleatoria)
    let exit_x = (room_x0 + goal_room_w / 2 + (rand_range(&mut s, 3) - 1))
        .clamp(room_x0 + 1, room_x0 + goal_room_w - 2) as usize;
    let exit_y = (room_y0 + goal_room_h / 2 + (rand_range(&mut s, 3) - 1))
        .clamp(room_y0 + 1, room_y0 + goal_room_h - 2) as usize;

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 { continue; }
            m[(exit_y as i32 + dy) as usize][(exit_x as i32 + dx) as usize] = TILE_HAZARD;
        }
    }
    if exit_x > 0 { m[exit_y][exit_x - 1] = TILE_EMPTY; } // canal de acceso
    m[exit_y][exit_x] = TILE_EXIT;

    m
}

// (Opcional) atajo determinista si no pasas seed:
// pub fn gym_agua_default() -> WorldMap { gym_agua(0xA2C59D3FF00DBABE) }

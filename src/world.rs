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

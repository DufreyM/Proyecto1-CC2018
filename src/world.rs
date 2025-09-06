use crate::constants::{MAP_H, MAP_W, TILE_EMPTY, TILE_EXIT, TILE_WALL, TILE_HAZARD};

pub type WorldMap = [[i32; MAP_W]; MAP_H];

pub fn gym_fuego() -> WorldMap {
    // Todo “Gimnasio Fuego”: solo paredes tipo fuego (1), lava (7) y salida (9)
    let mut m = [[TILE_WALL; MAP_W]; MAP_H];

    // Limpia interior
    for y in 1..MAP_H - 1 {
        for x in 1..MAP_W - 1 {
            m[y][x] = TILE_EMPTY;
        }
    }

    // “Arena” con pasillos y sala final (rectángulo de paredes)
    for x in 3..MAP_W - 3 {
        m[5][x] = TILE_WALL;
        m[MAP_H - 6][x] = TILE_WALL;
    }
    for y in 6..MAP_H - 6 {
        m[y][3] = TILE_WALL;
        m[y][MAP_W - 4] = TILE_WALL;
    }

    // Obstáculos internos (llamas/columnas)
    for x in (6..MAP_W - 6).step_by(3) {
        m[9][x] = TILE_WALL;
        m[14][x] = TILE_WALL;
    }

    // Río de lava (transitable, hace daño)
    for y in 8..16 {
        for x in 7..17 {
            if (x + y) % 2 == 0 {
                m[y][x] = TILE_HAZARD; // mosaicos alternados de lava (7)
            }
        }
    }

    // Sala final y salida (abre un “portal” en el muro derecho)
    for x in 18..22 {
        for y in 9..13 {
            m[y][x] = TILE_EMPTY;
        }
    }
    m[11][22] = TILE_EXIT; // meta (x=22, y=11)

    m
}

#[inline]
pub fn is_passable(tile: i32) -> bool {
    // 🔴 Clave: la lava cuenta como transitable (te hace daño, pero puedes pasar)
    tile == TILE_EMPTY || tile == TILE_EXIT || tile == TILE_HAZARD
}

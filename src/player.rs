use crate::constants::PLAYER_MAX_HP;
use crate::constants::{MAP_H, MAP_W};
use crate::world::{is_passable, WorldMap};

pub struct Player {
    pub x: f64, pub y: f64,
    pub dir_x: f64, pub dir_y: f64,
    pub plane_x: f64, pub plane_y: f64,
    pub hp: i32,
    pub invuln: f64,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: 12.0, y: 12.0,
            dir_x: -1.0, dir_y: 0.0,
            plane_x: 0.0, plane_y: 0.66,
            hp: PLAYER_MAX_HP,
            invuln: 0.0,
        }
    }

    pub fn rotate(&mut self, angle: f64) {
        let old_dir_x = self.dir_x;
        self.dir_x =  self.dir_x * angle.cos() - self.dir_y * angle.sin();
        self.dir_y =  old_dir_x   * angle.sin() + self.dir_y * angle.cos();

        let old_plane_x = self.plane_x;
        self.plane_x = self.plane_x * angle.cos() - self.plane_y * angle.sin();
        self.plane_y =  old_plane_x * angle.sin() + self.plane_y * angle.cos();
    }

    pub fn try_move(&mut self, speed: f64, dx: f64, dy: f64, map: &WorldMap) {
        let nx = self.x + dx * speed;
        let ny = self.y + dy * speed;

        // Colisión por eje (suave)
        if nx >= 0.0 && nx < (MAP_W as f64) {
            let tile = map[self.y as usize][nx as usize];
            if is_passable(tile) { self.x = nx; }
        }
        if ny >= 0.0 && ny < (MAP_H as f64) {
            let tile = map[ny as usize][self.x as usize];
            if is_passable(tile) { self.y = ny; }
        }
    }

    pub fn tick(&mut self, dt: f64) {
        if self.invuln > 0.0 { self.invuln -= dt; }
    }
    pub fn damage(&mut self, amount: i32) {
        if self.invuln <= 0.0 {
            self.hp = (self.hp - amount).max(0);
            self.invuln = 0.6; // 600 ms de i-frames
        }
    }
}

use crate::constants::{rgb, alpha_blend};
use image::GenericImageView;

pub struct SpriteFrame {
    pub w: usize,
    pub h: usize,
    pub rgba: Vec<u32>,   // 0xAARRGGBB
}
impl SpriteFrame {
    #[inline]
    pub fn sample(&self, u: f64, v: f64) -> (u32, u8) {
        // u,v en [0,1)
        let uu = (u.fract() + 1.0).fract();
        let vv = (v.fract() + 1.0).fract();
        let x = (uu * self.w as f64) as usize;
        let y = (vv * self.h as f64) as usize;
        let px = self.rgba[y * self.w + x];
        let a  = ((px >> 24) & 0xFF) as u8;
        (px & 0x00FF_FFFF, a) // color RGB, alpha separado
    }
}

fn load_frame_rgba(path: &str) -> Option<SpriteFrame> {
    if let Ok(img) = image::open(path) {
        let img = img.to_rgba8();
        let (w, h) = img.dimensions();
        let mut data = Vec::with_capacity((w*h) as usize);
        for p in img.pixels() {
            let a = p[3] as u32;
            let r = p[0] as u32;
            let g = p[1] as u32;
            let b = p[2] as u32;
            data.push((a << 24) | (r << 16) | (g << 8) | b);
        }
        Some(SpriteFrame { w: w as usize, h: h as usize, rgba: data })
    } else { None }
}

// Fallback: genera 4 frames de una pokébola simple (círculo con banda negra).
fn gen_pokeball_frames(size: usize) -> Vec<SpriteFrame> {
    let mut frames = Vec::new();
    for f in 0..4 {
        let mut rgba = vec![0u32; size*size];
        let cx = (size as i32)/2;
        let cy = (size as i32)/2;
        let r  = (size as i32)/2 - 1;
        // “brillo” girando
        let highlight = ((f as i32) * 2) % (r.max(1));

        for y in 0..size as i32 {
            for x in 0..size as i32 {
                let dx = x - cx;
                let dy = y - cy;
                let d2 = dx*dx + dy*dy;
                let inside = d2 <= r*r;
                let idx = (y as usize)*size + x as usize;
                if !inside {
                    rgba[idx] = 0x0000_0000; // alpha 0
                    continue;
                }
                // top rojo, bottom blanco, banda negra central
                let mut col = if y < cy { rgb(220,60,60) } else { rgb(245,245,245) };
                if (y - cy).abs() <= (size as i32/16) { col = rgb(10,10,12); }
                // botón central
                if dx*dx + (dy/2)*(dy/2) <= (size as i32/16)*(size as i32/16) {
                    col = rgb(245,245,245);
                }
                // brillo móvil (resalta borde superior-izquierdo)
                if inside && (dx*dx + (dy - (r - highlight)).pow(2)) <= (size as i32/9).pow(2) && y < cy {
                    col = rgb(255,230,230);
                }
                rgba[idx] = 0xFF00_0000 | col; // alpha 255
            }
        }
        frames.push(SpriteFrame { w: size, h: size, rgba });
    }
    frames
}

pub struct AnimatedSprite {
    pub x: f64,
    pub y: f64,
    pub frames: Vec<SpriteFrame>,
    pub fps: f64,
    t: f64,
}

impl AnimatedSprite {
    pub fn new(x: f64, y: f64, frames: Vec<SpriteFrame>, fps: f64) -> Self {
        Self { x, y, frames, fps, t: 0.0 }
    }
    pub fn update(&mut self, dt: f64) {
        self.t = (self.t + dt * self.fps) % (self.frames.len() as f64);
    }
    pub fn current(&self) -> &SpriteFrame {
        let i = self.t.floor() as usize % self.frames.len();
        &self.frames[i]
    }
}

pub struct SpriteManager {
    pub list: Vec<AnimatedSprite>,
}
impl SpriteManager {
    pub fn new_fire_gym() -> Self {
        // Intenta cargar pokébola desde assets; si no, genera procedural.
        let mut frames = Vec::new();
        for i in 0..4 {
            let path = format!("assets/pokeball_{}.png", i);
            if let Some(fr) = load_frame_rgba(&path) { frames.push(fr); }
        }
        if frames.is_empty() {
            frames = gen_pokeball_frames(96);
        }
        // Colócala cerca de la meta (x≈21.5, y≈11.5)
        let pokeball = AnimatedSprite::new(21.5, 11.5, frames, 6.0);
        Self { list: vec![pokeball] }
    }

    pub fn update(&mut self, dt: f64) {
        for s in &mut self.list { s.update(dt); }
    }
}

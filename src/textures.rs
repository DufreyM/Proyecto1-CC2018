use crate::constants::{rgb};

pub struct Texture {
    pub w: usize,
    pub h: usize,
    pub data: Vec<u32>, // 0xRRGGBB
}

impl Texture {
    pub fn sample(&self, u: f64, v: f64) -> u32 {
        // u, v en [0,1)
        let mut uu = (u.fract() + 1.0).fract();
        let mut vv = (v.fract() + 1.0).fract();
        // clamp suave por seguridad
        if uu < 0.0 { uu = 0.0 } ; if uu > 0.999_999 { uu = 0.999_999; }
        if vv < 0.0 { vv = 0.0 } ; if vv > 0.999_999 { vv = 0.999_999; }

        let x = (uu * self.w as f64) as usize;
        let y = (vv * self.h as f64) as usize;
        self.data[y * self.w + x]
    }
}

/// Carga una textura desde archivo, o genera una “lava/fire” si falla.
pub fn load_or_fire(path: &str, w_fallback: usize, h_fallback: usize) -> Texture {
    if let Ok(img) = image::open(path) {
        let img = img.to_rgb8();
        let (w, h) = img.dimensions();
        let mut data = Vec::with_capacity((w * h) as usize);
        for p in img.pixels() {
            data.push(rgb(p[0], p[1], p[2]));
        }
        Texture { w: w as usize, h: h as usize, data }
    } else {
        // Procedural “fuego”: bandas y ruido simple
        let w = w_fallback.max(16);
        let h = h_fallback.max(16);
        let mut data = vec![0; w*h];
        for y in 0..h {
            for x in 0..w {
                let t = y as f64 / h as f64;
                // gradiente naranja -> rojo
                let r = (220.0 + 35.0 * (1.0 - t)) as u8;
                let g = (60.0 + 80.0 * (1.0 - t)) as u8;
                let b = (30.0 + 10.0 * (1.0 - t)) as u8;
                // patrón de llama
                let stripe = ((x / 6) % 2) as u8;
                let r = r.saturating_add(if stripe == 0 { 20 } else { 0 });
                data[y*w + x] = rgb(r, g, b);
            }
        }
        Texture { w, h, data }
    }
}

// NUEVO: sky fallback (degradé celeste) o carga desde archivo
pub fn load_or_sky(path: &str, w: usize, h: usize) -> Texture {
    if let Ok(img) = image::open(path) {
        let img = img.to_rgb8();
        let (iw, ih) = img.dimensions();
        let mut data = Vec::with_capacity((iw * ih) as usize);
        for p in img.pixels() {
            data.push(((p[0] as u32) << 16) | ((p[1] as u32) << 8) | (p[2] as u32));
        }
        Texture { w: iw as usize, h: ih as usize, data }
    } else {
        // Degradé celeste (fallback)
        let w = w.max(64);
        let h = h.max(32);
        let mut data = vec![0; w*h];
        for y in 0..h {
            let t = y as f64 / (h as f64 - 1.0);
            let r = (110.0 + 20.0 * (1.0 - t)) as u8;
            let g = (150.0 + 40.0 * (1.0 - t)) as u8;
            let b = (220.0 + 35.0 * (1.0 - t)) as u8;
            for x in 0..w { data[y*w + x] = ((r as u32)<<16)|((g as u32)<<8)|(b as u32); }
        }
        Texture { w, h, data }
    }
}

pub struct TextureSet {
    pub wall_fire_a: Texture,
    pub wall_fire_b: Texture,
    pub sky: Texture,
}

impl TextureSet {
    pub fn load() -> Self {
        Self {
            wall_fire_a: load_or_fire("assets/fire_brick.png", 128, 128),
            wall_fire_b: load_or_fire("assets/magma.png", 128, 128),
            // NUEVO: intenta cargar assets/cielo.jpg
            sky: load_or_sky("assets/cielo.jpg", 1024, 256),
        }
    }
}


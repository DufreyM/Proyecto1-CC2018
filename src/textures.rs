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

// -------- Helpers de color --------
#[inline]
fn clamp8(x: f32) -> u8 { x.max(0.0).min(255.0) as u8 }

#[inline]
fn unpack_rgb(px: u32) -> (u8, u8, u8) {
    (((px >> 16) & 0xFF) as u8, ((px >> 8) & 0xFF) as u8, (px & 0xFF) as u8)
}
#[inline]
fn pack_rgb(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

#[inline]
fn tint_cool_blue(data: &mut [u32], r_mul: f32, g_mul: f32, b_mul: f32) {
    for px in data.iter_mut() {
        let (r, g, b) = unpack_rgb(*px);
        let rr = clamp8(r as f32 * r_mul);
        let gg = clamp8(g as f32 * g_mul);
        let bb = clamp8(b as f32 * b_mul);
        *px = pack_rgb(rr, gg, bb);
    }
}

#[inline]
fn lerp_u8(a: u8, b: u8, t: f32) -> u8 {
    clamp8(a as f32 + (b as f32 - a as f32) * t)
}

#[inline]
fn mix_rgb(a: (u8,u8,u8), b: (u8,u8,u8), t: f32) -> u32 {
    let (ar,ag,ab) = a; let (br,bg,bb) = b;
    pack_rgb(lerp_u8(ar,br,t), lerp_u8(ag,bg,t), lerp_u8(ab,bb,t))
}

// -------- Agua: paredes/tiles --------
/// Carga una textura desde archivo y la “enfría” hacia azul.
/// Si falla, genera un patrón procedural de olas en azules.
pub fn load_or_water(path: &str, w_fallback: usize, h_fallback: usize) -> Texture {
    if let Ok(img) = image::open(path) {
        let img = img.to_rgb8();
        let (w, h) = img.dimensions();
        let mut data = Vec::with_capacity((w * h) as usize);
        for p in img.pixels() {
            data.push(rgb(p[0], p[1], p[2]));
        }
        // Tinte frío (reduce rojos, sube azules)
        tint_cool_blue(&mut data, 0.65, 0.9, 1.25);
        Texture { w: w as usize, h: h as usize, data }
    } else {
        // Procedural “olas”: senos entrecruzados + pequeño tramado
        let w = w_fallback.max(16);
        let h = h_fallback.max(16);
        let mut data = vec![0; w*h];
        for y in 0..h {
            for x in 0..w {
                let fx = x as f64 / w as f64;
                let fy = y as f64 / h as f64;

                // Ondas superpuestas
                let a = (fx * std::f64::consts::PI * 6.0 + (fy * 4.0).sin() * 0.8).sin();
                let b = (fy * std::f64::consts::PI * 5.0 + (fx * 7.0).cos() * 0.6).cos();
                let v = ((a + b) * 0.25 + 0.5).clamp(0.0, 1.0) as f32;

                // Paleta: azul profundo -> celeste
                let c_deep = (14u8, 42u8, 91u8);    // #0E2A5B
                let c_light= (105u8, 167u8, 255u8); // #69A7FF
                // Ligeros “reflejos” cada 6 px
                let spark = if ((x + y) % 6) == 0 { 0.08 } else { 0.0 };
                let t = (v + spark as f32).min(1.0);

                data[y*w + x] = mix_rgb(c_deep, c_light, t);
            }
        }
        Texture { w, h, data }
    }
}

// -------- Cielo azul para agua --------
pub fn load_or_sky_blue(path: &str, w: usize, h: usize) -> Texture {
    if let Ok(img) = image::open(path) {
        let img = img.to_rgb8();
        let (iw, ih) = img.dimensions();
        let mut data = Vec::with_capacity((iw * ih) as usize);
        for p in img.pixels() {
            data.push(rgb(p[0], p[1], p[2]));
        }
        // Refría un poco el cielo (más azul)
        let mut_ref: &mut [u32] = &mut data;
        tint_cool_blue(mut_ref, 0.85, 0.95, 1.10);
        Texture { w: iw as usize, h: ih as usize, data }
    } else {
        // Degradé cielo: oscuro arriba -> claro al horizonte
        let w = w.max(64);
        let h = h.max(32);
        let mut data = vec![0; w*h];
        for y in 0..h {
            let t = y as f32 / (h as f32 - 1.0);
            // top (azul profundo) -> bottom (celeste claro)
            let top = (10u8, 32u8, 80u8);    // #0A2050
            let bot = (120u8, 185u8, 255u8); // #78B9FF
            let col = mix_rgb(top, bot, t);
            for x in 0..w { data[y*w + x] = col; }
        }
        Texture { w, h, data }
    }
}

impl TextureSet {
    pub fn load_water() -> Self {
        Self {
            // Si existen, intenta cargar estos paths. Si no, usa el procedural azul.
            wall_fire_a: load_or_water("assets/water_tiles.png", 128, 128),
            wall_fire_b: load_or_water("assets/water_bricks.png", 128, 128),
            // Cielo más azul (o carga assets/cielo_azul.jpg)
            sky: load_or_sky_blue("assets/cielo_azul.jpg", 1024, 256),
        }
    }
}

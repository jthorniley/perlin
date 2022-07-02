use std::{collections::hash_map::DefaultHasher, hash::Hasher};

pub struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }

    fn from_direction(direction: f32) -> Vec2 {
        let (x, y) = direction.sin_cos();
        Vec2 { x, y }
    }

    fn dot(&self, other: &Vec2) -> f32 {
        return self.x * other.x + self.y * other.y;
    }
}

fn corner_vector(x: i16, y: i16) -> Vec2 {
    let mut hasher = DefaultHasher::new();
    hasher.write_i16(x);
    hasher.write_i16(y);
    let hash_value = hasher.finish();
    let direction = hash_value as f32 / u64::MAX as f32;
    Vec2::from_direction(direction * std::f32::consts::PI * 2.0)
}

fn interpolate(a0: f32, a1: f32, w: f32) -> f32 {
    (a1 - a0) * ((w * (w * 6.0 - 15.0) + 10.0) * w * w * w) + a0
}

pub fn perlin_cube(x: i16, y: i16, nx: u16, ny: u16) -> Vec<u8> {
    let c1 = corner_vector(x, y);
    let c2 = corner_vector(x, y + 1);
    let c3 = corner_vector(x + 1, y);
    let c4 = corner_vector(x + 1, y + 1);

    let mut buf = vec![0u8; (nx * ny).into()];
    let mut bufIdx = 0usize;

    let dx = 1.0f32 / nx as f32;
    let dy = 1.0f32 / ny as f32;

    let mut x = dx / 2.0;
    let mut y = dy / 2.0;

    for i in 0..nx {
        for j in 0..ny {
            let p1 = Vec2::new(-x, -y).dot(&c1);
            let p2 = Vec2::new(-x, 1.0 - y).dot(&c2);
            let p3 = Vec2::new(1.0 - x, -y).dot(&c3);
            let p4 = Vec2::new(1.0 - x, 1.0 - y).dot(&c4);

            let i1 = interpolate(p1, p2, y);
            let i2 = interpolate(p3, p4, y);
            let i = interpolate(i1, i2, x);

            buf[bufIdx] = (u8::MAX as f32 * (i / 2.0 + 0.5)) as u8;
            y += dy;
            bufIdx += 1;
        }
        y = dy / 2.0;
        x += dx;
    }

    return buf;
}

#[cfg(test)]
mod tests {}

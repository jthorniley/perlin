pub fn perlin_cube(x: i16, y: i16, nx: u16, ny: u16) -> Vec<u8> {
    return vec![140; (nx * ny).into()];
}

#[cfg(test)]
mod tests {}

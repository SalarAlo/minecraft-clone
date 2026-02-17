use noise::{Fbm, MultiFractal, Perlin};
use once_cell::sync::Lazy;

const SEED: u32 = 42;

pub static FBM_PLAINS: Lazy<Fbm<Perlin>> = Lazy::new(|| {
    Fbm::<Perlin>::new(SEED + 100)
        .set_frequency(0.5)
        .set_octaves(2)
        .set_lacunarity(2.0)
        .set_persistence(0.5)
});

pub static FBM_DESERT: Lazy<Fbm<Perlin>> = Lazy::new(|| {
    Fbm::<Perlin>::new(SEED + 200)
        .set_frequency(0.5)
        .set_octaves(1)
        .set_lacunarity(2.0)
        .set_persistence(0.5)
});

pub static FBM_JUNGLE: Lazy<Fbm<Perlin>> = Lazy::new(|| {
    Fbm::<Perlin>::new(SEED + 300)
        .set_frequency(0.5)
        .set_octaves(3)
        .set_lacunarity(2.0)
        .set_persistence(0.55)
});

pub static FBM_TUNDRA: Lazy<Fbm<Perlin>> = Lazy::new(|| {
    Fbm::<Perlin>::new(SEED + 400)
        .set_frequency(0.5)
        .set_octaves(2)
        .set_lacunarity(2.0)
        .set_persistence(0.5)
});

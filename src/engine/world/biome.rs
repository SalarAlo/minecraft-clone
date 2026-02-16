use super::climate_sampler::ClimateSample;
use noise::{Fbm, MultiFractal, NoiseFn, Perlin};
use once_cell::sync::Lazy;

#[derive(Copy, Clone)]
pub struct SurfaceRules {
    pub desired_temperature: f64,
    pub desired_moisture: f64,

    pub temp_weight: f64,
    pub moist_weight: f64,
}

pub type HeightFn = fn(base_height: i32, x: i32, z: i32, climate: &ClimateSample) -> i32;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum BiomeKind {
    Plains,
    Desert,
    Tundra,
    Jungle,
    Savanna,
    BorealForest,
    TemperateForest,
}

#[derive(Clone)]
pub struct Biome {
    pub surface: SurfaceRules,
    pub height_fn: HeightFn,
    pub kind: BiomeKind,
}

pub struct BiomeSelector {
    biomes: Vec<Biome>,
}

impl SurfaceRules {
    fn compute_delta(&self, sample: &ClimateSample) -> f64 {
        const EXPONENT: f64 = 4.;
        let dt = (sample.temperature - self.desired_temperature).abs();
        let dm = (sample.moisture - self.desired_moisture).abs();

        self.temp_weight * dt.powf(EXPONENT) + self.moist_weight * dm.powf(EXPONENT)
    }
}

impl BiomeSelector {
    pub fn pick(&self, sample: &ClimateSample) -> &Biome {
        let first = self
            .biomes
            .get(0)
            .expect("BiomeSelector can't pick biome if no biomes added");

        let mut min_delta = first.surface.compute_delta(&sample);
        let mut best_biome = first;

        for biome in &self.biomes[1..] {
            let current_delta = biome.surface.compute_delta(&sample);

            if min_delta > current_delta {
                best_biome = biome;
                min_delta = current_delta;
            }
        }

        best_biome
    }
}

impl Default for BiomeSelector {
    fn default() -> Self {
        Self {
            biomes: BIOMES.iter().map(|e| e.clone()).collect(),
        }
    }
}

pub const BIOMES: &[Biome] = &[
    Biome {
        kind: BiomeKind::Plains,
        surface: SurfaceRules {
            desired_temperature: 0.6,
            desired_moisture: 0.5,
            temp_weight: 1.0,
            moist_weight: 1.0,
        },
        height_fn: plains_height,
    },
    Biome {
        kind: BiomeKind::Desert,
        surface: SurfaceRules {
            desired_temperature: 0.85,
            desired_moisture: 0.25,
            temp_weight: 1.0,
            moist_weight: 5.0,
        },
        height_fn: desert_height,
    },
    Biome {
        kind: BiomeKind::Tundra,
        surface: SurfaceRules {
            desired_temperature: 0.15,
            desired_moisture: 0.3,
            temp_weight: 3.0,
            moist_weight: 0.8,
        },
        height_fn: tundra_height,
    },
    Biome {
        kind: BiomeKind::Jungle,
        surface: SurfaceRules {
            desired_temperature: 0.85,
            desired_moisture: 0.9,
            temp_weight: 2.0,
            moist_weight: 2.5,
        },
        height_fn: jungle_height,
    },
    Biome {
        kind: BiomeKind::Savanna,
        surface: SurfaceRules {
            desired_temperature: 0.8,
            desired_moisture: 0.4,
            temp_weight: 1.2,
            moist_weight: 1.5,
        },
        height_fn: savanna_height,
    },
    Biome {
        kind: BiomeKind::BorealForest,
        surface: SurfaceRules {
            desired_temperature: 0.3,
            desired_moisture: 0.6,
            temp_weight: 2.0,
            moist_weight: 1.2,
        },
        height_fn: boreal_height,
    },
    Biome {
        kind: BiomeKind::TemperateForest,
        surface: SurfaceRules {
            desired_temperature: 0.55,
            desired_moisture: 0.75,
            temp_weight: 1.5,
            moist_weight: 2.0,
        },
        height_fn: temperate_forest_height,
    },
];

const SEED: u32 = 12345;

static FBM: Lazy<Fbm<Perlin>> = Lazy::new(|| {
    Fbm::<Perlin>::new(SEED)
        .set_frequency(0.5)
        .set_octaves(2)
        .set_lacunarity(2.0)
        .set_persistence(0.5)
});

fn sample(x: i32, z: i32, freq: f64) -> f64 {
    FBM.get([x as f64 * freq, z as f64 * freq])
}

fn ridged(n: f64) -> f64 {
    1.0 - n.abs()
}

fn billowy(n: f64) -> f64 {
    n.abs()
}

fn stepped(n: f64, step: f64) -> f64 {
    (n / step).round() * step
}

pub fn plains_height(base: i32, x: i32, z: i32, _: &ClimateSample) -> i32 {
    let n = sample(x, z, 0.02);
    base + (n * 3.0) as i32
}

pub fn desert_height(base: i32, x: i32, z: i32, _: &ClimateSample) -> i32 {
    let n = billowy(sample(x, z, 0.01));
    base + (n * 10.0) as i32
}

pub fn tundra_height(base: i32, x: i32, z: i32, _: &ClimateSample) -> i32 {
    let n = stepped(sample(x, z, 0.03), 0.2);
    base + (n * 2.0) as i32
}

pub fn jungle_height(base: i32, x: i32, z: i32, climate: &ClimateSample) -> i32 {
    let n = sample(x, z, 0.06);
    base + (n * 12.0 * climate.moisture) as i32
}

pub fn savanna_height(base: i32, x: i32, z: i32, _: &ClimateSample) -> i32 {
    let n = stepped(sample(x, z, 0.015), 0.3);
    base + (n * 8.0) as i32
}

pub fn boreal_height(base: i32, x: i32, z: i32, climate: &ClimateSample) -> i32 {
    let n = ridged(sample(x, z, 0.04));
    base + (n * 8.0 * (1.0 - climate.temperature)) as i32
}

pub fn temperate_forest_height(base: i32, x: i32, z: i32, _: &ClimateSample) -> i32 {
    let n = sample(x, z, 0.025);
    base + (n * 6.0) as i32
}

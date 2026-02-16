use noise::{Fbm, MultiFractal, NoiseFn, Perlin};

const SEED_TEMPERATURE: u32 = 42;
const SEED_MOISTURE: u32 = 73;

pub struct ClimateSample {
    pub temperature: f64,
    pub moisture: f64,
}

pub struct ClimateSampler {
    scale: f64,
    temperature_gen: Fbm<Perlin>,
    moisture_gen: Fbm<Perlin>,
}

impl ClimateSampler {
    pub fn new() -> ClimateSampler {
        ClimateSampler {
            temperature_gen: Fbm::<Perlin>::new(SEED_TEMPERATURE)
                .set_frequency(0.5)
                .set_octaves(2)
                .set_lacunarity(2.0)
                .set_persistence(0.8),

            moisture_gen: Fbm::<Perlin>::new(SEED_MOISTURE)
                .set_frequency(0.7)
                .set_octaves(2)
                .set_lacunarity(1.4)
                .set_persistence(0.8),

            scale: 0.005,
        }
    }
}

impl ClimateSampler {
    pub fn sample(&self, world_x: i32, world_z: i32) -> ClimateSample {
        let coord = [world_x as f64 * self.scale, world_z as f64 * self.scale];
        let temperature = (self.temperature_gen.get(coord) + 1.0) * 0.5;
        let moisture = (self.moisture_gen.get(coord) + 1.0) * 0.5;

        ClimateSample {
            temperature,
            moisture,
        }
    }
}

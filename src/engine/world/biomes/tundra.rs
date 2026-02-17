use noise::NoiseFn;

use crate::engine::world::{
    biome::{Biome, SurfaceRules},
    biomes::terrain_noise::FBM_TUNDRA,
    block::BlockType,
    climate_sampler::ClimateSample,
};

pub struct Tundra;

impl Biome for Tundra {
    fn get_surface(&self) -> SurfaceRules {
        SurfaceRules {
            desired_temperature: 0.15,
            desired_moisture: 0.3,
            temp_weight: 3.0,
            moist_weight: 0.8,
        }
    }

    fn height_offset(&self, x: i32, z: i32, climate: &ClimateSample) -> f64 {
        const SCALE: f64 = 0.015;
        const AMP: f64 = 5.0;

        let n = FBM_TUNDRA.get([x as f64 * SCALE + 4000.0, z as f64 * SCALE + 4000.0]);

        let stepped = (n * 4.0).round() / 4.0;

        let cold = (1.0 - climate.temperature).clamp(0.0, 1.0);
        let strength = 0.5 + 0.8 * cold;

        stepped * AMP * strength + 5.
    }

    fn ground_block(&self) -> BlockType {
        BlockType::Snow
    }
}

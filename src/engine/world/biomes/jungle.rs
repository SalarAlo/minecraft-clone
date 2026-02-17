use noise::NoiseFn;

use crate::engine::world::{
    biome::{Biome, SurfaceRules},
    biomes::terrain_noise::FBM_JUNGLE,
    block::BlockType,
    climate_sampler::ClimateSample,
};

pub struct Jungle;

impl Biome for Jungle {
    fn get_surface(&self) -> SurfaceRules {
        SurfaceRules {
            desired_temperature: 0.85,
            desired_moisture: 0.9,
            temp_weight: 2.0,
            moist_weight: 2.5,
        }
    }

    fn height_offset(&self, x: i32, z: i32, climate: &ClimateSample) -> f64 {
        const SCALE: f64 = 0.02;
        const AMP: f64 = 5.0;

        let n = FBM_JUNGLE.get([x as f64 * SCALE + 3000.0, z as f64 * SCALE + 3000.0]);

        let uplift = (n + 0.3).clamp(-1.0, 1.0);

        let wet = climate.moisture.clamp(0.0, 1.0);
        let strength = 0.6 + 0.8 * wet;

        uplift * AMP * strength
    }

    fn ground_block(&self) -> BlockType {
        BlockType::Grass
    }
}

use noise::NoiseFn;

use crate::engine::world::{
    biome::{Biome, SurfaceRules},
    biomes::terrain_noise::FBM_PLAINS,
    block::BlockType,
    climate_sampler::ClimateSample,
};

pub struct Plains;

impl Biome for Plains {
    fn get_surface(&self) -> SurfaceRules {
        SurfaceRules {
            desired_temperature: 0.7,
            desired_moisture: 0.4,
            temp_weight: 1.0,
            moist_weight: 1.0,
        }
    }

    fn height_offset(&self, x: i32, z: i32, climate: &ClimateSample) -> f64 {
        const SCALE: f64 = 0.006;
        const AMP: f64 = 3.0;

        let n = FBM_PLAINS.get([x as f64 * SCALE + 1000.0, z as f64 * SCALE + 1000.0]);

        let m = climate.moisture.clamp(0.0, 1.0);
        let damp = 1.0 - 0.5 * m;

        n * AMP * damp
    }

    fn ground_block(&self) -> BlockType {
        BlockType::Grass
    }
}

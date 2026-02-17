use noise::NoiseFn;

use crate::engine::world::{
    biome::{Biome, SurfaceRules},
    biomes::terrain_noise::FBM_DESERT,
    block::BlockType,
    climate_sampler::ClimateSample,
};

pub struct Desert;

impl Biome for Desert {
    fn get_surface(&self) -> SurfaceRules {
        SurfaceRules {
            desired_temperature: 0.75,
            desired_moisture: 0.25,
            temp_weight: 5.0,
            moist_weight: 1.0,
        }
    }

    fn height_offset(&self, x: i32, z: i32, climate: &ClimateSample) -> f64 {
        const SCALE: f64 = 0.002; // very wide

        let n = FBM_DESERT.get([x as f64 * SCALE + 2000.0, z as f64 * SCALE + 2000.0]);

        // deserts suppress terrain variation
        let heat = climate.temperature.clamp(0.0, 1.0);
        let strength = 0.3 + 0.7 * heat;

        n * 2.0 * strength
    }

    fn ground_block(&self) -> BlockType {
        BlockType::Sand
    }
}

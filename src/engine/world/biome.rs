use crate::engine::world::block::BlockType;

use super::biomes::desert::Desert;
use super::biomes::jungle::Jungle;
use super::biomes::plains::Plains;
use super::biomes::tundra::Tundra;
use super::climate_sampler::ClimateSample;

#[derive(Copy, Clone)]
pub struct SurfaceRules {
    pub desired_temperature: f64,
    pub desired_moisture: f64,

    pub temp_weight: f64,
    pub moist_weight: f64,
}

pub trait Biome {
    fn get_surface(&self) -> SurfaceRules;
    fn height_offset(&self, x: i32, z: i32, climate: &ClimateSample) -> f64;
    fn ground_block(&self) -> BlockType;
}

pub struct BiomeSelector {
    biomes: Vec<Box<dyn Biome>>,
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
    pub fn pick(&self, sample: &ClimateSample) -> &dyn Biome {
        let first = self
            .biomes
            .get(0)
            .expect("BiomeSelector can't pick biome if no biomes added");

        let mut min_delta = first.get_surface().compute_delta(&sample);
        let mut best_biome = first;

        for biome in &self.biomes[1..] {
            let current_delta = biome.get_surface().compute_delta(&sample);

            if min_delta > current_delta {
                best_biome = biome;
                min_delta = current_delta;
            }
        }

        best_biome.as_ref()
    }

    pub fn blended_height(&self, base: i32, x: i32, z: i32, climate: &ClimateSample) -> i32 {
        const SHARPNESS: f64 = 2.5;
        const EPSILON: f64 = 0.0001;

        let mut total_weight = 0.0;
        let mut total_offset = 0.0;

        for biome in &self.biomes {
            let surface = biome.get_surface();
            let delta = surface.compute_delta(climate).max(EPSILON);

            let weight = 1.0 / delta.powf(SHARPNESS);

            total_weight += weight;
            total_offset += weight * biome.height_offset(x, z, climate);
        }

        let blended = total_offset / total_weight;

        base + blended as i32
    }
}

impl Default for BiomeSelector {
    fn default() -> Self {
        Self {
            biomes: vec![
                Box::new(Plains),
                Box::new(Jungle),
                Box::new(Desert),
                Box::new(Tundra),
            ],
        }
    }
}

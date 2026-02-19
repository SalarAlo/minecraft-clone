use bevy::math::IVec3;

use crate::engine::world::block::BlockWrite;

pub struct StructureRule {
    pub rarity: f64, // spawn probability
    pub min_height: i32,
    pub max_height: i32,
    pub generator: fn(IVec3, &mut dyn BlockWrite),
}

impl StructureRule {
    pub fn should_place(&self, x: i32, z: i32, surface_y: i32, seed: u32) -> bool {
        if surface_y < self.min_height || surface_y > self.max_height {
            return false;
        }

        let h = hash_2d(x, z, seed);
        let r = (h as f64) / (u32::MAX as f64);

        r < self.rarity
    }

    pub fn try_place(&self, pos: IVec3, world: &mut dyn BlockWrite, seed: u32) {
        if self.should_place(pos.x, pos.z, pos.y, seed) {
            (self.generator)(pos, world);
        }
    }
}

fn hash_2d(x: i32, z: i32, seed: u32) -> u32 {
    let mut h = seed ^ (x as u32).wrapping_mul(0x9E3779B9) ^ (z as u32).wrapping_mul(0x85EBCA6B);

    h ^= h >> 16;
    h = h.wrapping_mul(0x7FEB352D);
    h ^= h >> 15;
    h = h.wrapping_mul(0x846CA68B);
    h ^= h >> 16;

    h
}

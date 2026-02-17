use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use noise::{Fbm, MultiFractal, NoiseFn, Perlin};
use once_cell::sync::Lazy;

use super::block::{BlockAccess, BlockType};
use crate::engine::atlas::{ChunkMaterial, TextureAtlas};
use crate::engine::world::biome::BiomeSelector;
use crate::engine::world::climate_sampler::ClimateSampler;
use crate::engine::{face_direction::DIRECTIONS, mesh_builder::MeshBuilder};

pub const CHUNK_HEIGHT: usize = 128;
const HALF_CHUNK_HEIGHT: usize = CHUNK_HEIGHT / 2;
pub const WATER_HEIGHT: usize = HALF_CHUNK_HEIGHT + 20;
pub const CHUNK_SIZE: usize = 16;
const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_HEIGHT;

const PAD_CHUNK_SIZE: usize = CHUNK_SIZE + 2;

const PAD_CHUNK_HEIGHT: usize = CHUNK_HEIGHT + 2;

const PAD_CHUNK_VOLUME: usize = PAD_CHUNK_SIZE * PAD_CHUNK_SIZE * PAD_CHUNK_HEIGHT;

const SEED: u32 = 42;

pub static FBM: Lazy<Fbm<Perlin>> = Lazy::new(|| {
    Fbm::<Perlin>::new(SEED)
        .set_frequency(0.5)
        .set_octaves(2)
        .set_lacunarity(2.0)
        .set_persistence(0.5)
});

#[derive(Component, Clone)]
pub struct Chunk {
    coord: IVec2,
    blocks: [BlockType; CHUNK_VOLUME],
}

#[derive(Resource, Default)]
pub struct ChunkMap(pub HashMap<IVec2, Entity>);

// Methods
impl Chunk {
    pub fn build_chunk_mesh(&self, block_access: &impl BlockAccess, atlas: &TextureAtlas) -> Mesh {
        let mut mesh_builder = MeshBuilder::with_capacity_faces(CHUNK_VOLUME * 2);
        let origin = self.chunk_origin();

        let mut is_solid = [false; CHUNK_VOLUME];
        for i in 0..CHUNK_VOLUME {
            is_solid[i] = !self.blocks[i].is_seethrough();
        }

        let mut is_solid_padded = [false; PAD_CHUNK_VOLUME];

        for y in 0..CHUNK_HEIGHT as i32 {
            for z in 0..CHUNK_SIZE as i32 {
                for x in 0..CHUNK_SIZE as i32 {
                    let src = Self::to_index(IVec3::new(x, y, z));
                    let dst = Self::pad_index(x + 1, y + 1, z + 1);
                    is_solid_padded[dst] = is_solid[src];
                }
            }
        }

        // padding for x dimension
        for by in 0..CHUNK_HEIGHT as i32 {
            for z in 0..CHUNK_SIZE as i32 {
                is_solid_padded[Self::pad_index(0, by + 1, z + 1)] = block_access
                    .get_block(origin + IVec3::new(-1, by, z))
                    .map_or(false, |b| !b.is_seethrough());

                is_solid_padded[Self::pad_index(CHUNK_SIZE as i32 + 1, by + 1, z + 1)] =
                    block_access
                        .get_block(origin + IVec3::new(CHUNK_SIZE as i32, by, z))
                        .map_or(false, |b| !b.is_seethrough());
            }
        }

        // padding for z dimension
        for by in 0..CHUNK_HEIGHT as i32 {
            for bx in 0..CHUNK_SIZE as i32 {
                is_solid_padded[Self::pad_index(bx + 1, by + 1, 0)] = block_access
                    .get_block(origin + IVec3::new(bx, by, -1))
                    .map_or(false, |b| !b.is_seethrough());

                is_solid_padded[Self::pad_index(bx + 1, by + 1, CHUNK_SIZE as i32 + 1)] =
                    block_access
                        .get_block(origin + IVec3::new(bx, by, CHUNK_SIZE as i32))
                        .map_or(false, |b| !b.is_seethrough());
            }
        }

        // padding for y dimension
        for bz in 0..CHUNK_SIZE as i32 {
            for bx in 0..CHUNK_SIZE as i32 {
                is_solid_padded[Self::pad_index(bx + 1, 0, bz + 1)] = block_access
                    .get_block(origin + IVec3::new(bx, -1, bz))
                    .map_or(false, |b| !b.is_seethrough());

                is_solid_padded[Self::pad_index(bx + 1, CHUNK_HEIGHT as i32 + 1, bz + 1)] =
                    block_access
                        .get_block(origin + IVec3::new(bx, CHUNK_HEIGHT as i32, bz))
                        .map_or(false, |b| !b.is_seethrough());
            }
        }

        for py in 1..=CHUNK_HEIGHT as i32 {
            for pz in 1..=CHUNK_SIZE as i32 {
                for px in 1..=CHUNK_SIZE as i32 {
                    let idx = Self::pad_index(px, py, pz);

                    if !is_solid_padded[idx] {
                        continue;
                    }

                    let bx = px - 1;
                    let by = py - 1;
                    let bz = pz - 1;

                    let block_type = self.blocks[by as usize
                        + bz as usize * CHUNK_HEIGHT
                        + bx as usize * CHUNK_HEIGHT * CHUNK_SIZE];

                    for dir in DIRECTIONS {
                        let dir_vec3 = dir.normal();
                        let neighbour_solid = is_solid_padded
                            [Self::pad_index(px + dir_vec3.x, py + dir_vec3.y, pz + dir_vec3.z)];

                        if !neighbour_solid {
                            if let Some(texture_id) = block_type.texture_id(dir) {
                                let uvs = atlas.uvs(texture_id);
                                mesh_builder.add_face(dir, IVec3::new(bx, by, bz), uvs);
                            }
                        }
                    }
                }
            }
        }

        mesh_builder.build_mesh()
    }
}

// Associated Functions
impl Chunk {
    pub fn new(chunk_x: i32, chunk_z: i32) -> Chunk {
        let selector = BiomeSelector::default();
        let sampler = ClimateSampler::new();
        let scale = 0.01;

        let mut chunk = Chunk {
            coord: IVec2::new(chunk_x, chunk_z),
            blocks: [BlockType::Air; CHUNK_VOLUME],
        };

        for block_x in 0..CHUNK_SIZE as i32 {
            for block_z in 0..CHUNK_SIZE as i32 {
                let world_x = block_x + chunk_x * CHUNK_SIZE as i32;
                let world_z = block_z + chunk_z * CHUNK_SIZE as i32;

                let height = FBM.get([world_x as f64 * scale, world_z as f64 * scale]);
                let height = ((HALF_CHUNK_HEIGHT as f64)
                    + ((height + 1.0) * 0.5 * (HALF_CHUNK_HEIGHT as f64)))
                    as i32;

                let climate_sample = sampler.sample(world_x, world_z);
                let biome = selector.pick(&climate_sample);
                let height = selector.blended_height(
                    height,
                    world_x as i32,
                    world_z as i32,
                    &climate_sample,
                );

                let height = height.clamp(0, (CHUNK_HEIGHT - 1) as i32) as i32;

                for y in 0..height.max(WATER_HEIGHT as i32) {
                    let block = if y == 0 {
                        BlockType::Bedrock
                    } else if y < WATER_HEIGHT as i32 {
                        BlockType::Water
                    } else {
                        biome.ground_block()
                    };

                    chunk.blocks[Self::to_index(IVec3::new(block_x, y, block_z))] = block;
                }
            }
        }

        chunk
    }

    pub fn new_entity(commands: &mut Commands, material: &ChunkMaterial, coord: IVec2) -> Entity {
        let chunk = Chunk::new(coord.x, coord.y);

        let world_x = coord.x * CHUNK_SIZE as i32;
        let world_z = coord.y * CHUNK_SIZE as i32;

        commands
            .spawn((
                chunk,
                MeshMaterial3d(material.0.clone()),
                Transform::from_xyz(world_x as f32, 0.0, world_z as f32),
            ))
            .id()
    }
}

// Helper Functions
impl Chunk {
    #[inline(always)]
    fn to_index(v: IVec3) -> usize {
        v.y as usize + v.z as usize * CHUNK_HEIGHT + v.x as usize * CHUNK_HEIGHT * CHUNK_SIZE
    }

    pub fn get_local(&self, local: IVec3) -> Option<BlockType> {
        if local.x < 0
            || local.x >= CHUNK_SIZE as i32
            || local.y < 0
            || local.y >= CHUNK_HEIGHT as i32
            || local.z < 0
            || local.z >= CHUNK_SIZE as i32
        {
            return None;
        }
        Some(self.blocks[Self::to_index(local)])
    }

    pub fn chunk_origin(&self) -> IVec3 {
        IVec3::new(
            self.coord.x * CHUNK_SIZE as i32,
            0,
            self.coord.y * CHUNK_SIZE as i32,
        )
    }

    #[inline(always)]
    fn pad_index(x: i32, y: i32, z: i32) -> usize {
        (x as usize)
            + (z as usize) * PAD_CHUNK_SIZE
            + (y as usize) * PAD_CHUNK_SIZE * PAD_CHUNK_SIZE
    }
}

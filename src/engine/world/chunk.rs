use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use noise::{Fbm, MultiFractal, NoiseFn, Perlin};
use once_cell::sync::Lazy;

use super::block::{BlockAccess, BlockType};
use crate::engine::atlas::TextureAtlas;
use crate::engine::{cube::DIRECTIONS, mesh_builder::MeshBuilder};

pub const CHUNK_HEIGHT: usize = 32;
pub const CHUNK_SIZE: usize = 32;
const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_HEIGHT;

const PAD_SIZE: usize = CHUNK_SIZE + 2;
const PAD_HEIGHT: usize = CHUNK_HEIGHT + 2;
const PAD_VOLUME: usize = PAD_SIZE * PAD_SIZE * PAD_HEIGHT;

static FBM: Lazy<Fbm<Perlin>> = Lazy::new(|| {
    Fbm::<Perlin>::new(42)
        .set_frequency(0.5)
        .set_octaves(3)
        .set_lacunarity(2.0)
        .set_persistence(0.5)
});

#[derive(Component, Clone)]
pub struct Chunk {
    coord: IVec2,
    blocks: [BlockType; CHUNK_VOLUME],
}

#[derive(Resource, Default)]
pub struct ChunkMap {
    pub map: HashMap<IVec2, Entity>,
}

impl Chunk {
    pub fn x(&self) -> i32 {
        self.coord.x
    }

    pub fn z(&self) -> i32 {
        self.coord.y
    }

    pub fn build_chunk_mesh(&self, blocks: &impl BlockAccess, atlas: &TextureAtlas) -> Mesh {
        let mut mesh_builder = MeshBuilder::with_capacity_faces(CHUNK_VOLUME * 2);
        let origin = self.chunk_origin();

        let mut solid = [false; CHUNK_VOLUME];
        for i in 0..CHUNK_VOLUME {
            solid[i] = !self.blocks[i].is_seethrough();
        }

        let mut padded = [false; PAD_VOLUME];

        for y in 0..CHUNK_HEIGHT as i32 {
            for z in 0..CHUNK_SIZE as i32 {
                for x in 0..CHUNK_SIZE as i32 {
                    let src = to_index(IVec3::new(x, y, z));
                    let dst = Self::pad_index(x + 1, y + 1, z + 1);
                    padded[dst] = solid[src];
                }
            }
        }

        for y in 0..CHUNK_HEIGHT as i32 {
            for z in 0..CHUNK_SIZE as i32 {
                padded[Self::pad_index(0, y + 1, z + 1)] = blocks
                    .get_block(origin + IVec3::new(-1, y, z))
                    .map_or(false, |b| !b.is_seethrough());

                padded[Self::pad_index(CHUNK_SIZE as i32 + 1, y + 1, z + 1)] = blocks
                    .get_block(origin + IVec3::new(CHUNK_SIZE as i32, y, z))
                    .map_or(false, |b| !b.is_seethrough());
            }
        }

        for y in 0..CHUNK_HEIGHT as i32 {
            for x in 0..CHUNK_SIZE as i32 {
                padded[Self::pad_index(x + 1, y + 1, 0)] = blocks
                    .get_block(origin + IVec3::new(x, y, -1))
                    .map_or(false, |b| !b.is_seethrough());

                padded[Self::pad_index(x + 1, y + 1, CHUNK_SIZE as i32 + 1)] = blocks
                    .get_block(origin + IVec3::new(x, y, CHUNK_SIZE as i32))
                    .map_or(false, |b| !b.is_seethrough());
            }
        }

        for z in 0..CHUNK_SIZE as i32 {
            for x in 0..CHUNK_SIZE as i32 {
                padded[Self::pad_index(x + 1, 0, z + 1)] = blocks
                    .get_block(origin + IVec3::new(x, -1, z))
                    .map_or(false, |b| !b.is_seethrough());

                padded[Self::pad_index(x + 1, CHUNK_HEIGHT as i32 + 1, z + 1)] = blocks
                    .get_block(origin + IVec3::new(x, CHUNK_HEIGHT as i32, z))
                    .map_or(false, |b| !b.is_seethrough());
            }
        }

        for y in 1..=CHUNK_HEIGHT as i32 {
            for z in 1..=CHUNK_SIZE as i32 {
                for x in 1..=CHUNK_SIZE as i32 {
                    let idx = Self::pad_index(x, y, z);
                    if !padded[idx] {
                        continue;
                    }

                    let bx = x - 1;
                    let by = y - 1;
                    let bz = z - 1;

                    let block_type = self.blocks[by as usize
                        + bz as usize * CHUNK_HEIGHT
                        + bx as usize * CHUNK_HEIGHT * CHUNK_SIZE];

                    for dir in DIRECTIONS {
                        let n = dir.normal();
                        let neighbour_solid = padded[Self::pad_index(x + n.x, y + n.y, z + n.z)];

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

    pub fn new(chunk_x: i32, chunk_z: i32) -> Chunk {
        let mut blocks = [BlockType::Air; CHUNK_VOLUME];
        let scale = 0.02;

        for block_x in 0..CHUNK_SIZE as i32 {
            for block_z in 0..CHUNK_SIZE as i32 {
                let world_x = (block_x + chunk_x * CHUNK_SIZE as i32) as f64 * scale;
                let world_z = (block_z + chunk_z * CHUNK_SIZE as i32) as f64 * scale;

                let height = FBM.get([world_x, world_z]);
                let height = ((height + 1.0) * 0.5 * CHUNK_HEIGHT as f64) as usize;
                let height = height.clamp(0, CHUNK_HEIGHT - 1) as i32;

                for y in 0..height {
                    let block = if y == 0 {
                        BlockType::Bedrock
                    } else if y < CHUNK_HEIGHT as i32 / 2 {
                        BlockType::Sand
                    } else if y < height - 1 {
                        BlockType::Dirt
                    } else {
                        BlockType::Grass
                    };

                    blocks[to_index(IVec3::new(block_x, y, block_z))] = block;
                }
            }
        }

        Chunk {
            coord: IVec2::new(chunk_x, chunk_z),
            blocks,
        }
    }

    #[inline(always)]
    fn pad_index(x: i32, y: i32, z: i32) -> usize {
        (x as usize) + (z as usize) * PAD_SIZE + (y as usize) * PAD_SIZE * PAD_SIZE
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
        Some(self.blocks[to_index(local)])
    }

    pub fn chunk_origin(&self) -> IVec3 {
        IVec3::new(
            self.coord.x * CHUNK_SIZE as i32,
            0,
            self.coord.y * CHUNK_SIZE as i32,
        )
    }
}

#[inline(always)]
fn to_index(v: IVec3) -> usize {
    v.y as usize + v.z as usize * CHUNK_HEIGHT + v.x as usize * CHUNK_HEIGHT * CHUNK_SIZE
}

use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use noise::{Fbm, MultiFractal, NoiseFn, Perlin};

use super::atlas::TextureAtlas;
use super::block::{BlockAccess, BlockType};
use super::{cube::DIRECTIONS, mesh_builder::MeshBuilder};

pub const CHUNK_HEIGHT: usize = 32;
pub const CHUNK_SIZE: usize = 32;
const CHUNK_PLANE: usize = CHUNK_SIZE * CHUNK_SIZE;
const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_HEIGHT;

#[derive(Component)]
pub struct Chunk {
    coord: IVec2,
    blocks: [BlockType; CHUNK_VOLUME],
}

#[derive(Resource, Default)]
pub struct ChunkMap {
    pub map: HashMap<IVec2, Entity>,
}

impl Chunk {
    pub fn coord(&self) -> IVec2 {
        self.coord.clone()
    }

    pub fn x(&self) -> i32 {
        self.coord.x
    }

    pub fn z(&self) -> i32 {
        self.coord.y
    }

    pub fn build_chunk_mesh(&self, blocks: &impl BlockAccess, atlas: &TextureAtlas) -> Mesh {
        let mut mesh_builder = MeshBuilder::new();

        for y in 0..CHUNK_HEIGHT as i32 {
            for z in 0..CHUNK_SIZE as i32 {
                for x in 0..CHUNK_SIZE as i32 {
                    let coord = IVec3::new(x, y, z);
                    let local_idx = to_index(coord);
                    let block_type = self.blocks[local_idx];

                    if block_type.is_seethrough() {
                        continue;
                    }

                    self.add_necessary_faces(&mut mesh_builder, coord, blocks, atlas);
                }
            }
        }

        mesh_builder.build_mesh()
    }

    fn add_necessary_faces(
        &self,
        mesh_builder: &mut MeshBuilder,
        block_pos: IVec3,
        blocks: &impl BlockAccess,
        atlas: &TextureAtlas,
    ) {
        let world_pos = self.chunk_origin() + block_pos;
        let block_type = self.blocks[to_index(block_pos)];

        for dir in DIRECTIONS {
            let neighbour_world_pos = world_pos + dir.normal();

            let neighbour = blocks.get_block(neighbour_world_pos);
            let visible = neighbour.map_or(true, |b| b.is_seethrough());

            if visible {
                if let Some(texture_id) = block_type.texture_id(dir) {
                    let uvs = atlas.uvs(texture_id);
                    mesh_builder.add_face(dir, block_pos, uvs);
                }
            }
        }
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

        let idx = to_index(local);

        Some(self.blocks[idx])
    }

    pub fn chunk_origin(&self) -> IVec3 {
        IVec3::new(
            self.coord.x * CHUNK_SIZE as i32,
            0,
            self.coord.y * CHUNK_SIZE as i32,
        )
    }
}

fn to_index(vec3: IVec3) -> usize {
    return vec3.x as usize + vec3.z as usize * CHUNK_SIZE + vec3.y as usize * CHUNK_PLANE;
}

pub fn create_chunk_data(chunk_x: i32, chunk_z: i32) -> Chunk {
    let fbm = Fbm::<Perlin>::new(42)
        .set_frequency(0.5)
        .set_octaves(3)
        .set_lacunarity(2.1)
        .set_persistence(0.25);

    let mut blocks = [BlockType::Air; CHUNK_VOLUME];
    let scale = 0.02;

    for block_x in 0..CHUNK_SIZE as i32 {
        for block_z in 0..CHUNK_SIZE as i32 {
            let world_x = block_x as f64 + chunk_x as f64 * CHUNK_SIZE as f64;
            let world_z = block_z as f64 + chunk_z as f64 * CHUNK_SIZE as f64;

            let height = fbm.get([world_x as f64 * scale, world_z as f64 * scale]);
            let height = ((height + 1.0) * 0.5 * CHUNK_HEIGHT as f64) as usize;
            let height = height.clamp(0, CHUNK_HEIGHT - 1) as i32;

            for y in 0..height {
                let block = if y < CHUNK_HEIGHT as i32 / 2 {
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

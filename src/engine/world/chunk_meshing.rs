// understood

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use super::block::*;
use super::chunk::CHUNK_SIZE;
use super::chunk::*;
use crate::engine::atlas::BlockAtlas;

// reusable access pattern for ecs bevy data
#[derive(SystemParam)]
pub struct WorldBlockAccess<'w, 's> {
    chunks: Query<'w, 's, &'static Chunk>,
    map: Res<'w, ChunkMap>,
}

pub struct ChunkMeshingPlugin;

impl BlockAccess for WorldBlockAccess<'_, '_> {
    fn get_block(&self, world: IVec3) -> Option<BlockType> {
        let chunk_coord = IVec2::new(
            world.x.div_euclid(CHUNK_SIZE as i32),
            world.z.div_euclid(CHUNK_SIZE as i32),
        );

        let local = IVec3::new(
            world.x.rem_euclid(CHUNK_SIZE as i32),
            world.y,
            world.z.rem_euclid(CHUNK_SIZE as i32),
        );

        let entity = self.map.0.get(&chunk_coord)?;
        let chunk = self.chunks.get(*entity).ok()?;

        chunk.get_local(local)
    }
}

impl Plugin for ChunkMeshingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkMap>();

        app.add_systems(Update, mesh_chunks.run_if(resource_exists::<BlockAtlas>));
    }
}

fn mesh_chunks(
    mut commands: Commands,
    access: WorldBlockAccess,
    atlas: Res<BlockAtlas>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    query: Query<(Entity, &Chunk), Without<Mesh3d>>,
) {
    for (entity, chunk) in &query {
        let mesh = chunk.build_chunk_mesh(&access, &atlas.texture);

        commands
            .entity(entity)
            .insert(Mesh3d(mesh_assets.add(mesh)));

        println!("Meshed chunk at ({}, {})", chunk.x(), chunk.z());
    }
}

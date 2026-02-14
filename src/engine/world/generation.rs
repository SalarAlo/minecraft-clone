use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use super::block::*;
use super::chunk::CHUNK_SIZE;
use super::chunk::*;
use crate::engine::atlas::BlockAtlas;
use crate::engine::atlas::ChunkMaterial;

#[derive(SystemParam)]
pub struct WorldBlockAccess<'w, 's> {
    chunks: Query<'w, 's, &'static Chunk>,
    map: Res<'w, ChunkMap>,
}

pub struct WorldGenerationPlugin;

impl WorldGenerationPlugin {}

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

        let entity = self.map.map.get(&chunk_coord)?;
        let chunk = self.chunks.get(*entity).ok()?;

        chunk.get_local(local)
    }
}

impl Plugin for WorldGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkMap>();

        app.add_systems(Update, mesh_chunks.run_if(resource_exists::<BlockAtlas>));
    }
}

pub fn spawn_chunk_entity(
    commands: &mut Commands,
    material: &ChunkMaterial,
    coord: IVec2,
) -> (Entity, Chunk) {
    let chunk = Chunk::new(coord.x, coord.y);

    let world_x = coord.x * CHUNK_SIZE as i32;
    let world_z = coord.y * CHUNK_SIZE as i32;

    let entity = commands
        .spawn((
            chunk.clone(),
            MeshMaterial3d(material.0.clone()),
            Transform::from_xyz(world_x as f32, 0.0, world_z as f32),
        ))
        .id();

    (entity, chunk)
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

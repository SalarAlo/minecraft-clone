use bevy::ecs::schedule::common_conditions::resource_exists;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use super::atlas::BlockAtlas;
use super::block::*;
use super::chunk::*;
use super::chunk::{CHUNK_SIZE, create_chunk_data};

#[derive(SystemParam)]
pub struct WorldBlockAccess<'w, 's> {
    chunks: Query<'w, 's, &'static Chunk>,
    map: Res<'w, ChunkMap>,
}

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

pub struct WorldPlugin {
    pub start_size: usize,
}

#[derive(Resource, Clone)]
struct WorldConfig {
    pub start_size: usize,
}

#[derive(Resource)]
struct WorldSpawned;

#[derive(Component)]
struct World;

impl Default for WorldPlugin {
    fn default() -> Self {
        Self { start_size: 10 }
    }
}

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkMap>();

        app.insert_resource(WorldConfig {
            start_size: self.start_size,
        });

        app.add_systems(Startup, create_world);
        app.add_systems(Update, mesh_chunks.run_if(resource_exists::<WorldSpawned>));

        app.add_systems(
            Update,
            spawn_chunks
                .run_if(resource_exists::<BlockAtlas>)
                .run_if(not(resource_exists::<WorldSpawned>)),
        );
    }
}

fn create_world(mut commands: Commands) {
    commands.spawn(World);
}

#[derive(Resource)]
struct ChunksSpawned(bool);

fn spawn_chunks(
    mut commands: Commands,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
    world_config: Res<WorldConfig>,
    mut chunk_map: ResMut<ChunkMap>,
    atlas: Res<BlockAtlas>,
    spawned: Option<Res<ChunksSpawned>>,
) {
    if spawned.map(|s| s.0).unwrap_or(false) {
        return;
    }

    commands.insert_resource(ChunksSpawned(true));

    let size = world_config.start_size as i32;

    let chunk_material = material_assets.add(StandardMaterial {
        base_color_texture: Some(atlas.handle.clone()),
        perceptual_roughness: 1.0,
        metallic: 0.0,
        ..default()
    });

    for x in 0..size {
        for z in 0..size {
            let chunk_data = create_chunk_data(x, z);
            let coord = chunk_data.coord();

            let world_x = chunk_data.x() * CHUNK_SIZE as i32;
            let world_z = chunk_data.z() * CHUNK_SIZE as i32;

            let entity = commands
                .spawn((
                    chunk_data,
                    MeshMaterial3d(chunk_material.clone()),
                    Transform::from_xyz(world_x as f32, 0.0, world_z as f32),
                ))
                .id();

            chunk_map.map.insert(coord, entity);
        }
    }

    commands.insert_resource(WorldSpawned);

    println!("Spawned chunks");
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
    }
}

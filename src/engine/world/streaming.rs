use crate::engine::atlas::BlockAtlas;
use crate::engine::atlas::ChunkMaterial;
use crate::engine::world::generation::spawn_chunk_entity;
use bevy::{platform::collections::HashSet, prelude::*};

use crate::engine::world::chunk::ChunkMap;

use super::chunk::CHUNK_SIZE;

pub struct StreamingPlugin {
    pub render_distance: usize,
}

#[derive(Resource)]
struct StreamingResource {
    render_distance: usize,
}

#[derive(Resource)]
struct StreamingBudget {
    spawns_per_frame: usize,
    despawns_per_frame: usize,
}

#[derive(Resource, Default)]
struct DesiredChunks {
    set: HashSet<IVec2>,
}

#[derive(Resource, Default, PartialEq, Eq, Clone, Copy)]
struct PlayerChunk {
    current: IVec2,
    previous: IVec2,
}

#[derive(Resource, Default)]
struct SpawnQueue {
    list: Vec<IVec2>,
}

#[derive(Resource, Default)]
struct DespawnQueue {
    list: Vec<IVec2>,
}

impl Default for StreamingPlugin {
    fn default() -> Self {
        Self { render_distance: 8 }
    }
}

impl Plugin for StreamingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StreamingResource {
            render_distance: self.render_distance,
        });

        app.insert_resource(StreamingBudget {
            spawns_per_frame: 3,
            despawns_per_frame: 4,
        });

        app.insert_resource(PlayerChunk::default());
        app.insert_resource(DesiredChunks::default());

        app.insert_resource(SpawnQueue::default());
        app.insert_resource(DespawnQueue::default());

        app.add_systems(
            Update,
            (
                detect_player_chunk,
                rebuild_desired_chunks.run_if(resource_changed::<PlayerChunk>),
                reconcile_chunks.run_if(resource_changed::<DesiredChunks>),
                execute_spawns.run_if(resource_exists::<BlockAtlas>),
                execute_despawns,
            )
                .chain(),
        );
    }
}

fn detect_player_chunk(
    mut chunk_state: ResMut<PlayerChunk>,
    transform: Single<&Transform, With<Camera>>,
) {
    let world_pos = transform.translation;

    let new_chunk = IVec2::new(
        (world_pos.x as i32).div_euclid(CHUNK_SIZE as i32),
        (world_pos.z as i32).div_euclid(CHUNK_SIZE as i32),
    );

    if new_chunk != chunk_state.current {
        chunk_state.previous = chunk_state.current;
        chunk_state.current = new_chunk;
    }
}

fn rebuild_desired_chunks(
    player: Res<PlayerChunk>,
    settings: Res<StreamingResource>,
    mut desired: ResMut<DesiredChunks>,
) {
    let center = player.current;
    let r = settings.render_distance as i32;

    if player.current == player.previous && !desired.set.is_empty() {
        return;
    }

    desired.set.clear();
    desired.set.reserve(((2 * r + 1) * (2 * r + 1)) as usize);

    for x in -r..=r {
        for z in -r..=r {
            desired.set.insert(center + IVec2::new(x, z));
        }
    }
}

fn reconcile_chunks(
    desired: Res<DesiredChunks>,
    map: Res<ChunkMap>,
    mut spawn: ResMut<SpawnQueue>,
    mut despawn: ResMut<DespawnQueue>,
) {
    spawn.list.clear();
    despawn.list.clear();

    for coord in desired.set.iter() {
        if !map.map.contains_key(coord) {
            spawn.list.push(*coord);
        }
    }

    for coord in map.map.keys() {
        if !desired.set.contains(coord) {
            despawn.list.push(*coord);
        }
    }
}

fn execute_spawns(
    mut commands: Commands,
    mut spawn: ResMut<SpawnQueue>,
    mut map: ResMut<ChunkMap>,
    budget: Res<StreamingBudget>,
    chunk_material: Res<ChunkMaterial>,
) {
    let count = spawn.list.len().min(budget.spawns_per_frame);

    for _ in 0..count {
        if let Some(coord) = spawn.list.pop() {
            let (entity, _) = spawn_chunk_entity(&mut commands, &chunk_material, coord);

            map.map.insert(coord, entity);

            println!("Spawned chunk {:?}", coord);
        }
    }
}

fn execute_despawns(
    mut commands: Commands,
    mut despawn: ResMut<DespawnQueue>,
    mut map: ResMut<ChunkMap>,
    budget: Res<StreamingBudget>,
) {
    let count = despawn.list.len().min(budget.despawns_per_frame);

    for _ in 0..count {
        if let Some(coord) = despawn.list.pop() {
            if let Some(entity) = map.map.remove(&coord) {
                commands.entity(entity).despawn();
                println!("Despawned chunk {:?}", coord);
            }
        }
    }
}

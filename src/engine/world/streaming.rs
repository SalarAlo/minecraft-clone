// understood

use crate::engine::atlas::BlockAtlas;
use crate::engine::atlas::ChunkMaterial;
use crate::engine::world::chunk::Chunk;
use crate::engine::world::chunk_meshing::UnmeshedChunk;
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

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct DesiredChunkEntry {
    coord: IVec2,
    should_be_meshed: bool,
}

#[derive(Resource, Default)]
struct DesiredChunks(HashSet<DesiredChunkEntry>);

#[derive(Resource, Default, PartialEq, Eq, Clone, Copy)]
struct PlayerChunkPositionTracker {
    current: IVec2,
    previous: IVec2,
}

#[derive(Resource, Default)]
struct SpawnQueue {
    list: Vec<DesiredChunkEntry>,
}

#[derive(Resource, Default)]
struct DespawnQueue {
    list: Vec<IVec2>,
}

#[derive(Resource, Default)]
struct PromoteQueue {
    list: Vec<IVec2>,
}

impl Default for StreamingPlugin {
    fn default() -> Self {
        Self {
            render_distance: 12,
        }
    }
}

impl Plugin for StreamingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StreamingResource {
            render_distance: self.render_distance,
        });

        app.insert_resource(PlayerChunkPositionTracker::default());
        app.insert_resource(DesiredChunks::default());

        app.insert_resource(SpawnQueue::default());
        app.insert_resource(PromoteQueue::default());
        app.insert_resource(DespawnQueue::default());

        app.add_systems(
            Update,
            (
                detect_player_chunk,
                update_desired_chunk_set.run_if(resource_changed::<PlayerChunkPositionTracker>),
                reconcile_chunks.run_if(resource_changed::<DesiredChunks>),
                execute_spawns.run_if(resource_exists::<BlockAtlas>),
                execute_promotions,
                execute_despawns,
            )
                .chain(),
        );
    }
}

fn detect_player_chunk(
    mut chunk_state: ResMut<PlayerChunkPositionTracker>,
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

fn update_desired_chunk_set(
    player: Res<PlayerChunkPositionTracker>,
    settings: Res<StreamingResource>,
    mut desired: ResMut<DesiredChunks>,
) {
    let desired = &mut desired.0;
    let center = player.current;
    let r = settings.render_distance as i32;

    let player_stayed_still = player.current == player.previous;
    if player_stayed_still && !desired.is_empty() {
        return;
    }

    desired.clear();
    let side_len = (2 * r + 1) as usize;
    desired.reserve(side_len.pow(2));

    let pr = r + 1;
    for x in -pr..=pr {
        for z in -pr..=pr {
            let is_at_edge = x == pr || x == -pr || z == pr || z == -pr;

            desired.insert(DesiredChunkEntry {
                coord: center + IVec2::new(x, z),
                should_be_meshed: !is_at_edge,
            });
        }
    }
}

fn reconcile_chunks(
    desired: Res<DesiredChunks>,
    chunk_map: Res<ChunkMap>,
    mut spawn: ResMut<SpawnQueue>,
    mut despawn: ResMut<DespawnQueue>,
    mut promote: ResMut<PromoteQueue>,
    unmeshed: Query<(), With<UnmeshedChunk>>,
) {
    spawn.list.clear();
    despawn.list.clear();
    promote.list.clear();

    let desired = &desired.0;
    let chunk_map = &chunk_map.0;

    let desired_coords: HashSet<_> = desired.iter().map(|e| e.coord).collect();

    for entry in desired {
        match chunk_map.get(&entry.coord) {
            None => {
                spawn.list.push(*entry);
            }

            Some(&entity) => {
                if entry.should_be_meshed && unmeshed.contains(entity) {
                    promote.list.push(entry.coord);
                }
            }
        }
    }

    for coord in chunk_map.keys() {
        if !desired_coords.contains(coord) {
            despawn.list.push(*coord);
        }
    }
}

fn execute_spawns(
    mut commands: Commands,
    mut spawn: ResMut<SpawnQueue>,
    mut map: ResMut<ChunkMap>,
    chunk_material: Res<ChunkMaterial>,
) {
    let count = spawn.list.len();

    for _ in 0..count {
        if let Some(entry) = spawn.list.pop() {
            let entity = Chunk::new_entity(&mut commands, &chunk_material, entry.coord);

            if !entry.should_be_meshed {
                commands.entity(entity).insert(UnmeshedChunk);
            }

            map.0.insert(entry.coord, entity);
        }
    }
}

fn execute_despawns(
    mut commands: Commands,
    mut despawn: ResMut<DespawnQueue>,
    mut map: ResMut<ChunkMap>,
) {
    let count = despawn.list.len();

    for _ in 0..count {
        if let Some(coord) = despawn.list.pop() {
            if let Some(entity) = map.0.remove(&coord) {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn execute_promotions(
    mut commands: Commands,
    mut promote: ResMut<PromoteQueue>,
    map: Res<ChunkMap>,
) {
    let count = promote.list.len();

    for _ in 0..count {
        if let Some(coord) = promote.list.pop() {
            if let Some(&entity) = map.0.get(&coord) {
                commands.entity(entity).remove::<UnmeshedChunk>();
            }
        }
    }
}

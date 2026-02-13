use super::block::BlockTextureId;
use bevy::asset::RenderAssetUsages;
use bevy::image::ImageSampler;
use bevy::render::render_resource::{
    Extent3d, FilterMode, SamplerDescriptor, TextureDimension, TextureFormat,
};

use bevy::prelude::*;

use std::collections::HashMap;

pub const TILE_SIZE: u32 = 16;

pub struct AtlasPlugin;

impl Plugin for AtlasPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_texture_loading)
            .add_systems(Update, try_build_atlas);
    }
}

#[derive(Debug)]
pub struct TextureAtlas {
    tiles_per_row: u32,
    tile_uv_size: f32,
    indices: HashMap<BlockTextureId, u32>,
}

#[derive(Resource)]
pub struct PendingBlockTextures(pub HashMap<BlockTextureId, Handle<Image>>);

#[derive(Resource)]
pub struct BlockAtlas {
    pub texture: TextureAtlas,
    pub handle: Handle<Image>,
}

impl TextureAtlas {
    pub fn uvs(&self, id: BlockTextureId) -> [Vec2; 4] {
        let index = self.indices[&id];

        let x = index % self.tiles_per_row;
        let y = index / self.tiles_per_row;

        let s = self.tile_uv_size;

        let u_min = x as f32 * s;
        let u_max = u_min + s;

        let v_min = y as f32 * s;
        let v_max = v_min + s;

        [
            Vec2::new(u_max, v_max),
            Vec2::new(u_min, v_max),
            Vec2::new(u_min, v_min),
            Vec2::new(u_max, v_min),
        ]
    }
}

fn load_block_textures(asset_server: Res<AssetServer>) -> HashMap<BlockTextureId, Handle<Image>> {
    BlockTextureId::get_all()
        .iter()
        .map(|id| (*id, asset_server.load(id.path())))
        .collect()
}

pub fn setup_texture_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handles = load_block_textures(asset_server);
    commands.insert_resource(PendingBlockTextures(handles));
}

pub fn try_build_atlas(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    pending: Option<Res<PendingBlockTextures>>,
) {
    let pending = match pending {
        Some(p) => p,
        None => return,
    };

    if !pending.0.values().all(|h| images.get(h).is_some()) {
        return;
    }

    let (mut atlas_image, tiles_per_row, indices) = build_atlas(&images, &pending.0);

    atlas_image.sampler = ImageSampler::Descriptor(
        SamplerDescriptor {
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            ..default()
        }
        .into(),
    );

    atlas_image.texture_descriptor.mip_level_count = 1;

    let atlas_handle = images.add(atlas_image);

    let texture_atlas = TextureAtlas {
        tiles_per_row,
        tile_uv_size: 1.0 / tiles_per_row as f32,
        indices,
    };

    commands.insert_resource(BlockAtlas {
        texture: texture_atlas,
        handle: atlas_handle,
    });

    commands.remove_resource::<PendingBlockTextures>();
    println!("atlas built!");
}

fn build_atlas(
    images: &Assets<Image>,
    handles: &HashMap<BlockTextureId, Handle<Image>>,
) -> (Image, u32, HashMap<BlockTextureId, u32>) {
    let count = handles.len() as u32;
    let tiles_per_row = (count as f32).sqrt().ceil() as u32;

    let atlas_size = tiles_per_row * TILE_SIZE;
    let mut atlas_data = vec![0u8; (atlas_size * atlas_size * 4) as usize];

    let mut indices = HashMap::new();

    for (i, id) in BlockTextureId::get_all().into_iter().enumerate() {
        let index = i as u32;

        let x = index % tiles_per_row;
        let y = index / tiles_per_row;

        let src = images.get(&handles[&id]).unwrap();
        let src_data = src.data.as_ref().expect("Image has no CPU data");

        copy_tile(src_data, &mut atlas_data, x, y, tiles_per_row);

        indices.insert(id, index);
    }

    let image = Image::new(
        Extent3d {
            width: atlas_size,
            height: atlas_size,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        atlas_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );

    (image, tiles_per_row, indices)
}

fn copy_tile(src: &[u8], dst: &mut [u8], tile_x: u32, tile_y: u32, tiles_per_row: u32) {
    let atlas_width = tiles_per_row * TILE_SIZE;

    for row in 0..TILE_SIZE {
        let src_offset = (row * TILE_SIZE * 4) as usize;

        let dst_x = tile_x * TILE_SIZE;
        let dst_y = tile_y * TILE_SIZE + row;

        let dst_offset = ((dst_y * atlas_width + dst_x) * 4) as usize;

        dst[dst_offset..dst_offset + (TILE_SIZE * 4) as usize]
            .copy_from_slice(&src[src_offset..src_offset + (TILE_SIZE * 4) as usize]);
    }
}

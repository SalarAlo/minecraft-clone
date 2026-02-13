use crate::engine::cube::Direction;
use bevy::math::IVec3;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BlockType {
    Air = 0,
    Grass = 1,
    Dirt = 2,
    Sand = 3,
}

#[repr(u16)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, EnumIter)]
pub enum BlockTextureId {
    GrassSide = 0,
    GrassTop = 1,
    Sand = 2,
    Dirt = 3,
}

impl BlockTextureId {
    pub fn get_all() -> Vec<BlockTextureId> {
        BlockTextureId::iter().collect()
    }

    pub fn path(&self) -> String {
        let mut path = String::from("minecraft_assets/textures/block/");
        path.push_str(&String::from(match self {
            BlockTextureId::GrassTop => "grass_block_top.png",
            BlockTextureId::GrassSide => "grass_block_side.png",
            BlockTextureId::Dirt => "dirt.png",
            BlockTextureId::Sand => "sand.png",
        }));

        path
    }
}

impl BlockType {
    pub fn is_seethrough(&self) -> bool {
        match self {
            Self::Air => true,
            _ => false,
        }
    }

    pub fn texture_id(&self, face: Direction) -> Option<BlockTextureId> {
        match self {
            BlockType::Air => None,

            BlockType::Grass => Some(match face {
                Direction::Top => BlockTextureId::GrassTop,
                Direction::Bottom => BlockTextureId::Dirt,
                _ => BlockTextureId::GrassSide,
            }),

            BlockType::Dirt => Some(BlockTextureId::Dirt),
            BlockType::Sand => Some(BlockTextureId::Sand),
        }
    }
}

pub trait BlockAccess {
    fn get_block(&self, world_pos: IVec3) -> Option<BlockType>;
}

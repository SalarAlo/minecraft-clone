use crate::engine::face_direction::FaceDirection;
use bevy::math::IVec3;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BlockType {
    Air = 0,
    Grass = 1,
    Dirt = 2,
    Sand = 3,
    Bedrock = 4,
    OakWood = 5,
    OakLeaf = 6,
    Water = 7,
    Stone = 8,
    Snow = 9,
}

#[repr(u16)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, EnumIter)]
pub enum BlockTextureId {
    GrassSide = 0,
    GrassTop = 1,
    Sand = 2,
    Dirt = 3,
    Bedrock = 4,
    OakWoodSide = 5,
    OakWoodTop = 6,
    OakLeaf = 7,
    Water = 8,
    Stone = 9,
    Snow = 10,
}

impl BlockTextureId {
    pub fn get_all() -> Vec<BlockTextureId> {
        BlockTextureId::iter().collect()
    }

    pub fn path(&self) -> String {
        let path = String::from("minecraft_assets/textures/block");

        let specific = String::from(match self {
            BlockTextureId::GrassTop => "grass_block_top.png",
            BlockTextureId::GrassSide => "grass_block_side.png",
            BlockTextureId::Dirt => "dirt.png",
            BlockTextureId::Sand => "sand.png",
            BlockTextureId::Bedrock => "bedrock.png",
            BlockTextureId::OakLeaf => "pale_oak_leaves.png",
            BlockTextureId::OakWoodSide => "oak_log.png",
            BlockTextureId::OakWoodTop => "oak_log_top.png",
            BlockTextureId::Water => "water.png",
            BlockTextureId::Snow => "snow.png",
            BlockTextureId::Stone => "stone.png",
        });

        format!("{path}/{specific}")
    }
}

impl BlockType {
    pub fn is_seethrough(&self) -> bool {
        match self {
            Self::Air => true,
            _ => false,
        }
    }

    pub fn texture_id(&self, face: FaceDirection) -> Option<BlockTextureId> {
        match self {
            BlockType::Air => None,

            BlockType::Grass => Some(match face {
                FaceDirection::Top => BlockTextureId::GrassTop,
                FaceDirection::Bottom => BlockTextureId::Dirt,
                _ => BlockTextureId::GrassSide,
            }),

            BlockType::OakWood => Some(match face {
                FaceDirection::Top => BlockTextureId::OakWoodTop,
                FaceDirection::Bottom => BlockTextureId::OakWoodTop,
                _ => BlockTextureId::OakWoodSide,
            }),

            BlockType::OakLeaf => Some(BlockTextureId::OakLeaf),
            BlockType::Dirt => Some(BlockTextureId::Dirt),
            BlockType::Bedrock => Some(BlockTextureId::Bedrock),
            BlockType::Sand => Some(BlockTextureId::Sand),
            BlockType::Water => Some(BlockTextureId::Water),
            BlockType::Stone => Some(BlockTextureId::Stone),
            BlockType::Snow => Some(BlockTextureId::Snow),
        }
    }
}

pub trait BlockAccess {
    fn get_block(&self, world_pos: IVec3) -> Option<BlockType>;
}

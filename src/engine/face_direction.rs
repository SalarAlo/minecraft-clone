use bevy::prelude::*;

#[derive(Copy, Clone, Debug)]
pub enum FaceDirection {
    Right = 0,
    Left = 1,
    Top = 2,
    Bottom = 3,
    Front = 4,
    Back = 5,
}

pub const FACE_NORMALS: [[f32; 3]; 6] = [
    [1.0, 0.0, 0.0],  // Right
    [-1.0, 0.0, 0.0], // Left
    [0.0, 1.0, 0.0],  // Top
    [0.0, -1.0, 0.0], // Bottom
    [0.0, 0.0, 1.0],  // Front
    [0.0, 0.0, -1.0], // Back
];

pub const FACE_VERTICES: [[[f32; 3]; 4]; 6] = [
    // Right (+X)
    [
        [0.5, -0.5, 0.5],
        [0.5, -0.5, -0.5],
        [0.5, 0.5, -0.5],
        [0.5, 0.5, 0.5],
    ],
    // Left (-X)
    [
        [-0.5, -0.5, -0.5],
        [-0.5, -0.5, 0.5],
        [-0.5, 0.5, 0.5],
        [-0.5, 0.5, -0.5],
    ],
    // Top (+Y)
    [
        [-0.5, 0.5, 0.5],
        [0.5, 0.5, 0.5],
        [0.5, 0.5, -0.5],
        [-0.5, 0.5, -0.5],
    ],
    // Bottom (-Y)
    [
        [-0.5, -0.5, -0.5],
        [0.5, -0.5, -0.5],
        [0.5, -0.5, 0.5],
        [-0.5, -0.5, 0.5],
    ],
    // Front (+Z)
    [
        [-0.5, -0.5, 0.5],
        [0.5, -0.5, 0.5],
        [0.5, 0.5, 0.5],
        [-0.5, 0.5, 0.5],
    ],
    // Back (-Z)
    [
        [0.5, -0.5, -0.5],
        [-0.5, -0.5, -0.5],
        [-0.5, 0.5, -0.5],
        [0.5, 0.5, -0.5],
    ],
];

pub const DIRECTIONS: [FaceDirection; 6] = [
    FaceDirection::Top,
    FaceDirection::Bottom,
    FaceDirection::Left,
    FaceDirection::Right,
    FaceDirection::Front,
    FaceDirection::Back,
];

impl FaceDirection {
    pub const fn normal(self) -> IVec3 {
        match self {
            Self::Right => IVec3::X,
            Self::Left => IVec3::NEG_X,
            Self::Top => IVec3::Y,
            Self::Bottom => IVec3::NEG_Y,
            Self::Front => IVec3::Z,
            Self::Back => IVec3::NEG_Z,
        }
    }
}

use bevy::asset::RenderAssetUsages;
use bevy::mesh::Indices;
use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;

use super::cube::Direction;

#[derive(Default)]
pub struct MeshBuilder {
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    indices: Vec<u32>,
}

impl MeshBuilder {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn with_capacity_faces(face_count: usize) -> Self {
        let v = face_count * 4;
        let i = face_count * 6;
        Self {
            positions: Vec::with_capacity(v),
            normals: Vec::with_capacity(v),
            uvs: Vec::with_capacity(v),
            indices: Vec::with_capacity(i),
        }
    }

    #[inline]
    pub fn reserve_faces(&mut self, additional_faces: usize) {
        let v = additional_faces * 4;
        let i = additional_faces * 6;
        self.positions.reserve(v);
        self.normals.reserve(v);
        self.uvs.reserve(v);
        self.indices.reserve(i);
    }

    #[inline]
    pub fn add_face(&mut self, direction: Direction, pos: IVec3, face_uvs: [Vec2; 4]) {
        let base = self.positions.len() as u32;

        let px = pos.x as f32;
        let py = pos.y as f32;
        let pz = pos.z as f32;

        let n = direction.normal();
        let normal = [n.x as f32, n.y as f32, n.z as f32];

        let vs = direction.vertices();

        self.positions.extend([
            [vs[0].x + px, vs[0].y + py, vs[0].z + pz],
            [vs[1].x + px, vs[1].y + py, vs[1].z + pz],
            [vs[2].x + px, vs[2].y + py, vs[2].z + pz],
            [vs[3].x + px, vs[3].y + py, vs[3].z + pz],
        ]);

        self.normals.extend([normal; 4]);

        self.uvs.extend([
            [face_uvs[0].x, face_uvs[0].y],
            [face_uvs[1].x, face_uvs[1].y],
            [face_uvs[2].x, face_uvs[2].y],
            [face_uvs[3].x, face_uvs[3].y],
        ]);

        self.indices
            .extend([base, base + 1, base + 2, base, base + 2, base + 3]);
    }

    #[inline]
    pub fn build_mesh(self) -> Mesh {
        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        );

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs);
        mesh.insert_indices(Indices::U32(self.indices));

        mesh
    }
}

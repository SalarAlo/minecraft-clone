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
    pub fn new() -> MeshBuilder {
        MeshBuilder {
            positions: vec![],
            normals: vec![],
            uvs: vec![],
            indices: vec![],
        }
    }

    pub fn add_face(&mut self, direction: Direction, pos: IVec3, face_uvs: [Vec2; 4]) {
        let base = self.positions.len() as u32;

        let normal = direction.normal();

        for (i, v) in direction.vertices().iter().enumerate() {
            self.positions
                .push([v.x + pos.x as f32, v.y + pos.y as f32, v.z + pos.z as f32]);

            self.normals
                .push([normal.x as f32, normal.y as f32, normal.z as f32]);

            let uv = face_uvs[i];
            self.uvs.push([uv.x, uv.y]);
        }

        self.indices
            .extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }

    pub fn build_mesh(self) -> Mesh {
        let mut mesh: Mesh = Mesh::new(
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

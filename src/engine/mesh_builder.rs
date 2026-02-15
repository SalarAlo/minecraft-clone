use bevy::asset::RenderAssetUsages;
use bevy::mesh::Indices;
use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;

use crate::engine::face_direction::{FACE_NORMALS, FACE_VERTICES};

use super::face_direction::FaceDirection;

// Assumes Direction is a simple 0..=5 repr (as in your code).
// If it's not, add an explicit mapping function.

#[derive(Clone, Copy)]
struct FaceLut {
    verts: [[f32; 3]; 4],
    normal: [f32; 3],
}

// Build a single LUT to avoid multiple array lookups and bounds checks in hot code.
const FACE_LUT: [FaceLut; 6] = [
    FaceLut {
        verts: FACE_VERTICES[0],
        normal: FACE_NORMALS[0],
    },
    FaceLut {
        verts: FACE_VERTICES[1],
        normal: FACE_NORMALS[1],
    },
    FaceLut {
        verts: FACE_VERTICES[2],
        normal: FACE_NORMALS[2],
    },
    FaceLut {
        verts: FACE_VERTICES[3],
        normal: FACE_NORMALS[3],
    },
    FaceLut {
        verts: FACE_VERTICES[4],
        normal: FACE_NORMALS[4],
    },
    FaceLut {
        verts: FACE_VERTICES[5],
        normal: FACE_NORMALS[5],
    },
];

#[derive(Default)]
pub struct MeshBuilder {
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    indices: Vec<u32>,
}

impl MeshBuilder {
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

    #[inline(always)]
    pub fn add_face(&mut self, dir: FaceDirection, pos: IVec3, face_uvs: [[f32; 2]; 4]) {
        let base = self.positions.len() as u32;

        let px = pos.x as f32;
        let py = pos.y as f32;
        let pz = pos.z as f32;

        let face = unsafe { FACE_LUT.get_unchecked(dir as usize) };
        let v = &face.verts;
        let n = face.normal;

        self.positions.reserve(4);
        self.normals.reserve(4);
        self.uvs.reserve(4);
        self.indices.reserve(6);

        unsafe {
            // ---- positions (write 4) ----
            let p_len = self.positions.len();
            let p_ptr = self.positions.as_mut_ptr().add(p_len);
            self.positions.set_len(p_len + 4);
            *p_ptr.add(0) = [v[0][0] + px, v[0][1] + py, v[0][2] + pz];
            *p_ptr.add(1) = [v[1][0] + px, v[1][1] + py, v[1][2] + pz];
            *p_ptr.add(2) = [v[2][0] + px, v[2][1] + py, v[2][2] + pz];
            *p_ptr.add(3) = [v[3][0] + px, v[3][1] + py, v[3][2] + pz];

            // ---- normals (write 4) ----
            let n_len = self.normals.len();
            let n_ptr = self.normals.as_mut_ptr().add(n_len);
            self.normals.set_len(n_len + 4);
            *n_ptr.add(0) = n;
            *n_ptr.add(1) = n;
            *n_ptr.add(2) = n;
            *n_ptr.add(3) = n;

            // ---- uvs (write 4) ----
            let uv_len = self.uvs.len();
            let uv_ptr = self.uvs.as_mut_ptr().add(uv_len);
            self.uvs.set_len(uv_len + 4);
            *uv_ptr.add(0) = face_uvs[0];
            *uv_ptr.add(1) = face_uvs[1];
            *uv_ptr.add(2) = face_uvs[2];
            *uv_ptr.add(3) = face_uvs[3];

            // ---- indices (write 6) ----
            let i_len = self.indices.len();
            let i_ptr = self.indices.as_mut_ptr().add(i_len);
            self.indices.set_len(i_len + 6);
            *i_ptr.add(0) = base;
            *i_ptr.add(1) = base + 1;
            *i_ptr.add(2) = base + 2;
            *i_ptr.add(3) = base;
            *i_ptr.add(4) = base + 2;
            *i_ptr.add(5) = base + 3;
        }
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

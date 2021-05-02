use nalgebra::{Vector3, Vector2};

#[derive(Debug)]
pub struct Triangle {
    v:          Vector3<Vector3<f32>>,
    v_i32:      Vector3<Vector3<i32>>,
    color:      Vector3<Vector3<u8>>,
    tex_coords: Vector3<Vector2<i32>>,
    normals:    Vector3<Vector3<f32>>,
}

impl Triangle {
    pub fn new() -> Triangle {
        Triangle {
            v:          Default::default(),
            v_i32:      Default::default(),
            color:      Default::default(),
            tex_coords: Default::default(),
            normals:    Default::default(),
        }
    }

    pub fn set_vertices(&mut self, vertices: Vector3<Vector3<f32>>) {
        self.v = vertices;
    }

    pub fn set_vertices_i32(&mut self, vertices: Vector3<Vector3<i32>>) {
        self.v_i32 = vertices;
    }

    pub fn set_tex_coords(&mut self, tex_coords: Vector3<Vector2<i32>>) {
        self.tex_coords = tex_coords;
    }

    pub fn set_normals(&mut self, normals: Vector3<Vector3<f32>>) {
        self.normals = normals;
    }

    #[inline(always)]
    pub fn a(&self) -> Vector3<f32> {
        self.v[0]
    }
    #[inline(always)]
    pub fn b(&self) -> Vector3<f32> {
        self.v[1]
    }
    #[inline(always)]
    pub fn c(&self) -> Vector3<f32> {
        self.v[2]
    }
    #[inline(always)]
    pub fn ai(&self) -> Vector3<i32> {
        self.v_i32[0]
    }
    #[inline(always)]
    pub fn bi(&self) -> Vector3<i32> {
        self.v_i32[1]
    }
    #[inline(always)]
    pub fn ci(&self) -> Vector3<i32> {
        self.v_i32[2]
    }

    #[inline(always)]
    pub fn tex_a(&self) -> Vector2<i32> {
        self.tex_coords[0]
    }
    #[inline(always)]
    pub fn tex_b(&self) -> Vector2<i32> { self.tex_coords[1] }
    #[inline(always)]
    pub fn tex_c(&self) -> Vector2<i32> {
        self.tex_coords[2]
    }

    #[inline(always)]
    pub fn nor_a(&self) -> Vector3<f32> {
        self.normals[0]
    }
    #[inline(always)]
    pub fn nor_b(&self) -> Vector3<f32> {
        self.normals[1]
    }
    #[inline(always)]
    pub fn nor_c(&self) -> Vector3<f32> {
        self.normals[2]
    }
}
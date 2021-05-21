use std::fmt::{Display, Error, Formatter};

use nalgebra::{Matrix2, Matrix4, Unit, Vector2, Vector3, Vector4};

use crate::transform::*;

/////////////////////////////////////////////////////////////////////////////////

pub static MSAA_LEVEL: usize = 4;
pub static MSAA_OFFSET: f32 = 0.25;
pub static MSAA_SAMPLE_POS: Matrix2<Vector2<f32>> = Matrix2::new(
    Vector2::new(-MSAA_OFFSET, -MSAA_OFFSET),
    Vector2::new(MSAA_OFFSET, MSAA_OFFSET),
    Vector2::new(-MSAA_OFFSET, MSAA_OFFSET),
    Vector2::new(MSAA_OFFSET, -MSAA_OFFSET),
);

/////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct MsaaTensor {
    mask: Vector4<bool>,
    dept: Vector4<f32>,
    colo: Vector4<Vector3<f32>>,
}

impl MsaaTensor {
    pub fn new() -> MsaaTensor {
        MsaaTensor {
            mask: Vector4::repeat(false),
            dept: Vector4::repeat(f32::MIN),
            colo: Vector4::repeat(Vector3::repeat(0.0)),
        }
    }
    #[inline]
    pub fn set_mask(&mut self, idx: usize, flag: bool) {
        self.mask[idx] = flag;
    }
    #[inline]
    pub fn mask(&mut self, idx: usize) -> bool {
        self.mask[idx]
    }
    #[inline]
    pub fn set_dept(&mut self, idx: usize, value: f32) {
        self.dept[idx] = value;
    }
    #[inline]
    pub fn dept(&mut self, idx: usize) -> f32 {
        self.dept[idx]
    }
    #[inline]
    pub fn set_colo(&mut self, idx: usize, color: &Vector3<f32>) {
        self.colo[idx] = color.clone();
    }
    #[inline]
    pub fn colo(&mut self, idx: usize) -> &Vector3<f32> {
        &self.colo[idx]
    }
}

impl Display for MsaaTensor {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "[Active:{};Color:({},{},{});Depth:{}]  [Active:{};Color:({},{},{});Depth:{}]\n\
                   [Active:{};Color:({},{},{});Depth:{}]  [Active:{};Color:({},{},{});Depth:{}]",
            self.mask[0],
            self.colo[0].x,
            self.colo[0].y,
            self.colo[0].z,
            self.dept[0],
            self.mask[1],
            self.colo[1].x,
            self.colo[1].y,
            self.colo[1].z,
            self.dept[1],
            self.mask[2],
            self.colo[2].x,
            self.colo[2].y,
            self.colo[2].z,
            self.dept[2],
            self.mask[3],
            self.colo[3].x,
            self.colo[3].y,
            self.colo[3].z,
            self.dept[3]
        )
    }
}

/////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq)]
pub enum MsaaOptions {
    Disable,
    X4,
}

impl Display for MsaaOptions {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut info: &str = "";
        match self {
            MsaaOptions::Disable => info = "Disable",
            MsaaOptions::X4 => info = "4x",
        }
        write!(f, "{}", info)
    }
}

/////////////////////////////////////////////////////////////////////////////////

pub fn calc_conv() -> Matrix2<Vector2<f32>> {
    let mut conv_tmp: Matrix2<Vector2<f32>> = Default::default();
    let rotate: Matrix2<f32> = rotate_matrix2d(-26.6);
    for i in 0..4 {
        conv_tmp[i] = rotate * MSAA_SAMPLE_POS[i];
    }
    conv_tmp
}

/////////////////////////////////////////////////////////////////////////////////

pub fn bounding_box(pts: &Vector3<Vector4<f32>>, bboxmin: &mut [f32; 2], bboxmax: &mut [f32; 2]) {
    for i in 0..3 {
        for j in 0..2 {
            bboxmin[j] = bboxmin[j].min(pts[i][j] / pts[i][3]);
            bboxmax[j] = bboxmax[j].max(pts[i][j] / pts[i][3]);
        }
    }
}

/////////////////////////////////////////////////////////////////////////////////

#[inline(always)]
pub fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

/////////////////////////////////////////////////////////////////////////////////

#[inline(always)]
pub fn interior(weights: &Vector3<f32>) -> bool {
    if weights.x < 0.0 || weights.y < 0.0 || weights.z < 0.0 {
        return false;
    }
    true
}

/////////////////////////////////////////////////////////////////////////////////

#[inline(always)]
pub fn barycentric(
    a: &Vector4<f32>,
    b: &Vector4<f32>,
    c: &Vector4<f32>,
    x: f32,
    y: f32,
) -> Vector3<f32> {
    let gamma: f32 = ((b.x - a.x) * (y - a.y) - (x - a.x) * (b.y - a.y))
        / ((b.x - a.x) * (c.y - a.y) - (c.x - a.x) * (b.y - a.y));
    let beta = (x - a.x - gamma * (c.x - a.x)) / (b.x - a.x);
    let alpha = 1.0 - beta - gamma;
    Vector3::new(alpha, beta, gamma)
}

/////////////////////////////////////////////////////////////////////////////////

pub fn calc_m_model(axis: Vector3<f32>, angle: f32, scale: f32) -> Matrix4<f32> {
    let axis_unit: Unit<Vector3<f32>> = Unit::new_normalize(axis);
    let m_rotate = Matrix4::from_axis_angle(&axis_unit, degree_to_radian(angle));
    let mut m_scale = Matrix4::<f32>::identity();
    for i in 0..2 {
        m_scale[(i, i)] = scale;
    }
    m_rotate * m_scale
}

/////////////////////////////////////////////////////////////////////////////////

pub fn calc_m_camera(e: &Vector3<f32>, origin: &Vector3<f32>, up: &Vector3<f32>) -> Matrix4<f32> {
    let w: Vector3<f32> = -(origin - e).normalize();
    let u: Vector3<f32> = up.cross(&w).normalize();
    let v: Vector3<f32> = w.cross(&u).normalize();

    let mut m: Matrix4<f32> = Matrix4::<f32>::identity();
    for i in 0..3 {
        m[(0, i)] = u[i];
        m[(1, i)] = v[i];
        m[(2, i)] = w[i];
        m[(i, 3)] = -origin[i];
    }
    m
}

/////////////////////////////////////////////////////////////////////////////////

pub fn calc_m_projection(coeff: f32) -> Matrix4<f32> {
    let mut m: Matrix4<f32> = Matrix4::<f32>::identity();
    m[(3, 2)] = coeff;
    m
}

/////////////////////////////////////////////////////////////////////////////////

pub fn calc_m_viewport(width: usize, height: usize, factor: f32) -> Matrix4<f32> {
    let x = (width as f32 - width as f32 * factor) / 2.0;
    let y = (height as f32 - height as f32 * factor) / 2.0;
    let w = width as f32 * factor;
    let h = height as f32 * factor;
    let mut m: Matrix4<f32> = Matrix4::<f32>::identity();
    m[(0, 3)] = x + w / 2.0;
    m[(1, 3)] = y + h / 2.0;
    m[(2, 3)] = 255.0 / 2.0;
    m[(0, 0)] = (w - 1.0) / 2.0;
    m[(1, 1)] = (h - 1.0) / 2.0;
    m[(2, 2)] = 255.0 / 2.0;
    m
}

/////////////////////////////////////////////////////////////////////////////////

pub fn degree_to_radian(angle: f32) -> f32 {
    angle / 180.0 * std::f32::consts::PI
}

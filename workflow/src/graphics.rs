use nalgebra::{Vector2,Vector3,Matrix,Matrix4,Vector4,Matrix2};
use crate::transform::*;
use std::fmt::{Display, Formatter, Error};

/////////////////////////////////////////////////////////////////////////////////

pub static MSAA_LEVEL: usize = 4;
pub static MSAA_POS_STEP: f32 = 0.25;
pub static MSAA_SAMPLE_POS: Matrix2<Vector2<f32>> = Matrix2::new(
    Vector2::new(-MSAA_POS_STEP,-MSAA_POS_STEP),Vector2::new(MSAA_POS_STEP,MSAA_POS_STEP),
    Vector2::new(-MSAA_POS_STEP,MSAA_POS_STEP),Vector2::new(MSAA_POS_STEP,-MSAA_POS_STEP),
);

/////////////////////////////////////////////////////////////////////////////////

#[derive(Default,Clone)]
pub struct MsaaTensor {
    mask:  Vector4<bool>,
    dept:  Vector4<f32>,
    colo:  Vector4<Vector3<f32>>,
}
impl MsaaTensor {
    #[inline]
    pub fn set_mask(&mut self,idx: usize,flag: bool) { self.mask[idx] = flag; }
    #[inline]
    pub fn mask(&mut self,idx: usize) -> bool { self.mask[idx] }
    #[inline]
    pub fn set_dept(&mut self,idx: usize,value: f32) { self.dept[idx] = value; }
    #[inline]
    pub fn dept(&mut self,idx: usize) -> f32 { self.dept[idx] }
    #[inline]
    pub fn set_colo(&mut self,idx: usize,color: &Vector3<f32>) { self.colo[idx] = color.clone(); }
    #[inline]
    pub fn colo(&mut self,idx: usize) -> &Vector3<f32> { &self.colo[idx] }
}
impl Display for MsaaTensor {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "[Active:{};Color:({},{},{});Depth:{}]  [Active:{};Color:({},{},{});Depth:{}]\n\
                   [Active:{};Color:({},{},{});Depth:{}]  [Active:{};Color:({},{},{});Depth:{}]",
               self.mask[0],self.colo[0].x,self.colo[0].y,self.colo[0].z,self.dept[0],
               self.mask[1],self.colo[1].x,self.colo[1].y,self.colo[1].z,self.dept[1],
               self.mask[2],self.colo[2].x,self.colo[2].y,self.colo[2].z,self.dept[2],
               self.mask[3],self.colo[3].x,self.colo[3].y,self.colo[3].z,self.dept[3])
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
            MsaaOptions::Disable => { info = "Disable" }
            MsaaOptions::X4 => { info = "4x" }
        }
        write!(f,"{}",info)
    }
}

/////////////////////////////////////////////////////////////////////////////////

pub fn calc_conv() -> Matrix2<Vector2<f32>> {
    let mut conv_tmp: Matrix2<Vector2<f32>> = Default::default();
    let rotate: Matrix2<f32> = rotate_matrix2d(-26.6);
    for i in 0..4 {
        conv_tmp[i] = rotate*MSAA_SAMPLE_POS[i];
    }
    conv_tmp
}

/////////////////////////////////////////////////////////////////////////////////

pub fn bounding_box(pts: &Vector3<Vector4<f32>>,bboxmin:&mut [f32;2],bboxmax:&mut [f32;2]) {
    for i in 0..3 {
        for j in 0..2 {
            bboxmin[j] = bboxmin[j].min(pts[i][j]/pts[i][3]);
            bboxmax[j] = bboxmax[j].max(pts[i][j]/pts[i][3]);
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
    if weights.x<0.0 || weights.y<0.0 || weights.z<0.0 {
        return false;
    }
    true
}

/////////////////////////////////////////////////////////////////////////////////

#[inline(always)]
pub fn barycentric(a:&Vector4<f32>,b:&Vector4<f32>,c:&Vector4<f32>,x:f32,y:f32) -> Vector3<f32> {
    let gamma: f32 = ((b.x - a.x) * (y - a.y) - (x - a.x) * (b.y - a.y)) /
        ((b.x - a.x) * (c.y - a.y) - (c.x - a.x) * (b.y - a.y));
    let beta = (x - a.x - gamma * (c.x - a.x)) / (b.x - a.x);
    let alpha = 1.0 - beta - gamma;
    Vector3::new(alpha,beta,gamma)
}


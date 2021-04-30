use nalgebra::{Vector2,Vector3,Matrix,Matrix4,Vector4};
use crate::{primitives, graphics};
use crate::Objcracker;
use crate::shader::ShaderPayload;
use crate::graphics::*;
use std::fmt::{Display, Formatter, Error};

static SEG_POS: [Vector2<f32>;4] = [
    Vector2::new(0.25,0.25),Vector2::new(0.75,0.25),
    Vector2::new(0.35,0.75),Vector2::new(0.75,0.75),
];

#[derive(PartialEq)]
pub enum MsaaOptions {
    Disable,
    X4,
}
impl Display for MsaaOptions {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut info: &str = "";
        match self {
            MsaaOptions::Disable => { info = "Disabled" }
            MsaaOptions::X4 => { info = "4x" }
        }
        write!(f,"{}",info)
    }
}

pub struct Delusion {
    width      :  usize,
    height     : usize,
    modelview  : Matrix4<f32>,
    viewport   : Matrix4<f32>,
    projection : Matrix4<f32>,
    f_buffer   : Vec<u32>,
    d_buffer   : Vec<i32>,
    sample_mask: Vec<Vector4<bool>>,
    msaa       : MsaaOptions,
}

impl Delusion {
    pub fn new(width: usize, height: usize) -> Delusion {
        Delusion {
            width,
            height,
            modelview  :  Matrix4::<f32>::identity(),
            viewport   :  Matrix4::<f32>::identity(),
            projection : Matrix4::<f32>::identity(),
            f_buffer   : vec![0;width*height],
            d_buffer   : vec![i32::MIN;width*height*16+100],
            sample_mask: vec![Default::default();width*height],
            msaa       : MsaaOptions::Disable,
        }
    }

    pub fn rasterize_tri<T: ShaderPayload>(&mut self, pts: &Vector3<Vector4<f32>>, shader: &mut T, model: &Objcracker) {
        let mut bboxmin:[f32;2] = [f32::MAX, f32::MAX];
        let mut bboxmax:[f32;2] = [f32::MIN, f32::MIN];
        graphics::bounding_box(pts, &mut bboxmin, &mut bboxmax);
        if self.msaa == MsaaOptions::Disable {
            for x in bboxmin[0].ceil() as usize..bboxmax[0].ceil() as usize {
                for y in bboxmin[1].ceil() as usize..bboxmax[1].ceil() as usize {
                    let weights = barycentric(&(pts[0]/pts[0][3]),&(pts[1]/pts[1][3]),
                                                           &(pts[2]/pts[2][3]),x as f32, y as f32);
                    if interior(&weights) {
                        let z: f32 = pts[0][2]*weights.x + pts[1][2]*weights.y + pts[2][2]*weights.z;
                        let w: f32 = pts[0][3]*weights.x + pts[1][3]*weights.y + pts[2][3]*weights.z;
                        let dep: i32 = ((z/w+0.5) as i32).min(255).max(0);
                        if self.get_depth(x,y)<=dep {
                            self.set_depth(x,y,dep);
                            self.set_color(x,y,&shader.fragment(&weights,&model));
                        }
                    }
                }
            }
        }
        else {
            for x in bboxmin[0].ceil() as usize..bboxmax[0].ceil() as usize {
                for y in bboxmin[1].ceil() as usize..bboxmax[1].ceil() as usize {
                    let weights = barycentric(&(pts[0]/pts[0][3]),&(pts[1]/pts[1][3]),
                                              &(pts[2]/pts[2][3]),x as f32, y as f32);
                    if interior(&weights) {
                        let mut min_dep: i32 = i32::MAX;
                        let mut coverage: f32 = 0.0;
                        for seg in &SEG_POS {
                            let seg_weights = barycentric(&(pts[0]/pts[0][3]),&(pts[1]/pts[1][3]),
                                                      &(pts[2]/pts[2][3]),x as f32 + seg.x, y as f32 + seg.y);
                            if interior(&seg_weights) {
                                coverage += 1.0;
                                let z: f32 = pts[0][2]*weights.x + pts[1][2]*weights.y + pts[2][2]*weights.z;
                                let w: f32 = pts[0][3]*weights.x + pts[1][3]*weights.y + pts[2][3]*weights.z;
                                let dep: i32 = ((z/w+0.5) as i32).min(255).max(0);
                                if self.get_depth(x,y)<=dep {
                                    self.set_depth(x,y,dep);
                                    self.set_color(x,y,&shader.fragment(&weights,&model));
                                }
                                min_dep = dep.min(min_dep);
                            }
                        }
                        if coverage == 0.0 || self.get_depth(x,y)>min_dep {
                            continue;
                        }
                        self.set_depth(x,y,min_dep);
                        self.set_color(x,y,&(shader.fragment(&weights,&model)*(coverage/ SEG_POS.len()as f32)));
                    }
                }
            }
        }
    }

    /////////////////////////////////////////////////////////////////////////////////

    pub fn viewport(&mut self, factor: f32) {
        let x = (self.width  as f32 - self.width  as f32 * factor)/2.0;
        let y = (self.height as f32 - self.height as f32 * factor)/2.0;
        let w = self.width  as f32 * factor;
        let h = self.height as f32 * factor;

        self.viewport[(0,3)] = x+w/2.0;
        self.viewport[(1,3)] = y+h/2.0;
        self.viewport[(2,3)] = 255.0/2.0;
        self.viewport[(0,0)] = w/2.0;
        self.viewport[(1,1)] = h/2.0;
        self.viewport[(2,2)] = 255.0/2.0;
    }

    pub fn projection(&mut self, coeff: f32) {
        self.projection[(3,2)] = coeff;
    }

    pub fn lookat(&mut self, eye: &Vector3<f32>, up: &Vector3<f32>, origin: &Vector3<f32>) {
        let z: Vector3<f32> = (eye-origin).normalize();
        let x: Vector3<f32> = up.cross(&z).normalize();
        let y: Vector3<f32> = z.cross(&x).normalize();

        for i in 0..3 as usize {
            self.modelview[(0, i)] = x[i];
            self.modelview[(1, i)] = y[i];
            self.modelview[(2, i)] = z[i];
            self.modelview[(i, 3)] = -origin[i];
        }
    }

    /////////////////////////////////////////////////////////////////////////////////

    pub fn clear_frame_buff(&mut self, color: &Vector3<f32>) {
        self.f_buffer.fill(from_u8_rgb(color.x as u8, color.y as u8, color.z as u8));
    }

    pub fn clear_depth_buff(&mut self) {
        self.d_buffer.fill(i32::MIN);
    }

    pub fn set_color(&mut self, x: usize, y: usize, color: &Vector3<f32>) {
        if x >= self.width || y >= self.height {
            return;
        }
        let index = (self.height-1-y)*self.width+x;
        self.f_buffer[index] = from_u8_rgb(color.x as u8, color.y as u8, color.z as u8);
    }

    pub fn get_depth(&self, x: usize, y: usize) -> i32 {
        let idx = x + y*self.width;
        if idx >= self.width*self.height {
            return 0;
        }
        self.d_buffer[idx]
    }

    pub fn set_depth(&mut self, x: usize, y: usize, value: i32) {
        let idx = x + y*self.width;
        if idx >= self.width*self.height {
            return;
        }
        self.d_buffer[idx] = value;
    }

    /////////////////////////////////////////////////////////////////////////////////

    #[inline(always)]
    pub fn enable_msaa(&mut self, option: MsaaOptions) { self.msaa = option; }
    #[inline(always)]
    pub fn disable_msaa(&mut self) { self.msaa = MsaaOptions::Disable; }
    #[inline(always)]
    pub fn msaa(&self) -> &MsaaOptions { &self.msaa }
    #[inline(always)]
    pub fn transform(&self) -> Matrix4<f32> { self.viewport*self.projection*self.modelview }
    #[inline(always)]
    pub fn get_viewport(&self) -> &Matrix4<f32> { &self.viewport }
    #[inline(always)]
    pub fn get_projection(&self) -> &Matrix4<f32> { &self.projection }
    #[inline(always)]
    pub fn get_modelview(&self) -> &Matrix4<f32> { &self.modelview }
    #[inline(always)]
    pub fn get_frame_buff(&self) -> &Vec<u32> { &self.f_buffer }
    #[inline(always)]
    pub fn width(self) -> usize { self.width }
    #[inline(always)]
    pub fn height(self) -> usize { self.height }
}
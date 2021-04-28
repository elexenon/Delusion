use nalgebra::{
    Vector2, Vector3, Matrix, Matrix4, Vector4
};
use crate::primitives;
use crate::Objcracker;

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

pub struct Delusion {
    width:  usize,
    height: usize,
    modelview : Matrix4<f32>,
    viewport  : Matrix4<f32>,
    projection: Matrix4<f32>,
    f_buffer: Vec<u32>,
    d_buffer: Vec<i32>,
}

impl Delusion {
    pub fn new(width: usize, height: usize) -> Delusion {
        Delusion {
            width : width,
            height: height,
            modelview:  Matrix4::<f32>::identity(),
            viewport :  Matrix4::<f32>::identity(),
            projection: Matrix4::<f32>::identity(),
            f_buffer: vec![0;width*height],
            d_buffer: vec![i32::MIN;width*height],
        }
    }

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

    pub fn clear_frame_buff(&mut self, color: Vector3<f32>) {
        self.f_buffer.fill(from_u8_rgb(color.x as u8, color.y as u8, color.z as u8));
    }

    pub fn clear_depth_buff(&mut self) {
        self.d_buffer.fill(i32::MIN);
    }

    pub fn set_color(&mut self, x: usize, y: usize, color: Vector3<f32>) {
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

    #[inline(always)]
    pub fn transform(&self) -> Matrix4<f32> {
        self.viewport*self.projection*self.modelview
    }
    #[inline(always)]
    pub fn get_viewport(&self) -> &Matrix4<f32> {
        &self.viewport
    }
    #[inline(always)]
    pub fn get_projection(&self) -> &Matrix4<f32> {
        &self.projection
    }
    #[inline(always)]
    pub fn get_modelview(&self) -> &Matrix4<f32> {
        &self.modelview
    }
    #[inline(always)]
    pub fn get_frame_buff(&self) -> &Vec<u32> {
        &self.f_buffer
    }
    #[inline(always)]
    pub fn width(self) -> usize {
        self.width
    }
    #[inline(always)]
    pub fn height(self) -> usize {
        self.height
    }
}
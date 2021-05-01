use nalgebra::{Vector2,Vector3,Matrix,Matrix4,Vector4,Matrix2};
use crate::{primitives, graphics};
use crate::Objcracker;
use crate::shader::ShaderPayload;
use crate::graphics::*;
use crate::transform::rotate_matrix2d;

pub struct Delusion {
    width       : usize,
    height      : usize,
    modelview   : Matrix4<f32>,
    viewport    : Matrix4<f32>,
    projection  : Matrix4<f32>,
    f_buffer    : Vec<u32>,
    d_buffer    : Vec<f32>,
    msaa_status : MsaaOptions,
    msaa        : MsaaTensor,
}

impl Delusion {
    pub fn new(width: usize, height: usize) -> Delusion {
        Delusion {
            width,
            height,
            modelview   : Matrix4::<f32>::identity(),
            viewport    : Matrix4::<f32>::identity(),
            projection  : Matrix4::<f32>::identity(),
            f_buffer    : vec![0;width*height],
            d_buffer    : vec![f32::MIN;width*height],
            msaa_status : MsaaOptions::Disable,
            msaa        : MsaaTensor::new(width,height),
        }
    }

    pub fn rasterize_tri<T: ShaderPayload>(&mut self, pts: &Vector3<Vector4<f32>>, shader: &mut T, model: &Objcracker) {
        let mut bboxmin:[f32;2] = [f32::MAX, f32::MAX];
        let mut bboxmax:[f32;2] = [f32::MIN, f32::MIN];
        graphics::bounding_box(pts, &mut bboxmin, &mut bboxmax);
        if self.msaa_status == MsaaOptions::Disable {
            for x in bboxmin[0].ceil() as usize..bboxmax[0].ceil() as usize {
                for y in bboxmin[1].ceil() as usize..bboxmax[1].ceil() as usize {
                    let weights = barycentric(&(pts[0]/pts[0][3]),&(pts[1]/pts[1][3]),
                                                           &(pts[2]/pts[2][3]),x as f32, y as f32);
                    if interior(&weights) {
                        let z: f32 = pts[0][2]*weights.x + pts[1][2]*weights.y + pts[2][2]*weights.z;
                        let w: f32 = pts[0][3]*weights.x + pts[1][3]*weights.y + pts[2][3]*weights.z;
                        let dep: f32 = (z/w+0.5).min(255.0).max(0.0);
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
                    let ipixel: usize = x + y * self.height;
                    let mut min_dep: f32 = f32::MAX;
                    for idx in 0..MSAA_LEVEL {
                        let seg_weights = barycentric(&(pts[0]/pts[0][3]),&(pts[1]/pts[1][3]),
                                                  &(pts[2]/pts[2][3]),x as f32 + self.msaa.pos(idx).x, y as f32 + self.msaa.pos(idx).y);
                        if interior(&seg_weights) {
                            let z: f32 = pts[0][2]*seg_weights.x + pts[1][2]*seg_weights.y + pts[2][2]*seg_weights.z;
                            let w: f32 = pts[0][3]*seg_weights.x + pts[1][3]*seg_weights.y + pts[2][3]*seg_weights.z;
                            let dept: f32 = (z/w+0.5).min(255.0).max(0.0);
                            self.msaa.set_mask(ipixel,idx,true);
                            self.msaa.set_dept(ipixel,idx,dept);
                            min_dep = dept.min(min_dep);
                        }else {
                            self.msaa.set_mask(ipixel,idx,false);
                        }
                    }
                    let mut cnt:f32 = 0.0;
                    for idx in 0..MSAA_LEVEL {
                        if self.msaa.get_mask(ipixel,idx) == true {
                            cnt += 1.0;
                        }
                    }
                    if cnt == 0.0 || self.get_depth(x,y)>min_dep {
                        continue;
                    }
                    self.set_depth(x,y,min_dep);
                    self.set_color(x,y,&(shader.fragment(&weights,&model)*(cnt/ MSAA_LEVEL as f32)));
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
        self.d_buffer.fill(f32::MIN);
    }

    pub fn set_color(&mut self, x: usize, y: usize, color: &Vector3<f32>) {
        if x >= self.width || y >= self.height {
            return;
        }
        let index = (self.height-1-y)*self.width+x;
        self.f_buffer[index] = from_u8_rgb(color.x as u8, color.y as u8, color.z as u8);
    }

    pub fn get_depth(&self, x: usize, y: usize) -> f32 {
        let idx = x + y*self.width;
        if idx >= self.width*self.height {
            return 0.0;
        }
        self.d_buffer[idx]
    }

    pub fn set_depth(&mut self, x: usize, y: usize, value: f32) {
        let idx = x + y*self.width;
        if idx >= self.width*self.height {
            return;
        }
        self.d_buffer[idx] = value;
    }

    pub fn enable_msaa(&mut self, option: MsaaOptions) {
        self.msaa_status = option;
    }

    /////////////////////////////////////////////////////////////////////////////////

    #[inline]
    pub fn disable_msaa(&mut self) { self.msaa_status = MsaaOptions::Disable; }
    #[inline]
    pub fn msaa_status(&self) -> &MsaaOptions { &self.msaa_status }
    #[inline]
    pub fn transform(&self) -> Matrix4<f32> { self.viewport*self.projection*self.modelview }
    #[inline]
    pub fn get_viewport(&self) -> &Matrix4<f32> { &self.viewport }
    #[inline]
    pub fn get_projection(&self) -> &Matrix4<f32> { &self.projection }
    #[inline]
    pub fn get_modelview(&self) -> &Matrix4<f32> { &self.modelview }
    #[inline]
    pub fn get_frame_buff(&self) -> &Vec<u32> { &self.f_buffer }
    #[inline]
    pub fn width(self) -> usize { self.width }
    #[inline]
    pub fn height(self) -> usize { self.height }
}
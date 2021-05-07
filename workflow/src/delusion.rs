use crate::graphics;
use crate::graphics::*;
use crate::shader::ShaderPayload;
use crate::Objcracker;
use nalgebra::{Matrix2, Matrix4, Vector2, Vector3, Vector4};

/////////////////////////////////////////////////////////////////////////////////

pub struct Delusion {
    width: usize,
    height: usize,
    m_model: Matrix4<f32>,
    m_camera: Matrix4<f32>,
    m_viewport: Matrix4<f32>,
    m_projection: Matrix4<f32>,
    f_buffer: Vec<u32>,
    d_buffer: Vec<f32>,
    msaa_status: MsaaOptions,
    msaa_tensors: Vec<MsaaTensor>,
    conv_core: Matrix2<Vector2<f32>>,
}

impl Delusion {
    pub fn new(width: usize, height: usize) -> Delusion {
        Delusion {
            width,
            height,
            m_model: Matrix4::<f32>::identity(),
            m_camera: Default::default(),
            m_viewport: Default::default(),
            m_projection: Default::default(),
            f_buffer: vec![0; width * height],
            d_buffer: vec![f32::MIN; width * height],
            msaa_status: MsaaOptions::Disable,
            msaa_tensors: vec![MsaaTensor::new(); width * height],
            conv_core: calc_conv(),
        }
    }

    pub fn rasterize_tri<T: ShaderPayload>(
        &mut self,
        pts: &Vector3<Vector4<f32>>,
        shader: &mut T,
        model: &Objcracker,
    ) {
        let mut bboxmin: [f32; 2] = [f32::MAX, f32::MAX];
        let mut bboxmax: [f32; 2] = [f32::MIN, f32::MIN];
        graphics::bounding_box(pts, &mut bboxmin, &mut bboxmax);
        if self.msaa_status == MsaaOptions::Disable {
            for x in bboxmin[0].ceil() as usize..bboxmax[0].ceil() as usize {
                for y in bboxmin[1].ceil() as usize..bboxmax[1].ceil() as usize {
                    let weights = barycentric(
                        &(pts[0] / pts[0][3]),
                        &(pts[1] / pts[1][3]),
                        &(pts[2] / pts[2][3]),
                        x as f32,
                        y as f32,
                    );
                    if interior(&weights) {
                        let z: f32 =
                            pts[0][2] * weights.x + pts[1][2] * weights.y + pts[2][2] * weights.z;
                        let w: f32 =
                            pts[0][3] * weights.x + pts[1][3] * weights.y + pts[2][3] * weights.z;
                        let dep: f32 = (z / w + 0.5).min(255.0).max(0.0);
                        if self.get_depth(x, y) <= dep {
                            self.set_depth(x, y, dep);
                            self.set_color(x, y, &shader.fragment(&weights, &model));
                        }
                    }
                }
            }
        } else {
            for x in bboxmin[0].ceil() as usize..bboxmax[0].ceil() as usize {
                for y in bboxmin[1].ceil() as usize..bboxmax[1].ceil() as usize {
                    let ipixel: usize = x + y * self.height;
                    let tensor = &mut self.msaa_tensors[ipixel];
                    let mut hit: f32 = 0.0;

                    for idx in 0..MSAA_LEVEL {
                        let seg_weights = barycentric(
                            &(pts[0] / pts[0][3]),
                            &(pts[1] / pts[1][3]),
                            &(pts[2] / pts[2][3]),
                            x as f32 + self.conv_core[idx].x,
                            y as f32 + self.conv_core[idx].y,
                        );
                        if interior(&seg_weights) {
                            hit += 1.0;
                            let z: f32 = pts[0][2] * seg_weights.x
                                + pts[1][2] * seg_weights.y
                                + pts[2][2] * seg_weights.z;
                            let w: f32 = pts[0][3] * seg_weights.x
                                + pts[1][3] * seg_weights.y
                                + pts[2][3] * seg_weights.z;
                            let dept: f32 = (z / w + 0.5).min(255.0).max(0.0);
                            tensor.set_mask(idx, true);
                            tensor.set_dept(idx, dept);
                            tensor.set_colo(idx, &(shader.fragment(&seg_weights, &model)));
                        } else {
                            tensor.set_mask(idx, false);
                        }
                    }

                    //println!("{}\n{}",tensor,hit);

                    if hit == 0.0 {
                        continue;
                    } else if hit == 4.0 {
                        let weights = barycentric(
                            &(pts[0] / pts[0][3]),
                            &(pts[1] / pts[1][3]),
                            &(pts[2] / pts[2][3]),
                            x as f32,
                            y as f32,
                        );
                        let z: f32 =
                            pts[0][2] * weights.x + pts[1][2] * weights.y + pts[2][2] * weights.z;
                        let w: f32 =
                            pts[0][3] * weights.x + pts[1][3] * weights.y + pts[2][3] * weights.z;
                        let dep: f32 = (z / w + 0.5).min(255.0).max(0.0);
                        if self.get_depth(x, y) <= dep {
                            self.set_depth(x, y, dep);
                            self.set_color(x, y, &shader.fragment(&weights, &model));
                        }
                        continue;
                    } else {
                        let mut diffuse_blend: Vector3<f32> = Default::default();
                        let mut depth_blend: f32 = 0.0;

                        for idx in 0..MSAA_LEVEL {
                            if self.msaa_tensors[ipixel].mask(idx) == true {
                                diffuse_blend += self.msaa_tensors[ipixel].colo(idx);
                                depth_blend += self.msaa_tensors[ipixel].dept(idx);
                            }
                        }

                        depth_blend /= hit as f32;
                        if self.get_depth(x, y) > depth_blend {
                            continue;
                        }

                        self.set_depth(x, y, depth_blend);
                        self.set_color(x, y, &(diffuse_blend / hit as f32));
                    }
                }
            }
        }
    }

    /////////////////////////////////////////////////////////////////////////////////

    pub fn clear_frame_buff(&mut self, color: &Vector3<f32>) {
        self.f_buffer
            .fill(from_u8_rgb(color.x as u8, color.y as u8, color.z as u8));
    }

    pub fn clear_depth_buff(&mut self) {
        self.d_buffer.fill(f32::MIN);
    }

    pub fn set_color(&mut self, x: usize, y: usize, color: &Vector3<f32>) {
        if x >= self.width || y >= self.height {
            return;
        }
        let index = (self.height - 1 - y) * self.width + x;
        self.f_buffer[index] = from_u8_rgb(color.x as u8, color.y as u8, color.z as u8);
    }

    pub fn get_depth(&self, x: usize, y: usize) -> f32 {
        let idx = x + y * self.width;
        if idx >= self.width * self.height {
            return 0.0;
        }
        self.d_buffer[idx]
    }

    pub fn set_depth(&mut self, x: usize, y: usize, value: f32) {
        let idx = x + y * self.width;
        if idx >= self.width * self.height {
            return;
        }
        self.d_buffer[idx] = value;
    }

    /////////////////////////////////////////////////////////////////////////////////

    #[inline]
    pub fn enable_msaa(&mut self, option: MsaaOptions) {
        self.msaa_status = option;
    }
    #[inline]
    pub fn disable_msaa(&mut self) {
        self.msaa_status = MsaaOptions::Disable;
    }
    #[inline]
    pub fn msaa_status(&self) -> &MsaaOptions {
        &self.msaa_status
    }
    #[inline]
    pub fn transform(&self) -> Matrix4<f32> {
        self.m_viewport * self.m_projection * self.m_camera * self.m_model
    }
    #[inline]
    pub fn viewport(&self) -> &Matrix4<f32> {
        &self.m_viewport
    }
    #[inline]
    pub fn projection(&self) -> &Matrix4<f32> {
        &self.m_projection
    }
    #[inline]
    pub fn camera(&self) -> &Matrix4<f32> {
        &self.m_camera
    }
    #[inline]
    pub fn model(&self) -> &Matrix4<f32> {
        &self.m_model
    }
    #[inline]
    pub fn set_viewport(&mut self, m: Matrix4<f32>) {
        self.m_viewport = m;
    }
    #[inline]
    pub fn set_projection(&mut self, m: Matrix4<f32>) {
        self.m_projection = m;
    }
    #[inline]
    pub fn set_camera(&mut self, m: Matrix4<f32>) {
        self.m_camera = m;
    }
    #[inline]
    pub fn set_model(&mut self, m: Matrix4<f32>) {
        self.m_model = m;
    }
    #[inline]
    pub fn get_frame_buff(&self) -> &Vec<u32> {
        &self.f_buffer
    }
    #[inline]
    pub fn w(self) -> usize {
        self.width
    }
    #[inline]
    pub fn h(self) -> usize {
        self.height
    }
}

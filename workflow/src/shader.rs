use crate::delusion::Delusion;
use crate::transform::*;
use nalgebra::{Matrix2x3, Matrix3, Matrix4, Vector2, Vector3, Vector4};
use objcracker::Objcracker;

use std::fmt::{Display, Error, Formatter};

/////////////////////////////////////////////////////////////////////////////////

pub trait ShaderPayload {
    fn vertex(
        &mut self,
        iface: usize,
        ivert: usize,
        light: &Vector3<f32>,
        model: &Objcracker,
        render: &Delusion,
    ) -> Vector4<f32>;
    fn fragment(&mut self, weights: &Vector3<f32>, model: &Objcracker) -> Vector3<f32>;
}

/////////////////////////////////////////////////////////////////////////////////

pub struct GouraudShader {
    varying_intensity: Vector3<f32>,
    varying_uv: Matrix2x3<f32>,
}
impl GouraudShader {
    pub fn new() -> GouraudShader {
        GouraudShader {
            varying_intensity: Default::default(),
            varying_uv: Default::default(),
        }
    }
}
impl ShaderPayload for GouraudShader {
    fn vertex(
        &mut self,
        iface: usize,
        ivert: usize,
        light: &Vector3<f32>,
        model: &Objcracker,
        render: &Delusion,
    ) -> Vector4<f32> {
        self.varying_uv
            .set_column(ivert, &model.calc_uv(iface, ivert));
        self.varying_intensity[ivert] = model.calc_normal(iface, ivert).dot(light).max(0.0);
        let vt: Vector4<f32> =
            render.transform() * vec3f_to_vec4f(&model.calc_vert(iface, ivert), 1.0);
        vt
    }
    fn fragment(&mut self, weights: &Vector3<f32>, model: &Objcracker) -> Vector3<f32> {
        let intensity: f32 = self.varying_intensity.dot(&weights);
        let uv = self.varying_uv * weights;
        model.diffuse(&uv) * intensity
    }
}
impl Display for GouraudShader {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Gouraud_Shader::with texture")
    }
}

/////////////////////////////////////////////////////////////////////////////////

pub struct WeirdShader {
    varying_intensity: Vector3<f32>,
}
impl WeirdShader {
    pub fn new() -> WeirdShader {
        WeirdShader {
            varying_intensity: Default::default(),
        }
    }
}
impl ShaderPayload for WeirdShader {
    fn vertex(
        &mut self,
        iface: usize,
        ivert: usize,
        light: &Vector3<f32>,
        model: &Objcracker,
        render: &Delusion,
    ) -> Vector4<f32> {
        self.varying_intensity[ivert] = model.calc_normal(iface, ivert).dot(light).max(0.0);
        let vt: Vector4<f32> =
            render.transform() * vec3f_to_vec4f(&model.calc_vert(iface, ivert), 1.0);
        vt
    }
    fn fragment(&mut self, weights: &Vector3<f32>, model: &Objcracker) -> Vector3<f32> {
        let mut intensity: f32 = self.varying_intensity.dot(&weights);
        match intensity {
            x if x > 0.85 => intensity = 1.0,
            x if x > 0.6 => intensity = 0.8,
            x if x > 0.4 => intensity = 0.6,
            x if x > 0.3 => intensity = 0.4,
            x if x > 0.15 => intensity = 0.3,
            _ => intensity = 0.0,
        }
        Vector3::new(79.0, 147.0, 184.0) * intensity
    }
}
impl Display for WeirdShader {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Weird_Shader::without texture")
    }
}

/////////////////////////////////////////////////////////////////////////////////

pub struct PhongShaderNmSpec {
    varying_uv: Matrix2x3<f32>,
    uniform_light: Vector3<f32>,
    uniform_m: Matrix4<f32>,
    uniform_mit: Matrix4<f32>,
}
impl PhongShaderNmSpec {
    pub fn new(m: &Matrix4<f32>, mit: &Matrix4<f32>) -> PhongShaderNmSpec {
        PhongShaderNmSpec {
            varying_uv: Default::default(),
            uniform_light: Vector3::new(0.0, 0.0, 0.0),
            uniform_m: m.clone(),
            uniform_mit: mit.clone(),
        }
    }
}
impl ShaderPayload for PhongShaderNmSpec {
    fn vertex(
        &mut self,
        iface: usize,
        ivert: usize,
        light: &Vector3<f32>,
        model: &Objcracker,
        render: &Delusion,
    ) -> Vector4<f32> {
        self.uniform_light = light.clone();
        self.varying_uv
            .set_column(ivert, &model.calc_uv(iface, ivert));
        let vt: Vector4<f32> =
            render.transform() * vec3f_to_vec4f(&model.calc_vert(iface, ivert), 1.0);
        vt
    }
    fn fragment(&mut self, weights: &Vector3<f32>, model: &Objcracker) -> Vector3<f32> {
        let uv: Vector2<f32> = self.varying_uv * weights;
        let n: Vector3<f32> = (self.uniform_mit * vec3f_to_vec4f(&model.normal(&uv), 1.0))
            .xyz()
            .normalize();
        let l: Vector3<f32> = (self.uniform_m * vec3f_to_vec4f(&self.uniform_light, 1.0))
            .xyz()
            .normalize();
        let r: Vector3<f32> = (n * ((n.dot(&l)) * 2.0) - l).normalize();
        let spec: f32 = 0f32.max(r.z).powf(model.specular(&uv));
        let diff: f32 = n.dot(&l).max(0.0);
        let mut color: Vector3<f32> = model.diffuse(&uv);
        for i in 0..3 {
            color[i] = (5.0 + color[i] * (diff + spec)).min(235.0);
        }
        color
    }
}
impl Display for PhongShaderNmSpec {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Phong_Shader::with normal/specular mapping")
    }
}

/////////////////////////////////////////////////////////////////////////////////

pub struct PhongShaderNm {
    varying_uv: Matrix2x3<f32>,
    uniform_light: Vector3<f32>,
    uniform_m: Matrix4<f32>,
    uniform_mit: Matrix4<f32>,
}
impl PhongShaderNm {
    pub fn new(m: &Matrix4<f32>, mit: &Matrix4<f32>) -> PhongShaderNm {
        PhongShaderNm {
            varying_uv: Default::default(),
            uniform_light: Vector3::new(0.0, 0.0, 0.0),
            uniform_m: m.clone(),
            uniform_mit: mit.clone(),
        }
    }
}
impl ShaderPayload for PhongShaderNm {
    fn vertex(
        &mut self,
        iface: usize,
        ivert: usize,
        light: &Vector3<f32>,
        model: &Objcracker,
        render: &Delusion,
    ) -> Vector4<f32> {
        self.uniform_light = light.clone();
        self.varying_uv
            .set_column(ivert, &model.calc_uv(iface, ivert));
        let vt: Vector4<f32> =
            render.transform() * vec3f_to_vec4f(&model.calc_vert(iface, ivert), 1.0);
        vt
    }
    fn fragment(&mut self, weights: &Vector3<f32>, model: &Objcracker) -> Vector3<f32> {
        let uv: Vector2<f32> = self.varying_uv * weights;
        let n: Vector3<f32> = (self.uniform_mit * vec3f_to_vec4f(&model.normal(&uv), 1.0))
            .xyz()
            .normalize();
        let l: Vector3<f32> = (self.uniform_m * vec3f_to_vec4f(&self.uniform_light, 1.0))
            .xyz()
            .normalize();
        let diff: f32 = n.dot(&l).max(0.0);
        model.diffuse(&uv) * diff
    }
}
impl Display for PhongShaderNm {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Phong_Shader::with normal mapping")
    }
}

/////////////////////////////////////////////////////////////////////////////////

pub struct PhongShaderModel {
    varying_normal: Matrix3<f32>,
    uniform_light: Vector3<f32>,
}
impl PhongShaderModel {
    pub fn new() -> PhongShaderModel {
        PhongShaderModel {
            varying_normal: Default::default(),
            uniform_light: Vector3::new(0.0, 0.0, 0.0),
        }
    }
}
impl ShaderPayload for PhongShaderModel {
    fn vertex(
        &mut self,
        iface: usize,
        ivert: usize,
        light: &Vector3<f32>,
        model: &Objcracker,
        render: &Delusion,
    ) -> Vector4<f32> {
        self.uniform_light = light.clone();
        self.varying_normal
            .set_column(ivert, &model.calc_normal(iface, ivert));
        let vt: Vector4<f32> =
            render.transform() * vec3f_to_vec4f(&model.calc_vert(iface, ivert), 1.0);
        vt
    }
    fn fragment(&mut self, weights: &Vector3<f32>, model: &Objcracker) -> Vector3<f32> {
        let normal: Vector3<f32> = self.varying_normal * weights;
        let intensity: f32 = normal.dot(&self.uniform_light).max(0.0);
        Vector3::new(255.0, 255.0, 255.0) * intensity
    }
}
impl Display for PhongShaderModel {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Phong_Shader:::model mode")
    }
}

/////////////////////////////////////////////////////////////////////////////////

pub struct PhongShaderSpec {
    varying_normal: Matrix3<f32>,
    uniform_light: Vector3<f32>,
    varying_uv: Matrix2x3<f32>,
    uniform_m: Matrix4<f32>,
}
impl PhongShaderSpec {
    pub fn new(m: &Matrix4<f32>) -> PhongShaderSpec {
        PhongShaderSpec {
            varying_normal: Default::default(),
            uniform_light: Vector3::new(0.0, 0.0, 0.0),
            varying_uv: Default::default(),
            uniform_m: m.clone(),
        }
    }
}
impl ShaderPayload for PhongShaderSpec {
    fn vertex(
        &mut self,
        iface: usize,
        ivert: usize,
        light: &Vector3<f32>,
        model: &Objcracker,
        render: &Delusion,
    ) -> Vector4<f32> {
        self.uniform_light = light.clone();
        self.varying_normal
            .set_column(ivert, &model.calc_normal(iface, ivert));
        self.varying_uv
            .set_column(ivert, &model.calc_uv(iface, ivert));
        let vt: Vector4<f32> =
            render.transform() * vec3f_to_vec4f(&model.calc_vert(iface, ivert), 1.0);
        vt
    }
    fn fragment(&mut self, weights: &Vector3<f32>, model: &Objcracker) -> Vector3<f32> {
        let uv: Vector2<f32> = self.varying_uv * weights;
        let n: Vector3<f32> = self.varying_normal * weights;
        let l: Vector3<f32> = (self.uniform_m * vec3f_to_vec4f(&self.uniform_light, 1.0))
            .xyz()
            .normalize();
        let r: Vector3<f32> = (n * ((n.dot(&l)) * 2.0) - l).normalize();
        let spec: f32 = 0f32.max(r.z).powf(model.specular(&uv));
        let diff: f32 = n.dot(&l).max(0.0);
        let mut color: Vector3<f32> = model.diffuse(&uv);
        for i in 0..3 {
            color[i] = (5.0 + color[i] * (diff + spec)).min(235.0);
        }
        color
    }
}
impl Display for PhongShaderSpec {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Phong_Shader::with specular mapping.")
    }
}

/////////////////////////////////////////////////////////////////////////////////

pub struct PhongShaderDiff {
    varying_normal: Matrix3<f32>,
    uniform_light: Vector3<f32>,
    varying_uv: Matrix2x3<f32>,
}
impl PhongShaderDiff {
    pub fn new() -> PhongShaderDiff {
        PhongShaderDiff {
            varying_normal: Default::default(),
            uniform_light: Vector3::new(0.0, 0.0, 0.0),
            varying_uv: Default::default(),
        }
    }
}
impl ShaderPayload for PhongShaderDiff {
    fn vertex(
        &mut self,
        iface: usize,
        ivert: usize,
        light: &Vector3<f32>,
        model: &Objcracker,
        render: &Delusion,
    ) -> Vector4<f32> {
        self.uniform_light = light.clone();
        self.varying_normal
            .set_column(ivert, &model.calc_normal(iface, ivert));
        self.varying_uv
            .set_column(ivert, &model.calc_uv(iface, ivert));
        let vt: Vector4<f32> =
            render.transform() * vec3f_to_vec4f(&model.calc_vert(iface, ivert), 1.0);
        vt
    }
    fn fragment(&mut self, weights: &Vector3<f32>, model: &Objcracker) -> Vector3<f32> {
        let uv: Vector2<f32> = self.varying_uv * weights;
        let normal: Vector3<f32> = self.varying_normal * weights;
        let intensity: f32 = normal.dot(&self.uniform_light).max(0.0);
        model.diffuse(&uv) * intensity
    }
}
impl Display for PhongShaderDiff {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Phong_Shader:::diffuse mapping")
    }
}

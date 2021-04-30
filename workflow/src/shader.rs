use nalgebra::{
    Vector2, Vector3, Matrix, Matrix4, Vector4, Matrix2x3, Matrix3
};
use objcracker::Objcracker;
use crate::transform::*;
use crate::delusion::Delusion;

use std::fmt::{Display, Formatter, Error};

pub trait ShaderPayload {
    fn vertex(&mut self, iface: usize, ivert: usize, light: &Vector3<f32>,
              model: &Objcracker, render: &Delusion) -> Vector4<f32>;
    fn fragment(&mut self, weights: &Vector3<f32>, model: &Objcracker) -> Vector3<f32>;
}

/////////////////////////////////////////////////////////////////////////////////

pub struct GouraudShader {
    varying_intensity: Vector3<f32>,
    varying_uv       : Matrix2x3<f32>,
}
impl GouraudShader {
    pub fn new() -> GouraudShader {
        GouraudShader {
            varying_intensity: Default::default(),
            varying_uv       : Default::default(),
        }
    }
}
impl ShaderPayload for GouraudShader {
    fn vertex(&mut self, iface: usize, ivert: usize, light: &Vector3<f32>,
                         model: &Objcracker, render: &Delusion) -> Vector4<f32>
    {
        self.varying_uv.set_column(ivert, &model.calc_uv(iface, ivert));
        self.varying_intensity[ivert] =
            model.calc_normal(iface,ivert).dot(light).max(0.0);
        let vt: Vector4<f32> = render.transform()*vec3f_to_vec4f(&model.calc_vert(iface,ivert),1.0);
        vt
    }
    fn fragment(&mut self, weights: &Vector3<f32>, model: &Objcracker) -> Vector3<f32> {
        let intensity: f32 = self.varying_intensity.dot(&weights);
        let uv = self.varying_uv*weights;
        model.diffuse(&uv)*intensity
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
    fn vertex(&mut self, iface: usize, ivert: usize, light: &Vector3<f32>,
              model: &Objcracker, render: &Delusion) -> Vector4<f32>
    {
        self.varying_intensity[ivert] =
            model.calc_normal(iface,ivert).dot(light).max(0.0);
        let vt: Vector4<f32> = render.transform()*vec3f_to_vec4f(&model.calc_vert(iface,ivert),1.0);
        vt
    }
    fn fragment(&mut self, weights: &Vector3<f32>, model: &Objcracker) -> Vector3<f32> {
        let mut intensity: f32 = self.varying_intensity.dot(&weights);
        match intensity {
            x if x > 0.85 => intensity = 1.0,
            x if x > 0.6  => intensity = 0.8,
            x if x > 0.4  => intensity = 0.6,
            x if x > 0.3  => intensity = 0.4,
            x if x > 0.15 => intensity = 0.3,
            _ => intensity = 0.0
        }
        Vector3::new(79.0,147.0,184.0)*intensity
    }
}
impl Display for WeirdShader {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Weird_Shader::without texture")
    }
}

/////////////////////////////////////////////////////////////////////////////////

pub struct PhongShader {
    varying_normal : Matrix3<f32>,
    varying_uv     : Matrix2x3<f32>,
    varying_light  : Vector3<f32>,
}
impl PhongShader {
    pub fn new() -> PhongShader {
        PhongShader {
            varying_normal: Default::default(),
            varying_uv    : Default::default(),
            varying_light : Default::default(),
        }
    }
}
impl ShaderPayload for PhongShader {
    fn vertex(&mut self, iface: usize, ivert: usize, light: &Vector3<f32>,
              model: &Objcracker, render: &Delusion) -> Vector4<f32>
    {
        self.varying_light = light.clone();
        self.varying_uv.set_column(ivert, &model.calc_uv(iface, ivert));
        self.varying_normal.set_column(ivert, &model.calc_normal(iface, ivert));
        let vt: Vector4<f32> = render.transform()*vec3f_to_vec4f(&model.calc_vert(iface,ivert),1.0);
        vt
    }
    fn fragment(&mut self, weights: &Vector3<f32>, model: &Objcracker) -> Vector3<f32> {
        let uv    : Vector2<f32> = self.varying_uv*weights;
        let normal: Vector3<f32> = self.varying_normal*weights;
        let intensity: f32 = normal.dot(&self.varying_light).max(0.0);
        model.diffuse(&uv)*intensity
    }
}
impl Display for PhongShader {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Phong_Shader::with texture")
    }
}

/////////////////////////////////////////////////////////////////////////////////
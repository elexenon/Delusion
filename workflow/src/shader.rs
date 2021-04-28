use nalgebra::{
    Vector2, Vector3, Matrix, Matrix4, Vector4, Matrix2x3
};
use objcracker::Objcracker;
use crate::transform::*;
use crate::delusion::Delusion;

trait ShaderPayload {
    fn vertex(&mut self, iface: usize, ivert: usize, light: Vector3<f32>,
              model: &Objcracker, render: &Delusion) -> Vector4<f32>;
    fn fragment(&mut self, bar: Vector3<f32>, model: &Objcracker) -> Vector3<f32>;
}

struct GouraudShader {
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
    fn vertex(&mut self, iface: usize, ivert: usize, light: Vector3<f32>,
              model: &Objcracker, render: &Delusion) -> Vector4<f32>
    {
        self.varying_uv.set_column(ivert, &model.calc_uv(iface, ivert));
        self.varying_intensity[ivert] =
            model.calc_normal(iface,ivert)
                 .dot(&light)
                 .max(0.0);
        let vt: Vector4<f32> = render.transform()*vec3f_to_vec4f(&model.calc_vert(iface,ivert),1.0);
        vt
    }
    fn fragment(&mut self, bar: Vector3<f32>, model: &Objcracker) -> Vector3<f32> {
        let intensity: f32 = self.varying_intensity.dot(&bar);
        let uv = self.varying_uv*bar;
        model.diffuse(&uv)*intensity
    }
}
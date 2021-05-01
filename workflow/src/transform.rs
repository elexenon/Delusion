use nalgebra::{
    Vector2, Vector3, Matrix, Matrix4, Vector4, Matrix2
};

/////////////////////////////////////////////////////////////////////////////////

#[inline]
pub fn normalize_vec3f(a: &Vector3<f32>) -> Vector3<f32> {
    a / (a.x*a.x+a.y*a.y+a.z*a.z).sqrt()
}

/////////////////////////////////////////////////////////////////////////////////

#[inline]
pub fn vec3f_to_vec4f(v: &Vector3<f32>, w: f32) -> Vector4<f32> {
    Vector4::new(v.x, v.y, v.z, w)
}

/////////////////////////////////////////////////////////////////////////////////

#[inline]
pub fn vec4f_vec3f_homo(v: &Vector4<f32>, w: f32) -> Vector3<f32> {
    Vector3::new(v.x, v.y,w)
}

/////////////////////////////////////////////////////////////////////////////////

#[inline]
pub fn vec3i_to_vec3f(v: &Vector3<i32>) -> Vector3<f32> { Vector3::new(v.x as f32, v.y as f32, v.z as f32) }

/////////////////////////////////////////////////////////////////////////////////

#[inline]
pub fn vec4f_to_vec3f(v: &Vector4<f32>) -> Vector3<f32> {
    Vector3::new(v.x, v.y, v.z)
}

/////////////////////////////////////////////////////////////////////////////////

#[inline]
pub fn vec3f_to_vec3i(v: &Vector3<f32>) -> Vector3<i32> {
    Vector3::new((v.x + 0.5) as i32, (v.y + 0.5) as i32, (v.z + 0.5) as i32)
}

/////////////////////////////////////////////////////////////////////////////////

#[inline]
pub fn vec2i_to_vec2f(v: &Vector2<i32>) -> Vector2<f32> {
    Vector2::new(v.x as f32, v.y as f32)
}

/////////////////////////////////////////////////////////////////////////////////

#[inline]
pub fn vec2f_to_vec2i(v: &Vector2<f32>) -> Vector2<i32> { Vector2::new((v.x + 0.5) as i32, (v.y + 0.5) as i32) }

/////////////////////////////////////////////////////////////////////////////////

#[inline]
pub fn cross_product(v1: &Vector3<f32>,v2: &Vector3<f32>) -> Vector3<f32> {
    Vector3::new(v1.y*v2.z - v1.z*v2.y, v1.z*v2.x - v1.x*v2.z, v1.x*v2.y - v1.y*v2.x)
}

/////////////////////////////////////////////////////////////////////////////////

pub fn rotate_matrix2d(angle: f32) -> Matrix2<f32> {
    let theta: f32 = (angle/180.0)*std::f32::consts::PI;
    Matrix2::new(
        theta.cos(),-theta.sin(),
        theta.sin(),theta.cos(),
    )
}

/////////////////////////////////////////////////////////////////////////////////

#[inline]
pub fn vec2f_to_vec2u(v: &Vector2<f32>) -> Vector2<u32> {
    Vector2::new(v.x as u32, v.y as u32)
}

/////////////////////////////////////////////////////////////////////////////////

#[inline]
pub fn vec3f_to_vec2f(v: &Vector3<f32>) -> Vector2<f32> {
    Vector2::new(v.x, v.y)
}

/////////////////////////////////////////////////////////////////////////////////

pub fn v34f_to_v33i(v34: &Vector3<Vector4<f32>>) -> Vector3<Vector3<i32>> {
    let mut v33: Vector3<Vector3<i32>> = Default::default();
    for i in 0..3 as usize{
        v33[i].x = (v34[i].x + 0.5) as i32;
        v33[i].y = (v34[i].y + 0.5) as i32;
        v33[i].z = (v34[i].z + 0.5) as i32;
    }
    v33
}

/////////////////////////////////////////////////////////////////////////////////

pub fn v34f_to_v33f(v34: &Vector3<Vector4<f32>>) -> Vector3<Vector3<f32>> {
    let mut v33: Vector3<Vector3<f32>> = Default::default();
    for i in 0..3 as usize{
        v33[i].x = v34[i].x;
        v33[i].y = v34[i].y;
        v33[i].z = v34[i].z;
    }
    v33
}

/////////////////////////////////////////////////////////////////////////////////

/*
TODO
pub fn inverse_transpose(m: &Matrix4<f32>) -> Matrix4<f32> {

}
*/

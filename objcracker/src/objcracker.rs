extern crate nalgebra as na;

use na::{Vector3, Vector2};
use image::{
    GenericImageView,
    DynamicImage,
};
use std::{
  fs::File,io::{BufReader,prelude::*}
};

/////////////////////////////////////////////////////////////////////////////////

pub struct Objcracker {
    prefix      : String,
    verts       : Vec<Vector3<f32>>,
    uvs         : Vec<Vector2<f32>>,
    normals     : Vec<Vector3<f32>>,
    faces       : Vec<Vec<Vector3<usize>>>,
    diffuse_map : DynamicImage,
    normal_map  : DynamicImage,
    specular_map: DynamicImage,
    diff_w      : u32,
    diff_h      : u32,
    norm_w      : u32,
    norm_h      : u32,
    spec_w      : u32,
    spec_h      : u32,
}

impl Objcracker {
    pub fn new(prefix: &str) -> Objcracker {
        Objcracker {
            prefix      : prefix.to_string(),
            verts       : Vec::new(),
            uvs         : Vec::new(),
            normals     : Vec::new(),
            faces       : Vec::new(),
            diffuse_map : image::open(&format!("{}_diffuse.tga",prefix)).unwrap(),
            normal_map  : image::open(&format!("{}_nm.tga"     ,prefix)).unwrap(),
            specular_map: image::open(&format!("{}_spec.tga"   ,prefix)).unwrap(),
            diff_w      : 0,
            diff_h      : 0,
            norm_w      : 0,
            norm_h      : 0,
            spec_w      : 0,
            spec_h      : 0,
        }
    }

    pub fn interpret(&mut self) -> std::io::Result<()> {
        let file = File::open(&format!("{}.obj",self.prefix))?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;

        for line in contents.lines() {
            if line.starts_with("v ") {
                let tmp: Vec<&str> = line.split_whitespace().collect();
                let mut v: Vector3<f32> = Vector3::new(0.0,0.0,0.0);
                v.x = tmp[1].parse().unwrap();
                v.y = tmp[2].parse().unwrap();
                v.z = tmp[3].parse().unwrap();
                self.verts.push(v);
            } else if line.starts_with("vt") {
                let tmp: Vec<&str> = line.split_whitespace().collect();
                let mut vt: Vector2<f32> = Vector2::new(0.0,0.0);
                vt.x = tmp[1].parse().unwrap();
                vt.y = tmp[2].parse().unwrap();
                self.uvs.push(vt);
            } else if line.starts_with("vn") {
                let tmp: Vec<&str> = line.split_whitespace().collect();
                let mut v: Vector3<f32> = Vector3::new(0.0,0.0,0.0);
                v.x = tmp[1].parse().unwrap();
                v.y = tmp[2].parse().unwrap();
                v.z = tmp[3].parse().unwrap();
                self.normals.push(v);
            } else if line.starts_with("f ") {
                let mut iter = line
                    .split_whitespace();
                iter.next(); // 跳过"f"
                let mut f: Vec<Vector3<usize>> = Vec::new();
                for str in iter {
                    let mut it = str.split('/');
                    f.push(Vector3::new(
                        it.next().unwrap().parse::<usize>().unwrap()-1,
                        it.next().unwrap().parse::<usize>().unwrap()-1,
                        it.next().unwrap().parse::<usize>().unwrap()-1)
                    );
                }
                self.faces.push(f);
            }
        }

        self.diff_w = self.diffuse_map.width();
        self.diff_h = self.diffuse_map.height();
        self.norm_w = self.normal_map.width();
        self.norm_h = self.normal_map.height();
        self.spec_w = self.specular_map.width();
        self.spec_h = self.specular_map.height();

        println!("*****Delusion::Debug*******");
        println!("obj_cracker::模型::{}", format!("{}.obj",self.prefix));
        println!("obj_cracker::顶点::{}", self.nverts());
        println!("obj_cracker::纹理::{}", self.nuvs());
        println!("obj_cracker::法线::{}", self.nnormals());
        println!("obj_cracker::面片::{}\n", self.nfaces());
        println!("obj_cracker::纹理::{}", format!("{}_diffuse.tga",self.prefix));
        println!("obj_cracker::分辨率::{:?}", self.diffuse_map.dimensions());
        println!("obj_cracker::通道::{:?}", self.diffuse_map.color());
        println!("obj_cracker::法线::{}", format!("{}_nm.tga",self.prefix));
        println!("obj_cracker::分辨率::{:?}", self.normal_map.dimensions());
        println!("obj_cracker::通道::{:?}", self.normal_map.color());
        println!("obj_cracker::高光::{}", format!("{}_spec.tga",self.prefix));
        println!("obj_cracker::分辨率::{:?}", self.specular_map.dimensions());
        println!("obj_cracker::通道::{:?}", self.specular_map.color());
        println!("***************************\n");
        Ok(())
    }

    /////////////////////////////////////////////////////////////////////////////////

    pub fn vert(&self, idx: usize) -> Vector3<f32> {
        self.verts[idx]
    }

    pub fn calc_vert(&self, iface: usize, ivert: usize) -> Vector3<f32> {
        let vt_idx = self.faces[iface][ivert][0] as usize;
        self.verts[vt_idx]
    }

    pub fn calc_uv(&self, iface: usize, ivert: usize) -> Vector2<f32> {
        self.uvs[self.faces[iface][ivert][1]]
    }

    pub fn calc_normal(&self, iface: usize, ivert: usize) -> Vector3<f32> {
        let vn_idx = self.faces[iface][ivert][2] as usize;
        self.normals[vn_idx].normalize()
    }

    pub fn face(&self, idx: usize) -> Vector3<usize> {
        let mut face: Vector3<usize> = Vector3::new(0,0,0);
        for i in 0..3 {
            face[i] = self.faces[idx][i][0] as usize;
        }
        face
    }

    /////////////////////////////////////////////////////////////////////////////////

    pub fn diffuse(&self, uv: &Vector2<f32>) -> Vector3<f32> {
        let x = (self.diff_w as f32 * uv.x) as u32;
        let y = self.diff_h-1-(self.diff_h as f32 * uv.y) as u32;
        if x >= self.diff_w || y >= self.diff_h {
            return Vector3::new(255.0,255.0,255.0);
        }
        let color = self.diffuse_map.get_pixel(x,y);
        Vector3::new(color[0] as f32,color[1] as f32, color[2] as f32)
    }

    pub fn normal(&self, uv: &Vector2<f32>) -> Vector3<f32> {
        let color = self.normal_map.
            get_pixel(uv.x as u32*self.norm_w, uv.y as u32*self.norm_h);
        let mut res: Vector3<f32> = Default::default();
        for i in 0..3 {
            res[2-i] = color[i] as f32/255.0*2.0-1.0;
        }
        res
    }

    pub fn specular(&self, uv: Vector2<f32>) -> f32 {
        self.specular_map
            .get_pixel(uv.x as u32*self.spec_w, uv.y as u32*self.spec_h)[0] as f32 / 1.0
    }

    /////////////////////////////////////////////////////////////////////////////////

    #[inline]
    pub fn nverts(&self) -> usize { self.verts.len() }

    #[inline]
    pub fn nnormals(&self) -> usize { self.normals.len() }

    #[inline]
    pub fn nfaces(&self) -> usize { self.faces.len() }

    #[inline]
    pub fn nuvs(&self) -> usize { self.uvs.len() }

}
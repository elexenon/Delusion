extern crate nalgebra as na;
extern crate image;
extern crate objcracker;
extern crate minifb;

mod primitives;
mod delusion;
mod transform;
mod shader;

use crate::primitives::Triangle;
use crate::transform::*;
use crate::objcracker::Objcracker;
use crate::shader::ShaderPayload;

use std::{
    cmp::min, mem, env,
    time::{
        Duration, SystemTime
    }
};
use minifb::{
    Key, Window, WindowOptions, KeyRepeat
};
use na::{Vector2, Vector3, Matrix, Matrix4, Vector4};

/////////////////////////////////////////////////////////////////////////////////

static TITLE: &str = "Delusion Renderer";

static WIDTH:  usize = 500;
static HEIGHT: usize = 500;

static UP    : Vector3<f32> = Vector3::new(0.0,1.0,0.0);
static ORIGIN: Vector3<f32> = Vector3::new(0.0,0.0,0.0);

static WHITE_COLOR   : Vector3<f32> = Vector3::new(255.0,255.0,255.0);
static CLEAR_COLOR   : Vector3<f32> = Vector3::new(5.0,5.0,5.0);
static CLEAR_COLOR_2 : Vector3<f32> = Vector3::new(79.0,147.0,184.0);

/////////////////////////////////////////////////////////////////////////////////

fn main() {
    let args: Vec<String> = env::args().collect();

    /////////////////////////////////////////////////////////////////////////////////

    let mut model = Objcracker::new(&args[1]);
    model.interpret();

    /////////////////////////////////////////////////////////////////////////////////

    let mut light: Vector3<f32> = Vector3::new(1.0,-1.0,1.0).normalize();
    let mut eye  : Vector3<f32> = Vector3::new(1.0,1.0,3.0);
    let mut shader = shader::WeirdShader::new();
    let mut clear_color: Vector3<f32> = CLEAR_COLOR;

    /////////////////////////////////////////////////////////////////////////////////

    let mut d = delusion::Delusion::new(WIDTH, HEIGHT);
    d.viewport(0.75);
    d.projection(-1.0/(eye-ORIGIN).norm());

    /////////////////////////////////////////////////////////////////////////////////

    let mut window = Window::new(TITLE, WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    while window.is_open() && !window.is_key_down(Key::Escape) {

        let frame_start_time = SystemTime::now();

        /////////////////////////////////////////////////////////////////////////////////

        d.clear_frame_buff(&clear_color);
        d.clear_depth_buff();
        d.lookat(&eye,&UP,&ORIGIN);

        for i in 0..model.nfaces() {
            let mut screen_coords: Vector3<Vector4<f32>> = Default::default();
            for j in 0..3 {
                screen_coords[j] = shader.vertex(i,j,&light,&model,&d);
                println!("{:?}", screen_coords[j]);
            }
            d.rasterize_tri(&screen_coords, &mut shader, &model);
        }

        /////////////////////////////////////////////////////////////////////////////////

        let frame_time = SystemTime::now()
            .duration_since(frame_start_time)
            .unwrap()
            .as_millis();

        window.get_keys_released().map(|keys| {
            for t in keys {
                match t {
                    Key::Left => {
                        println!("Left Pressed");
                        eye.x = eye.x - 0.8;
                    },
                    Key::Right => {
                        println!("Right Pressed");
                        eye.x = eye.x + 0.8;
                    },
                    Key::Up => {
                        println!("Up Pressed");
                        eye.y = eye.y + 0.8;
                    },
                    Key::Down => {
                        println!("Down Pressed");
                        eye.y = eye.y - 0.8;
                    },
                    Key::A => {
                        println!("W Pressed");
                        light.x = light.x - 0.5;
                    },
                    Key::D => {
                        println!("Right Pressed");
                        light.x = light.x + 0.5;
                    },
                    Key::W => {
                        println!("Up Pressed");
                        light.y = light.y + 0.5;
                    },
                    Key::S => {
                        println!("Down Pressed");
                        light.y = light.y - 0.5;
                    },
                    Key::Key1 => {
                        println!("Key1 Pressed");
                        clear_color = CLEAR_COLOR;
                    },
                    Key::Key2 => {
                        println!("Key2 Pressed");
                        clear_color = CLEAR_COLOR_2;
                    },
                    _ => (),
                }
            }
        });
        window
            .update_with_buffer(d.get_frame_buff(), WIDTH, HEIGHT)
            .unwrap();
        window
            .set_title(&format!("MSAA::OFF  {} - 帧时间:{}ms/{}fps  着色器:{}",TITLE,frame_time,1000/frame_time,shader.to_string()));
    }
}
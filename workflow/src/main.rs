extern crate nalgebra as na;
extern crate image;
extern crate objcracker;
extern crate minifb;

mod primitives;
mod delusion;
mod transform;
mod shader;

use crate::primitives::*;
use crate::transform::*;
use crate::objcracker::*;

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

static TITLE: &str = "Delusion Canvas";

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

    let mut light: Vector3<f32> = Vector3::new(1.0,-1.0,1.0);
    let mut eye  : Vector3<f32> = Vector3::new(1.0,1.0,3.0);

    /////////////////////////////////////////////////////////////////////////////////

    let mut d = delusion::Delusion::new(WIDTH, HEIGHT);
    d.lookat(&eye,&UP,&ORIGIN);
    d.viewport(0.85);
    d.projection(-1.0/(eye-ORIGIN).norm());

    /////////////////////////////////////////////////////////////////////////////////

    let mut window = Window::new(TITLE, WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let frame_start_time = SystemTime::now();

        d.clear_frame_buff(CLEAR_COLOR);
        d.clear_depth_buff();

        for i in 0..model.nfaces() {

        }

        let frame_time = SystemTime::now()
            .duration_since(frame_start_time)
            .unwrap()
            .as_millis();

        window.get_keys_released().map(|keys| {
            for t in keys {
                match t {
                    Key::A => {
                        println!("A Pressed");
                        eye.x = eye.x - 0.8;
                    },
                    Key::D => {
                        println!("D Pressed");
                        eye.x = eye.x + 0.8;
                    },
                    Key::W => {
                        println!("W Pressed");
                        eye.y = eye.y + 0.8;
                    },
                    Key::S => {
                        println!("S Pressed");
                        eye.y = eye.y - 0.8;
                    },
                    _ => (),
                }
            }
        });
        window
            .update_with_buffer(d.get_frame_buff(), WIDTH, HEIGHT)
            .unwrap();
        window
            .set_title(&format!("{} - 帧时间:{}ms", TITLE, frame_time.to_string()));
    }
}
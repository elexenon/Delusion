extern crate image;
extern crate minifb;
extern crate nalgebra as na;
extern crate objcracker;

use std::{env, time::SystemTime};

use minifb::{Key, Window, WindowOptions};
use na::{Matrix4, Vector3, Vector4};

use crate::graphics::MsaaOptions;
use crate::objcracker::Objcracker;
use crate::shader::*;
use crate::transform::*;

mod delusion;
mod graphics;
mod primitives;
mod shader;
mod transform;

/////////////////////////////////////////////////////////////////////////////////

static TITLE: &str = "Delusion Canvas";

static WIDTH: usize = 800;
static HEIGHT: usize = 800;

static UP: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
static ORIGIN: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);

static WHITE_COLOR: Vector3<f32> = Vector3::new(255.0, 255.0, 255.0);
static CLEAR_COLOR: Vector3<f32> = Vector3::new(5.0, 5.0, 5.0);
static CLEAR_COLOR_2: Vector3<f32> = Vector3::new(255.0, 255.0, 255.0);

static AXIS_X: Vector3<f32> = Vector3::new(1.0, 0.0, 0.0);
static AXIS_Y: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
static AXIS_Z: Vector3<f32> = Vector3::new(0.0, 0.0, 1.0);

/////////////////////////////////////////////////////////////////////////////////

fn main() {
    let args: Vec<String> = env::args().collect();

    /////////////////////////////////////////////////////////////////////////////////

    let mut models: Vec<Objcracker> = Vec::new();
    for i in 2..args.len() {
        let mut model = Objcracker::new(&format!("{}{}", args[1], args[i]));
        model.interpret().unwrap();
        models.push(model);
    }

    /////////////////////////////////////////////////////////////////////////////////

    let mut light: Vector3<f32> = Vector3::new(0.0, 1.0, 1.0).normalize();
    let mut eye: Vector3<f32> = Vector3::new(0.0, 1.0, 3.0);
    let mut clear_color: Vector3<f32> = WHITE_COLOR;

    let mut m_model: Matrix4<f32> = Matrix4::<f32>::identity();

    /////////////////////////////////////////////////////////////////////////////////

    let mut d = delusion::Delusion::new(WIDTH, HEIGHT);
    d.set_camera(graphics::calc_m_camera(&eye, &ORIGIN, &UP));
    d.set_viewport(graphics::calc_m_viewport(WIDTH, HEIGHT, 0.75));
    d.set_projection(graphics::calc_m_projection(-1.0 / (eye - ORIGIN).norm()));

    /////////////////////////////////////////////////////////////////////////////////

    let m = d.projection() * d.camera();
    let mit = m.try_inverse().unwrap().transpose();
    let mut shader: Box<dyn ShaderPayload> = Box::new(shader::PhongShaderNmSpec::new(&m, &mit));
    //let mut shader = shader::PhongShaderNmSpec::new(&m,&mit);
    //let mut shader = shader::PhongShaderModel::new();
    //let mut shader = shader::PhongShaderSpec::new(&m);
    //let mut shader = shader::PhongShaderDiff::new();

    /////////////////////////////////////////////////////////////////////////////////

    let mut window = Window::new(TITLE, WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let frame_start_time = SystemTime::now();

        /////////////////////////////////////////////////////////////////////////////////

        d.clear_frame_buff(&clear_color);
        d.clear_depth_buff();
        d.set_model(m_model);
        d.set_camera(graphics::calc_m_camera(&eye, &ORIGIN, &UP));

        for model in &models {
            for i in 0..model.nfaces() {
                let mut screen_coords: Vector3<Vector4<f32>> = Default::default();
                for j in 0..3 {
                    screen_coords[j] = shader.vertex(i, j, &light, &model, &d);
                }
                //println!("{:?}", screen_coords[i]);
                d.rasterize_tri(&screen_coords, &mut shader, &model);
            }
        }

        /////////////////////////////////////////////////////////////////////////////////

        let frame_time = SystemTime::now()
            .duration_since(frame_start_time)
            .unwrap()
            .as_millis();

        window.get_keys_released().map(|keys| {
            for t in keys {
                match t {
                    Key::Key1 => {
                        println!("1 Pressed");
                        shader = Box::new(WeirdShader::new());
                    }
                    Key::Key2 => {
                        println!("2 Pressed");
                        shader = Box::new(PhongShaderModel::new());
                    }
                    Key::Key3 => {
                        println!("3 Pressed");
                        shader = Box::new(GouraudShader::new());
                    }
                    Key::Key4 => {
                        shader = Box::new(PhongShaderDiff::new());
                        println!("4 Pressed");
                    }
                    Key::Key5 => {
                        shader = Box::new(PhongShaderNm::new(&m, &mit));
                        println!("5 Pressed");
                    }
                    Key::Key6 => {
                        shader = Box::new(PhongShaderSpec::new(&m));
                        println!("6 Pressed");
                    }
                    Key::Key7 => {
                        shader = Box::new(PhongShaderNmSpec::new(&m, &mit));
                        println!("7 Pressed");
                    }
                    Key::Key8 => {
                        shader = Box::new(DepthShader::new());
                        println!("8 Pressed");
                    }
                    Key::Left => {
                        println!("Left Pressed");
                        eye.x = eye.x - 0.8;
                    }
                    Key::Right => {
                        println!("Right Pressed");
                        eye.x = eye.x + 0.8;
                    }
                    Key::Up => {
                        println!("Up Pressed");
                        eye.y = eye.y + 0.8;
                    }
                    Key::Down => {
                        println!("Down Pressed");
                        eye.y = eye.y - 0.8;
                    }
                    Key::A => {
                        println!("W Pressed");
                        light = (graphics::calc_m_model(AXIS_Y, -20.0, 1.0) * vec3f_to_vec4f(&light, 1.0)).xyz();
                    }
                    Key::D => {
                        println!("Right Pressed");
                        light = (graphics::calc_m_model(AXIS_Y, 20.0, 1.0) * vec3f_to_vec4f(&light, 1.0)).xyz();
                    }
                    Key::W => {
                        println!("Up Pressed");
                        light.y = light.y + 0.5;
                    }
                    Key::S => {
                        println!("Down Pressed");
                        light.y = light.y - 0.5;
                    }
                    Key::Q => {
                        println!("Q Pressed");
                        match clear_color == CLEAR_COLOR {
                            true => clear_color = CLEAR_COLOR_2,
                            false => clear_color = CLEAR_COLOR,
                        };
                    }
                    Key::M => {
                        println!("M Pressed");
                        d.disable_msaa();
                    }
                    Key::N => {
                        println!("N Pressed");
                        d.enable_msaa(MsaaOptions::X4);
                    }
                    Key::I => {
                        println!("I Pressed");
                        m_model = graphics::calc_m_model(AXIS_X, -20.0, 1.0) * d.model();
                    }
                    Key::J => {
                        println!("J Pressed");
                        m_model = graphics::calc_m_model(AXIS_Y, -20.0, 1.0) * d.model();
                    }
                    Key::K => {
                        println!("K Pressed");
                        m_model = graphics::calc_m_model(AXIS_X, 20.0, 1.0) * d.model();
                    }
                    Key::L => {
                        println!("L Pressed");
                        m_model = graphics::calc_m_model(AXIS_Y, 20.0, 1.0) * d.model();
                    }
                    Key::Minus => {
                        println!("Minus Pressed");
                        m_model = graphics::calc_m_model(AXIS_Y, 0.0, 0.8) * d.model();
                    }
                    Key::Equal => {
                        println!("Equal Pressed");
                        m_model = graphics::calc_m_model(AXIS_Y, 0.0, 1.2) * d.model();
                    }
                    _ => (),
                }
            }
        });
        window
            .update_with_buffer(d.get_frame_buff(), WIDTH, HEIGHT)
            .unwrap();
        window.set_title(&format!(
            "{}MSAA  {} - 帧时间:{}ms/{}fps",
            d.msaa_status(),
            TITLE,
            frame_time,
            1000 / frame_time
        ));
    }
}

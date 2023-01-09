use std::fs::File;
use std::io::Write;
use float_cmp::{approx_eq, F64Margin};
use raylib::ffi::ImageFormat;
use vector::*;
use point::*;
use mat4::*;
use crate::line::Line;
use crate::math::{as_degrees, as_radians};
use crate::quaternion::Quaternion;
use crate::surface::Surface;
use crate::camera::Camera;

use raylib::prelude::*;
use cstr::cstr;
use crate::object::Object;
use crate::raycasthit::RayCastHit;

mod vector;
mod point;
mod mat4;
mod math;
mod quaternion;
mod surface;
mod line;
mod object;
mod camera;
mod raycasthit;

// globals
const WIDTH: i32 = 1000;
const HEIGHT: i32 = 1000;

const RENDER_WIDTH: i32 = 60;
const RENDER_HEIGHT: i32 = 60;
const  PIXEL_SIZE: i32 = 12;

const OFFSET: (i32, i32) = (WIDTH / 2, RENDER_HEIGHT * PIXEL_SIZE / 2);

// background color
static BG_COLOR: Color = Color {
    r: 0,
    g: 172,
    b: 210,
    a: 255,
};

pub fn save_to_file(hits: &Vec<RayCastHit>) {
    //save to file as ASCII
    let mut file = File::create("output.txt").unwrap();

    for (i, hit) in hits.iter().enumerate() {
        if hit.is_some() {
            file.write(b"0").unwrap();
        } else {
            file.write(b".").unwrap();
        }
        if (i + 1) % RENDER_WIDTH as usize == 0 {
            file.write(b"\n").unwrap();
        }

    }
}

pub fn draw_slider(d: &mut RaylibDrawHandle, text: String, x: i32, y: &mut i32, value: &f32, range: (f32, f32)) -> f32 {
    d.draw_text(text.as_str(), x, *y, 32, Color::WHITE);

    let out = d.gui_slider_bar(
        Rectangle::new((x + 175) as f32, *y as f32, 300.0, 30.0),
        None,
        None,
        *value,
        range.0, range.1);

    d.draw_text(&format!("{:.2}", out), x + 250, *y, 32, Color::DARKGRAY);
    *y += 50;
    out
}

fn main() {
    // window init
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("ray casting")
        .build();
    rl.set_target_fps(72);
    // window init

    // let mut surface = Surface::new_vw(
    //     Vector::new(30.0, 30.0, 0.0),
    //     Vector::new(1.0, 0.0, 0.0),
    //     Vector::new(0.0, 1.0, 0.0),
    //     (-15.0, 15.0),
    //     (-15.0, 15.0));

    // initialize surfaces that create a cube
    let mut front = Surface::new_vw(
        Vector::new(0.0, 0.0, 15.0),
        Vector::new(1.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        (-15.0, 15.0),
        (-15.0, 15.0),
        Vector::new(0.0, 0.0, -1.0));
    let mut back = Surface::new_vw(
        Vector::new(0.0, 0.0, -15.0),
        Vector::new(1.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        (-15.0, 15.0),
        (-15.0, 15.0),
        Vector::new(0.0, 0.0, 1.0));
    let mut left = Surface::new_vw(
        Vector::new(-15.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
        Vector::new(0.0, 1.0, 0.0),
        (-15.0, 15.0),
        (-15.0, 15.0),
        Vector::new(1.0, 0.0, 0.0));
    let mut right = Surface::new_vw(
        Vector::new(15.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
        Vector::new(0.0, 1.0, 0.0),
        (-15.0, 15.0),
        (-15.0, 15.0),
        Vector::new(-1.0, 0.0, 0.0));
    let mut top = Surface::new_vw(
        Vector::new(0.0, 15.0, 0.0),
        Vector::new(1.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
        (-15.0, 15.0),
        (-15.0, 15.0),
        Vector::new(0.0, -1.0, 0.0));
    let mut bottom = Surface::new_vw(
        Vector::new(0.0, -15.0, 0.0),
        Vector::new(1.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 1.0),
        (-15.0, 15.0),
        (-15.0, 15.0),
        Vector::new(0.0, 1.0, 0.0));

    let mut surfaces = vec![front, back, left, right, top, bottom];
    //let mut surfaces = vec![back, left, right,front];
    let mut surfaces = Object::new(surfaces);

    //let mut hits: Vec<Vec<bool>> = vec![vec![false; RENDER_HEIGHT as usize]; RENDER_WIDTH as usize];
    //let mut angles: Vec<Vec<f64>> = vec![vec![0.0; RENDER_HEIGHT as usize]; RENDER_WIDTH as usize];

    let mut hits: Vec<RayCastHit> = Vec::new();

    let mut q: Quaternion = Quaternion::identity();

    let mut camera_q: Quaternion = Quaternion::identity();

    let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
    let (mut cam_x, mut cam_y, mut cam_z) = (0.0, 0.0, 0.0);

    let mut camera_pos = Vector::new(0.0, 0.0, 20.0);

    let mut camera = Camera::new(
        camera_pos.clone(),
        Vector::new(0.0, 0.0, -1.0),
        60, 60,
        Vector::new(0.0, 1.0, 0.0),
        Vector::new(-1.0, 0.0, 0.0));

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(BG_COLOR);

        hits = camera.render(&surfaces);

        for i in (-RENDER_HEIGHT / 2)..(RENDER_HEIGHT / 2) {
            for j in (-RENDER_WIDTH / 2)..(RENDER_WIDTH / 2) {
                if i == -RENDER_WIDTH / 2 || i == RENDER_WIDTH / 2 - 1 || j == -RENDER_HEIGHT / 2 || j == RENDER_HEIGHT / 2 - 1 {
                    d.draw_rectangle((i * PIXEL_SIZE) as i32 + OFFSET.0, (j * PIXEL_SIZE) as i32 + OFFSET.1,PIXEL_SIZE as i32, PIXEL_SIZE as i32, Color::BLACK);
                }
            }
        }

        for hit in hits.iter() {
            if hit.is_some() {
                let color = Color::color_from_hsv(1.0, 1.0, (hit.angle().cos() as f32));
                let (i, j) = hit.pos_on_screen;
                d.draw_rectangle((i * PIXEL_SIZE) as i32 + OFFSET.0, (j * PIXEL_SIZE) as i32 + OFFSET.1,PIXEL_SIZE as i32, PIXEL_SIZE as i32, color);
            }
        }

        d.draw_text(&format!("Cube Control"), 205, 700, 32, Color::WHITE);
        d.draw_text(&format!("Camera Control"), 700, 550, 32, Color::WHITE);

        if d.gui_button(Rectangle::new(400.0, 900.0,100.0, 50.0), None) {
            save_to_file(&hits);
        }
        d.draw_text("save", 410, 905, 32, Color::WHITE);
        q = Quaternion::identity();

        let mut slider_height = 750;
        x = draw_slider(&mut d, "x cube rot".to_string(), 25, &mut slider_height, &mut x, (-30.0, 30.0));
        y = draw_slider(&mut d, "y cube rot".to_string(), 25, &mut slider_height, &mut y, (-30.0, 30.0));
        z = draw_slider(&mut d, "z cube rot".to_string(), 25, &mut slider_height, &mut z, (-30.0, 30.0));
        q.rotate(as_radians(x as f64), Vector::new(1.0, 0.0, 0.0));
        q.rotate(as_radians(y as f64), Vector::new(0.0, 1.0, 0.0));
        q.rotate(as_radians(z as f64), Vector::new(0.0, 0.0, 1.0));

        let mut slider_height = 600;

        cam_x = draw_slider(&mut d, "x cam rot".to_string(), 520, &mut slider_height, &mut cam_x, (-180.0, 180.0));
        cam_y = draw_slider(&mut d, "y cam rot".to_string(), 520, &mut slider_height, &mut cam_y, (-180.0, 180.0));
        cam_z = draw_slider(&mut d, "z cam rot".to_string(), 520, &mut slider_height, &mut cam_z, (-180.0, 180.0));
        camera_q = Quaternion::identity();
        camera_q.rotate(as_radians(cam_x as f64), Vector::new(1.0, 0.0, 0.0));
        camera_q.rotate(as_radians(cam_y as f64), Vector::new(0.0, 1.0, 0.0));
        camera_q.rotate(as_radians(cam_z as f64), Vector::new(0.0, 0.0, 1.0));
        camera_pos.x = draw_slider(&mut d, "x cam pos".to_string(), 520, &mut slider_height, &mut (camera_pos.x as f32), (-30.0, 30.0)) as f64;
        camera_pos.y = draw_slider(&mut d, "y cam pos".to_string(), 520, &mut slider_height, &mut (camera_pos.y as f32), (-30.0, 30.0)) as f64;
        camera_pos.z = draw_slider(&mut d, "z cam pos".to_string(), 520, &mut slider_height, &mut (camera_pos.z as f32), (-30.0, 30.0)) as f64;

        camera.set_camera_rotation(&camera_q);
        camera.set_camera_position(&camera_pos);

        surfaces.rotate(&q);
        //println!("v: {}, w: {}, n: {}", surface.v.unwrap().to_string(), surface.w.unwrap().to_string(), surface.normal.to_string());

    }
}
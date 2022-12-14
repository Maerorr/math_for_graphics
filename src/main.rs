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
const WIDTH: i32 = 1850;
const HEIGHT: i32 = 750;

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

pub fn display_debug(c: &Camera) {
    println!("{}", c.get_debug_info());
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

    let mut v1 = Vector::new(1.0, 0.0, 0.0);
    let mut v2 = Vector::new(-1.0, 0.0, 0.0);
    let mut v3 = Vector::new(2.0, 0.0, 0.0);
    let mut v4 = v1.clone();
    println!("v1.dot(v2) {}", v1.dot(&v2));
    println!("v1.dot(v3) {}", v1.dot(&v3));
    println!("v1.cross(v2) {}", v1.cross(&v4).to_string());
    println!("v1.cross(v3) {}", v1.cross(&v3).to_string());



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
    //surfaces.scale(&2.0);

    //let mut hits: Vec<Vec<bool>> = vec![vec![false; RENDER_HEIGHT as usize]; RENDER_WIDTH as usize];
    //let mut angles: Vec<Vec<f64>> = vec![vec![0.0; RENDER_HEIGHT as usize]; RENDER_WIDTH as usize];

    let mut hits: Vec<RayCastHit> = Vec::new();

    let mut q: Quaternion = Quaternion::identity();

    let mut camera_q: Quaternion = Quaternion::identity();

    let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
    let (mut cam_x, mut cam_y, mut cam_z) = (0.0, 0.0, 0.0);

    let mut camera_pos = Vector::new(0.0, 0.0, 50.0);

    let mut camera = Camera::new(
        camera_pos.clone(),
        Vector::new(0.0, 0.0, -1.0),
        RENDER_WIDTH, RENDER_HEIGHT,
        Vector::new(0.0, 1.0, 0.0),
        Vector::new(1.0, 0.0, 0.0));

    let mut first_frame: bool = true;

    let mut cube_color: Color = Color::new(255, 0, 0, 255);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(BG_COLOR);

        hits = camera.render(&surfaces);

        for hit in hits.iter() {
            if hit.is_some() {
                // the hit color value calculated to be sqrt( sin(|angle|) )
                let color_value = {
                    let angle_cos = hit.angle().cos();
                    if angle_cos >= 0.0 {
                        angle_cos.sqrt()
                    } else {
                        angle_cos.abs().sqrt()
                    }
                };

                //let color = Color::color_from_hsv(1.0, 1.0, color_value as f32);
                let color = Color::new(
                    ((color_value) * cube_color.r as f64) as u8,
                    ((color_value) * cube_color.g as f64) as u8,
                    ((color_value) * cube_color.b as f64) as u8,
                    255);
                let (i, mut j) = hit.pos_on_screen;
                j = -j;
                d.draw_rectangle((i * PIXEL_SIZE) as i32 + OFFSET.0, (j * PIXEL_SIZE) as i32 + OFFSET.1,PIXEL_SIZE as i32, PIXEL_SIZE as i32, color);
            }
        }

        for i in (-RENDER_HEIGHT / 2)..(RENDER_HEIGHT / 2 + 1) {
            for j in (-RENDER_WIDTH / 2)..(RENDER_WIDTH / 2 + 1) {
                if i == -RENDER_WIDTH / 2 || i == RENDER_WIDTH / 2 || j == -RENDER_HEIGHT / 2 || j == RENDER_HEIGHT / 2 {
                    d.draw_rectangle((i * PIXEL_SIZE) as i32 + OFFSET.0, (j * PIXEL_SIZE) as i32 + OFFSET.1,PIXEL_SIZE as i32, PIXEL_SIZE as i32, Color::BLACK);
                }
            }
        }

        d.draw_text(&format!("Cube Control"), 125, 20, 32, Color::WHITE);
        d.draw_text(&format!("Camera Control"), 1450, 20, 32, Color::WHITE);


        q = Quaternion::identity();

        let mut slider_height = 75;

        d.draw_line_ex(
            Vector2::new(25.0, slider_height as f32),
            Vector2::new(535.0, slider_height as f32), 4.0, Color::BLACK);

        slider_height += 20;

        x = draw_slider(&mut d, "x cube rot".to_string(), 25, &mut slider_height, &mut x, (-10.0, 10.0));
        y = draw_slider(&mut d, "y cube rot".to_string(), 25, &mut slider_height, &mut y, (-10.0, 10.0));
        z = draw_slider(&mut d, "z cube rot".to_string(), 25, &mut slider_height, &mut z, (-10.0, 10.0));
        q.rotate(as_radians(x as f64), Vector::new(1.0, 0.0, 0.0));
        q.rotate(as_radians(y as f64), Vector::new(0.0, 1.0, 0.0));
        q.rotate(as_radians(z as f64), Vector::new(0.0, 0.0, 1.0));

        slider_height += 20;

        d.draw_line_ex(
            Vector2::new(25.0, slider_height as f32),
            Vector2::new(535.0, slider_height as f32), 4.0, Color::BLACK);

        surfaces.rotate(&q);

        slider_height += 20;

        if d.gui_button(Rectangle::new(400.0, slider_height as f32,100.0, 50.0), None) {
            save_to_file(&hits);
        }
        d.draw_text("save", 410, slider_height + 5, 32, Color::WHITE);

        d.draw_text("Cube Color", 100, slider_height + 50, 32, Color::WHITE);

        slider_height += 100;

        cube_color = d.gui_color_picker(Rectangle::new(25., slider_height as f32, 300., 300.), cube_color);

        let mut slider_height = 75;

        d.draw_line_ex(
            Vector2::new(1300.0, slider_height as f32),
            Vector2::new(1830.0, slider_height as f32), 4.0, Color::BLACK);

        slider_height += 20;

        cam_x = draw_slider(&mut d, "x cam rot".to_string(), 1350, &mut slider_height, &mut cam_x, (-180.0, 180.0));
        cam_y = draw_slider(&mut d, "y cam rot".to_string(), 1350, &mut slider_height, &mut cam_y, (-180.0, 180.0));
        cam_z = draw_slider(&mut d, "z cam rot".to_string(), 1350, &mut slider_height, &mut cam_z, (-180.0, 180.0));
        camera_q = Quaternion::identity();
        camera_q.rotate(as_radians(cam_x as f64), Vector::new(1.0, 0.0, 0.0));
        camera_q.rotate(as_radians(cam_y as f64), Vector::new(0.0, 1.0, 0.0));
        camera_q.rotate(as_radians(cam_z as f64), Vector::new(0.0, 0.0, 1.0));

        slider_height += 20;

        d.draw_line_ex(
            Vector2::new(1300.0, slider_height as f32),
            Vector2::new(1830.0, slider_height as f32), 4.0, Color::BLACK);

        slider_height += 20;

        camera_pos.x = draw_slider(&mut d, "x cam pos".to_string(), 1350, &mut slider_height, &mut (camera_pos.x as f32), (-30.0, 30.0)) as f64;
        camera_pos.y = draw_slider(&mut d, "y cam pos".to_string(), 1350, &mut slider_height, &mut (camera_pos.y as f32), (-30.0, 30.0)) as f64;
        camera_pos.z = draw_slider(&mut d, "z cam pos".to_string(), 1350, &mut slider_height, &mut (camera_pos.z as f32), (-120.0, 120.0)) as f64;

        camera.set_camera_rotation(&camera_q);
        camera.set_camera_position(&camera_pos);

        slider_height += 20;

        d.draw_line_ex(
            Vector2::new(1300.0, slider_height as f32),
            Vector2::new(1830.0, slider_height as f32), 4.0, Color::BLACK);
        slider_height += 20;

        camera.backface_culling = d.gui_check_box(Rectangle::new(1750.0, slider_height as f32,50.0, 50.0), None, camera.backface_culling);

        d.draw_text("Backface Culling", 1410, slider_height + 5, 32, Color::WHITE);

        //println!("v: {}, w: {}, n: {}", surface.v.unwrap().to_string(), surface.w.unwrap().to_string(), surface.normal.to_string());

    }
}
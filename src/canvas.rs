//! Draw object on a canvas

use crate::vec_space::*;
use macroquad::prelude::*;
use std::sync::{Mutex, OnceLock};

const Z_DEFAULT: f64 = 0.95;

pub struct Canvas {
    view_angle: f64,
    view_xy: VecSpace,
    view_z: f64,
    grid_z: f64,
    maximal_orbit_radius: f64,
}

impl Canvas {
    pub fn new() -> Canvas {
        Canvas {
            view_angle: 0.,
            view_xy: VecSpace::ZERO,
            view_z: Z_DEFAULT,
            grid_z: Z_DEFAULT,
            maximal_orbit_radius: 1.,
        }
    }

    fn start(&mut self) {
        clear_background(BLACK);
        self.grid();
    }

    fn add_view(&mut self, x: f64, y: f64, z: f64, a: f64) {
        self.view_xy.add_x(x);
        self.view_xy.add_y(y);
        self.view_z *= 1. + 1. * z;
        self.view_angle += a;
    }

    fn set_maximal_orbit_radius(&mut self, val: f64) {
        self.maximal_orbit_radius = val;
    }

    fn circle(&self, position: VecSpace, diameter: f64, color: Color) {
        const DRAW_FACT: f64 = 5.;
        const DRAW_MIN: f64 = 3.;
        const DRAW_MAX: f64 = 200.;

        let size = diameter / DRAW_FACT; // Not * self.z_view: zooming will NOT change the size ...
        let size = size.clamp(DRAW_MIN, DRAW_MAX) as f32;
        let (x, y) = self.to_pixel(position);

        macroquad::shapes::draw_circle(x, y, size, color);
    }

    fn rectangle(&self, position: VecSpace, color: Color) {
        let (x, y) = self.to_pixel(position);
        macroquad::shapes::draw_rectangle(x, y, 1., 1., color);
    }

    fn hud(&self, text: &String, ship_speed: f64) {
        draw_text(
            format!("{} v:{}", text, ship_speed as i32).as_str(),
            20.0,
            20.0,
            30.0,
            DARKGRAY,
        );
    }

    fn grid(&mut self) {
        if self.view_z > self.grid_z {
            self.grid_z *= 2.0;
            // println!("z_draw: {}", &masses.z_grid);
        }
        if self.view_z < self.grid_z {
            self.grid_z /= 2.0;
            // println!("z_draw: {}", &masses.z_grid);
        }

        let max = 4. * self.maximal_orbit_radius / self.grid_z;
        //self.from_pixel(self.window_with, self.window_height).x();
        //println!("{}", max);
        let step = max / 100.;

        // todo??? on loop for x and y
        // vertial lines x spread
        let mut x = -max;
        let mut sub = 0;
        loop {
            let (beg_x, beg_y) = self.to_pixel(VecSpace::new(x, max));
            let (end_x, end_y) = self.to_pixel(VecSpace::new(x, -max));
            draw_line(beg_x, beg_y, end_x, end_y, 1., line_color(sub));

            sub += 1;
            x += step;
            if x > max {
                break;
            }
        }

        // horizontal lines, y spread
        let mut y = -max;
        let mut sub = 0;
        loop {
            let (beg_x, beg_y) = self.to_pixel(VecSpace::new(max, y));
            let (end_x, end_y) = self.to_pixel(VecSpace::new(-max, y));
            draw_line(beg_x, beg_y, end_x, end_y, 1., line_color(sub));

            sub += 1;
            y += step;
            if y > max {
                break;
            }
        }
    }

    ///////////////// local functions ///////////////////////

    /// calculate the pixel position from the simulated metric values
    /// by maximal orbit and screen-view values
    fn to_pixel(&self, position: VecSpace) -> (f32, f32) {
        let window_center: VecSpace = VecSpace::new(
            screen_width() as f64 / 2.,
            screen_height() as f64 / 2.,
            //self.window_width as f64 / 2.,
            //self.window_height as f64 / 2.,
        );

        // Die kleinere Fenster-Ausdehnung zählt als normaler darstellbar Bildpunktebereich
        // The smallest extend of the window counts as visible screen range
        let max_pixel_from_center = screen_width().min(screen_height()) as i32 / 2;

        let scale = self.view_z / self.maximal_orbit_radius * max_pixel_from_center as f64;

        // Scale by view, divide by scene multiply by screen, add screen center
        let mut screen_pos = position * scale + self.view_xy;
        if self.view_angle != 0. {
            screen_pos = screen_pos.rotate_angle(self.view_angle.to_radians());
        }
        screen_pos += window_center;

        (screen_pos.x() as f32, screen_pos.y() as f32)
    }
}

/// Create a colour for main or intermediate lines
/// Every 5th line is medium and bright, alternating
/// The others are dark
fn line_color(sub: i16) -> Color {
    const LIGHT: f32 = 0.3;
    const MEDIUM: f32 = 0.25;
    const DARK: f32 = 0.2;
    let mut a: f32 = DARK;
    if sub % 5 == 0 {
        if sub % 10 == 0 { a = LIGHT } else { a = MEDIUM }
    }
    Color {
        r: 0.,
        g: 1.,
        b: 0.,
        a,
    }
}

/////////// global Canvas instance //////////

static CANVAS: OnceLock<Mutex<Canvas>> = OnceLock::new();

fn canvas() -> &'static Mutex<Canvas> {
    CANVAS.get_or_init(|| Mutex::new(Canvas::new()))
}

pub fn draw_reset() {
    let mut canvas = canvas().lock().unwrap();
    canvas.view_angle = 0.;
    canvas.view_xy = VecSpace::ZERO;
    canvas.view_z = Z_DEFAULT;
    canvas.grid_z = Z_DEFAULT;
    canvas.maximal_orbit_radius = 1.;
}

pub fn draw_circle(position: VecSpace, diameter: f64, color: Color) {
    let canvas = canvas().lock().unwrap();
    canvas.circle(position, diameter, color);
}

pub fn draw_rectangle(position: VecSpace, color: Color) {
    let canvas = canvas().lock().unwrap();
    canvas.rectangle(position, color);
}

pub fn draw_hud(text: &String, ship_speed: f64) {
    let canvas = canvas().lock().unwrap();
    canvas.hud(&text, ship_speed);
}

pub fn draw_set_view_xy(val: VecSpace) {
    let mut canvas = canvas().lock().unwrap();
    let (x, y) = canvas.to_pixel(val);
    println!("{}/{} < {:?}", x, y, val);
    canvas.view_xy = VecSpace::new(-x as f64 / 1., -y as f64 / 1.);
}

pub fn draw_add_view(x: f64, y: f64, z: f64, a: f64) {
    let mut canvas = canvas().lock().unwrap();
    canvas.add_view(x, y, z, a);
}

pub fn draw_set_maximal_orbit_radius(val: f64) {
    let mut canvas = canvas().lock().unwrap();
    canvas.set_maximal_orbit_radius(val);
}

pub fn draw_start() {
    let mut canvas = canvas().lock().unwrap();
    canvas.start();
}

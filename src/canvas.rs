use crate::vec_space::*;
use macroquad::prelude::*;

pub struct Canvas {
    window_with: i32,
    window_height: i32,
    z_view: f64,
    z_grid: f64,
    // copy from masses
    maximal_orbit_radius: f64,
    max_pixel_from_center: i32,
}

impl Canvas {
    pub fn new(conf: &Conf) -> Canvas {
        Canvas {
            window_with: conf.window_width,
            window_height: conf.window_height,
            z_view: 0.9,
            z_grid: 0.9,
            maximal_orbit_radius: 1.,
            // Die kleinere Fenster-Ausdehnung zählt als normaler darstellbar Bildpunktebereich
            // The smallest extend of the window counts as visible screen range
            max_pixel_from_center: conf.window_height.min(conf.window_width) / 2,
        }
    }

    pub fn mul_z_view(&mut self, fakt: f64) {
        self.z_view *= fakt;
    }

    pub fn set_maximal_orbit_radius(&mut self, val: f64) {
        self.maximal_orbit_radius = val;
    }

    pub fn _set_z_view(&mut self, val: f64) {
        self.z_view = val;
    }

    fn scale(&self, position: &VecSpace) -> (f32, f32) {
        let window_center: VecSpace =
            VecSpace::new(self.window_with as f64 / 2., self.window_height as f64 / 2.);
        // Scale by view, divide by scene multiply by screen, add screen center
        let screen_pos = *position
            * (self.z_view / self.maximal_orbit_radius * self.max_pixel_from_center as f64)
            + window_center;
        (screen_pos.x() as f32, screen_pos.y() as f32)
    }

    pub fn draw_circle(&self, position: &VecSpace, diameter: f64, color: Color) {
        pub const DRAW_FACT: f64 = 5.;
        pub const DRAW_MIN: f64 = 3.;
        pub const DRAW_MAX: f64 = 200.;

        let size = diameter / DRAW_FACT * self.z_view;
        let size = size.clamp(DRAW_MIN, DRAW_MAX) as f32;

        let (x, y) = self.scale(position);

        draw_circle(x, y, size, color);
    }

    pub fn draw_rectangle(&self, position: &VecSpace, color: Color) {
        let (x, y) = self.scale(position);
        draw_rectangle(x, y, 1., 1., color);
    }

    pub fn draw_hud(&self, text: &String, position_index: usize) {
        draw_text(
            format!("{} {}", text, position_index).as_str(),
            20.0,
            20.0,
            30.0,
            DARKGRAY,
        );
    }

    pub fn draw_grid(&mut self) {
        if self.z_view > self.z_grid {
            self.z_grid *= 2.0;
            // println!("z_draw: {}", &masses.z_grid);
        }
        if self.z_view < self.z_grid {
            self.z_grid /= 2.0;
            // println!("z_draw: {}", &masses.z_grid);
        }

        let max = self.maximal_orbit_radius * 2. / self.z_grid;
        let step = max / 50.;

        let mut x = -max;
        let mut sub = 0;
        loop {
            let (beg_x, beg_y) = self.scale(&VecSpace::new(x, max));
            let (end_x, end_y) = self.scale(&VecSpace::new(x, -max));
            draw_line(beg_x, beg_y, end_x, end_y, 1., line_color(sub));

            sub += 1;
            x += step;
            if x > max {
                break;
            }
        }

        let mut x = -max;
        let mut sub = 0;
        loop {
            let (beg_x, beg_y) = self.scale(&VecSpace::new(max, x));
            let (end_x, end_y) = self.scale(&VecSpace::new(-max, x));
            draw_line(beg_x, beg_y, end_x, end_y, 1., line_color(sub));

            sub += 1;
            x += step;
            if x > max {
                break;
            }
        }
    }
}

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

use crate::masses::*;
use crate::vec_space::*;
use macroquad::prelude::*;

pub struct Canvas {
    window_with: i32,
    window_height: i32,
    xy_view: VecSpace,
    z_view: f64,
    z_grid: f64,
    // copy from masses
    predict_count: f64,
    pub predict_show: usize,
    maximal_orbit_radius: f64,
    max_pixel_from_center: i32,
}

impl Canvas {
    pub fn new(conf: &Conf) -> Canvas {
        Canvas {
            window_with: conf.window_width,
            window_height: conf.window_height,
            xy_view: VecSpace::ZERO,
            z_view: 0.95,
            z_grid: 0.95,
            predict_count: PREDICT_COUNT as f64,
            predict_show: 0,
            maximal_orbit_radius: 1.,
            // Die kleinere Fenster-Ausdehnung zählt als normaler darstellbar Bildpunktebereich
            // The smallest extend of the window counts as visible screen range
            max_pixel_from_center: conf.window_height.min(conf.window_width) / 2,
        }
    }

    pub fn set_maximal_orbit_radius(&mut self, val: f64) {
        self.maximal_orbit_radius = val;
    }

    pub fn mul_predict_count(&mut self, fakt: f64) {
        self.predict_count *= fakt;
        if self.predict_count > PREDICT_COUNT as f64 {
            self.predict_count = PREDICT_COUNT as f64;
        };
    }

    pub fn ramp_predict_show(&mut self, up: bool) {
        if up {
            if self.predict_show < self.predict_count as usize {
                self.predict_show += 1;
            }
        } else {
            if self.predict_show > 0 {
                self.predict_show -= 1;
            }
        }
    }

    pub fn add_view(&mut self, x: f64, y: f64) {
        self.xy_view += VecSpace::new(x, y);
    }

    pub fn _mul_z_view(&mut self, fakt: f64) {
        self.z_view *= fakt;
    }

    pub fn set_z_view(&mut self, val: f64) {
        self.z_view = val;
    }

    /// calculate the metric simulated values from the pixel position
    /// by screen-center maximal orbit and screen and z-zoom faktor
    fn _from_pixel(&self, x: i32, y: i32) -> VecSpace {
        let scale = self.maximal_orbit_radius / self.z_view / self.max_pixel_from_center as f64;
        // window_center in self? Dymamic by resize todo
        let window_center: VecSpace =
            VecSpace::new(self.window_with as f64 / 2., self.window_height as f64 / 2.);

        VecSpace::new(x as f64, y as f64) - window_center * scale //abs??
    }

    /// calculate the pixel position from the metric simulated values
    /// by maximal orbit and screen and z-zoom faktor and screen center
    fn to_pixel(&self, position: VecSpace) -> (f32, f32) {
        let window_center: VecSpace =
            VecSpace::new(self.window_with as f64 / 2., self.window_height as f64 / 2.);
        let scale = self.z_view / self.maximal_orbit_radius * self.max_pixel_from_center as f64;

        // Scale by view, divide by scene multiply by screen, add screen center
        let screen_pos = position * scale + self.xy_view + window_center;
        (screen_pos.x() as f32, screen_pos.y() as f32)
    }

    pub fn draw_circle(&self, position: VecSpace, diameter: f64, color: Color) {
        pub const DRAW_FACT: f64 = 5.;
        pub const DRAW_MIN: f64 = 3.;
        pub const DRAW_MAX: f64 = 200.;

        let size = diameter / DRAW_FACT * self.z_view;
        let size = size.clamp(DRAW_MIN, DRAW_MAX) as f32;

        let (x, y) = self.to_pixel(position);

        draw_circle(x, y, size, color);
    }

    fn _draw_rectangle(&self, position: VecSpace, color: Color) {
        let (x, y) = self.to_pixel(position);
        draw_rectangle(x, y, 1., 1., color);
    }

    pub fn draw_rectangles(&self, positions: [VecSpace; PREDICT_COUNT], color: Color) {
        for index in 0..self.predict_count as i32 {
            let i = index as usize;
            let position = positions[i];
            let (x, y) = self.to_pixel(position);
            draw_rectangle(x, y, 1., 1., color);
        }
        //for position in positions {
        //    //self.draw_rectangle(position, color);
        //    let (x, y) = self.to_pixel(position);
        //    draw_rectangle(x, y, 1., 1., color);
        //}
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

        let max = 4. * self.maximal_orbit_radius / self.z_grid;
        //self.from_pixel(self.window_with, self.window_height).x();
        //println!("{}", max);
        let step = max / 100.;

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

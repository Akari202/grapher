use crate::colors::WHITE;
use crate::config;
use crate::coordinate::{CartesianCoordinate2D, PixelCoordinate2D};
use crate::renderer::{Drawable, Renderer};

pub struct Axis2D {
    pixel_origin: PixelCoordinate2D,
    x_scale: f32,
    y_scale: f32,
    step: f32,
    x_range: (f32, f32),
    y_range: (f32, f32),
    objects: Vec<Box<dyn DrawableAxis2D>>
}

pub trait DrawableAxis2D {
    fn draw(&self, renderer: &mut Renderer, axis2d: &Axis2D) -> Result<(), String>;
    fn evaluate(&self, x: f32, y: f32) -> Option<(f32, f32)>;
}

impl Axis2D {
    pub fn new(pixel_origin: PixelCoordinate2D, x_scale: f32, y_scale: f32, step: f32) -> Axis2D {
        let x_range = (
            -pixel_origin.x as f32 / x_scale,
            (config::WINDOW_WIDTH as i32 - pixel_origin.x) as f32 / x_scale
        );
        let y_range = (
            -pixel_origin.y as f32 / y_scale,
            (config::WINDOW_HEIGHT as i32 - pixel_origin.y) as f32 / y_scale
        );
        Axis2D {
            pixel_origin,
            x_scale,
            y_scale,
            step,
            x_range,
            y_range,
            objects: Vec::new()
        }
    }

    pub fn to_pixel(&self, position: CartesianCoordinate2D) -> PixelCoordinate2D {
        PixelCoordinate2D::new(
            (position.x * self.x_scale) as i32 + self.pixel_origin.x,
            (position.y * self.y_scale * -1.0) as i32 + self.pixel_origin.y
        )
    }

    pub fn to_cartesian(&self, position: PixelCoordinate2D) -> CartesianCoordinate2D {
        CartesianCoordinate2D::new(
            (position.x - self.pixel_origin.x) as f32 / self.x_scale,
            (position.y - self.pixel_origin.y) as f32 / self.y_scale * -1.0
        )
    }

    pub fn x_range(&self) -> (f32, f32) {
        self.x_range
    }

    pub fn y_range(&self) -> (f32, f32) {
        self.y_range
    }

    pub fn x_steps(&self) -> i32 {
        ((self.x_range.1 - self.x_range.0) / self.step) as i32
    }

    pub fn y_steps(&self) -> i32 {
        ((self.y_range.1 - self.y_range.0) / self.step) as i32
    }

    pub fn range(&self) -> ((f32, f32), (f32, f32)) {
        (self.x_range, self.y_range)
    }

    pub fn steps(&self) -> (i32, i32) {
        (self.x_steps(), self.y_steps())
    }

    pub fn step_size(&self) -> f32 {
        self.step
    }

    pub fn add_object(&mut self, object: Box<dyn DrawableAxis2D>) {
        self.objects.push(object);
    }

    pub fn add_objects<I>(&mut self, objects: I) where I: IntoIterator<Item = Box<dyn DrawableAxis2D>> {
        for object in objects {
            self.add_object(object);
        }
    }

    pub fn highest_object(&self, x: f32) -> Option<f32> {
        let mut highest: Option<f32> = None;
        for object in &self.objects {
            if let Some(xy) = object.evaluate(x, 0.0) {
                if highest.is_none() || xy.1.abs() > highest.unwrap().abs() {
                    highest = Some(xy.1);
                }
            }
        }
        highest
    }
}

impl Drawable for Axis2D {
    fn draw(&self, renderer: &mut Renderer) -> Result<(), String> {
        // renderer.draw_line(0, self.pixel_origin.y, config::WINDOW_WIDTH as i32, self.pixel_origin.y, WHITE)?;
        // renderer.draw_line(self.pixel_origin.x, 0, self.pixel_origin.x, config::WINDOW_HEIGHT as i32, WHITE)?;
        renderer.draw_line(
            PixelCoordinate2D::new(0, self.pixel_origin.y),
            PixelCoordinate2D::new(config::WINDOW_WIDTH as i32, self.pixel_origin.y),
            WHITE
        )?;
        renderer.draw_line(
            PixelCoordinate2D::new(self.pixel_origin.x, 0),
            PixelCoordinate2D::new(self.pixel_origin.x, config::WINDOW_HEIGHT as i32),
            WHITE
        )?;

        self.objects.iter().for_each(|object| {
            object.draw(renderer, self).unwrap();
        });
        Ok(())
    }
}

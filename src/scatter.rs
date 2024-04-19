use crate::axis::{Axis2D, DrawableAxis2D};
use crate::coordinate::CartesianCoordinate2D;
use crate::renderer::Renderer;

pub struct Scatter2D {
    objects: Vec<CartesianCoordinate2D>,
    color: sdl2::pixels::Color
}

impl Scatter2D {
    pub fn new(objects: Vec<CartesianCoordinate2D>, color: sdl2::pixels::Color) -> Scatter2D {
        Scatter2D { objects, color }
    }

    pub fn add(&mut self, object: CartesianCoordinate2D) {
        self.objects.push(object);
    }
}

impl DrawableAxis2D for Scatter2D {
    fn draw(&self, renderer: &mut Renderer, axis2d: &Axis2D) -> Result<(), String> {
        for object in &self.objects {
            let position = axis2d.to_pixel(*object);
            renderer.draw_dot(position, self.color)?;
        }
        Ok(())
    }

    fn evaluate(&self, x: f32, _y: f32) -> Option<(f32, f32)> {
        for object in &self.objects {
            if object.x == x {
                return Some((0.0, object.y));
            }
        }
        None
    }
}

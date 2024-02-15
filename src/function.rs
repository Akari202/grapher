use std::thread::current;
use sdl2::pixels::Color;
use crate::axis::{Axis2D, DrawableAxis2D};
use crate::colors::{GREEN, RED};
use crate::coordinate::CartesianCoordinate2D;
use crate::renderer::Renderer;

pub struct UnaryFunction {
    function: Box<dyn Fn(f32) -> f32>,
    color: Color
}

impl UnaryFunction {
    pub fn new(function: Box<dyn Fn(f32) -> f32>, color: Color) -> UnaryFunction {
        UnaryFunction { function, color }
    }
}

impl DrawableAxis2D for UnaryFunction {
    // TODO: Fix the stair stepping effect
    fn draw(&self, renderer: &mut Renderer, axis2d: &Axis2D) -> Result<(), String> {
        let range = axis2d.range();
        let steps = axis2d.steps();
        let step_size = axis2d.step_size();
        let mut last = axis2d.to_pixel(
            CartesianCoordinate2D::new(range.0.0, (self.function)(range.0.0))
        );
        for i in  0..steps.0 {
            let x = range.0.0 + i as f32 * step_size;
            let y = (self.function)(x);
            let current = axis2d.to_pixel(CartesianCoordinate2D::new(x, y));
            // println!("({}, {})", x, y);
            renderer.draw_line(last, current, self.color)?;
            // renderer.draw_dot(last, GREEN)?;
            last = current;
        }
        Ok(())
    }

    fn evaluate(&self, x: f32) -> Option<f32> {
        Some((self.function)(x))
    }
}

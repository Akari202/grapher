use crate::axis::{Axis2D, DrawableAxis2D};
use crate::colors::{BLUE, GREEN, LIGHT_GRAY, RED};
use crate::config::{CENTER_X, CENTER_Y, WINDOW_WIDTH};
use crate::coordinate::{CartesianCoordinate2D, PixelCoordinate2D};
use crate::renderer::{Drawable, Renderer};

pub struct MohrsCircle {
    pub center: CartesianCoordinate2D,
    pub radius: [f32; 3],
    pub normal_stress: [f32; 3],
    pub shear_stress: [f32; 3],
    pub axis: Axis2D,
    circles: [Circle; 3]
}

#[derive(Clone, Copy)]
pub struct Circle {
    pub point1: CartesianCoordinate2D,
    pub point2: CartesianCoordinate2D,
    pub center: CartesianCoordinate2D,
    pub radius: f32
}

impl MohrsCircle {
    /// Normal stress is an array of the stresses in the x, y, and z directions respectively.
    /// Shear stress is an array of the stresses in the xy, yz, and xz directions respectively.
    pub fn new(normal_stress: [f32; 3], shear_stress: [f32; 3]) -> MohrsCircle {
        // Circle one
        let point1 = CartesianCoordinate2D::new(normal_stress[0], shear_stress[0]);
        let point2 = CartesianCoordinate2D::new(normal_stress[1], -shear_stress[0]);
        let center = CartesianCoordinate2D::new(
            (point1.x + point2.x) / 2.0,
            (point1.y + point2.y) / 2.0
        );
        let radius1 = ((point1.x - center.x).powi(2) + (point1.y - center.y).powi(2)).sqrt();
        let circle1 = Circle {
            point1,
            point2,
            center,
            radius: radius1
        };

        // Circle two
        let point1 = CartesianCoordinate2D::new(normal_stress[1], shear_stress[1]);
        let point2 = CartesianCoordinate2D::new(normal_stress[2], -shear_stress[1]);
        let center = CartesianCoordinate2D::new(
            (point1.x + point2.x) / 2.0,
            (point1.y + point2.y) / 2.0
        );
        let radius2 = ((point2.x - center.x).powi(2) + (point1.y - center.y).powi(2)).sqrt();
        let circle2 = Circle {
            point1,
            point2,
            center,
            radius: radius2
        };

        // Circle three
        let point1 = CartesianCoordinate2D::new(normal_stress[0], shear_stress[2]);
        let point2 = CartesianCoordinate2D::new(normal_stress[2], -shear_stress[2]);
        let center = CartesianCoordinate2D::new(
            (point1.x + point2.x) / 2.0,
            (point1.y + point2.y) / 2.0
        );
        let radius3 = ((point1.x - center.x).powi(2) + (point1.y - center.y).powi(2)).sqrt();
        let circle3 = Circle {
            point1,
            point2,
            center,
            radius: radius3
        };

        // Find the largest circle
        let radius = [radius1, radius2, radius3];
        let circles = [circle1, circle2, circle3];
        let max_radius = radius.iter().fold(0.0, |acc, &x| if x > acc { x } else { acc });
        let center = circles.iter().find(|&x| x.radius == max_radius).unwrap().center;

        // Build axis
        let scale = WINDOW_WIDTH as f32 / 6.0 / max_radius;
        let mut axis = Axis2D::new(
            PixelCoordinate2D::new(CENTER_X as i32, CENTER_Y as i32),
            scale,
            scale,
            0.01
        );
        axis.add_object(Box::new(circle1));
        axis.add_object(Box::new(circle2));
        axis.add_object(Box::new(circle3));

        MohrsCircle {
            center,
            radius,
            normal_stress,
            shear_stress,
            axis,
            circles
        }
    }
}

impl Drawable for MohrsCircle {
    fn draw(&self, renderer: &mut Renderer) -> Result<(), String> {
        renderer.draw_object(&self.axis)
    }
}

impl DrawableAxis2D for Circle {
    fn draw(&self, renderer: &mut Renderer, axis2d: &Axis2D) -> Result<(), String> {
        renderer.draw_circle(
            self.point1.to_pixel(&axis2d),
            self.radius as u32,
            GREEN
        )?;
        renderer.draw_dashed_line(
            self.point1.to_pixel(&axis2d),
            self.point2.to_pixel(&axis2d),
            LIGHT_GRAY,
            5,
            5
        )?;
        renderer.draw_dot(self.center.to_pixel(&axis2d), BLUE)
    }

    // TODO: Implement snapping to nearest circle
    fn evaluate(&self, _x: f32, _y: f32) -> Option<(f32, f32)> {
        None
    }
}


use crate::axis::Axis2D;
use crate::config::{CENTER_X, CENTER_Y};
use crate::renderer::Renderer;

#[derive(Clone, Copy, Debug)]
pub struct PixelCoordinate2D {
    pub x: i32,
    pub y: i32
}

impl PixelCoordinate2D {
    pub fn new(x: i32, y: i32) -> PixelCoordinate2D {
        PixelCoordinate2D { x, y }
    }

    pub fn new_tuple(tuple: (i32, i32)) -> PixelCoordinate2D {
        PixelCoordinate2D { x: tuple.0, y: tuple.1 }
    }

    pub fn to_cartesian(&self, axis2d: &Axis2D) -> CartesianCoordinate2D {
        axis2d.to_cartesian(*self)
    }

    pub fn origin() -> PixelCoordinate2D {
        PixelCoordinate2D { x: 0, y: 0 }
    }

    pub fn center() -> PixelCoordinate2D {
        PixelCoordinate2D { x: CENTER_X as i32, y: CENTER_Y as i32 }
    }
}

impl PartialEq for PixelCoordinate2D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CartesianCoordinate2D {
    pub x: f32,
    pub y: f32
}

impl CartesianCoordinate2D {
    pub fn new(x: f32, y: f32) -> CartesianCoordinate2D {
        CartesianCoordinate2D { x, y }
    }

    pub fn new_tuple(tuple: (f32, f32)) -> CartesianCoordinate2D {
        CartesianCoordinate2D { x: tuple.0, y: tuple.1 }
    }

    pub fn to_pixel(&self, axis2d: &Axis2D) -> PixelCoordinate2D {
        axis2d.to_pixel(*self)
    }

    pub fn origin() -> CartesianCoordinate2D {
        CartesianCoordinate2D { x: 0.0, y: 0.0 }
    }
}

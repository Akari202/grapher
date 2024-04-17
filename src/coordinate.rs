use crate::axis::Axis2D;

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

    pub fn to_polar(&self, axis2d: &Axis2D) -> PolarCoordinate2D {
        self.to_cartesian(axis2d).to_polar()
    }

    pub fn origin() -> PixelCoordinate2D {
        PixelCoordinate2D { x: 0, y: 0 }
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

    pub fn to_polar(&self) -> PolarCoordinate2D {
        PolarCoordinate2D::new(
            (self.x.powi(2) + self.y.powi(2)).sqrt(),
            self.y.atan2(self.x)
        )
    }

    pub fn origin() -> CartesianCoordinate2D {
        CartesianCoordinate2D { x: 0.0, y: 0.0 }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PolarCoordinate2D {
    pub r: f32,
    pub theta: f32
}

impl PolarCoordinate2D {
    pub fn new(r: f32, theta: f32) -> PolarCoordinate2D {
        PolarCoordinate2D { r, theta }
    }

    pub fn new_tuple(tuple: (f32, f32)) -> PolarCoordinate2D {
        PolarCoordinate2D { r: tuple.0, theta: tuple.1 }
    }

    pub fn to_pixel(&self, axis2d: &Axis2D) -> PixelCoordinate2D {
        self.to_cartesian().to_pixel(axis2d)
    }

    pub fn to_cartesian(&self) -> CartesianCoordinate2D {
        CartesianCoordinate2D::new(
            self.r * self.theta.cos(),
            self.r * self.theta.sin()
        )
    }

    pub fn origin() -> PolarCoordinate2D {
        PolarCoordinate2D { r: 0.0, theta: 0.0 }
    }
}

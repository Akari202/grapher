use sdl2::{EventPump, Sdl};
use crate::axis::{Axis2D, DrawableAxis2D};
use crate::colors::{GRAY, LIGHT_GRAY, WHITE};
use crate::config::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::coordinate::{CartesianCoordinate2D, PixelCoordinate2D};
use crate::function::UnaryFunction;
use crate::renderer::{Drawable, Renderer};

pub struct Cursor;

impl Cursor {
    pub fn new(sdl: &Sdl) -> Cursor {
        sdl.mouse().show_cursor(false);
        Cursor
    }

    fn mouse_position(&self, event_pump: &EventPump) -> PixelCoordinate2D {
        PixelCoordinate2D::new(event_pump.mouse_state().x(), event_pump.mouse_state().y())
    }
}

impl Drawable for Cursor {
    fn draw(&self, renderer: &mut Renderer) -> Result<(), String> {
        // if mouse is not in window, do not draw
        if !renderer.sdl.mouse().focused_window_id().is_some_and(|id| id == renderer.canvas.window().id()) {
            return Ok(());
        }
        // if mouse not hidden hide it
        if renderer.sdl.mouse().is_cursor_showing() {
            renderer.sdl.mouse().show_cursor(false);
        }
        let position = self.mouse_position(&renderer.event_pump);
        renderer.draw_dashed_line(
            PixelCoordinate2D::new(position.x, 0),
            PixelCoordinate2D::new(position.x, WINDOW_HEIGHT as i32),
            LIGHT_GRAY,
            15,
            20
        )?;
        renderer.draw_dashed_line(
            PixelCoordinate2D::new(0, position.y),
            PixelCoordinate2D::new(WINDOW_WIDTH as i32, position.y),
            LIGHT_GRAY,
            15,
            20
        )?;
        Ok(())
    }
}

// TODO: refactor readout types into an axis dependent object and a general object for pixel readout
pub enum ReadoutType {
    Cartesian,
    Pixel
}

pub struct CursorReadout {
    position: PixelCoordinate2D,
    readout_type: ReadoutType
}

impl CursorReadout {
    pub fn new(position: PixelCoordinate2D, readout_type: ReadoutType) -> CursorReadout {
        CursorReadout { position, readout_type }
    }
}

impl DrawableAxis2D for CursorReadout {
    fn draw(&self, renderer: &mut Renderer, axis2d: &Axis2D) -> Result<(), String> {
        let value = PixelCoordinate2D::new(
            renderer.event_pump.mouse_state().x(),
            renderer.event_pump.mouse_state().y()
        );
        match self.readout_type {
            ReadoutType::Cartesian => {
                let value = axis2d.to_cartesian(value);
                let highest = axis2d.highest_object(value.x);
                let y = highest.unwrap_or(value.y);
                let value = CartesianCoordinate2D::new(value.x, y);
                renderer.draw_text(
                    &format!("({:.2}, {:.2})", value.x, value.y),
                    self.position,
                    WHITE,
                    20
                )?;
            },
            ReadoutType::Pixel => {
                renderer.draw_text(
                    &format!("({}, {})", value.x, value.y),
                    self.position,
                    GRAY,
                    20
                )?;
            }
        }

        Ok(())
    }

    fn evaluate(&self, _x: f32) -> Option<f32> {
        None
    }
}

pub type SnappingCursor = Cursor;

impl DrawableAxis2D for SnappingCursor {
    fn draw(&self, renderer: &mut Renderer, axis2d: &Axis2D) -> Result<(), String> {
        // if mouse is not in window, do not draw
        if !renderer.sdl.mouse().focused_window_id().is_some_and(|id| id == renderer.canvas.window().id()) {
            return Ok(());
        }
        // if mouse not hidden hide it
        if renderer.sdl.mouse().is_cursor_showing() {
            renderer.sdl.mouse().show_cursor(false);
        }
        let position = axis2d.to_cartesian(self.mouse_position(&renderer.event_pump));
        // snap mouse y to the axis highest
        let highest = axis2d.highest_object(position.x);
        let y = highest.unwrap_or(position.y);
        let position = axis2d.to_pixel(CartesianCoordinate2D::new(position.x, y));
        renderer.draw_dashed_line(
            PixelCoordinate2D::new(position.x, 0),
            PixelCoordinate2D::new(position.x, WINDOW_HEIGHT as i32),
            LIGHT_GRAY,
            15,
            20
        )?;
        renderer.draw_dashed_line(
            PixelCoordinate2D::new(0, position.y),
            PixelCoordinate2D::new(WINDOW_WIDTH as i32, position.y),
            LIGHT_GRAY,
            15,
            20
        )?;
        Ok(())
    }

    fn evaluate(&self, _x: f32) -> Option<f32> {
        None
    }
}

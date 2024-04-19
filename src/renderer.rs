use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::{EventPump, Sdl};
use sdl2::video::Window;
use crate::coordinate::PixelCoordinate2D;

pub struct Renderer {
    pub(crate) canvas: WindowCanvas,
    pub sdl: Sdl,
    pub event_pump: EventPump
}

impl Renderer {
    pub fn new(window: Window, sdl: Sdl) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let mut event_pump = sdl.event_pump()?;
        Ok(Renderer { canvas, sdl, event_pump})
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn draw_dot(&mut self, position: PixelCoordinate2D, color: Color) -> Result<(), String> {
        self.canvas.set_draw_color(color);
        self.canvas.draw_point(sdl2::rect::Point::new(position.x, position.y)).unwrap();
        Ok(())
    }

    pub fn draw_rect(&mut self, position: PixelCoordinate2D, w: u32, h: u32, color: Color) -> Result<(), String> {
        self.canvas.set_draw_color(color);
        self.canvas.draw_rect(sdl2::rect::Rect::new(position.x, position.y, w, h)).unwrap();
        Ok(())
    }

    pub fn draw_fill_rect(&mut self, position: PixelCoordinate2D, w: u32, h: u32, color: Color) -> Result<(), String> {
        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(sdl2::rect::Rect::new(position.x, position.y, w, h)).unwrap();
        Ok(())
    }

    // pub fn draw_function(&mut self, f: fn(i32, i32) -> (i32, i32), color: Color) -> Result<(), String> {
    //     for x in 0..800 {
    //         let (x, y) = f(x, 400);
    //         self.draw_dot(x, y, color)?;
    //     }
    //     Ok(())
    // }

    pub fn draw_circle(&mut self, position: PixelCoordinate2D, radius: u32, color: Color) -> Result<(), String> {
        self.canvas.set_draw_color(color);
        // Draw a circle as a series of dots
        let mut x = 0;
        let mut y = radius as i32;
        let mut d = 3 - 2 * radius as i32;
        while x <= y {
            self.canvas.draw_point(sdl2::rect::Point::new(position.x + x, position.y + y)).unwrap();
            self.canvas.draw_point(sdl2::rect::Point::new(position.x + x, position.y - y)).unwrap();
            self.canvas.draw_point(sdl2::rect::Point::new(position.x - x, position.y + y)).unwrap();
            self.canvas.draw_point(sdl2::rect::Point::new(position.x - x, position.y - y)).unwrap();
            self.canvas.draw_point(sdl2::rect::Point::new(position.x + y, position.y + x)).unwrap();
            self.canvas.draw_point(sdl2::rect::Point::new(position.x + y, position.y - x)).unwrap();
            self.canvas.draw_point(sdl2::rect::Point::new(position.x - y, position.y + x)).unwrap();
            self.canvas.draw_point(sdl2::rect::Point::new(position.x - y, position.y - x)).unwrap();
            if d < 0 {
                d += 4 * x + 6;
            } else {
                d += 4 * (x - y) + 10;
                y -= 1;
            }
            x += 1;
        }
        Ok(())
    }

    pub fn draw_line(&mut self, position1: PixelCoordinate2D, position2: PixelCoordinate2D, color: Color) -> Result<(), String> {
        self.canvas.set_draw_color(color);
        self.canvas.draw_line(
            sdl2::rect::Point::new(position1.x, position1.y),
            sdl2::rect::Point::new(position2.x, position2.y)
        ).unwrap();
        Ok(())
    }

    pub fn draw_dotted_line(&mut self, position1: PixelCoordinate2D, position2: PixelCoordinate2D, color: Color, step: u32) -> Result<(), String> {
        self.canvas.set_draw_color(color);
        let mut x = position1.x;
        let mut y = position1.y;
        let dx = position2.x - position1.x;
        let dy = position2.y - position1.y;
        let steps = if dx.abs() > dy.abs() { dx.abs() } else { dy.abs() };
        let x_inc = dx as f32 / steps as f32;
        let y_inc = dy as f32 / steps as f32;
        for _ in 0..steps {
            self.canvas.draw_point(sdl2::rect::Point::new(x, y)).unwrap();
            x += x_inc as i32 * step as i32;
            y += y_inc as i32 * step as i32;
        }
        Ok(())
    }

    pub fn draw_triangle(&mut self, position1: PixelCoordinate2D, position2: PixelCoordinate2D, position3: PixelCoordinate2D, color: Color) -> Result<(), String> {
        self.canvas.set_draw_color(color);
        self.canvas.draw_line(
            sdl2::rect::Point::new(position1.x, position1.y),
            sdl2::rect::Point::new(position2.x, position2.y)
        ).unwrap();
        self.canvas.draw_line(
            sdl2::rect::Point::new(position2.x, position2.y),
            sdl2::rect::Point::new(position3.x, position3.y)
        ).unwrap();
        self.canvas.draw_line(
            sdl2::rect::Point::new(position3.x, position3.y),
            sdl2::rect::Point::new(position1.x, position1.y)
        ).unwrap();
        Ok(())
    }

    pub fn draw_arrow(&mut self, position1: PixelCoordinate2D, position2: PixelCoordinate2D, color: Color) -> Result<(), String> {
        self.canvas.set_draw_color(color);
        self.canvas.draw_line(
            sdl2::rect::Point::new(position1.x, position1.y),
            sdl2::rect::Point::new(position2.x, position2.y)
        ).unwrap();
        // Draw the arrowhead as a triangle
        let arrow_size = 10;
        let dx = position2.x - position1.x;
        let dy = position2.y - position1.y;
        let angle = dy as f32 / dx as f32;
        let angle = angle.atan();
        let angle = if dx < 0 { angle + std::f32::consts::PI } else { angle };
        let angle1 = angle + std::f32::consts::PI / 6.0;
        let angle2 = angle - std::f32::consts::PI / 6.0;
        let x1 = position2.x as f32 - arrow_size as f32 * angle1.cos();
        let y1 = position2.y as f32 - arrow_size as f32 * angle1.sin();
        let x2 = position2.x as f32 - arrow_size as f32 * angle2.cos();
        let y2 = position2.y as f32 - arrow_size as f32 * angle2.sin();
        self.canvas.draw_line(
            sdl2::rect::Point::new(position2.x, position2.y),
            sdl2::rect::Point::new(x1 as i32, y1 as i32)
        ).unwrap();
        self.canvas.draw_line(
            sdl2::rect::Point::new(position2.x, position2.y),
            sdl2::rect::Point::new(x2 as i32, y2 as i32)
        ).unwrap();
        Ok(())
    }




    // TODO: I want to be able to specify a starting point that the dashes will propagate out from
    pub fn draw_dashed_line(&mut self, position1: PixelCoordinate2D, position2: PixelCoordinate2D, color: Color, blank_step: u32, dash_step: u32) -> Result<(), String> {
        self.canvas.set_draw_color(color);
        let mut x = position1.x;
        let mut y = position1.y;
        let dx = position2.x - position1.x;
        let dy = position2.y - position1.y;
        let horizontal = dx.abs() > dy.abs();
        let steps = if horizontal {
            dx.abs() as u32 / (blank_step + dash_step) * 2
        } else {
            dy.abs() as u32 / (blank_step + dash_step) * 2
        };
        let mut blank = true;
        let mut last = position1;
        for _ in 0..steps {
            if blank {
                x += if horizontal { blank_step as i32 } else { 0 };
                y += if horizontal { 0 } else { blank_step as i32 };
            } else {
                x += if horizontal { dash_step as i32 } else { 0 };
                y += if horizontal { 0 } else { dash_step as i32 };
                self.canvas.draw_line(
                    sdl2::rect::Point::new(last.x, last.y),
                    sdl2::rect::Point::new(x, y)
                ).unwrap();
            }
            last = PixelCoordinate2D::new(x, y);
            blank = !blank;
        }
        Ok(())
    }

    pub fn draw_object(&mut self, object: &dyn Drawable) -> Result<(), String> {
        object.draw(self)
    }

    pub fn draw_all_objects(&mut self, objects: &Vec<&dyn Drawable>) -> Result<(), String> {
        for object in objects {
            object.draw(self)?;
        }
        Ok(())
    }

    pub fn draw_text(&mut self, text: &str, position: PixelCoordinate2D, color: Color, size: u16) -> Result<(), String> {
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
        let font = ttf_context.load_font("assets/JetBrainsMono.ttf", size).map_err(|e| e.to_string())?;
        let surface = font.render(text).blended(color).map_err(|e| e.to_string())?;
        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;
        self.canvas.copy(&texture, None, sdl2::rect::Rect::new(position.x, position.y, surface.width(), surface.height())).map_err(|e| e.to_string())?;
        Ok(())
    }
}

pub trait Drawable {
    fn draw(&self, renderer: &mut Renderer) -> Result<(), String>;
}

#![feature(is_sorted)]
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use crate::axis::Axis2D;
use crate::colors::{BLUE, GREEN, RED, WHITE};
use crate::config::{CENTER_Y, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::coordinate::PixelCoordinate2D;
use crate::cursor::ReadoutType;
use crate::function::UnaryFunction;

pub mod config;
pub mod renderer;
mod coordinate;
mod axis;
mod colors;
mod function;
mod scatter;
mod cursor;
mod sort;
mod audio;
mod mohr;
mod graph;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "Graphed",
            config::WINDOW_WIDTH,
            config::WINDOW_HEIGHT
        )
        .position_centered()
        // .vulkan()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut renderer = renderer::Renderer::new(window, sdl_context)?;

    let mut axis: Axis2D;
    {
        axis = axis::Axis2D::new(
            PixelCoordinate2D::new(config::CENTER_X as i32, config::CENTER_Y as i32),
            WINDOW_WIDTH as f32 / (4.0 * std::f32::consts::PI),
            WINDOW_HEIGHT as f32 / 2.5,
            0.01
        );
        axis.add_object(Box::new(cursor::SnappingCursor::new(&renderer.sdl)));
        axis.add_object(Box::new(cursor::CursorReadout::new(PixelCoordinate2D::new(WINDOW_WIDTH as i32 - 175, 0), ReadoutType::Cartesian)));
    }

    let mohr = mohr::MohrsCircle::new([200.0, 100.0, 0.0], [80.0, 20.0, 0.0]);

    let mut graph = graph::Graph::new(0);
    // graph.fill_random(20);
    graph.add_vertex_from_list(&[(0,1), (0,2), (0,5), (1,4), (2,3), (3,8), (4,9), (5,6), (5,7), (6,7)])?;

    'running: loop {
        for event in renderer.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        renderer.clear();
        // renderer.draw_object(&axis)?;
        // renderer.draw_arrow(PixelCoordinate2D::new(0, 0), PixelCoordinate2D::new(100, 100), RED)?;
        // renderer.draw_object(&mohr)?;
        renderer.draw_object(&graph)?;

        renderer.present();

        std::thread::sleep(Duration::new(0, 1000000000 / config::FRAMERATE));
    }

    Ok(())
}







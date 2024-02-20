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
        axis.add_object(Box::new(cursor::CursorReadout::new(PixelCoordinate2D::new(WINDOW_WIDTH as i32 - 175, 25), ReadoutType::Pixel)));

        axis.add_object(Box::new(UnaryFunction::new(Box::new(|x| x.sin()), BLUE)));
        axis.add_object(Box::new(UnaryFunction::new(Box::new(
            |x| {
                4.0 / std::f32::consts::PI * (0..3).fold(0.0, |acc, n| {
                    let n = 2 * n + 1;
                    acc + (1.0 / n as f32) * (n as f32 * x).sin()
                })
            }
        ), GREEN)));
        axis.add_object(Box::new(UnaryFunction::new(Box::new(
            |x| {
                if x.sin() > 0.0 {
                    1.0
                } else {
                    -1.0
                }
            }
        ), RED)));
        // axis.add_object(Box::new(UnaryFunction::new(Box::new(|x| x.powi(5) - 10.0 * x.powi(4) + 40.0 * x.powi(3) - 80.0 * x.powi(2) + 80.0 * x - 32.0))));
    }


    // let mut sort_vis = sort::SortingVisualization::new(sort::SortType::Shell);
    let mut sort_vis = sort::SortingVisualization::new(sort::SortType::Insertion);

    'running: loop {
        for event in renderer.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => { sort_vis.shuffle() },
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => { if !sort_vis.sorted { sort_vis.step() }},
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => { if !sort_vis.sorted { sort_vis.auto_sort = !sort_vis.auto_sort }},
                _ => {}
            }
        }

        if sort_vis.auto_sort {
            sort_vis.step();
        }

        renderer.clear();
        // renderer.draw_object(&axis)?;
        renderer.draw_object(&sort_vis)?;

        renderer.present();

        std::thread::sleep(Duration::new(0, 1000000000 / config::FRAMERATE));
    }

    Ok(())
}







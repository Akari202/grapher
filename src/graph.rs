use std::cmp::PartialEq;
use rand::Rng;
use crate::colors::{BLUE, CYAN, DARK_YELLOW, GRAY, LIGHT_GRAY, WHITE, YELLOW};
use crate::coordinate::PixelCoordinate2D;
use crate::renderer::{Drawable, Renderer};

const RADIUS: u32 = 20;
const LINE_LENGTH: i32 = 60;
const FONT_SIZE: u16 = 25;
const ANGLE: f64 = 1.0;

pub struct Graph {
    pub objects: Vec<(Vertex, Edge)>
}

pub struct Edge {
    pub start: PixelCoordinate2D,
    pub end: PixelCoordinate2D,
    tree_type: bool
}

#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: PixelCoordinate2D,
    pub value: i32,
    children: u8,
    origin: bool
}

impl Graph {
    pub fn new(source: i32) -> Graph {
        let mut objects = Vec::new();
        objects.push((
            Vertex {
                position: PixelCoordinate2D::center(),
                value: source,
                children: 0,
                origin: true
            },
            Edge {
                start: PixelCoordinate2D::center(),
                end: PixelCoordinate2D::center(),
                tree_type: false
            }
        ));
        Graph { objects }
    }

    pub fn fill_random(&mut self, node_count: i32) {
        let mut rng = rand::thread_rng();
        // for i in 1..node_count {
        //     let source = rng.gen_range(0..i);
        //     if let Some(vertex) = self.get_vertex(source) {
        //         if
        //         self.add_vertex(vertex.value, i).unwrap();
        //     } else {
        //     self.add_vertex(rng.gen_range(0..i), i).unwrap();
        // }
        let mut i = 1;
        let mut source = 0;
        while i < node_count {
            if let Some(vertex) = self.get_vertex(source) {
                if vertex.children < 2 {
                    self.add_vertex(vertex.value, i).unwrap();
                    i += 1;
                }
            }
            source = rng.gen_range(0..i);
        }
    }

    fn get_vertex(&self, value: i32) -> Option<&Vertex> {
        for (vertex, _) in &self.objects {
            if vertex.value == value {
                return Some(vertex);
            }
        }
        None
    }

    pub fn add_vertex_from_list(&mut self, list: &[(i32, i32)]) -> Result<(), String> {
        for (source, target) in list {
            self.add_vertex(*source, *target)?;
        }
        Ok(())
    }

    pub fn add_vertex(&mut self, source: i32, target: i32) -> Result<(), String> {
        let mut source_vertex = None;
        let mut target_vertex = None;
        for (vertex, _) in &mut self.objects {
            if vertex.value == source {
                source_vertex = Some(vertex);
            } else if vertex.value == target {
                target_vertex = Some(vertex);
            }
        }
        if source_vertex.is_none() {
            return Err("Source vertex not found".to_string());
        }
        let source_vertex = source_vertex.unwrap();
        source_vertex.children += 1;
        let source_vertex = source_vertex.clone();
        if target_vertex.is_none() {
            let angle = ANGLE * source_vertex.children as f64;
            let target_vertex = Vertex {
                position: PixelCoordinate2D::new(
                    source_vertex.position.x + (LINE_LENGTH as f64 * angle.cos()) as i32,
                    source_vertex.position.y + (LINE_LENGTH as f64 * angle.sin()) as i32
                ),
                value: target,
                children: source_vertex.children,
                origin: false
            };
            let edge = Edge {
                start: PixelCoordinate2D::new(
                    source_vertex.position.x + (RADIUS as f64 * angle.cos()) as i32,
                    source_vertex.position.y + (RADIUS as f64 * angle.sin()) as i32
                ),
                end: PixelCoordinate2D::new(
                    target_vertex.position.x - (RADIUS as f64 * angle.cos()) as i32,
                    target_vertex.position.y - (RADIUS as f64 * angle.sin()) as i32
                ),
                tree_type: true
            };
            self.objects.push((
                target_vertex,
                edge
            ));
        } else {
            let target_vertex = target_vertex.unwrap().clone();
            // calculate angle from horizontal
            let angle = (target_vertex.position.y as f64 - source_vertex.position.y as f64).atan2(
                target_vertex.position.x as f64 - source_vertex.position.x as f64
            );
            let edge = Edge {
                start: PixelCoordinate2D::new(
                    source_vertex.position.x + (RADIUS as f64 * angle.cos()) as i32,
                    source_vertex.position.y + (RADIUS as f64 * angle.sin()) as i32
                ),
                end: PixelCoordinate2D::new(
                    target_vertex.position.x - (RADIUS as f64 * angle.cos()) as i32,
                    target_vertex.position.y - (RADIUS as f64 * angle.sin()) as i32
                ),
                tree_type: false
            };
            self.objects.push((
                target_vertex,
                edge
            ));
        }
        Ok(())
    }
}

impl Drawable for Graph {
    fn draw(&self, renderer: &mut Renderer) -> Result<(), String> {
        for (vertex, edge) in &self.objects {
            let color = if vertex.origin { CYAN } else { WHITE };
            renderer.draw_circle(vertex.position, RADIUS, color)?;
            if edge.start != edge.end {
                if edge.tree_type {
                    renderer.draw_line(edge.start, edge.end, LIGHT_GRAY)?;
                } else {
                    renderer.draw_line(edge.start, edge.end, YELLOW)?;
                }
            }
            renderer.draw_text(
                &vertex.value.to_string(),
                PixelCoordinate2D::new(
                    vertex.position.x - FONT_SIZE as i32 / 3,
                    vertex.position.y - FONT_SIZE as i32 / 2
                ),
                WHITE,
                FONT_SIZE
            )?;
        }
        Ok(())
    }
}

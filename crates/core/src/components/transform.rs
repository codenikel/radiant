use serde::{Deserialize, Serialize};

use crate::{RadiantComponent, RadiantTransformable};

const MIN_SIZE: f32 = 8.0;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransformComponent {
    position: [f32; 3],
    scale: [f32; 3],
    rotation: f32,
}

impl TransformComponent {
    pub fn new() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            scale: [MIN_SIZE, MIN_SIZE, 0.0],
            rotation: 0.0,
        }
    }
}

impl RadiantTransformable for TransformComponent {
    fn transform_xy(&mut self, position: &[f32; 2]) {
        self.position = [
            self.position[0] + position[0],
            self.position[1] + position[1],
            0.0,
        ]
    }

    fn transform_scale(&mut self, scale: &[f32; 2]) {
        self.scale = [(self.scale[0] + scale[0]).max(MIN_SIZE), (self.scale[1] + scale[1]).max(MIN_SIZE), 0.0];
    }

    fn set_xy(&mut self, position: &[f32; 2]) {
        self.position = [position[0], position[1], 0.0];
    }

    fn set_scale(&mut self, scale: &[f32; 2]) {
        self.scale = [scale[0].max(MIN_SIZE), scale[1].max(MIN_SIZE), 0.0];
    }

    fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    fn get_xy(&self) -> [f32; 2] {
        [self.position[0], self.position[1]]
    }

    fn get_scale(&self) -> [f32; 2] {
        [self.scale[0], self.scale[1]]
    }

    fn get_rotation(&self) -> f32 {
        self.rotation
    }
}

impl RadiantComponent for TransformComponent {}

use std::fmt::Debug;

use crate::RadiantMessageHandler;
use crate::RadiantScene;
use crate::SelectionMessage;
use crate::TransformMessage;
use crate::{RadiantIdentifiable, RadiantSelectable, SelectionComponent};
use crate::{RadiantRenderable, TransformComponent};
use epaint::ClippedPrimitive;
use epaint::ClippedShape;
use epaint::Color32;
use epaint::Pos2;
use epaint::Rect;
use epaint::TessellationOptions;
use epaint::Vertex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum RadiantPathMessage {
    Transform(TransformMessage),
    Selection(SelectionMessage),
}

#[derive(Serialize, Deserialize)]
pub struct RadiantPathNode {
    pub id: u64,
    pub transform: TransformComponent,
    pub selection: SelectionComponent,
    #[serde(skip)]
    pub primitives: Vec<ClippedPrimitive>,
    #[serde(skip)]
    pub selection_primitives: Vec<ClippedPrimitive>,
    #[serde(skip)]
    pub pixels_per_point: f32,
}

impl Clone for RadiantPathNode {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            transform: self.transform.clone(),
            selection: self.selection.clone(),
            primitives: Vec::new(),
            selection_primitives: Vec::new(),
            pixels_per_point: 1.0,
        }
    }
}

impl Debug for RadiantPathNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RadiantPathNode")
            .field("id", &self.id)
            .field("transform", &self.transform)
            .field("selection", &self.selection)
            .finish()
    }
}

impl RadiantPathNode {
    pub fn new(id: u64, position: [f32; 2]) -> Self {
        let mut transform = TransformComponent::new();
        transform.set_xy(&position);

        let selection = SelectionComponent::new();

        Self {
            id,
            transform,
            selection,
            primitives: Vec::new(),
            selection_primitives: Vec::new(),
            pixels_per_point: 1.0,
        }
    }

    fn tessellate(&mut self) {
        let tessellator =
            epaint::Tessellator::new(self.pixels_per_point, Default::default(), [1, 1], vec![]);

        let position = self.transform.get_xy();
        let rect = epaint::Rect::from_min_max(
            epaint::Pos2::new(
                position[0] / self.pixels_per_point,
                position[1] / self.pixels_per_point,
            ),
            epaint::Pos2::new(
                position[0] / self.pixels_per_point + 200.0,
                position[1] / self.pixels_per_point + 200.0,
            ),
        );
        let points = vec![
            epaint::Pos2::new(
                position[0] / self.pixels_per_point,
                position[1] / self.pixels_per_point,
            ),
            epaint::Pos2::new(
                position[0] / self.pixels_per_point + 200.0,
                position[1] / self.pixels_per_point + 200.0,
            ),
            epaint::Pos2::new(
                position[0] / self.pixels_per_point,
                position[1] / self.pixels_per_point + 400.0,
            ),
            epaint::Pos2::new(
                position[0] / self.pixels_per_point - 200.0,
                position[1] / self.pixels_per_point + 200.0,
            ),
        ];
        let rounding = epaint::Rounding::default();

        log::info!("tessellate {}", self.selection.is_selected());
        let color = if self.selection.is_selected() {
            epaint::Color32::RED
        } else {
            epaint::Color32::LIGHT_RED
        };
        let stroke = epaint::Stroke::new(1.0, color);
        let path_shape = epaint::PathShape::convex_polygon(points.clone(), color, stroke);
        let shapes = vec![ClippedShape(
            Rect::EVERYTHING,
            epaint::Shape::Path(path_shape),
        )];
        self.primitives = epaint::tessellator::tessellate_shapes(
            self.pixels_per_point,
            TessellationOptions::default(),
            [1, 1],
            vec![],
            shapes,
        );

        let color = epaint::Color32::from_rgb(
            (self.id + 1 >> 0) as u8 & 0xFF,
            (self.id + 1 >> 8) as u8 & 0xFF,
            (self.id + 1 >> 16) as u8 & 0xFF,
        );
        let stroke = epaint::Stroke::new(1.0, color);
        let path_shape = epaint::PathShape::convex_polygon(points, color, stroke);
        let shapes = vec![ClippedShape(
            Rect::EVERYTHING,
            epaint::Shape::Path(path_shape),
        )];
        self.selection_primitives = epaint::tessellator::tessellate_shapes(
            self.pixels_per_point,
            TessellationOptions::default(),
            [1, 1],
            vec![],
            shapes,
        );
    }
}

impl RadiantIdentifiable for RadiantPathNode {
    fn get_id(&self) -> u64 {
        return self.id;
    }
}

impl RadiantSelectable for RadiantPathNode {
    fn set_selected(&mut self, selected: bool) {
        self.selection.set_selected(selected);
        self.tessellate();
    }
}

impl RadiantRenderable for RadiantPathNode {
    fn attach_to_scene(&mut self, scene: &mut RadiantScene) {
        self.pixels_per_point = scene.screen_descriptor.pixels_per_point;
        self.tessellate();
    }

    fn detach(&mut self) {
        self.primitives.clear();
    }
}

impl RadiantMessageHandler<RadiantPathMessage> for RadiantPathNode {
    fn handle_message(&mut self, message: RadiantPathMessage) {
        match message {
            RadiantPathMessage::Transform(message) => {
                self.transform.handle_message(message);
                self.tessellate();
                // self.renderer.set_position(&self.transform.get_xy());
                // self.offscreen_renderer.set_position(&self.transform.get_xy());
            }
            RadiantPathMessage::Selection(message) => {
                self.selection.handle_message(message);
                self.tessellate();
                // self.renderer
                //     .set_selection_color([1.0, 0.0, 0.0, if self.selection.is_selected() { 1.0 } else { 0.0 }]);
            }
        }
    }
}

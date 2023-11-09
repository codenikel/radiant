use radiant_core::{
    RadiantRectangleNode, RadiantSceneMessage, RadiantSceneResponse, RadiantTessellatable, Runtime,
    SelectionTool, View,
};
use radiant_image_node::{image_loader, RadiantImageNode};
use radiant_text_node::RadiantTextNode;
use radiant_winit::RadiantView;

use crate::{RadiantMessage, RadiantNodeType, RadiantResponse};

pub struct RadiantRuntime {
    pub view: RadiantView<RadiantMessage, RadiantNodeType>,
}

impl RadiantRuntime {
    pub async fn new() -> Self {
        Self {
            view: RadiantView::new(SelectionTool::new()).await,
        }
    }
}

impl Runtime<'_, RadiantMessage, RadiantNodeType, RadiantResponse> for RadiantRuntime {
    type View = RadiantView<RadiantMessage, RadiantNodeType>;

    fn view(&self) -> &RadiantView<RadiantMessage, RadiantNodeType> {
        &self.view
    }

    fn view_mut(&mut self) -> &mut RadiantView<RadiantMessage, RadiantNodeType> {
        &mut self.view
    }

    fn handle_message(&mut self, message: RadiantMessage) -> Option<RadiantResponse> {
        match message {
            RadiantMessage::SceneMessage(message) => {
                let response = self.view.scene_mut().handle_message(message);
                if let Some(response) = response {
                    match response {
                        RadiantSceneResponse::Message { message } => {
                            return self.handle_message(message.into())
                        }
                        _ => return Some(response.into()),
                    }
                }
            }
            RadiantMessage::AddRectangle { position, scale } => {
                let id = self.view.scene().document().counter;
                let node = RadiantRectangleNode::new(id, position, scale);
                self.view.scene_mut().add(node.into());
                return self
                    .handle_message(RadiantSceneMessage::SelectNode { id: Some(id) }.into());
            }
            RadiantMessage::AddImage { path, .. } => {
                let screen_descriptor = self.view.scene().screen_descriptor;
                let texture_manager = self.view.scene_mut().texture_manager.clone();
                let document = self.view.scene_mut().document.clone();
                image_loader::load_image(path, move |response| {
                    let image = response
                        .unwrap_or(epaint::ColorImage::new([400, 100], epaint::Color32::RED));
                    if let Ok(mut document) = document.write() {
                        let texture_handle =
                            texture_manager.load_texture("test", image, Default::default());
                        let id = document.counter;
                        let mut node = RadiantImageNode::new(
                            id,
                            [100.0, 200.0],
                            [100.0, 100.0],
                            texture_handle,
                        );
                        node.attach(&screen_descriptor);
                        document.add(node.into());
                    }
                });
            }
            RadiantMessage::AddText { position, .. } => {
                let id = self.view.scene().document().counter;
                let node = RadiantTextNode::new(id, position, [100.0, 100.0]);
                self.view.scene_mut().add(node.into());
                return self
                    .handle_message(RadiantSceneMessage::SelectNode { id: Some(id) }.into());
            }
        }
        None
    }
}
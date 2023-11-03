use serde::{Deserialize, Serialize};
use macro_magic::export_tokens;

#[export_tokens]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RadiantImageMessage {
    AddImage {
        name: String,
        path: String,
    },
}


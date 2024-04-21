use serde::{Deserialize, Serialize};

use crate::{sfx::SFX, r#trait::Trait};
#[derive(Debug, Serialize, Deserialize)]
pub struct Distinction{
    name: String,
    description: String,
    sfxs: Vec<SFX>,
    highlight_traits: Vec<Trait>
}
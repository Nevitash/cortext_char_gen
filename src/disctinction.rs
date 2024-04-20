use derive_builder::Builder;

use crate::{sfx::SFX, r#trait::Trait};
#[derive(Debug)]
pub struct Distinction{
    name: String,
    description: String,
    sfxs: Vec<SFX>,
    highlight_traits: Vec<Trait>
}
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{die::Die, sfx::SFX};

#[derive(Debug, Serialize, Deserialize)]
pub struct TraitSet{
    name: String,
    traits: Vec<Trait>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trait{
    die: Die,
    name: String,
    statement: Statement,
    sfxs: Vec<SFX>,
    specialities: Vec<String>
}
#[derive(Debug, Builder, Clone, Serialize, Deserialize)]
pub struct Statement{
    test: String,
    broken: bool
}
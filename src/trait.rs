use derive_builder::Builder;

use crate::{die::Die, sfx::SFX};

pub trait TraitSet{

}

#[derive(Debug)]
pub struct Trait{
    die: Die,
    name: String,
    statement: Statement,
    sfxs: Vec<SFX>,
    specialities: Vec<String>
}
#[derive(Debug, Builder, Clone)]
pub struct Statement{
    test: String,
    broken: bool
}
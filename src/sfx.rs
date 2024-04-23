use std::{fmt::Debug};

use dyn_clone::DynClone;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SFX{
    name: String,
    description: String,
    costs: Vec<Box<dyn Cost>>,
    benefits: Vec<Box<dyn Benefit>>
}

#[typetag::serde(tag="cost")]
pub trait Cost: Debug + DynClone {

}

#[typetag::serde(tag="benefit")]
pub trait Benefit: Debug + DynClone {
    
}

dyn_clone::clone_trait_object!(Cost);
dyn_clone::clone_trait_object!(Benefit);
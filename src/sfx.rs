use std::{fmt::Debug, sync::Arc};

use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct SFX{
    name: String,
    description: String,
    costs: Vec<Box<dyn Cost>>,
    benefits: Vec<Box<dyn Benefit>>
}

#[typetag::serde(tag="cost")]
pub trait Cost: Debug {

}

#[typetag::serde(tag="benefit")]
pub trait Benefit: Debug {

}
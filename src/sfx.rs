use std::{fmt::Debug, sync::Arc};


#[derive(Debug)]
pub struct SFX{
    name: String,
    description: String,
    costs: Vec<Box<dyn Cost>>,
    benefits: Vec<Box<dyn Benefit>>
}

pub trait Cost: Debug {

}

pub trait Benefit: Debug {

}
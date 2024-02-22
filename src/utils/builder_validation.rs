pub trait BuilderValidator {}

#[derive(Clone, Debug, Copy)]
pub struct Yes;
impl BuilderValidator for Yes {}

#[derive(Clone, Debug, Copy)]
pub struct No;
impl BuilderValidator for No {}

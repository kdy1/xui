use std::{fmt::Debug, hash::Hash};

pub trait Constraints: Debug + Clone + PartialEq + Hash {}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct BoxConstraints {}

impl Constraints for BoxConstraints {}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub struct SliverConstraints {}

impl Constraints for SliverConstraints {}

impl SliverConstraints {
    pub fn as_box_constrains(&self) -> BoxConstraints {}
}

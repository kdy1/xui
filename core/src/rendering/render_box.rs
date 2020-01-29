use super::RenderObject;

pub trait RenderBox {}

#[derive(Debug)]
pub struct RenderBoxObject<R: RenderBox> {
    inner: R,
}

impl<R> RenderObject for RenderBoxObject<R> where R: RenderBox {}

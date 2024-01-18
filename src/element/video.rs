/// A video [`Element`].
///
/// [`Element`]: crate::element::Element
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Video {
    pub start_time: f64,
}

impl Video {
    pub const fn new(start_time: f64) -> Self {
        Self { start_time }
    }
}

/// A sample [`Element`].
///
/// [`Element`]: crate::element::Element
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sample {
    pub start_time: f64,
    pub volume: i32,
}

impl Sample {
    pub const fn new(start_time: f64, volume: i32) -> Self {
        Self { start_time, volume }
    }
}

/// A sample [`StoryboardElement`].
///
/// [`StoryboardElement`]: crate::element::StoryboardElement
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StoryboardSample {
    pub start_time: f64,
    pub volume: i32,
}

impl StoryboardSample {
    pub const fn new(start_time: f64, volume: i32) -> Self {
        Self { start_time, volume }
    }
}

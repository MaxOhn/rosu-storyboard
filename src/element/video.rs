#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StoryboardVideo {
    pub start_time: f64,
}

impl StoryboardVideo {
    pub const fn new(start_time: f64) -> Self {
        Self { start_time }
    }
}

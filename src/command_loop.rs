use std::{cell::RefCell, rc::Rc};

use crate::timeline_group::CommandTimelineGroup;

/// Command loop of a [`StoryboardSprite`].
///
/// [`StoryboardSprite`]: crate::element::StoryboardSprite
#[derive(Clone, Debug, PartialEq)]
pub struct CommandLoop {
    pub group: Rc<RefCell<CommandTimelineGroup>>,
    pub loop_start_time: f64,
    pub total_iterations: u32,
}

impl CommandLoop {
    /// Create a new [`CommandLoop`].
    pub fn new(start_time: f64, repeat_count: u32) -> Self {
        Self {
            group: Rc::new(RefCell::new(CommandTimelineGroup::default())),
            loop_start_time: start_time,
            total_iterations: repeat_count + 1,
        }
    }

    pub fn start_time(&self) -> f64 {
        self.loop_start_time + self.group.borrow().start_time()
    }

    pub fn end_time(&self) -> f64 {
        self.start_time() + self.group.borrow().duration()
    }
}

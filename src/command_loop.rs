use std::{cell::RefCell, rc::Rc};

use crate::timeline_group::CommandTimelineGroup;

/// Command loop of a [`Sprite`].
///
/// [`Sprite`]: crate::element::Sprite
#[derive(Clone, Debug, PartialEq)]
pub struct CommandLoop {
    pub group: CommandTimelineGroup,
    pub loop_start_time: f64,
    pub total_iterations: u32,
}

impl CommandLoop {
    /// Create a new [`CommandLoop`].
    pub fn new(start_time: f64, repeat_count: u32) -> Self {
        Self {
            group: CommandTimelineGroup::default(),
            loop_start_time: start_time,
            total_iterations: repeat_count + 1,
        }
    }

    pub fn start_time(&self) -> f64 {
        self.loop_start_time + self.group.start_time()
    }

    pub fn end_time(&self) -> f64 {
        self.start_time() + self.group.duration()
    }
}

pub(crate) struct CommandLoopInternal {
    pub group: Rc<RefCell<CommandTimelineGroup>>,
    pub loop_start_time: f64,
    pub total_iterations: u32,
}

impl From<CommandLoopInternal> for CommandLoop {
    fn from(l: CommandLoopInternal) -> Self {
        Self {
            group: Rc::into_inner(l.group)
                .expect("multiple strong references around")
                .into_inner(),
            loop_start_time: l.loop_start_time,
            total_iterations: l.total_iterations,
        }
    }
}

impl CommandLoopInternal {
    pub fn new(start_time: f64, repeat_count: u32) -> Self {
        Self {
            group: Rc::new(RefCell::new(CommandTimelineGroup::default())),
            loop_start_time: start_time,
            total_iterations: repeat_count + 1,
        }
    }
}

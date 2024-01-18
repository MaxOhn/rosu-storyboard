use std::{cell::RefCell, rc::Rc};

use crate::timeline_group::CommandTimelineGroup;

/// Command trigger for a [`StoryboardSprite`].
///
/// [`StoryboardSprite`]: crate::element::StoryboardSprite
#[derive(Clone, Debug, PartialEq)]
pub struct CommandTrigger {
    pub group: Rc<RefCell<CommandTimelineGroup>>,
    pub name: String,
    pub start_time: f64,
    pub end_time: f64,
    pub group_num: i32,
}

impl CommandTrigger {
    /// Create a new [`CommandTrigger`].
    pub fn new(name: String, start_time: f64, end_time: f64, group_num: i32) -> Self {
        Self {
            group: Rc::new(RefCell::new(CommandTimelineGroup::default())),
            name,
            start_time,
            end_time,
            group_num,
        }
    }
}

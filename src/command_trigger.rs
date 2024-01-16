use std::{cell::RefCell, rc::Rc};

use crate::timeline_group::CommandTimelineGroup;

#[derive(Clone, Debug, PartialEq)]
pub struct CommandTrigger {
    pub group: Rc<RefCell<CommandTimelineGroup>>,
    pub trigger_name: String,
    pub trigger_start_time: f64,
    pub trigger_end_time: f64,
    pub group_num: i32,
}

impl CommandTrigger {
    pub fn new(trigger_name: String, start_time: f64, end_time: f64, group_num: i32) -> Self {
        Self {
            group: Rc::new(RefCell::new(CommandTimelineGroup::default())),
            trigger_name,
            trigger_start_time: start_time,
            trigger_end_time: end_time,
            group_num,
        }
    }
}

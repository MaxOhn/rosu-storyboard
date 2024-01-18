use std::{cell::RefCell, rc::Rc};

use rosu_map::util::Pos;

use crate::{
    command_loop::{CommandLoop, CommandLoopInternal},
    command_trigger::{CommandTrigger, CommandTriggerInternal},
    timeline_group::CommandTimelineGroup,
    visual::Anchor,
};

/// A sprite [`StoryboardElement`].
///
/// [`StoryboardElement`]: crate::element::StoryboardElement
#[derive(Clone, Debug, PartialEq)]
pub struct StoryboardSprite {
    pub origin: Anchor,
    pub initial_pos: Pos,
    pub timeline_group: CommandTimelineGroup,
    pub loops: Vec<CommandLoop>,
    pub triggers: Vec<CommandTrigger>,
}

impl StoryboardSprite {
    /// Create a new [`StoryboardSprite`].
    pub fn new(origin: Anchor, initial_pos: Pos) -> Self {
        Self {
            origin,
            initial_pos,
            timeline_group: CommandTimelineGroup::default(),
            loops: Vec::new(),
            triggers: Vec::new(),
        }
    }

    pub fn has_commands(&self) -> bool {
        self.timeline_group.has_commands() || self.loops.iter().any(|l| l.group.has_commands())
    }

    pub fn is_drawable(&self) -> bool {
        self.has_commands()
    }

    pub fn start_time(&self) -> f64 {
        let mut min_alpha = None;

        let mut update_min = |start_time, value: f32| match min_alpha {
            Some((ref min_start_time, _)) if start_time < *min_start_time => {
                min_alpha = Some((start_time, value.abs() < f32::EPSILON));
            }
            Some(_) => {}
            None => min_alpha = Some((start_time, value.abs() < f32::EPSILON)),
        };

        if let Some(command) = self.timeline_group.alpha.commands.first() {
            update_min(command.start_time, command.start_value);
        }

        for l in self.loops.iter() {
            if let Some(command) = l.group.alpha.commands.first() {
                update_min(command.start_time + l.loop_start_time, command.start_value);
            }
        }

        min_alpha
            .and_then(|(start_time, is_zero_start_value)| is_zero_start_value.then_some(start_time))
            .unwrap_or_else(|| self.earliest_transform_time())
    }

    pub fn earliest_transform_time(&self) -> f64 {
        self.loops
            .iter()
            .fold(self.timeline_group.start_time(), |min, l| {
                min.min(l.start_time())
            })
    }

    pub fn end_time(&self) -> f64 {
        self.loops
            .iter()
            .fold(self.timeline_group.end_time(), |max, l| {
                max.max(l.end_time())
            })
    }

    pub fn end_time_for_display(&self) -> f64 {
        self.loops
            .iter()
            .fold(self.timeline_group.end_time(), |max, l| {
                max.max(l.start_time() + l.group.duration() * f64::from(l.total_iterations))
            })
    }

    /// Add a [`CommandLoop`] to the sprite.
    // false positive
    #[allow(clippy::missing_panics_doc)]
    pub fn add_loop(&mut self, start_time: f64, repeat_count: u32) -> &mut CommandLoop {
        let new_loop = CommandLoop::new(start_time, repeat_count);
        self.loops.push(new_loop);

        self.loops.last_mut().unwrap()
    }

    /// Add a [`CommandTrigger`] to the sprite.
    // false positive
    #[allow(clippy::missing_panics_doc)]
    pub fn add_trigger(
        &mut self,
        trigger_name: String,
        start_time: f64,
        end_time: f64,
        group_num: i32,
    ) -> &mut CommandTrigger {
        let trigger = CommandTrigger::new(trigger_name, start_time, end_time, group_num);
        self.triggers.push(trigger);

        self.triggers.last_mut().unwrap()
    }
}

pub(crate) struct StoryboardSpriteInternal {
    pub origin: Anchor,
    pub initial_pos: Pos,
    pub timeline_group: Rc<RefCell<CommandTimelineGroup>>,
    pub loops: Vec<CommandLoopInternal>,
    pub triggers: Vec<CommandTriggerInternal>,
}

impl From<StoryboardSpriteInternal> for StoryboardSprite {
    fn from(sprite: StoryboardSpriteInternal) -> Self {
        Self {
            origin: sprite.origin,
            initial_pos: sprite.initial_pos,
            timeline_group: Rc::into_inner(sprite.timeline_group)
                .expect("multiple strong references around")
                .into_inner(),
            loops: sprite.loops.into_iter().map(CommandLoop::from).collect(),
            triggers: sprite
                .triggers
                .into_iter()
                .map(CommandTrigger::from)
                .collect(),
        }
    }
}

impl StoryboardSpriteInternal {
    pub fn new(origin: Anchor, initial_pos: Pos) -> Self {
        Self {
            origin,
            initial_pos,
            timeline_group: Rc::new(RefCell::new(CommandTimelineGroup::default())),
            loops: Vec::new(),
            triggers: Vec::new(),
        }
    }

    // false positive
    #[allow(clippy::missing_panics_doc)]
    pub fn add_loop(&mut self, start_time: f64, repeat_count: u32) -> &mut CommandLoopInternal {
        let new_loop = CommandLoopInternal::new(start_time, repeat_count);
        self.loops.push(new_loop);

        self.loops.last_mut().unwrap()
    }

    // false positive
    #[allow(clippy::missing_panics_doc)]
    pub fn add_trigger(
        &mut self,
        trigger_name: String,
        start_time: f64,
        end_time: f64,
        group_num: i32,
    ) -> &mut CommandTriggerInternal {
        let trigger = CommandTriggerInternal::new(trigger_name, start_time, end_time, group_num);
        self.triggers.push(trigger);

        self.triggers.last_mut().unwrap()
    }
}

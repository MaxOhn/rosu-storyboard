use crate::visual::Easing;

/// A timeline of commands.
#[derive(Clone, Debug, PartialEq)]
pub struct CommandTimeline<T> {
    pub start_time: f64,
    pub end_time: f64,
    pub start_value: T,
    pub end_value: T,
    pub(crate) commands: Vec<TypedCommand<T>>,
}

impl<T: Default> CommandTimeline<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T: Copy> CommandTimeline<T> {
    /// Add a command to the timeline.
    pub fn add(
        &mut self,
        easing: Easing,
        start_time: f64,
        mut end_time: f64,
        start_value: T,
        end_value: T,
    ) {
        if end_time < start_time {
            end_time = start_time;
        }

        self.commands.push(TypedCommand {
            easing,
            start_time,
            end_time,
            start_value,
            end_value,
        });

        if start_time < self.start_time {
            self.start_value = start_value;
            self.start_time = start_time;
        }

        if end_time > self.end_time {
            self.end_value = end_value;
            self.end_time = end_time;
        }
    }
}

impl<T: Default> Default for CommandTimeline<T> {
    fn default() -> Self {
        Self {
            start_time: f64::MAX,
            end_time: f64::MIN,
            start_value: T::default(),
            end_value: T::default(),
            commands: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TypedCommand<T> {
    pub(crate) easing: Easing,
    pub(crate) start_time: f64,
    pub(crate) end_time: f64,
    pub(crate) start_value: T,
    pub(crate) end_value: T,
}

/// Interface of [`CommandTimeline`] without its generic type.
pub trait ICommandTimeline {
    fn start_time(&self) -> f64;
    fn end_time(&self) -> f64;
    fn has_commands(&self) -> bool;
}

impl<T> ICommandTimeline for CommandTimeline<T> {
    fn start_time(&self) -> f64 {
        self.start_time
    }

    fn end_time(&self) -> f64 {
        self.end_time
    }

    fn has_commands(&self) -> bool {
        !self.commands.is_empty()
    }
}

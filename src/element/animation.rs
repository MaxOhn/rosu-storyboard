use std::{cell::RefCell, rc::Rc};

use rosu_map::util::Pos;

use crate::visual::Anchor;

use super::StoryboardSprite;

#[derive(Clone, Debug, PartialEq)]
pub struct StoryboardAnimation {
    pub sprite: Rc<RefCell<StoryboardSprite>>,
    pub frame_count: i32,
    pub frame_delay: f64,
    pub loop_kind: AnimationLoopType,
}

impl StoryboardAnimation {
    pub fn new(
        origin: Anchor,
        initial_pos: Pos,
        frame_count: i32,
        frame_delay: f64,
        loop_kind: AnimationLoopType,
    ) -> Self {
        Self {
            sprite: Rc::new(RefCell::new(StoryboardSprite::new(origin, initial_pos))),
            frame_count,
            frame_delay,
            loop_kind,
        }
    }

    pub fn has_commands(&self) -> bool {
        self.sprite.borrow().has_commands()
    }

    pub fn is_drawable(&self) -> bool {
        self.sprite.borrow().is_drawable()
    }

    pub fn start_time(&self) -> f64 {
        self.sprite.borrow().start_time()
    }

    pub fn earliest_transform_time(&self) -> f64 {
        self.sprite.borrow().earliest_transform_time()
    }

    pub fn end_time(&self) -> f64 {
        self.sprite.borrow().end_time()
    }

    pub fn end_time_for_display(&self) -> f64 {
        self.sprite.borrow().end_time_for_display()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AnimationLoopType {
    LoopForever,
    LoopOnce,
}

impl AnimationLoopType {
    pub fn parse(s: &str) -> Self {
        match s.parse::<u8>() {
            Ok(0) => Self::LoopForever,
            Ok(1) => Self::LoopOnce,
            _ => match s {
                "LoopOnce" => Self::LoopOnce,
                _ => Self::LoopForever,
            },
        }
    }
}

use rosu_map::util::Pos;

use crate::visual::Anchor;

use super::{Sprite, SpriteInternal};

/// An animation [`Element`].
///
/// [`Element`]: crate::element::Element
#[derive(Clone, Debug, PartialEq)]
pub struct Animation {
    pub sprite: Sprite,
    pub frame_count: i32,
    pub frame_delay: f64,
    pub loop_kind: AnimationLoopType,
}

impl Animation {
    /// Create a new [`Animation`].
    pub fn new(
        origin: Anchor,
        initial_pos: Pos,
        frame_count: i32,
        frame_delay: f64,
        loop_kind: AnimationLoopType,
    ) -> Self {
        Self {
            sprite: Sprite::new(origin, initial_pos),
            frame_count,
            frame_delay,
            loop_kind,
        }
    }

    pub fn has_commands(&self) -> bool {
        self.sprite.has_commands()
    }

    pub fn is_drawable(&self) -> bool {
        self.sprite.is_drawable()
    }

    pub fn start_time(&self) -> f64 {
        self.sprite.start_time()
    }

    pub fn earliest_transform_time(&self) -> f64 {
        self.sprite.earliest_transform_time()
    }

    pub fn end_time(&self) -> f64 {
        self.sprite.end_time()
    }

    pub fn end_time_for_display(&self) -> f64 {
        self.sprite.end_time_for_display()
    }
}

/// The loop type of an [`Animation`].
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AnimationLoopType {
    LoopForever = 0,
    LoopOnce = 1,
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

pub(crate) struct AnimationInternal {
    pub sprite: SpriteInternal,
    pub frame_count: i32,
    pub frame_delay: f64,
    pub loop_kind: AnimationLoopType,
}

impl From<AnimationInternal> for Animation {
    fn from(animation: AnimationInternal) -> Self {
        Self {
            sprite: animation.sprite.into(),
            frame_count: animation.frame_count,
            frame_delay: animation.frame_delay,
            loop_kind: animation.loop_kind,
        }
    }
}

impl AnimationInternal {
    pub fn new(
        origin: Anchor,
        initial_pos: Pos,
        frame_count: i32,
        frame_delay: f64,
        loop_kind: AnimationLoopType,
    ) -> Self {
        Self {
            sprite: SpriteInternal::new(origin, initial_pos),
            frame_count,
            frame_delay,
            loop_kind,
        }
    }
}

use rosu_map::util::Pos;

use crate::visual::Anchor;

use super::{StoryboardSprite, StoryboardSpriteInternal};

/// An animation [`StoryboardElement`].
///
/// [`StoryboardElement`]: crate::element::StoryboardElement
#[derive(Clone, Debug, PartialEq)]
pub struct StoryboardAnimation {
    pub sprite: StoryboardSprite,
    pub frame_count: i32,
    pub frame_delay: f64,
    pub loop_kind: AnimationLoopType,
}

impl StoryboardAnimation {
    /// Create a new [`StoryboardAnimation`].
    pub fn new(
        origin: Anchor,
        initial_pos: Pos,
        frame_count: i32,
        frame_delay: f64,
        loop_kind: AnimationLoopType,
    ) -> Self {
        Self {
            sprite: StoryboardSprite::new(origin, initial_pos),
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

/// The loop type of a [`StoryboardAnimation`].
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

pub(crate) struct StoryboardAnimationInternal {
    pub sprite: StoryboardSpriteInternal,
    pub frame_count: i32,
    pub frame_delay: f64,
    pub loop_kind: AnimationLoopType,
}

impl From<StoryboardAnimationInternal> for StoryboardAnimation {
    fn from(animation: StoryboardAnimationInternal) -> Self {
        Self {
            sprite: animation.sprite.into(),
            frame_count: animation.frame_count,
            frame_delay: animation.frame_delay,
            loop_kind: animation.loop_kind,
        }
    }
}

impl StoryboardAnimationInternal {
    pub fn new(
        origin: Anchor,
        initial_pos: Pos,
        frame_count: i32,
        frame_delay: f64,
        loop_kind: AnimationLoopType,
    ) -> Self {
        Self {
            sprite: StoryboardSpriteInternal::new(origin, initial_pos),
            frame_count,
            frame_delay,
            loop_kind,
        }
    }
}

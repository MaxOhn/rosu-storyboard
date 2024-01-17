//! TODO: docs

#![deny(rustdoc::broken_intra_doc_links, rustdoc::missing_crate_level_docs)]
#![warn(clippy::missing_const_for_fn, clippy::pedantic)]
#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::struct_excessive_bools,
    clippy::match_same_arms,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::explicit_iter_loop,
    clippy::default_trait_access
)]

pub use self::{
    command_loop::CommandLoop,
    command_trigger::CommandTrigger,
    decode::{ParseStoryboardError, StoryboardState},
    layer::StoryboardLayer,
    storyboard::Storyboard,
    timeline::{CommandTimeline, ICommandTimeline},
    timeline_group::CommandTimelineGroup,
};

mod command_loop;
mod command_trigger;
mod decode;
pub mod element;
mod encode;
mod layer;
mod storyboard;
mod timeline;
mod timeline_group;
pub mod visual;

pub mod reexport {
    pub use rosu_map::{section::colors::Color, util::Pos};
}

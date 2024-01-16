use rosu_map::{section::colors::Color, util::Pos};

use crate::{
    timeline::{CommandTimeline, ICommandTimeline},
    visual::BlendingParameters,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CommandTimelineGroup {
    pub x: CommandTimeline<f32>,
    pub y: CommandTimeline<f32>,
    pub scale: CommandTimeline<f32>,
    pub vector_scale: CommandTimeline<Pos>,
    pub rotation: CommandTimeline<f32>,
    pub color: CommandTimeline<Color>,
    pub alpha: CommandTimeline<f32>,
    pub blending_parameters: CommandTimeline<BlendingParameters>,
    pub flip_h: CommandTimeline<bool>,
    pub flip_v: CommandTimeline<bool>,
}

impl CommandTimelineGroup {
    pub fn has_commands(&self) -> bool {
        self.fold_timelines(false, |any, timeline| any || timeline.has_commands())
    }

    pub fn start_time(&self) -> f64 {
        self.fold_timelines(f64::MAX, |min, timeline| min.min(timeline.start_time()))
    }

    pub fn end_time(&self) -> f64 {
        self.fold_timelines(f64::MIN, |max, timeline| max.max(timeline.end_time()))
    }

    pub fn duration(&self) -> f64 {
        self.end_time() - self.start_time()
    }

    pub fn fold_timelines<B, F>(&self, init: B, mut f: F) -> B
    where
        F: FnMut(B, &dyn ICommandTimeline) -> B,
    {
        let mut res = init;

        res = f(res, &self.x);
        res = f(res, &self.y);
        res = f(res, &self.scale);
        res = f(res, &self.vector_scale);
        res = f(res, &self.rotation);
        res = f(res, &self.color);
        res = f(res, &self.alpha);
        res = f(res, &self.blending_parameters);
        res = f(res, &self.flip_h);
        res = f(res, &self.flip_v);

        res
    }
}

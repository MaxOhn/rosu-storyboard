use std::{borrow::Cow, cell::RefCell, cmp, collections::HashMap, rc::Rc, str::Split};

use rosu_map::{
    section::{
        colors::Color,
        events::{BreakPeriod, EventType, ParseEventTypeError},
    },
    util::{KeyValue, ParseNumber, ParseNumberError, Pos, StrExt},
    DecodeBeatmap, DecodeState,
};

use crate::{
    element::{
        AnimationInternal, AnimationLoopType, ElementInternal, ElementKindInternal, Sample,
        SpriteInternal, Video,
    },
    layer::StoryLayer,
    storyboard::StoryboardInternal,
    timeline_group::CommandTimelineGroup,
    visual::{BlendingParameters, Easing, Origins},
    Storyboard,
};

use self::pending::PendingSprite;

/// All the ways that parsing a `.osu` file into [`Storyboard`] can fail.
#[derive(Debug, thiserror::Error)]
pub enum ParseStoryboardError {
    #[error("failed to parse event type")]
    EventType(#[from] ParseEventTypeError),
    #[error("invalid line")]
    InvalidLine,
    #[error("failed to parse number")]
    Number(#[from] ParseNumberError),
    #[error("unknown command type")]
    UnknownCommandType,
}

/// The parsing state for [`Storyboard`] in [`DecodeBeatmap`].
pub struct StoryboardState {
    format_version: i32,
    use_skin_sprites: bool,
    background_file: String,
    breaks: Vec<BreakPeriod>,
    storyboard: StoryboardInternal,
    sprite: PendingSprite,
    timeline_group: Option<Rc<RefCell<CommandTimelineGroup>>>,
    variables: HashMap<Box<str>, Box<str>>,
}

impl StoryboardState {
    fn decode_variables<'a>(&self, line: &'a str) -> Cow<'a, str> {
        let mut line = Cow::Borrowed(line);

        while line.contains('$') {
            let mut no_change = true;

            for (key, value) in self.variables.iter() {
                if let Some(idx) = line.find(key.as_ref()) {
                    line.to_mut().replace_range(idx..idx + key.len(), value);
                    no_change = false;
                }
            }

            if no_change {
                break;
            }
        }

        line
    }

    fn parse_video(&mut self, split: &mut Split<'_, char>) -> Result<(), ParseStoryboardError> {
        const VIDEO_EXTENSIONS: &[[u8; 3]] = &[
            *b"mp4", *b"mov", *b"avi", *b"flv", *b"mpg", *b"wmv", *b"m4v",
        ];

        let Some((offset, path)) = split.next().zip(split.next()) else {
            return Err(ParseStoryboardError::InvalidLine);
        };

        let offset = i32::parse(offset)?;
        let path = path.clean_filename();

        if let [.., a, b, c] = path.as_bytes() {
            let extension = [
                a.to_ascii_lowercase(),
                b.to_ascii_lowercase(),
                c.to_ascii_lowercase(),
            ];

            if VIDEO_EXTENSIONS.contains(&extension) {
                let video = Video::new(f64::from(offset));
                self.storyboard
                    .get_layer("Video")
                    .elements
                    .push(ElementInternal {
                        path,
                        kind: ElementKindInternal::Video(video),
                    });
            } else {
                self.background_file = path;
            }
        }

        Ok(())
    }

    fn parse_sprite(&mut self, split: &mut Split<'_, char>) -> Result<(), ParseStoryboardError> {
        let Some(((((layer, origin), path), x), y)) = split
            .next()
            .zip(split.next())
            .zip(split.next())
            .zip(split.next())
            .zip(split.next())
        else {
            return Err(ParseStoryboardError::InvalidLine);
        };

        let layer = StoryLayer::parse(layer);
        let origin = Origins::parse(origin);
        let path = path.clean_filename();
        let x = f32::parse_with_limits(x, MAX_COORDINATE_VALUE as f32)?;
        let y = f32::parse_with_limits(y, MAX_COORDINATE_VALUE as f32)?;
        let sprite = SpriteInternal::new(origin, Pos::new(x, y));

        if self.background_file.is_empty() {
            self.background_file = path.clone();
        }

        self.sprite.set_sprite(path, &layer, sprite);

        Ok(())
    }

    fn parse_animation(&mut self, split: &mut Split<'_, char>) -> Result<(), ParseStoryboardError> {
        let Some(((((((layer, origin), path), x), y), frame_count), frame_delay)) = split
            .next()
            .zip(split.next())
            .zip(split.next())
            .zip(split.next())
            .zip(split.next())
            .zip(split.next())
            .zip(split.next())
        else {
            return Err(ParseStoryboardError::InvalidLine);
        };

        let layer = StoryLayer::parse(layer);
        let origin = Origins::parse(origin);
        let path = path.clean_filename();
        let x = f32::parse_with_limits(x, MAX_COORDINATE_VALUE as f32)?;
        let y = f32::parse_with_limits(y, MAX_COORDINATE_VALUE as f32)?;
        let frame_count = i32::parse(frame_count)?;
        let mut frame_delay = f64::parse(frame_delay)?;

        if self.format_version < 6 {
            frame_delay = (0.015 * frame_delay).round() * 1.186 * (1000.0 / 60.0);
        }

        let loop_type = if let Some(loop_type) = split.next() {
            AnimationLoopType::parse(loop_type)
        } else {
            AnimationLoopType::LoopForever
        };

        let animation =
            AnimationInternal::new(origin, Pos::new(x, y), frame_count, frame_delay, loop_type);

        self.sprite.set_animation(path, &layer, animation);

        Ok(())
    }

    fn parse_sample(&mut self, split: &mut Split<'_, char>) -> Result<(), ParseStoryboardError> {
        let Some(((time, layer), path)) = split.next().zip(split.next()).zip(split.next()) else {
            return Err(ParseStoryboardError::InvalidLine);
        };

        let time = f64::parse(time)?;
        let layer = StoryLayer::parse(layer);
        let path = path.clean_filename();

        let volume = if let Some(volume) = split.next() {
            f32::parse(volume)?
        } else {
            100.0
        };

        let sample = Sample::new(time, volume as i32);
        self.storyboard
            .get_layer(layer.as_str())
            .elements
            .push(ElementInternal {
                path,
                kind: ElementKindInternal::Sample(sample),
            });

        Ok(())
    }

    fn parse_background(
        &mut self,
        split: &mut Split<'_, char>,
    ) -> Result<(), ParseStoryboardError> {
        let background_file = split.nth(1).ok_or(ParseStoryboardError::InvalidLine)?;
        self.background_file = background_file.clean_filename();

        Ok(())
    }

    fn parse_break(&mut self, split: &mut Split<'_, char>) -> Result<(), ParseStoryboardError> {
        let Some((start_time, end_time)) = split.next().zip(split.next()) else {
            return Err(ParseStoryboardError::InvalidLine);
        };

        let start_time = f64::parse(start_time)?;
        let end_time = start_time.max(f64::parse(end_time)?);

        self.breaks.push(BreakPeriod {
            start_time,
            end_time,
        });

        Ok(())
    }

    fn parse_trigger(&mut self, split: &mut Split<'_, char>) -> Result<(), ParseStoryboardError> {
        let Some(name) = split.next() else {
            return Err(ParseStoryboardError::InvalidLine);
        };

        let Some(sprite) = self.sprite.inner_mut() else {
            return Ok(());
        };

        let start_time = if let Some(start_time) = split.next() {
            f64::parse(start_time)?
        } else {
            f64::MIN
        };

        let end_time = if let Some(end_time) = split.next() {
            f64::parse(end_time)?
        } else {
            f64::MAX
        };

        let group_num = if let Some(group_num) = split.next() {
            i32::parse(group_num)?
        } else {
            0
        };

        let trigger = sprite.add_trigger(name.to_owned(), start_time, end_time, group_num);
        self.timeline_group = Some(Rc::clone(&trigger.group));

        Ok(())
    }

    fn parse_loop(&mut self, split: &mut Split<'_, char>) -> Result<(), ParseStoryboardError> {
        let Some((start_time, repeat_count)) = split.next().zip(split.next()) else {
            return Err(ParseStoryboardError::InvalidLine);
        };

        let Some(sprite) = self.sprite.inner_mut() else {
            return Ok(());
        };

        let start_time = f64::parse(start_time)?;
        let repeat_count = i32::parse(repeat_count)?;

        let new_loop = sprite.add_loop(start_time, cmp::max(0, repeat_count - 1) as u32);
        self.timeline_group = Some(Rc::clone(&new_loop.group));

        Ok(())
    }

    fn parse_alpha(
        &mut self,
        split: &mut Split<'_, char>,
        easing: Easing,
        start_time: f64,
        end_time: f64,
    ) -> Result<(), ParseStoryboardError> {
        let Some(start_value) = split.next() else {
            return Err(ParseStoryboardError::InvalidLine);
        };

        let Some(ref group) = self.timeline_group else {
            return Ok(());
        };

        let start_value = f32::parse(start_value)?;

        let end_value = if let Some(end_value) = split.next() {
            f32::parse(end_value)?
        } else {
            start_value
        };

        group
            .borrow_mut()
            .alpha
            .add(easing, start_time, end_time, start_value, end_value);

        Ok(())
    }

    fn parse_scale(
        &mut self,
        split: &mut Split<'_, char>,
        easing: Easing,
        start_time: f64,
        end_time: f64,
    ) -> Result<(), ParseStoryboardError> {
        let Some(start_value) = split.next() else {
            return Err(ParseStoryboardError::InvalidLine);
        };

        let Some(ref group) = self.timeline_group else {
            return Ok(());
        };

        let start_value = f32::parse(start_value)?;

        let end_value = if let Some(end_value) = split.next() {
            f32::parse(end_value)?
        } else {
            start_value
        };

        group
            .borrow_mut()
            .scale
            .add(easing, start_time, end_time, start_value, end_value);

        Ok(())
    }

    fn parse_vector_scale(
        &mut self,
        split: &mut Split<'_, char>,
        easing: Easing,
        start_time: f64,
        end_time: f64,
    ) -> Result<(), ParseStoryboardError> {
        let Some((start_x, start_y)) = split.next().zip(split.next()) else {
            return Err(ParseStoryboardError::InvalidLine);
        };

        let Some(ref group) = self.timeline_group else {
            return Ok(());
        };

        let start_x = f32::parse(start_x)?;
        let start_y = f32::parse(start_y)?;

        let end_x = if let Some(end_x) = split.next() {
            f32::parse(end_x)?
        } else {
            start_x
        };

        let end_y = if let Some(end_y) = split.next() {
            f32::parse(end_y)?
        } else {
            start_y
        };

        group.borrow_mut().vector_scale.add(
            easing,
            start_time,
            end_time,
            Pos::new(start_x, start_y),
            Pos::new(end_x, end_y),
        );

        Ok(())
    }

    fn parse_rotation(
        &mut self,
        split: &mut Split<'_, char>,
        easing: Easing,
        start_time: f64,
        end_time: f64,
    ) -> Result<(), ParseStoryboardError> {
        let Some(start_value) = split.next() else {
            return Err(ParseStoryboardError::InvalidLine);
        };

        let Some(ref group) = self.timeline_group else {
            return Ok(());
        };

        let start_value = f32::parse(start_value)?;

        let end_value = if let Some(end_value) = split.next() {
            f32::parse(end_value)?
        } else {
            start_value
        };

        group.borrow_mut().rotation.add(
            easing,
            start_time,
            end_time,
            start_value.to_degrees(),
            end_value.to_degrees(),
        );

        Ok(())
    }

    fn parse_pos(
        &mut self,
        split: &mut Split<'_, char>,
        easing: Easing,
        start_time: f64,
        end_time: f64,
    ) -> Result<(), ParseStoryboardError> {
        let Some((start_x, start_y)) = split.next().zip(split.next()) else {
            return Err(ParseStoryboardError::InvalidLine);
        };

        let Some(ref group) = self.timeline_group else {
            return Ok(());
        };

        let start_x = f32::parse(start_x)?;
        let start_y = f32::parse(start_y)?;

        let end_x = if let Some(end_x) = split.next() {
            f32::parse(end_x)?
        } else {
            start_x
        };

        let end_y = if let Some(end_y) = split.next() {
            f32::parse(end_y)?
        } else {
            start_y
        };

        group
            .borrow_mut()
            .x
            .add(easing, start_time, end_time, start_x, end_x);
        group
            .borrow_mut()
            .y
            .add(easing, start_time, end_time, start_y, end_y);

        Ok(())
    }

    fn parse_x(
        &mut self,
        split: &mut Split<'_, char>,
        easing: Easing,
        start_time: f64,
        end_time: f64,
    ) -> Result<(), ParseStoryboardError> {
        let Some(start_value) = split.next() else {
            return Err(ParseStoryboardError::InvalidLine);
        };

        let Some(ref group) = self.timeline_group else {
            return Ok(());
        };

        let start_value = f32::parse(start_value)?;

        let end_value = if let Some(end_value) = split.next() {
            f32::parse(end_value)?
        } else {
            start_value
        };

        group
            .borrow_mut()
            .x
            .add(easing, start_time, end_time, start_value, end_value);

        Ok(())
    }

    fn parse_y(
        &mut self,
        split: &mut Split<'_, char>,
        easing: Easing,
        start_time: f64,
        end_time: f64,
    ) -> Result<(), ParseStoryboardError> {
        let Some(start_value) = split.next() else {
            return Err(ParseStoryboardError::InvalidLine);
        };

        let Some(ref group) = self.timeline_group else {
            return Ok(());
        };

        let start_value = f32::parse(start_value)?;

        let end_value = if let Some(end_value) = split.next() {
            f32::parse(end_value)?
        } else {
            start_value
        };

        group
            .borrow_mut()
            .y
            .add(easing, start_time, end_time, start_value, end_value);

        Ok(())
    }

    fn parse_color(
        &mut self,
        split: &mut Split<'_, char>,
        easing: Easing,
        start_time: f64,
        end_time: f64,
    ) -> Result<(), ParseStoryboardError> {
        let Some(((start_red, start_green), start_blue)) =
            split.next().zip(split.next()).zip(split.next())
        else {
            return Err(ParseStoryboardError::InvalidLine);
        };

        let Some(ref group) = self.timeline_group else {
            return Ok(());
        };

        let start_red = f32::parse(start_red)?;
        let start_green = f32::parse(start_green)?;
        let start_blue = f32::parse(start_blue)?;

        let end_red = if let Some(end_red) = split.next() {
            f32::parse(end_red)?
        } else {
            start_red
        };

        let end_green = if let Some(end_green) = split.next() {
            f32::parse(end_green)?
        } else {
            start_green
        };

        let end_blue = if let Some(end_blue) = split.next() {
            f32::parse(end_blue)?
        } else {
            start_blue
        };

        group.borrow_mut().color.add(
            easing,
            start_time,
            end_time,
            Color::new(start_red as u8, start_green as u8, start_blue as u8, 255),
            Color::new(end_red as u8, end_green as u8, end_blue as u8, 255),
        );

        Ok(())
    }

    fn add_blending(&mut self, easing: Easing, start_time: f64, end_time: f64) {
        if let Some(ref group) = self.timeline_group {
            group.borrow_mut().blending_parameters.add(
                easing,
                start_time,
                end_time,
                BlendingParameters::ADDITIVE,
                if (end_time - start_time).abs() < f64::EPSILON {
                    BlendingParameters::ADDITIVE
                } else {
                    BlendingParameters::INHERIT
                },
            );
        }
    }

    fn add_flip_h(&mut self, easing: Easing, start_time: f64, end_time: f64) {
        if let Some(ref group) = self.timeline_group {
            group.borrow_mut().flip_h.add(
                easing,
                start_time,
                end_time,
                true,
                (end_time - start_time).abs() < f64::EPSILON,
            );
        }
    }

    fn add_flip_v(&mut self, easing: Easing, start_time: f64, end_time: f64) {
        if let Some(ref group) = self.timeline_group {
            group.borrow_mut().flip_v.add(
                easing,
                start_time,
                end_time,
                true,
                (end_time - start_time).abs() < f64::EPSILON,
            );
        }
    }
}

impl DecodeState for StoryboardState {
    fn create(format_version: i32) -> Self {
        let storyboard = Storyboard {
            format_version,
            ..Default::default()
        };

        Self {
            format_version: storyboard.format_version,
            use_skin_sprites: storyboard.use_skin_sprites,
            background_file: storyboard.background_file,
            breaks: storyboard.breaks,
            storyboard: StoryboardInternal::default(),
            sprite: PendingSprite::default(),
            timeline_group: None,
            variables: HashMap::default(),
        }
    }
}

impl From<StoryboardState> for Storyboard {
    fn from(mut state: StoryboardState) -> Self {
        state.sprite.add(&mut state.storyboard);

        // drop the Rc so that the Rcs in the layers are unique
        state.timeline_group = None;

        let min_layer_depth = state.storyboard.min_layer_depth;
        let layers = state.storyboard.convert_layers();

        Storyboard {
            format_version: state.format_version,
            use_skin_sprites: state.use_skin_sprites,
            background_file: state.background_file,
            breaks: state.breaks,
            layers,
            min_layer_depth,
        }
    }
}

const MAX_COORDINATE_VALUE: i32 = 131_072;

impl DecodeBeatmap for Storyboard {
    type Error = ParseStoryboardError;
    type State = StoryboardState;

    fn should_skip_line(line: &str) -> bool {
        line.is_empty() || line.starts_with("//")
    }

    fn parse_general(state: &mut Self::State, line: &str) -> Result<(), Self::Error> {
        let mut split = line.trim_comment().split(':').map(str::trim);

        if split.next() == Some("UseSkinSprites") {
            state.use_skin_sprites = split.next() == Some("1");
        }

        Ok(())
    }

    fn parse_editor(_: &mut Self::State, _: &str) -> Result<(), Self::Error> {
        Ok(())
    }

    fn parse_metadata(_: &mut Self::State, _: &str) -> Result<(), Self::Error> {
        Ok(())
    }

    fn parse_difficulty(_: &mut Self::State, _: &str) -> Result<(), Self::Error> {
        Ok(())
    }

    fn parse_events(state: &mut Self::State, line: &str) -> Result<(), Self::Error> {
        let line = state.decode_variables(line.trim_comment());

        let depth = line
            .chars()
            .take_while(|ch| matches!(ch, ' ' | '_'))
            .count();

        let line = &line[depth..];
        let mut split = line.split(',');

        if depth == 0 {
            let Some(event_type) = split.next() else {
                return Err(ParseStoryboardError::InvalidLine);
            };

            state.sprite.add(&mut state.storyboard);

            return match event_type.parse()? {
                EventType::Video => state.parse_video(&mut split),
                EventType::Sprite => state.parse_sprite(&mut split),
                EventType::Animation => state.parse_animation(&mut split),
                EventType::Sample => state.parse_sample(&mut split),
                EventType::Background => state.parse_background(&mut split),
                EventType::Break => state.parse_break(&mut split),
                EventType::Color => Ok(()),
            };
        }

        if depth < 2 {
            if let Some(sprite) = state.sprite.inner() {
                state.timeline_group = Some(Rc::clone(&sprite.timeline_group));
            }
        }

        let Some(command_type) = split.next() else {
            return Err(ParseStoryboardError::InvalidLine);
        };

        match command_type {
            "T" => return state.parse_trigger(&mut split),
            "L" => return state.parse_loop(&mut split),
            _ => {}
        }

        let Some(((easing, start_time), end_time)) =
            split.next().zip(split.next()).zip(split.next())
        else {
            return Err(ParseStoryboardError::InvalidLine);
        };

        let easing = Easing::from(i32::parse(easing)?);
        let start_time = f64::parse(start_time)?;

        let end_time = if end_time.is_empty() {
            start_time
        } else {
            f64::parse(end_time)?
        };

        match command_type {
            "F" => state.parse_alpha(&mut split, easing, start_time, end_time),
            "S" => state.parse_scale(&mut split, easing, start_time, end_time),
            "V" => state.parse_vector_scale(&mut split, easing, start_time, end_time),
            "R" => state.parse_rotation(&mut split, easing, start_time, end_time),
            "M" => state.parse_pos(&mut split, easing, start_time, end_time),
            "MX" => state.parse_x(&mut split, easing, start_time, end_time),
            "MY" => state.parse_y(&mut split, easing, start_time, end_time),
            "C" => state.parse_color(&mut split, easing, start_time, end_time),
            "P" => {
                let Some(kind) = split.next() else {
                    return Err(ParseStoryboardError::InvalidLine);
                };

                match kind {
                    "A" => state.add_blending(easing, start_time, end_time),
                    "H" => state.add_flip_h(easing, start_time, end_time),
                    "V" => state.add_flip_v(easing, start_time, end_time),
                    _ => {}
                }

                Ok(())
            }
            _ => Err(ParseStoryboardError::UnknownCommandType),
        }
    }

    fn parse_timing_points(_: &mut Self::State, _: &str) -> Result<(), Self::Error> {
        Ok(())
    }

    fn parse_colors(_: &mut Self::State, _: &str) -> Result<(), Self::Error> {
        Ok(())
    }

    fn parse_hit_objects(_: &mut Self::State, _: &str) -> Result<(), Self::Error> {
        Ok(())
    }

    fn parse_variables(state: &mut Self::State, line: &str) -> Result<(), Self::Error> {
        let pair = match line.split_once('=') {
            Some((key, value)) => KeyValue { key, value },
            None => KeyValue {
                key: line,
                value: "",
            },
        };

        if let Some(value) = state.variables.get_mut(pair.key) {
            *value = pair.value.into();
        } else {
            state.variables.insert(pair.key.into(), pair.value.into());
        }

        Ok(())
    }

    fn parse_catch_the_beat(_: &mut Self::State, _: &str) -> Result<(), Self::Error> {
        Ok(())
    }

    fn parse_mania(_: &mut Self::State, _: &str) -> Result<(), Self::Error> {
        Ok(())
    }
}

// Prevent access of fields by abstracting through a module
mod pending {
    use crate::{
        element::{AnimationInternal, ElementInternal, ElementKindInternal, SpriteInternal},
        layer::StoryLayer,
        storyboard::StoryboardInternal,
    };

    #[derive(Default)]
    pub struct PendingSprite(Option<PendingSpriteInner>);

    impl PendingSprite {
        pub fn set_sprite(&mut self, path: String, layer: &StoryLayer<'_>, sprite: SpriteInternal) {
            self.0 = Some(PendingSpriteInner {
                path,
                layer: layer.as_str().into(),
                kind: PendingSpriteKind::Sprite(sprite),
            });
        }

        pub fn set_animation(
            &mut self,
            path: String,
            layer: &StoryLayer<'_>,
            animation: AnimationInternal,
        ) {
            self.0 = Some(PendingSpriteInner {
                path,
                layer: layer.as_str().into(),
                kind: PendingSpriteKind::Animation(animation),
            });
        }

        pub fn add(&mut self, storyboard: &mut StoryboardInternal) {
            let Some(inner) = self.0.take() else { return };

            match inner.kind {
                PendingSpriteKind::Animation(animation) => storyboard
                    .get_layer(inner.layer.as_ref())
                    .elements
                    .push(ElementInternal {
                        path: inner.path,
                        kind: ElementKindInternal::Animation(animation),
                    }),
                PendingSpriteKind::Sprite(sprite) => storyboard
                    .get_layer(inner.layer.as_ref())
                    .elements
                    .push(ElementInternal {
                        path: inner.path,
                        kind: ElementKindInternal::Sprite(sprite),
                    }),
            }
        }

        pub fn inner(&self) -> Option<&SpriteInternal> {
            self.0.as_ref().map(|inner| match inner.kind {
                PendingSpriteKind::Animation(ref animation) => &animation.sprite,
                PendingSpriteKind::Sprite(ref sprite) => sprite,
            })
        }

        pub fn inner_mut(&mut self) -> Option<&mut SpriteInternal> {
            self.0.as_mut().map(|inner| match inner.kind {
                PendingSpriteKind::Animation(ref mut animation) => &mut animation.sprite,
                PendingSpriteKind::Sprite(ref mut sprite) => sprite,
            })
        }
    }

    struct PendingSpriteInner {
        path: String,
        layer: Box<str>,
        kind: PendingSpriteKind,
    }

    enum PendingSpriteKind {
        Animation(AnimationInternal),
        Sprite(SpriteInternal),
    }
}

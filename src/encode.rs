use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    fs::File,
    io::{BufWriter, Error as IoError, ErrorKind, Result as IoResult, Write},
    path::Path,
};

use rosu_map::{section::events::EventType, util::StrExt};

use crate::{
    command::{CommandTimelineGroup, TypedCommand},
    element::ElementKind,
    visual::Origins,
    Storyboard,
};

impl Storyboard {
    /// Encode a [`Storyboard`] into content of a `.osb` file and store it at
    /// the given path.
    pub fn encode_to_path<P: AsRef<Path>>(&self, path: P) -> IoResult<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);

        self.encode(writer)
    }

    /// Encode a [`Storyboard`] into content of a `.osb` file and store it into
    /// a [`String`].
    pub fn encode_to_string(&self) -> IoResult<String> {
        let mut writer = Vec::with_capacity(4096);
        self.encode(&mut writer)?;

        String::from_utf8(writer).map_err(|e| IoError::new(ErrorKind::Other, e))
    }

    /// Encode a [`Storyboard`] into content of a `.osb` file.
    pub fn encode<W: Write>(&self, mut writer: W) -> IoResult<()> {
        writeln!(writer, "osu file format v{}", self.format_version)?;

        writer.write_all(b"\n")?;
        self.encode_general(&mut writer)?;

        writer.write_all(b"\n")?;
        self.encode_events(&mut writer)?;

        writer.flush()
    }

    fn encode_general<W: Write>(&self, writer: &mut W) -> IoResult<()> {
        writeln!(
            writer,
            "[General]\nUseSkinSprites: {}",
            if self.use_skin_sprites { "1" } else { "0" }
        )
    }

    fn encode_events<W: Write>(&self, writer: &mut W) -> IoResult<()> {
        writer.write_all(b"[Events]\n")?;

        self.encode_background_and_video(writer)?;
        self.encode_breaks(writer)?;
        self.encode_layers(writer)?;
        self.encode_samples(writer)?;

        Ok(())
    }

    fn encode_background_and_video<W: Write>(&self, writer: &mut W) -> IoResult<()> {
        writer.write_all(b"//Background and Video events\n")?;

        if !self.background_file.is_empty() {
            writeln!(
                writer,
                "{},0,\"{}\",0,0",
                EventType::Background as i32,
                self.background_file.to_standardized_path()
            )?;
        }

        let video_elems = self
            .layers
            .values()
            .flat_map(|layer| layer.elements.iter())
            .filter_map(|elem| {
                if let ElementKind::Video(ref video) = elem.kind {
                    Some((elem.path.as_str(), video))
                } else {
                    None
                }
            });

        for (path, video) in video_elems {
            writeln!(
                writer,
                "{},{},\"{path}\"",
                EventType::Video as i32,
                video.start_time,
            )?;
        }

        Ok(())
    }

    fn encode_breaks<W: Write>(&self, writer: &mut W) -> IoResult<()> {
        writer.write_all(b"//Break Periods\n")?;

        for b in self.breaks.iter() {
            writeln!(
                writer,
                "{},{},{}",
                EventType::Break as i32,
                b.start_time,
                b.end_time
            )?;
        }

        Ok(())
    }

    fn encode_layers<W: Write>(&self, writer: &mut W) -> IoResult<()> {
        writer.write_all(b"//Storyboard layers\n")?;

        let elems = self
            .layers
            .iter()
            .filter_map(|(name, layer)| {
                let int = match name.as_str() {
                    "Background" => 0,
                    "Fail" => 1,
                    "Pass" => 2,
                    "Foreground" => 3,
                    _ => return None,
                };

                Some((layer, int))
            })
            .flat_map(|(layer, int)| layer.elements.iter().map(move |elem| (int, elem)));

        for (layer, elem) in elems {
            let sprite = match elem.kind {
                ElementKind::Animation(ref animation) => {
                    writeln!(
                        writer,
                        "{},{layer},{},\"{}\",{},{},{},{},{}",
                        EventType::Animation as i32,
                        Origins::from(animation.sprite.origin) as u8,
                        elem.path,
                        animation.sprite.initial_pos.x,
                        animation.sprite.initial_pos.y,
                        animation.frame_count,
                        animation.frame_delay,
                        animation.loop_kind as u8,
                    )?;

                    &animation.sprite
                }
                ElementKind::Sprite(ref sprite) => {
                    writeln!(
                        writer,
                        "{},{layer},{},\"{}\",{},{}",
                        EventType::Sprite as i32,
                        Origins::from(sprite.origin) as u8,
                        elem.path,
                        sprite.initial_pos.x,
                        sprite.initial_pos.y
                    )?;

                    sprite
                }
                ElementKind::Sample(_) | ElementKind::Video(_) => continue,
            };

            write_group(writer, 1, &sprite.timeline_group)?;

            for l in sprite.loops.iter() {
                writeln!(writer, " L,{},{}", l.loop_start_time, l.total_iterations)?;
                write_group(writer, 2, &l.group)?;
            }

            for trigger in sprite.triggers.iter() {
                write!(writer, " T,{}", trigger.name)?;

                if trigger.start_time > f64::MIN
                    || trigger.end_time < f64::MAX
                    || trigger.group_num != 0
                {
                    write!(writer, ",{}", trigger.start_time)?;
                }

                if trigger.end_time < f64::MAX || trigger.group_num != 0 {
                    write!(writer, ",{}", trigger.end_time)?;
                }

                if trigger.group_num != 0 {
                    write!(writer, ",{}", trigger.group_num)?;
                }

                writer.write_all(b"\n")?;

                write_group(writer, 2, &trigger.group)?;
            }
        }

        Ok(())
    }

    fn encode_samples<W: Write>(&self, writer: &mut W) -> IoResult<()> {
        writer.write_all(b"//Storyboard Sound Samples\n")?;

        let samples = self
            .layers
            .iter()
            .filter_map(|(name, layer)| {
                let int = match name.as_str() {
                    "Background" => 0,
                    "Fail" => 1,
                    "Pass" => 2,
                    "Foreground" => 3,
                    _ => return None,
                };

                Some((layer, int))
            })
            .flat_map(|(layer, int)| {
                layer.elements.iter().filter_map(move |elem| {
                    if let ElementKind::Sample(ref sample) = elem.kind {
                        Some((int, elem.path.as_str(), sample))
                    } else {
                        None
                    }
                })
            });

        for (layer, path, sample) in samples {
            writeln!(
                writer,
                "{},{},{layer},\"{}\",{}",
                EventType::Sample as i32,
                sample.start_time,
                path.to_standardized_path(),
                sample.volume,
            )?;
        }

        Ok(())
    }
}

fn write_group<W: Write>(
    writer: &mut W,
    indent: usize,
    group: &CommandTimelineGroup,
) -> IoResult<()> {
    for command in group.x.commands.iter() {
        write_f32_command(writer, indent, "MX", command)?;
    }

    for command in group.y.commands.iter() {
        write_f32_command(writer, indent, "MY", command)?;
    }

    for command in group.scale.commands.iter() {
        write_f32_command(writer, indent, "S", command)?;
    }

    for command in group.vector_scale.commands.iter() {
        write_command_prefix(writer, indent, "V", command)?;

        write!(
            writer,
            "{},{}",
            command.start_value.x, command.start_value.y,
        )?;

        if (command.end_value.x - command.start_value.x).abs() >= f32::EPSILON
            || (command.end_value.y - command.start_value.y).abs() >= f32::EPSILON
        {
            write!(writer, ",{},{}", command.end_value.x, command.end_value.y)?;
        }

        writer.write_all(b"\n")?;
    }

    for command in group.rotation.commands.iter() {
        write_command_prefix(writer, indent, "R", command)?;
        write!(writer, "{}", command.start_value.to_radians())?;

        if (command.end_value - command.start_value).abs() >= f32::EPSILON {
            write!(writer, ",{}", command.end_value.to_radians())?;
        }

        writer.write_all(b"\n")?;
    }

    for command in group.color.commands.iter() {
        write_command_prefix(writer, indent, "C", command)?;

        write!(
            writer,
            "{},{},{}",
            command.start_value.red(),
            command.start_value.green(),
            command.start_value.blue(),
        )?;

        if command.start_value.0[..3] != command.end_value.0[..3] {
            write!(
                writer,
                ",{},{},{}",
                command.end_value.red(),
                command.end_value.green(),
                command.end_value.blue(),
            )?;
        }

        writer.write_all(b"\n")?;
    }

    for command in group.alpha.commands.iter() {
        write_f32_command(writer, indent, "F", command)?;
    }

    for command in group.blending_parameters.commands.iter() {
        write_command_prefix(writer, indent, "P", command)?;
        writeln!(writer, "A")?;
    }

    for command in group.flip_h.commands.iter() {
        write_command_prefix(writer, indent, "P", command)?;
        writeln!(writer, "H")?;
    }

    for command in group.flip_v.commands.iter() {
        write_command_prefix(writer, indent, "P", command)?;
        writeln!(writer, "V")?;
    }

    Ok(())
}

fn write_f32_command<W>(
    writer: &mut W,
    indent: usize,
    acronym: &str,
    command: &TypedCommand<f32>,
) -> IoResult<()>
where
    W: Write,
{
    write_command_prefix(writer, indent, acronym, command)?;
    write!(writer, "{}", command.start_value)?;

    if (command.end_value - command.start_value).abs() >= f32::EPSILON {
        write!(writer, ",{}", command.end_value)?;
    }

    writer.write_all(b"\n")
}

fn write_command_prefix<W, T>(
    writer: &mut W,
    indent: usize,
    acronym: &str,
    command: &TypedCommand<T>,
) -> IoResult<()>
where
    W: Write,
{
    struct WriteEndTime<'a, T> {
        command: &'a TypedCommand<T>,
    }

    impl<'a, T> WriteEndTime<'a, T> {
        const fn new(command: &'a TypedCommand<T>) -> Self {
            Self { command }
        }
    }

    impl<T> Display for WriteEndTime<'_, T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            fn inner(start_time: f64, end_time: f64, f: &mut Formatter<'_>) -> FmtResult {
                if (end_time - start_time).abs() < f64::EPSILON {
                    Ok(())
                } else {
                    write!(f, "{end_time}")
                }
            }

            inner(self.command.start_time, self.command.end_time, f)
        }
    }

    write!(
        writer,
        "{:>indent$}{acronym},{},{},{},",
        " ",
        command.easing as u8,
        command.start_time,
        WriteEndTime::new(command),
    )
}

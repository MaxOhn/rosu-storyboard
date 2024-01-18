use std::{
    collections::{hash_map::Values, HashMap},
    io,
    path::Path,
    str::FromStr,
};

use rosu_map::{
    section::events::{BreakPeriod, Events},
    LATEST_FORMAT_VERSION,
};

use crate::{
    element::{StoryboardElement, StoryboardElementKind},
    layer::StoryboardLayer,
};

/// The storyboard of a beatmap.
#[derive(Clone, Debug, PartialEq)]
pub struct Storyboard {
    pub format_version: i32,
    pub use_skin_sprites: bool,
    pub background_file: String,
    pub breaks: Vec<BreakPeriod>,
    layers: HashMap<String, StoryboardLayer>,
    min_layer_depth: i32,
}

impl Storyboard {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, io::Error> {
        rosu_map::from_bytes(bytes)
    }

    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, io::Error> {
        rosu_map::from_path(path)
    }

    /// Return the [`StoryboardLayer`] with the given name.
    ///
    /// If no layer has that name, a new one is created.
    // false positive
    #[allow(clippy::missing_panics_doc)]
    pub fn get_layer(&mut self, name: &str) -> &mut StoryboardLayer {
        // Workaround for NLL
        // See <https://github.com/rust-lang/rust/issues/43234>
        if self.layers.contains_key(name) {
            return self.layers.get_mut(name).unwrap();
        }

        self.min_layer_depth -= 1;
        let layer = StoryboardLayer::new(name.to_owned(), self.min_layer_depth, true);

        self.layers.entry(name.to_owned()).or_insert(layer)
    }

    /// Return the [`StoryboardLayer`] with the given name.
    ///
    /// If no layer has that name, return `None`.
    pub fn try_get_layer(&self, name: &str) -> Option<&StoryboardLayer> {
        self.layers.get(name)
    }

    /// Return an [`Iterator`] over all current [`StoryboardLayer`]s.
    pub fn layers(&self) -> Values<'_, String, StoryboardLayer> {
        self.layers.values()
    }

    pub fn earliest_event_time(&self) -> Option<f64> {
        self.layers()
            .flat_map(|layer| layer.elements.iter())
            .filter_map(|elem| match elem.kind {
                StoryboardElementKind::Animation(ref elem) => Some(elem.start_time()),
                StoryboardElementKind::Sample(ref elem) => Some(elem.start_time),
                StoryboardElementKind::Sprite(ref elem) => Some(elem.start_time()),
                StoryboardElementKind::Video(_) => None,
            })
            .min_by(f64::total_cmp)
    }

    pub fn latest_event_time(&self) -> Option<f64> {
        self.layers()
            .flat_map(|layer| layer.elements.iter())
            .filter_map(|elem| match elem.kind {
                StoryboardElementKind::Animation(ref elem) => Some(elem.end_time()),
                StoryboardElementKind::Sample(ref elem) => Some(elem.start_time),
                StoryboardElementKind::Sprite(ref elem) => Some(elem.end_time()),
                StoryboardElementKind::Video(_) => None,
            })
            .max_by(f64::total_cmp)
    }

    pub fn has_drawable(&self) -> bool {
        self.layers()
            .flat_map(|layer| layer.elements.iter())
            .any(StoryboardElement::is_drawable)
    }
}

impl Default for Storyboard {
    fn default() -> Self {
        let mut layers = HashMap::new();

        layers.insert(
            "Video".to_owned(),
            StoryboardLayer::new("Video".to_owned(), 4, false),
        );

        layers.insert(
            "Background".to_owned(),
            StoryboardLayer::new("Background".to_owned(), 3, true),
        );

        layers.insert(
            "Fail".to_owned(),
            StoryboardLayer {
                visible_when_passing: false,
                ..StoryboardLayer::new("Fail".to_owned(), 2, true)
            },
        );

        layers.insert(
            "Pass".to_owned(),
            StoryboardLayer {
                visible_when_failing: false,
                ..StoryboardLayer::new("Pass".to_owned(), 1, true)
            },
        );

        layers.insert(
            "Foreground".to_owned(),
            StoryboardLayer::new("Foreground".to_owned(), 0, true),
        );

        layers.insert(
            "Overlay".to_owned(),
            StoryboardLayer::new("Overlay".to_owned(), i32::MIN, true),
        );

        let Events {
            background_file,
            breaks,
        } = Events::default();

        Self {
            format_version: LATEST_FORMAT_VERSION,
            background_file,
            breaks,
            use_skin_sprites: Default::default(),
            min_layer_depth: 0,
            layers,
        }
    }
}

impl FromStr for Storyboard {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        rosu_map::from_str(s)
    }
}

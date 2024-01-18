use std::{collections::HashMap, io, path::Path, str::FromStr};

use rosu_map::{
    section::events::{BreakPeriod, Events},
    LATEST_FORMAT_VERSION,
};

use crate::{
    element::{Element, ElementKind},
    layer::{Layer, LayerInternal},
};

/// The storyboard of a beatmap.
#[derive(Clone, Debug, PartialEq)]
pub struct Storyboard {
    pub format_version: i32,
    pub use_skin_sprites: bool,
    pub background_file: String,
    pub breaks: Vec<BreakPeriod>,
    pub layers: HashMap<String, Layer>,
    pub(crate) min_layer_depth: i32,
}

impl Storyboard {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, io::Error> {
        rosu_map::from_bytes(bytes)
    }

    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, io::Error> {
        rosu_map::from_path(path)
    }

    /// Return the [`Layer`] with the given name.
    ///
    /// If no layer has that name, a new one is created.
    // false positive
    #[allow(clippy::missing_panics_doc)]
    pub fn get_layer(&mut self, name: &str) -> &mut Layer {
        // Workaround for NLL
        // See <https://github.com/rust-lang/rust/issues/43234>
        if self.layers.contains_key(name) {
            return self.layers.get_mut(name).unwrap();
        }

        self.min_layer_depth -= 1;
        let layer = Layer::new(self.min_layer_depth, true);

        self.layers.entry(name.to_owned()).or_insert(layer)
    }

    /// Return the [`Layer`] with the given name.
    ///
    /// If no layer has that name, return `None`.
    pub fn try_get_layer(&self, name: &str) -> Option<&Layer> {
        self.layers.get(name)
    }

    pub fn earliest_event_time(&self) -> Option<f64> {
        self.layers
            .values()
            .flat_map(|layer| layer.elements.iter())
            .filter_map(|elem| match elem.kind {
                ElementKind::Animation(ref elem) => Some(elem.start_time()),
                ElementKind::Sample(ref elem) => Some(elem.start_time),
                ElementKind::Sprite(ref elem) => Some(elem.start_time()),
                ElementKind::Video(_) => None,
            })
            .min_by(f64::total_cmp)
    }

    pub fn latest_event_time(&self) -> Option<f64> {
        self.layers
            .values()
            .flat_map(|layer| layer.elements.iter())
            .filter_map(|elem| match elem.kind {
                ElementKind::Animation(ref elem) => Some(elem.end_time()),
                ElementKind::Sample(ref elem) => Some(elem.start_time),
                ElementKind::Sprite(ref elem) => Some(elem.end_time()),
                ElementKind::Video(_) => None,
            })
            .max_by(f64::total_cmp)
    }

    pub fn has_drawable(&self) -> bool {
        self.layers
            .values()
            .flat_map(|layer| layer.elements.iter())
            .any(Element::is_drawable)
    }
}

impl Default for Storyboard {
    fn default() -> Self {
        let mut layers = HashMap::new();

        layers.insert("Video".to_owned(), Layer::new(4, false));
        layers.insert("Background".to_owned(), Layer::new(3, true));
        layers.insert(
            "Fail".to_owned(),
            Layer {
                visible_when_passing: false,
                ..Layer::new(2, true)
            },
        );
        layers.insert(
            "Pass".to_owned(),
            Layer {
                visible_when_failing: false,
                ..Layer::new(1, true)
            },
        );
        layers.insert("Foreground".to_owned(), Layer::new(0, true));
        layers.insert("Overlay".to_owned(), Layer::new(i32::MIN, true));

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

pub(crate) struct StoryboardInternal {
    pub layers: HashMap<String, LayerInternal>,
    pub min_layer_depth: i32,
}

impl StoryboardInternal {
    pub fn get_layer(&mut self, name: &str) -> &mut LayerInternal {
        // Workaround for NLL
        // See <https://github.com/rust-lang/rust/issues/43234>
        if self.layers.contains_key(name) {
            return self.layers.get_mut(name).unwrap();
        }

        self.min_layer_depth -= 1;
        let layer = LayerInternal::new(self.min_layer_depth, true);

        self.layers.entry(name.to_owned()).or_insert(layer)
    }

    pub fn convert_layers(self) -> HashMap<String, Layer> {
        self.layers
            .into_iter()
            .map(|(name, layer)| (name, layer.into()))
            .collect()
    }
}

impl Default for StoryboardInternal {
    fn default() -> Self {
        let storyboard = Storyboard::default();

        let layers = storyboard
            .layers
            .into_iter()
            .map(|(name, layer)| {
                debug_assert!(layer.elements.is_empty());

                let layer = LayerInternal {
                    depth: layer.depth,
                    masking: layer.masking,
                    visible_when_passing: layer.visible_when_passing,
                    visible_when_failing: layer.visible_when_failing,
                    elements: Vec::new(),
                };

                (name, layer)
            })
            .collect();

        Self {
            layers,
            min_layer_depth: storyboard.min_layer_depth,
        }
    }
}

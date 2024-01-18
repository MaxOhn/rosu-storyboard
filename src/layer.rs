use crate::element::{StoryboardElement, StoryboardElementInternal};

/// A layer of a [`Storyboard`].
///
/// [`Storyboard`]: crate::Storyboard
#[derive(Clone, Debug, PartialEq)]
pub struct StoryboardLayer {
    pub depth: i32,
    pub masking: bool,
    pub visible_when_passing: bool,
    pub visible_when_failing: bool,
    pub elements: Vec<StoryboardElement>,
}

impl StoryboardLayer {
    /// Create a new [`StoryboardLayer`].
    pub fn new(depth: i32, masking: bool) -> Self {
        Self {
            depth,
            masking,
            ..Default::default()
        }
    }

    /// Add a [`StoryboardElement`] to the layer.
    pub fn add(&mut self, element: StoryboardElement) {
        self.elements.push(element);
    }
}

impl Default for StoryboardLayer {
    fn default() -> Self {
        Self {
            depth: Default::default(),
            masking: Default::default(),
            visible_when_passing: true,
            visible_when_failing: true,
            elements: Vec::new(),
        }
    }
}

pub(crate) struct StoryLayer<'a>(&'a str);

impl<'a> StoryLayer<'a> {
    pub(crate) fn parse(s: &'a str) -> Self {
        match s.parse::<u8>() {
            Ok(0) => Self("Background"),
            Ok(1) => Self("Fail"),
            Ok(2) => Self("Pass"),
            Ok(3) => Self("Foreground"),
            Ok(4) => Self("Overlay"),
            Ok(5) => Self("Video"),
            // lazer throws an error if the string doesn't match the variant
            // name but we'll accept it as is
            _ => Self(s),
        }
    }

    pub(crate) const fn as_str(&self) -> &str {
        self.0
    }
}

pub(crate) struct StoryboardLayerInternal {
    pub depth: i32,
    pub masking: bool,
    pub visible_when_passing: bool,
    pub visible_when_failing: bool,
    pub elements: Vec<StoryboardElementInternal>,
}

impl From<StoryboardLayerInternal> for StoryboardLayer {
    fn from(layer: StoryboardLayerInternal) -> Self {
        Self {
            depth: layer.depth,
            masking: layer.masking,
            visible_when_passing: layer.visible_when_passing,
            visible_when_failing: layer.visible_when_failing,
            elements: layer
                .elements
                .into_iter()
                .map(StoryboardElement::from)
                .collect(),
        }
    }
}

impl StoryboardLayerInternal {
    pub fn new(depth: i32, masking: bool) -> Self {
        Self {
            depth,
            masking,
            ..Default::default()
        }
    }
}

impl Default for StoryboardLayerInternal {
    fn default() -> Self {
        let layer = StoryboardLayer::default();

        Self {
            depth: layer.depth,
            masking: layer.masking,
            visible_when_passing: layer.visible_when_passing,
            visible_when_failing: layer.visible_when_failing,
            elements: Vec::new(),
        }
    }
}

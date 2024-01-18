use crate::element::{Element, ElementInternal};

/// A layer of a [`Storyboard`].
///
/// [`Storyboard`]: crate::Storyboard
#[derive(Clone, Debug, PartialEq)]
pub struct Layer {
    pub depth: i32,
    pub masking: bool,
    pub visible_when_passing: bool,
    pub visible_when_failing: bool,
    pub elements: Vec<Element>,
}

impl Layer {
    /// Create a new [`Layer`].
    pub fn new(depth: i32, masking: bool) -> Self {
        Self {
            depth,
            masking,
            ..Default::default()
        }
    }

    /// Add an [`Element`] to the layer.
    pub fn add(&mut self, element: Element) {
        self.elements.push(element);
    }
}

impl Default for Layer {
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

pub(crate) struct LayerInternal {
    pub depth: i32,
    pub masking: bool,
    pub visible_when_passing: bool,
    pub visible_when_failing: bool,
    pub elements: Vec<ElementInternal>,
}

impl From<LayerInternal> for Layer {
    fn from(layer: LayerInternal) -> Self {
        Self {
            depth: layer.depth,
            masking: layer.masking,
            visible_when_passing: layer.visible_when_passing,
            visible_when_failing: layer.visible_when_failing,
            elements: layer.elements.into_iter().map(Element::from).collect(),
        }
    }
}

impl LayerInternal {
    pub fn new(depth: i32, masking: bool) -> Self {
        Self {
            depth,
            masking,
            ..Default::default()
        }
    }
}

impl Default for LayerInternal {
    fn default() -> Self {
        let layer = Layer::default();

        Self {
            depth: layer.depth,
            masking: layer.masking,
            visible_when_passing: layer.visible_when_passing,
            visible_when_failing: layer.visible_when_failing,
            elements: Vec::new(),
        }
    }
}

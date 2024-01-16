use crate::element::StoryboardElement;

#[derive(Clone, Debug, PartialEq)]
pub struct StoryboardLayer {
    pub name: String,
    pub depth: i32,
    pub masking: bool,
    pub visible_when_passing: bool,
    pub visible_when_failing: bool,
    pub elements: Vec<StoryboardElement>,
}

impl StoryboardLayer {
    pub fn new(name: String, depth: i32, masking: bool) -> Self {
        Self {
            name,
            depth,
            masking,
            ..Default::default()
        }
    }

    pub fn add(&mut self, element: StoryboardElement) {
        self.elements.push(element);
    }
}

impl Default for StoryboardLayer {
    fn default() -> Self {
        Self {
            name: Default::default(),
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

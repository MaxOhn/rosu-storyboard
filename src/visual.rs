/// Legacy [`Anchor`] type.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Origins {
    TopLeft,
    Centre,
    CentreLeft,
    TopRight,
    BottomCentre,
    TopCentre,
    Custom,
    CentreRight,
    BottomLeft,
    BottomRight,
}

impl Origins {
    pub fn parse(s: &str) -> Anchor {
        let origins = match s.parse::<u8>() {
            Ok(0) => Origins::TopLeft,
            Ok(1) => Origins::Centre,
            Ok(2) => Origins::CentreLeft,
            Ok(3) => Origins::TopRight,
            Ok(4) => Origins::BottomCentre,
            Ok(5) => Origins::TopCentre,
            Ok(6) => Origins::Custom,
            Ok(7) => Origins::CentreRight,
            Ok(8) => Origins::BottomLeft,
            Ok(9) => Origins::BottomRight,
            _ => match s {
                "TopLeft" => Origins::TopLeft,
                "Centre" => Origins::Centre,
                "CentreLeft" => Origins::CentreLeft,
                "TopRight" => Origins::TopRight,
                "BottomCentre" => Origins::BottomCentre,
                "TopCentre" => Origins::TopCentre,
                "Custom" => Origins::Custom,
                "CentreRight" => Origins::CentreRight,
                "BottomLeft" => Origins::BottomLeft,
                "BottomRight" => Origins::BottomRight,
                _ => Origins::Custom,
            },
        };

        match origins {
            Origins::TopLeft => Anchor::TOP_LEFT,
            Origins::Centre => Anchor::CENTER,
            Origins::CentreLeft => Anchor::CENTER_LEFT,
            Origins::TopRight => Anchor::TOP_RIGHT,
            Origins::BottomCentre => Anchor::BOTTOM_CENTER,
            Origins::TopCentre => Anchor::TOP_CENTER,
            Origins::CentreRight => Anchor::CENTER_RIGHT,
            Origins::BottomLeft => Anchor::BOTTOM_LEFT,
            Origins::BottomRight => Anchor::BOTTOM_RIGHT,
            Origins::Custom => Anchor::TOP_LEFT,
        }
    }
}

impl From<Anchor> for Origins {
    fn from(anchor: Anchor) -> Self {
        match anchor {
            Anchor::TOP_LEFT => Self::TopLeft,
            Anchor::TOP_CENTER => Self::TopCentre,
            Anchor::TOP_RIGHT => Self::TopRight,
            Anchor::CENTER_LEFT => Self::CentreLeft,
            Anchor::CENTER => Self::Centre,
            Anchor::CENTER_RIGHT => Self::CentreRight,
            Anchor::BOTTOM_LEFT => Self::BottomLeft,
            Anchor::BOTTOM_CENTER => Self::BottomCentre,
            Anchor::BOTTOM_RIGHT => Self::BottomRight,
            _ => Self::Custom,
        }
    }
}

/// General type to specify an "anchor" or "origin" point from the standard
/// nine points on a rectangle.
///
/// x and y counterparts can be accessed using bitwise flags.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Anchor(pub u8);

impl Anchor {
    pub const Y0: u8 = 1 << 0;
    pub const Y1: u8 = 1 << 1;
    pub const Y2: u8 = 1 << 2;
    pub const X0: u8 = 1 << 3;
    pub const X1: u8 = 1 << 4;
    pub const X2: u8 = 1 << 5;
    pub const CUSTOM: u8 = 1 << 6;

    pub const TOP_LEFT: Self = Self(Self::Y0 | Self::X0);
    pub const TOP_CENTER: Self = Self(Self::Y0 | Self::X1);
    pub const TOP_RIGHT: Self = Self(Self::Y0 | Self::X2);

    pub const CENTER_LEFT: Self = Self(Self::Y1 | Self::X0);
    pub const CENTER: Self = Self(Self::Y1 | Self::X1);
    pub const CENTER_RIGHT: Self = Self(Self::Y1 | Self::X2);

    pub const BOTTOM_LEFT: Self = Self(Self::Y2 | Self::X0);
    pub const BOTTOM_CENTER: Self = Self(Self::Y2 | Self::X1);
    pub const BOTTOM_RIGHT: Self = Self(Self::Y2 | Self::X2);
}

/// Contains information about how a drawable element should be blended into
/// its destination.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct BlendingParameters {
    /// The blending factor for the source color of the blend.
    pub src: Blending,
    /// The blending factor for the destination color of the blend.
    pub dst: Blending,
    /// The blending factor for the source alpha of the blend.
    pub src_alpha: Blending,
    /// The blending factor for the destination alpha of the blend.
    pub dst_alpha: Blending,
    /// The blending equation for the RGB components of the blend.
    pub rgb_equation: BlendingEquation,
    /// The blending equation for the alpha component of the blend.
    pub alpha_equation: BlendingEquation,
}

impl BlendingParameters {
    pub const NONE: Self = Self {
        src: Blending::One,
        dst: Blending::Zero,
        src_alpha: Blending::One,
        dst_alpha: Blending::Zero,
        rgb_equation: BlendingEquation::Add,
        alpha_equation: BlendingEquation::Add,
    };

    pub const INHERIT: Self = Self {
        src: Blending::Inherit,
        dst: Blending::Inherit,
        src_alpha: Blending::Inherit,
        dst_alpha: Blending::Inherit,
        rgb_equation: BlendingEquation::Inherit,
        alpha_equation: BlendingEquation::Inherit,
    };

    pub const MIXTURE: Self = Self {
        src: Blending::SrcAlpha,
        dst: Blending::OneMinusSrcAlpha,
        src_alpha: Blending::One,
        dst_alpha: Blending::One,
        rgb_equation: BlendingEquation::Add,
        alpha_equation: BlendingEquation::Add,
    };

    pub const ADDITIVE: Self = Self {
        src: Blending::SrcAlpha,
        dst: Blending::One,
        src_alpha: Blending::One,
        dst_alpha: Blending::One,
        rgb_equation: BlendingEquation::Add,
        alpha_equation: BlendingEquation::Add,
    };
}

/// A blending type for [`BlendingParameters`].
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum Blending {
    #[default]
    Inherit,
    ConstantAlpha,
    ConstantColor,
    DstAlpha,
    DstColor,
    One,
    OneMinusConstantAlpha,
    OneMinusConstantColor,
    OneMinusDstAlpha,
    OneMinusDstColor,
    OneMinusSrcAlpha,
    OneMinusSrcColor,
    SrcAlpha,
    SrcAlphaSaturate,
    SrcColor,
    Zero,
}

/// A blending equation for [`BlendingParameters`].
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum BlendingEquation {
    /// Inherits from parent.
    #[default]
    Inherit,
    /// Adds the source and destination colours.
    Add,
    /// Chooses the minimum of each component of the source and destination colours.
    Min,
    /// Chooses the maximum of each component of the source and destination colours.
    Max,
    /// Subtracts the destination colour from the source colour.
    Subtract,
    /// Subtracts the source colour from the destination colour.
    ReverseSubtract,
}

macro_rules! define_easing {
    ( $( $variant:ident = $discriminant:literal, )* ) => {
        /// See <http://easings.net/> for more samples.
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        pub enum Easing {
            $( $variant, )*
        }

        impl From<i32> for Easing {
            fn from(value: i32) -> Self {
                match value {
                    $( $discriminant => Easing::$variant, )*
                    _ => Easing::None,
                }
            }
        }
    }
}

define_easing! {
    None = 0,
    Out = 1,
    In = 2,
    InQuad = 3,
    OutQuad = 4,
    InOutQuad = 5,
    InCubic = 6,
    OutCubic = 7,
    InOutCubic = 8,
    InQuart = 9,
    OutQuart = 10,
    InOutQuart = 11,
    InQuint = 12,
    OutQuint = 13,
    InOutQuint = 14,
    InSine = 15,
    OutSine = 16,
    InOutSine = 17,
    InExpo = 18,
    OutExpo = 19,
    InOutExpo = 20,
    InCirc = 21,
    OutCirc = 22,
    InOutCirc = 23,
    InElastic = 24,
    OutElastic = 25,
    OutElasticHalf = 26,
    OutElasticQuarter = 27,
    InOutElastic = 28,
    InBack = 29,
    OutBack = 30,
    InOutBack = 31,
    InBounce = 32,
    OutBounce = 33,
    InOutBounce = 34,
    OutPow10 = 35,
}

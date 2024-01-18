[![crates.io](https://img.shields.io/crates/v/rosu-storyboard.svg)](https://crates.io/crates/rosu-storyboard) [![docs](https://docs.rs/rosu-storyboard/badge.svg)](https://docs.rs/rosu-storyboard)

# rosu-storyboard

<!-- cargo-rdme start -->

Library to de- and encode [osu!] storyboards.

## Usage

Based on `rosu-map`'s [`DecodeBeatmap`] trait, the [`Storyboard`] struct provides a way
to decode `.osu` or `.osb` files.

```rust
use rosu_storyboard::Storyboard;
use rosu_storyboard::element::ElementKind;

let path = "./resources/Himeringo - Yotsuya-san ni Yoroshiku (RLC) [Winber1's Extreme].osu";
let storyboard = Storyboard::from_path(path).unwrap();

let first_bg_elem = &storyboard.layers["Background"].elements[0];
assert!(matches!(first_bg_elem.kind, ElementKind::Sprite(_)));
```

[osu!]: https://osu.ppy.sh/
[`DecodeBeatmap`]: rosu_map::DecodeBeatmap
[`Storyboard`]: https://docs.rs/rosu-storyboard/latest/rosu_storyboard/storyboard/struct.Storyboard.html

<!-- cargo-rdme end -->

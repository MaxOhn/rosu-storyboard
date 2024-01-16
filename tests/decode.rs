use rosu_map::util::Pos;
use rosu_storyboard::{
    element::{AnimationLoopType, StoryboardElementKind, StoryboardVideo},
    visual::Anchor,
    Storyboard,
};
use test_log::test;

#[test]
fn decode_events() {
    let storyboard: Storyboard = rosu_map::from_path(
        "./resources/Himeringo - Yotsuya-san ni Yoroshiku (RLC) [Winber1's Extreme].osu",
    )
    .unwrap();

    assert!(storyboard.has_drawable());
    assert_eq!(storyboard.layers().count(), 6);

    let background = storyboard.layers().find(|layer| layer.depth == 3).unwrap();
    assert_eq!(background.elements.len(), 16);
    assert!(background.visible_when_failing);
    assert!(background.visible_when_passing);
    assert_eq!(background.name, "Background");

    let fail = storyboard.layers().find(|layer| layer.depth == 2).unwrap();
    assert!(fail.elements.is_empty());
    assert!(fail.visible_when_failing);
    assert!(!fail.visible_when_passing);
    assert_eq!(fail.name, "Fail");

    let pass = storyboard.layers().find(|layer| layer.depth == 1).unwrap();
    assert!(pass.elements.is_empty());
    assert!(!pass.visible_when_failing);
    assert!(pass.visible_when_passing);
    assert_eq!(pass.name, "Pass");

    let foreground = storyboard.layers().find(|layer| layer.depth == 0).unwrap();
    assert_eq!(foreground.elements.len(), 151);
    assert!(foreground.visible_when_failing);
    assert!(foreground.visible_when_passing);
    assert_eq!(foreground.name, "Foreground");

    let overlay = storyboard
        .layers()
        .find(|layer| layer.depth == i32::MIN)
        .unwrap();
    assert!(overlay.elements.is_empty());
    assert!(overlay.visible_when_failing);
    assert!(overlay.visible_when_passing);
    assert_eq!(overlay.name, "Overlay");

    let sprite_count = background
        .elements
        .iter()
        .filter(|elem| matches!(elem.kind, StoryboardElementKind::Sprite(_)))
        .count();

    let animation_count = background
        .elements
        .iter()
        .filter(|elem| matches!(elem.kind, StoryboardElementKind::Animation(_)))
        .count();

    let sample_count = background
        .elements
        .iter()
        .filter(|elem| matches!(elem.kind, StoryboardElementKind::Sample(_)))
        .count();

    assert_eq!(sprite_count, 15);
    assert_eq!(animation_count, 1);
    assert_eq!(sample_count, 0);
    assert_eq!(
        background.elements.len(),
        sprite_count + animation_count + sample_count
    );

    let StoryboardElementKind::Sprite(ref sprite) = background.elements[0].kind else {
        panic!("expected sprite");
    };

    let sprite = sprite.borrow();

    assert!(sprite.has_commands());
    assert_eq!(sprite.initial_pos, Pos::new(320.0, 240.0));
    assert!(sprite.is_drawable());
    assert_eq!(sprite.origin, Anchor::CENTER);
    assert_eq!(background.elements[0].path, "SB/lyric/ja-21.png");

    let elem = background
        .elements
        .iter()
        .find(|elem| matches!(elem.kind, StoryboardElementKind::Animation(_)))
        .unwrap();

    let StoryboardElementKind::Animation(ref animation) = elem.kind else {
        unreachable!()
    };

    assert_eq_f64(animation.end_time(), 141175.0);
    assert_eq!(animation.frame_count, 10);
    assert_eq_f64(animation.frame_delay, 30.0);
    assert!(animation.has_commands());
    assert_eq!(
        animation.sprite.borrow().initial_pos,
        Pos::new(320.0, 240.0)
    );
    assert!(animation.is_drawable());
    assert_eq!(animation.loop_kind, AnimationLoopType::LoopForever);
    assert_eq!(animation.sprite.borrow().origin, Anchor::CENTER);
    assert_eq!(elem.path, "SB/red jitter/red_0000.jpg");
    assert_eq_f64(animation.start_time(), 78993.0);
}

#[test]
fn loop_without_explicit_fadeout() {
    let storyboard: Storyboard =
        rosu_map::from_path("./resources/animation-loop-no-explicit-end-time.osb").unwrap();

    let background = storyboard.layers().find(|layer| layer.depth == 3).unwrap();

    assert_eq!(background.elements.len(), 1);

    assert_eq_f64(background.elements[0].start_time(), 2000.0);

    let StoryboardElementKind::Animation(ref animation) = background.elements[0].kind else {
        panic!("expected animation");
    };

    assert_eq_f64(animation.earliest_transform_time(), 2000.0);

    assert_eq_f64(animation.end_time(), 3000.0);
    assert_eq_f64(animation.end_time_for_display(), 12_000.0);
}

#[test]
fn correct_animation_start_time() {
    let storyboard: Storyboard =
        rosu_map::from_path("./resources/animation-starts-before-alpha.osb").unwrap();

    let background = storyboard.layers().find(|layer| layer.depth == 3).unwrap();

    assert_eq!(background.elements.len(), 1);

    assert_eq_f64(background.elements[0].start_time(), 2000.0);

    let StoryboardElementKind::Animation(ref animation) = background.elements[0].kind else {
        panic!("expected animation")
    };

    assert_eq_f64(animation.earliest_transform_time(), 1000.0);
}

#[test]
fn out_of_order_start_times() {
    let storyboard: Storyboard =
        rosu_map::from_path("./resources/out-of-order-starttimes.osb").unwrap();

    let background = storyboard.layers().find(|layer| layer.depth == 3).unwrap();

    assert_eq!(background.elements.len(), 2);
    assert_eq_f64(background.elements[0].start_time(), 1500.0);
    assert_eq_f64(background.elements[1].start_time(), 1000.0);

    assert_eq_f64(storyboard.earliest_event_time().unwrap(), 1000.0);
}

#[test]
fn earliest_start_time_with_loop_alphas() {
    let storyboard: Storyboard =
        rosu_map::from_path("./resources/loop-containing-earlier-non-zero-fade.osb").unwrap();

    let background = storyboard.layers().find(|layer| layer.depth == 3).unwrap();

    assert_eq!(background.elements.len(), 2);
    assert_eq_f64(background.elements[0].start_time(), 1000.0);
    assert_eq_f64(background.elements[1].start_time(), 1000.0);

    assert_eq_f64(storyboard.earliest_event_time().unwrap(), 1000.0);
}

#[test]
fn decode_variable_with_suffix() {
    let storyboard: Storyboard =
        rosu_map::from_path("./resources/variable-with-suffix.osb").unwrap();

    let background = storyboard.layers().find(|layer| layer.depth == 3).unwrap();

    let sprite = match background.elements[0].kind {
        StoryboardElementKind::Animation(ref elem) => &elem.sprite,
        StoryboardElementKind::Sprite(ref elem) => elem,
        StoryboardElementKind::Sample(_) | StoryboardElementKind::Video(_) => {
            panic!("expected sprite")
        }
    };

    assert_eq_f32(sprite.borrow().initial_pos.x, 3456.0);
}

#[test]
fn decode_video_with_lowercase_extension() {
    let storyboard: Storyboard =
        rosu_map::from_path("./resources/video-with-lowercase-extension.osb").unwrap();

    let video = storyboard
        .layers()
        .find(|layer| layer.name == "Video")
        .unwrap();

    assert_eq!(video.elements.len(), 1);

    assert!(matches!(
        video.elements[0].kind,
        StoryboardElementKind::Video(_)
    ));

    assert_eq!(video.elements[0].path, "Video.avi");
}

#[test]
fn decode_video_with_uppercase_extension() {
    let storyboard: Storyboard =
        rosu_map::from_path("./resources/video-with-uppercase-extension.osb").unwrap();

    let video = storyboard
        .layers()
        .find(|layer| layer.name == "Video")
        .unwrap();

    assert_eq!(video.elements.len(), 1);

    assert!(matches!(
        video.elements[0].kind,
        StoryboardElementKind::Video(_)
    ));

    assert_eq!(video.elements[0].path, "Video.AVI");
}

#[test]
fn decode_image_specified_as_video() {
    let storyboard: Storyboard =
        rosu_map::from_path("./resources/image-specified-as-video.osb").unwrap();

    let video = storyboard
        .layers()
        .find(|layer| layer.name == "Video")
        .unwrap();
    assert!(video.elements.is_empty());
}

#[test]
fn decode_out_of_range_loop_animation_type() {
    let storyboard: Storyboard = rosu_map::from_path("./resources/animation-types.osb").unwrap();

    let foreground = storyboard.layers().find(|layer| layer.depth == 0).unwrap();

    let mut animations = foreground.elements.iter().map(|elem| match elem.kind {
        StoryboardElementKind::Animation(ref elem) => elem,
        StoryboardElementKind::Sample(_)
        | StoryboardElementKind::Sprite(_)
        | StoryboardElementKind::Video(_) => panic!("expected animation"),
    });

    assert_eq!(
        animations.next().unwrap().loop_kind,
        AnimationLoopType::LoopForever
    );
    assert_eq!(
        animations.next().unwrap().loop_kind,
        AnimationLoopType::LoopOnce
    );
    assert_eq!(
        animations.next().unwrap().loop_kind,
        AnimationLoopType::LoopForever
    );
    assert_eq!(
        animations.next().unwrap().loop_kind,
        AnimationLoopType::LoopOnce
    );
    assert_eq!(
        animations.next().unwrap().loop_kind,
        AnimationLoopType::LoopForever
    );
    assert_eq!(
        animations.next().unwrap().loop_kind,
        AnimationLoopType::LoopForever
    );
}

#[test]
fn decode_loop_count() {
    const LOOP_DURATION: f64 = 2000.0;

    let storyboard: Storyboard = rosu_map::from_path("./resources/loop-count.osb").unwrap();

    let background = storyboard.layers().find(|layer| layer.depth == 3).unwrap();

    let zero_times = background
        .elements
        .iter()
        .filter(|elem| match elem.kind {
            StoryboardElementKind::Animation(_) | StoryboardElementKind::Sprite(_) => true,
            StoryboardElementKind::Sample(_) | StoryboardElementKind::Video(_) => false,
        })
        .find(|elem| elem.path == "zero-times.png")
        .unwrap();
    assert_eq_f64(zero_times.end_time(), 1000.0 + LOOP_DURATION);

    let one_time = background
        .elements
        .iter()
        .filter(|elem| match elem.kind {
            StoryboardElementKind::Animation(_) | StoryboardElementKind::Sprite(_) => true,
            StoryboardElementKind::Sample(_) | StoryboardElementKind::Video(_) => false,
        })
        .find(|elem| elem.path == "one-time.png")
        .unwrap();
    assert_eq_f64(one_time.end_time(), 4000.0 + LOOP_DURATION);

    let many_times = background
        .elements
        .iter()
        .filter(|elem| match elem.kind {
            StoryboardElementKind::Animation(_) | StoryboardElementKind::Sprite(_) => true,
            StoryboardElementKind::Sample(_) | StoryboardElementKind::Video(_) => false,
        })
        .find(|elem| elem.path == "many-times.png")
        .unwrap();
    assert_eq_f64(many_times.end_time(), 9000.0 + LOOP_DURATION);
}

#[test]
fn video_and_background_events_do_not_affect_storyboard_bounds() {
    let mut storyboard: Storyboard =
        rosu_map::from_path("./resources/video-background-events-ignored.osb").unwrap();

    let elements = &storyboard.get_layer("Video").elements;

    assert_eq!(elements.len(), 1);
    assert!(matches!(
        elements[0].kind,
        StoryboardElementKind::Video(StoryboardVideo { start_time }) if (-5678.0 - start_time).abs() < f64::EPSILON
    ));

    assert_eq!(storyboard.earliest_event_time(), None);
    assert_eq!(storyboard.latest_event_time(), None);
}

#[track_caller]
fn assert_eq_f64(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < f64::EPSILON,
        "actual={actual} | expected={expected}"
    );
}

#[track_caller]
fn assert_eq_f32(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() < f32::EPSILON,
        "actual={actual} | expected={expected}"
    );
}
use std::fs;

use rosu_storyboard::Storyboard;
use test_log::test;

#[test]
fn stability() {
    let mut bytes = Vec::with_capacity(4096);

    for entry in fs::read_dir("./resources").unwrap() {
        let entry = entry.unwrap();
        let filename = entry.file_name();
        let filename = filename.to_str().unwrap();

        if !(filename.ends_with(".osu") || filename.ends_with(".osb")) {
            continue;
        }

        let decoded = Storyboard::from_path(entry.path())
            .unwrap_or_else(|e| panic!("Failed to decode storyboard {filename:?}: {e:?}"));

        bytes.clear();

        decoded
            .encode(&mut bytes)
            .unwrap_or_else(|e| panic!("Failed to encode storyboard {filename:?}: {e:?}"));

        let decoded_after_encode = Storyboard::from_bytes(&bytes).unwrap_or_else(|e| {
            panic!("Failed to decode storyboard after encoding {filename:?}: {e:?}")
        });

        assert_eq!(decoded, decoded_after_encode, "{filename:?}");
    }
}

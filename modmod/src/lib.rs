pub use types::*;

pub mod error;
pub mod types;
pub use error::{Error, Result};
mod load;

fn to_numbered_tag(s: &str, i: i32) -> String {
    to_tag(format!("{i}-{s}"))
}

fn to_tag(mut s: String) -> String {
    s.make_ascii_lowercase();
    let mut tag = String::new();
    let mut words = s.split_whitespace();
    tag.push_str(&words.next().unwrap());
    for word in words {
        tag.push('-');
        tag.push_str(&word);
    }
    tag
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::Track;
    const TRACK_ENTRY: &str = "data/rust-intro.track.toml";
    const OUTPUT_DIR: &str = "output";

    #[test]
    fn deserialize_tree() {
        let track_path = Path::new(TRACK_ENTRY);
        let track = Track::load(track_path).unwrap();
        dbg!(&track);

        let modules = track.load_modules().unwrap();
        dbg!(&modules);

        for module in modules {
            let units = module.load_topics().unwrap();
            dbg!(&units);
        }
    }

    #[test]
    fn render_track() {
        Track::render(Path::new(TRACK_ENTRY), Path::new(OUTPUT_DIR), true).unwrap();
    }
}

use crate::gamemodes::Gamemode;

pub fn parse_gamemode_file(file_path: &str) -> Vec<Gamemode> {
    serde_json::from_reader(
        std::fs::File::open(file_path)
                .expect(&format!("unable to open {}", file_path))
    ).expect(&format!("unable to parse {}", file_path))
}
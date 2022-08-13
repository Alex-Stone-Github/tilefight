use crate::tile::Tile;

pub struct World {
    tiles: Vec<Tile>
}

pub struct WorldManager {
    worlds: Vec<World>,
}

impl WorldManager {
    pub fn new() {
        let level_file_content = std::fs::read_to_string("./res/levels")
            .expect("Could not read levels file");
        let parsed = serde_json::from_str(level_file_content.as_str());
        if parsed.is_ok() {
            let json: serde_json::Value = parsed.unwrap();
        }
    }
}

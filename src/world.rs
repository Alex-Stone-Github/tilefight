use crate::tile::Tile;
use crate::vec::Vec2;
use crate::player::Player;
use crate::settings::TILE_SIZE;

#[derive(Clone)]
pub struct World {
    pub tiles: Vec<Tile>,
    pub player: Player
}
pub struct WorldManager {
    current: isize,
    worlds: Vec<World>
}
impl WorldManager {
    pub fn new() -> Self {
        // create worlds
        let mut worlds: Vec<World> = Vec::new();

        // read in level data
        let level_file_content = std::fs::read_to_string("./res/levels")
            .expect("Could not read levels file");
        let worlds_tile_placement_info = level_file_content.split("|");

        // for each level section turn into a format the game can understand
        for world_tile_placement_info in worlds_tile_placement_info {
            let mut world_tiles = Vec::new();
            let mut player = Player::new(Vec2{x:0.0,y:0.0}, Vec2{x:20.0,y:20.0});
            let mut position = Vec2{x:0.0, y:0.0};
            for tile_char in world_tile_placement_info.chars() {
                match tile_char {
                    '#' => {
                        world_tiles.push(Tile {
                            position,
                            size: Vec2{x:TILE_SIZE, y:TILE_SIZE},
                            integrity: 100});
                        position.x += TILE_SIZE;
                    }
                    ' ' => {position.x += TILE_SIZE;}
                    'p' => {
                        player.position = position;
                        position.x += TILE_SIZE;
                    }
                    '/' => {
                        position.y += TILE_SIZE;
                        position.x = 0.0;
                    }
                    _ => ()
                }
            }
            let world = World{
                tiles: world_tiles,
                player
            };
            worlds.push(world);
        }
        Self {
            current: -1,
            worlds,
        }
    }
}

impl Iterator for WorldManager {
    type Item = World;
    fn next(&mut self) -> Option<Self::Item> {
        // TODO: check for the end of a game
        self.current += 1;
        if self.current < self.worlds.len() as isize {
            return Some(self.worlds[self.current as usize].clone())
        }
        None
    }
}







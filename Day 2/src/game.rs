pub struct Game {
    id: u8,
    game_sets: Vec<[u32; 3]>,
}

impl Game {

    pub fn new(id: u8, game_sets:  Vec<[u32; 3]>) -> Self{
        /*let parsed_data = game_parser::parse(game_string);
        Self { id: parsed_data.0, game_sets: parsed_data.1 }*/

        Self {id, game_sets}
    }

    pub fn id(&self) -> &u8{
        &self.id
    }

    pub fn sets(&self) -> &Vec<[u32; 3]> {
        &self.game_sets
    }
}
mod game;
mod game_state;

use game::Game;

const DEFAULT_DIM: usize = 4;
const DEFAULT_UNDOS: usize = 3;

fn main() {
    let mut game = Game::<DEFAULT_DIM>::new(DEFAULT_UNDOS);
    game.left();
    game.right();
    game.up();
    game.down();
    game.undo();
}

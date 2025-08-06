use rand::{Rng, rng, seq::IndexedRandom};

const PROB_2: f64 = 0.9;
const TILE_2: u32 = 2;
const TILE_4: u32 = 4;

#[derive(Clone)]
pub struct GameState<const DIM: usize> {
    tiles: [[u32; DIM]; DIM],
    score: u32,
}

impl<const DIM: usize> GameState<DIM> {
    pub fn new() -> Self {
        let mut game_state = Self {
            tiles: [[0; DIM]; DIM],
            score: 0,
        };
        game_state.add_random();
        game_state.add_random();
        game_state
    }

    fn get(&self, i: usize, j: usize) -> u32 {
        self.tiles[i][j]
    }

    fn set(&mut self, i: usize, j: usize, value: u32) {
        self.tiles[i][j] = value;
    }

    fn add_random(&mut self) -> bool {
        let avail_idxs: Vec<(usize, usize)> = (0..DIM)
            .flat_map(|i| (0..DIM).map(move |j| (i, j))) // enum over all idxs
            .filter(|&(i, j)| self.get(i, j) == 0)
            .collect();

        if avail_idxs.is_empty() {
            return false;
        }

        // randomly pick index of empty tiles to add either 2 or 4 to it
        let mut rng = rng();
        let Some(&(i, j)) = avail_idxs.choose(&mut rng) else {
            return false;
        };

        let value = if rng.random_bool(PROB_2) {
            TILE_2
        } else {
            TILE_4
        };

        self.set(i, j, value);
        return true;
    }

    fn slide_left(&mut self) {
        for row in 0..DIM {
            // sliding results in tiles keeping their order but 0:s moved to end
            let mut tiles: Vec<u32> = (0..DIM)
                .map(|col| self.get(row, col))
                .filter(|&tile| tile != 0)
                .collect();

            tiles.resize(DIM, 0);

            tiles
                .into_iter()
                .enumerate()
                .for_each(|(j, value)| self.set(row, j, value));
        }
    }

    fn merge_left(&mut self) -> u32 {
        let mut score: u32 = 0;

        for row in 0..DIM {
            for col in 0..(DIM - 1) {
                let current_tile = self.get(row, col);
                let next_tile = self.get(row, col + 1);
                if current_tile != 0 && current_tile == next_tile {
                    let merged_value = next_tile * 2;
                    self.set(row, col, merged_value);
                    self.set(row, col + 1, 0);
                    score += merged_value;
                }
            }
        }
        score
    }

    pub fn slide_and_merge_left(&mut self) {
        self.slide_left();
        self.score += self.merge_left();
        self.slide_left();
    }

    fn transpose(&mut self) {
        for i in 0..DIM {
            for j in (i + 1)..DIM {
                let tmp = self.get(i, j);
                self.set(i, j, self.get(j, i));
                self.set(j, i, tmp);
            }
        }
    }

    fn swap_symmetric_horizontally(&mut self, i: usize, j: usize) {
        let tmp = self.get(i, j);
        self.set(i, j, self.get(i, DIM - 1 - j));
        self.set(i, DIM - 1 - j, tmp);
    }

    fn reverse_rows(&mut self) {
        for i in 0..DIM {
            for j in 0..(DIM / 2) {
                self.swap_symmetric_horizontally(i, j);
            }
        }
    }

    fn reverse_cols(&mut self) {
        for j in 0..DIM {
            for i in 0..(DIM / 2) {
                self.swap_symmetric_horizontally(i, j);
            }
        }
    }

    pub fn rotate90(&mut self) {
        self.transpose();
        self.reverse_rows();
    }

    pub fn rotate180(&mut self) {
        self.reverse_rows();
        self.reverse_cols();
    }

    pub fn rotate270(&mut self) {
        self.reverse_rows();
        self.transpose();
    }
}

impl<const DIM: usize> PartialEq for GameState<DIM> {
    fn eq(&self, other: &Self) -> bool {
        self.tiles == other.tiles
    }
}

use std::convert::TryInto;

use itertools::Itertools;
use shared::puzzle_input;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Facing {
    Right,
    Down,
    Left,
    Up,
}
impl Facing {
    pub fn score(&self) -> usize {
        match self {
            Facing::Right => 0,
            Facing::Down => 1,
            Facing::Left => 2,
            Facing::Up => 3,
        }
    }
    pub fn turn_right(&mut self) {
        *self = match self {
            Facing::Right => Facing::Down,
            Facing::Down => Facing::Left,
            Facing::Left => Facing::Up,
            Facing::Up => Facing::Right,
        };
    }
    pub fn turn_left(&mut self) {
        *self = match self {
            Facing::Right => Facing::Up,
            Facing::Down => Facing::Right,
            Facing::Left => Facing::Down,
            Facing::Up => Facing::Left,
        };
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Walker {
    facing: Facing,
    row: u8,
    col: u8,
}
impl Walker {
    fn walk_forward(&mut self, distance: usize, map: &Map, cube: bool) {
        for _ in 0..distance {
            let (w, is_wall) = map.next(self, cube);
            if is_wall {
                break;
            }
            *self = w;
        }
    }
    pub fn walk_path(&mut self, path: &str, map: &Map, cube: bool) {
        let mut distance = 0;
        for c in path.chars() {
            match c {
                'R' => {
                    self.walk_forward(distance, map, cube);
                    distance = 0;
                    self.facing.turn_right();
                }
                'L' => {
                    self.walk_forward(distance, map, cube);
                    distance = 0;
                    self.facing.turn_left();
                }
                c if c.is_ascii_digit() => {
                    distance = distance * 10 + c.to_digit(10).unwrap() as usize;
                }
                _ => unreachable!(),
            }
        }
        self.walk_forward(distance, map, cube);
    }
    pub fn password(&self) -> u64 {
        (self.row as u64 + 1) * 1000 + (self.col as u64 + 1) * 4 + (self.facing.score() as u64)
    }
}
#[derive(Debug)]
struct MapRow {
    min: u8,
    max: u8,
    walls: Vec<u8>,
}
impl MapRow {
    pub fn horiz(s: &str) -> Self {
        let mut walls = vec![];
        let mut min = 255;
        let mut max = 0;
        for (col, c) in s.chars().enumerate() {
            let col = col.try_into().unwrap();
            if c != ' ' {
                min = min.min(col);
                max = max.max(col);
            }
            if c == '#' {
                walls.push(col);
            }
        }
        MapRow { min, max, walls }
    }
    pub fn vert<'a, I: Iterator<Item = &'a MapRow>>(col: u8, horiz: I) -> Self {
        let mut walls = vec![];
        let mut min = 255;
        let mut max = 0;
        for (row, row_data) in horiz.enumerate() {
            let row = row.try_into().unwrap();
            if (row_data.min..=row_data.max).contains(&col) {
                min = min.min(row);
                max = max.max(row);
                if row_data.walls.contains(&col) {
                    walls.push(row);
                }
            }
        }
        MapRow { min, max, walls }
    }
}
#[derive(Debug)]
struct Map {
    by_row: Vec<MapRow>,
    by_col: Vec<MapRow>,
}
impl Map {
    pub fn new(s: &str) -> Self {
        let by_row = s.split('\n').map(MapRow::horiz).collect_vec();
        let num_cols = s.find('\n').unwrap().try_into().unwrap();
        let by_col = (0..num_cols)
            .map(|col| MapRow::vert(col, by_row.iter()))
            .collect_vec();
        Map { by_row, by_col }
    }
    fn start(&self) -> Walker {
        Walker {
            facing: Facing::Right,
            row: 0,
            col: self.by_row[0].min,
        }
    }
    fn cube_next(walker: &Walker) -> Walker {
        match walker {
            //    [1][4]
            //    [2]
            // [3][6]
            // [5]
            // top of 1 -> left of 5, turn right
            Walker {
                facing: Facing::Up,
                row: 0,
                col,
            } if (50..100).contains(col) => Walker {
                facing: Facing::Right,
                row: col + 100,
                col: 0,
            },
            // top of 4 -> bottom of 5, same orientation
            Walker {
                facing: Facing::Up,
                row: 0,
                col,
            } if (100..150).contains(col) => Walker {
                facing: Facing::Up,
                row: 199,
                col: col - 100,
            },
            // right of 4 -> right of 6, upside down
            Walker {
                facing: Facing::Right,
                row,
                col: 149,
            } if (0..50).contains(row) => Walker {
                facing: Facing::Left,
                row: 149 - row,
                col: 99,
            },
            // bottom of 4 -> right of 2, turn right
            Walker {
                facing: Facing::Down,
                row: 49,
                col,
            } if (100..150).contains(col) => Walker {
                facing: Facing::Left,
                row: col - 50,
                col: 99,
            },
            // right of 2 -> bottom of 4, turn left
            Walker {
                facing: Facing::Right,
                row,
                col: 99,
            } if (50..100).contains(row) => Walker {
                facing: Facing::Up,
                row: 49,
                col: row + 50,
            },
            // right of 6 -> right of 4, turn 180
            Walker {
                facing: Facing::Right,
                row,
                col: 99,
            } if (100..150).contains(row) => Walker {
                facing: Facing::Left,
                row: 149 - row,
                col: 149,
            },
            // bottom of 6 -> right of 5, turn right
            Walker {
                facing: Facing::Down,
                row: 149,
                col,
            } if (50..100).contains(col) => Walker {
                facing: Facing::Left,
                row: col + 100,
                col: 49,
            },
            //    [1][4]
            //    [2]
            // [3][6]
            // [5]
            // right of 5 -> bottom of 6, turn left
            Walker {
                facing: Facing::Right,
                row,
                col: 49,
            } if (150..200).contains(row) => Walker {
                facing: Facing::Up,
                row: 149,
                col: row - 100,
            },
            // bottom of 5 -> top of 4, same orientation
            Walker {
                facing: Facing::Down,
                row: 199,
                col,
            } if (0..50).contains(col) => Walker {
                facing: Facing::Down,
                row: 0,
                col: col + 100,
            },
            // left of 5 -> top of 1, turn left
            Walker {
                facing: Facing::Left,
                row,
                col: 0,
            } if (150..200).contains(row) => Walker {
                facing: Facing::Down,
                row: 0,
                col: row - 100,
            },
            // left of 3 -> left of 1, turn 180
            Walker {
                facing: Facing::Left,
                row,
                col: 0,
            } if (100..150).contains(row) => Walker {
                facing: Facing::Right,
                row: 149 - row,
                col: 50,
            },
            // top of 3 -> left of 2, turn right
            Walker {
                facing: Facing::Up,
                row: 100,
                col,
            } if (0..50).contains(col) => Walker {
                facing: Facing::Right,
                row: col + 50,
                col: 50,
            },
            // left of 2 -> top of 3, turn left
            Walker {
                facing: Facing::Left,
                row,
                col: 50,
            } if (50..100).contains(row) => Walker {
                facing: Facing::Down,
                row: 100,
                col: row - 50,
            },
            // left of 1 -> left of 3, turn 180
            Walker {
                facing: Facing::Left,
                row,
                col: 50,
            } if (0..50).contains(row) => Walker {
                facing: Facing::Right,
                row: 149 - row,
                col: 0,
            },

            Walker {
                facing: Facing::Right,
                row,
                col,
            } => Walker {
                facing: Facing::Right,
                row: *row,
                col: col + 1,
            },
            Walker {
                facing: Facing::Down,
                row,
                col,
            } => Walker {
                facing: Facing::Down,
                row: row + 1,
                col: *col,
            },
            Walker {
                facing: Facing::Left,
                row,
                col,
            } => Walker {
                facing: Facing::Left,
                row: *row,
                col: col - 1,
            },
            Walker {
                facing: Facing::Up,
                row,
                col,
            } => Walker {
                facing: Facing::Up,
                row: row - 1,
                col: *col,
            },
        }
    }
    fn next(&self, walker: &Walker, cube: bool) -> (Walker, bool) {
        if cube {
            let w = Map::cube_next(walker);
            let r = w.row;
            let c = w.col;
            return (w, self.by_row[r as usize].walls.contains(&c));
        }
        match walker.facing {
            Facing::Right => {
                let maprow = &self.by_row[walker.row as usize];
                if walker.col >= maprow.max {
                    (
                        Walker {
                            col: maprow.min,
                            ..*walker
                        },
                        maprow.walls.contains(&maprow.min),
                    )
                } else {
                    (
                        Walker {
                            col: walker.col + 1,
                            ..*walker
                        },
                        maprow.walls.contains(&(walker.col + 1)),
                    )
                }
            }
            Facing::Left => {
                let maprow = &self.by_row[walker.row as usize];
                if walker.col <= maprow.min {
                    (
                        Walker {
                            col: maprow.max,
                            ..*walker
                        },
                        maprow.walls.contains(&maprow.max),
                    )
                } else {
                    (
                        Walker {
                            col: walker.col - 1,
                            ..*walker
                        },
                        maprow.walls.contains(&(walker.col - 1)),
                    )
                }
            }
            Facing::Down => {
                let mapcol = &self.by_col[walker.col as usize];
                if walker.row >= mapcol.max {
                    (
                        Walker {
                            row: mapcol.min,
                            ..*walker
                        },
                        mapcol.walls.contains(&mapcol.min),
                    )
                } else {
                    (
                        Walker {
                            row: walker.row + 1,
                            ..*walker
                        },
                        mapcol.walls.contains(&(walker.row + 1)),
                    )
                }
            }
            Facing::Up => {
                let mapcol = &self.by_col[walker.col as usize];
                if walker.row <= mapcol.min {
                    (
                        Walker {
                            row: mapcol.max,
                            ..*walker
                        },
                        mapcol.walls.contains(&mapcol.max),
                    )
                } else {
                    (
                        Walker {
                            row: walker.row - 1,
                            ..*walker
                        },
                        mapcol.walls.contains(&(walker.row - 1)),
                    )
                }
            }
        }
    }
}

pub fn main() {
    let input = puzzle_input!();
    //let input = EXAMPLE;
    let (map, path) = input.split_once("\n\n").unwrap();
    let map = Map::new(map);

    let mut walker = map.start();
    walker.walk_path(path, &map, false);
    let p1 = walker.password();

    let mut walker = map.start();
    walker.walk_path(path, &map, true);
    let p2 = walker.password();

    // 13566, 11451
    println!("part 1: {}\npart 2: {}", p1, p2);
}

#[test]
fn test_cube() {
    //    [1][4]
    //    [2]
    // [3][6]
    // [5]

    // up from 1
    {
        let w = Walker {
            facing: Facing::Up,
            row: 0,
            col: 51,
        };
        let w2 = Map::cube_next(&w);
        assert_eq!(
            Walker {
                facing: Facing::Right,
                row: 151,
                col: 0
            },
            w2
        );
    }

    // up from 4
    {
        let w = Walker {
            facing: Facing::Up,
            row: 0,
            col: 101,
        };
        let w2 = Map::cube_next(&w);
        assert_eq!(
            Walker {
                facing: Facing::Up,
                row: 199,
                col: 1
            },
            w2
        );
    }

    // right from 4
    {
        let w = Walker {
            facing: Facing::Right,
            row: 1,
            col: 149,
        };
        let w2 = Map::cube_next(&w);
        assert_eq!(
            Walker {
                facing: Facing::Left,
                row: 148,
                col: 99
            },
            w2
        );
    }

    // down from 4
    {
        let w = Walker {
            facing: Facing::Down,
            row: 49,
            col: 101,
        };
        let w2 = Map::cube_next(&w);
        assert_eq!(
            Walker {
                facing: Facing::Left,
                row: 51,
                col: 99
            },
            w2
        );
    }

    //    [1][4]
    //    [2]
    // [3][6]
    // [5]

    // right from 2
    {
        let w = Walker {
            facing: Facing::Right,
            row: 51,
            col: 99,
        };
        let w2 = Map::cube_next(&w);
        assert_eq!(
            Walker {
                facing: Facing::Up,
                row: 49,
                col: 101
            },
            w2
        );
    }

    // right from 6
    {
        let w = Walker {
            facing: Facing::Right,
            row: 101,
            col: 99,
        };
        let w2 = Map::cube_next(&w);
        assert_eq!(
            Walker {
                facing: Facing::Left,
                row: 48,
                col: 149
            },
            w2
        );
    }

    // down from 6
    {
        let w = Walker {
            facing: Facing::Down,
            row: 149,
            col: 51,
        };
        let w2 = Map::cube_next(&w);
        assert_eq!(
            Walker {
                facing: Facing::Left,
                row: 151,
                col: 49
            },
            w2
        );
    }

    //    [1][4]
    //    [2]
    // [3][6]
    // [5]

    // right from 5
    {
        let w = Walker {
            facing: Facing::Right,
            row: 151,
            col: 49,
        };
        let w2 = Map::cube_next(&w);
        assert_eq!(
            Walker {
                facing: Facing::Up,
                row: 149,
                col: 51
            },
            w2
        );
    }

    // down from 5
    {
        let w = Walker {
            facing: Facing::Down,
            row: 199,
            col: 1,
        };
        let w2 = Map::cube_next(&w);
        assert_eq!(
            Walker {
                facing: Facing::Down,
                row: 0,
                col: 101
            },
            w2
        );
    }

    // left from 5
    {
        let w = Walker {
            facing: Facing::Left,
            row: 151,
            col: 0,
        };
        let w2 = Map::cube_next(&w);
        assert_eq!(
            Walker {
                facing: Facing::Down,
                row: 0,
                col: 51
            },
            w2
        );
    }

    // left from 3
    {
        let w = Walker {
            facing: Facing::Left,
            row: 101,
            col: 0,
        };
        let w2 = Map::cube_next(&w);
        assert_eq!(
            Walker {
                facing: Facing::Right,
                row: 48,
                col: 50
            },
            w2
        );
    }

    //    [1][4]
    //    [2]
    // [3][6]
    // [5]

    // up from 3
    {
        let w = Walker {
            facing: Facing::Up,
            row: 100,
            col: 1,
        };
        let w2 = Map::cube_next(&w);
        assert_eq!(
            Walker {
                facing: Facing::Right,
                row: 51,
                col: 50
            },
            w2
        );
    }

    // left from 2
    {
        let w = Walker {
            facing: Facing::Left,
            row: 51,
            col: 50,
        };
        let w2 = Map::cube_next(&w);
        assert_eq!(
            Walker {
                facing: Facing::Down,
                row: 100,
                col: 1
            },
            w2
        );
    }

    // left from 1
    {
        let w = Walker {
            facing: Facing::Left,
            row: 1,
            col: 50,
        };
        let w2 = Map::cube_next(&w);
        assert_eq!(
            Walker {
                facing: Facing::Right,
                row: 148,
                col: 0
            },
            w2
        );
    }
}

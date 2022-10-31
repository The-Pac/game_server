use rand::Rng;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct TetrisGame {
    pub tetris_grid: Vec<Vec<String>>,
    score: i32,
    life: i8,
    pub next_piece: Vec<Vec<String>>,
}


impl Tetris {
    fn shuffle_pattern() -> Vec<Vec<String>> {
        let patterns: Vec<Vec<Vec<String>>> = vec![
            vec![
                vec!["".to_string(), "".to_string(), "".to_string(), "".to_string()],
                vec!["lightblue".to_string(), "lightblue".to_string(), "lightblue".to_string(), "lightblue".to_string()],
                vec!["".to_string(), "".to_string(), "".to_string(), "".to_string()],
                vec!["".to_string(), "".to_string(), "".to_string(), "".to_string()],
            ],
            vec![
                vec!["blue".to_string(), "".to_string(), "".to_string(), "".to_string()],
                vec!["blue".to_string(), "blue".to_string(), "blue".to_string(), "".to_string()],
                vec!["".to_string(), "".to_string(), "".to_string(), "".to_string()],
                vec!["".to_string(), "".to_string(), "".to_string(), "".to_string()],
            ],
            vec![
                vec!["".to_string(), "yellow".to_string(), "yellow".to_string(), "".to_string()],
                vec!["".to_string(), "yellow".to_string(), "yellow".to_string(), "".to_string()],
                vec!["".to_string(), "".to_string(), "".to_string(), "".to_string()],
                vec!["".to_string(), "".to_string(), "".to_string(), "".to_string()],
            ],
            vec![
                vec!["".to_string(), "green".to_string(), "green".to_string(), "".to_string()],
                vec!["green".to_string(), "green".to_string(), "".to_string(), "".to_string()],
                vec!["".to_string(), "".to_string(), "".to_string(), "".to_string()],
                vec!["".to_string(), "".to_string(), "".to_string(), "".to_string()],
            ],
            vec![
                vec!["".to_string(), "pink".to_string(), "".to_string(), "".to_string()],
                vec!["pink".to_string(), "pink".to_string(), "pink".to_string(), "".to_string()],
                vec!["".to_string(), "".to_string(), "".to_string(), "".to_string()],
                vec!["".to_string(), "".to_string(), "".to_string(), "".to_string()],
            ],
        ];

        let mut rng = rand::thread_rng();
        let mut pattern: Vec<Vec<String>> = patterns.get(rng.gen_range(1..patterns.len())).unwrap().clone();
        Self::rotate_2d_array(&mut pattern, rng.gen_range(1..=5));
        return pattern;
    }

    /**
    1 / Revert H
    2 / Revert V
    3 / Rotation 90
    4 / Rotation 180
    5 / Rotation 270
     **/
    fn rotate_2d_array(mut arrays: &mut Vec<Vec<String>>, option: i32) {
        let length_arrays = arrays.len();
        match option {
            1 => {
                let pivot = (arrays.len() / 2);
                if pivot % 2 == 0 {
                    for array in arrays {
                        let length_array = array.len();
                        for i in 0..length_array {
                            if i == pivot { break; }
                            array.swap(i, ((length_array - 1) - i));
                        }
                    }
                } else {
                    for array in arrays {
                        let length_array = array.len();
                        for i in 0..length_array {
                            if i == pivot - 1 { break; }
                            array.swap(i, ((length_array - 1) - i));
                        }
                    }
                }
            }
            2 => {
                let pivot = (arrays.len() / 2);
                if pivot % 2 == 0 {
                    for i in 0..length_arrays {
                        if i * 2 > length_arrays - 1 { break; }
                        arrays.swap(i, ((length_arrays - 1) - i));
                    }
                } else {
                    for i in 0..length_arrays {
                        if i == pivot { break; }
                        arrays.swap(i, ((length_arrays - 1) - i));
                    }
                }
            }
            3 => {
                for i in 0..length_arrays {
                    for j in 0..i {
                        let temp = arrays[i][j].to_string();
                        arrays[i][j] = arrays[j][i].to_string();
                        arrays[j][i] = temp;
                    }
                }
                Self::rotate_2d_array(&mut arrays, 2);
            }
            4 => {
                for _ in 0..=1 {
                    Self::rotate_2d_array(&mut arrays, 3)
                }
            }
            5 => {
                for _ in 0..=2 {
                    Self::rotate_2d_array(&mut arrays, 3)
                }
            }
            _ => {}
        }
    }

    pub fn game(&mut self) {
        for rows in &self.tetris_grid {
            for row in rows {}
        }


        self.next_piece = Self::shuffle_pattern();

        let first_array_length = self.tetris_grid[0].len();
        Self::insert_array_in_2d_array(
            &mut self.next_piece,
            &mut self.tetris_grid,
            (first_array_length / 2) - 2,
            0,
        );
    }

    fn insert_array_in_2d_array(arrays_to_insert: &mut Vec<Vec<String>>, mut arrays: &mut Vec<Vec<String>>, x: usize, y: usize) {
        let arrays_to_insert_length = arrays_to_insert.len();
        let mut number_non_valid_row = 0;
        for i in 0..arrays_to_insert_length {
            if arrays_to_insert[i].iter().all(|value| value.to_string() == "".to_string()) {
                number_non_valid_row += 1;
            }
            for j in 0..arrays_to_insert[i].len() {
                let cell = &arrays_to_insert[i][j];
                if cell != "" {
                    arrays[y + i - number_non_valid_row][x + j] = cell.to_string();
                }
            }
        }
    }
}


pub struct SnakeGame {
    x: usize,
    y: usize,
    icon: char,
    tails: Vec<Tails>,
    direction: char,
}

pub struct Snake {
    x: usize,
    y: usize,
    icon: char,
    tails: Vec<Tails>,
    direction: char,
}

struct Tails {
    x: usize,
    y: usize,
    icon: char,
}

impl SnakeGame {

    pub fn game(){
        let mut map: Vec<Vec<char>> = vec![vec![WALL_CHAR; X_SIZE_MAP]; Y_SIZE_MAP];
        let mut is_game_done: bool = false;
        let device_state = DeviceState::new();
        let mut keys: Vec<Keycode>;
        let mut points: u32 = 0;

        let mut snake: Snake = Snake {
            x: X_SIZE_MAP / 2,
            y: Y_SIZE_MAP / 2,
            icon: SNAKE_CHAR,
            tails: vec![],
            direction: 'Z',
        };

        generate_map(&mut map);

        //place snake
        map[snake.y][snake.x] = snake.icon;
        //place first apple
        place_apple(&mut map);

        while !is_game_done {
            keys = device_state.get_keys();
            let last_snake_position_x: usize;
            let last_snake_position_y: usize;

            for key in keys.iter() {
                match key {
                    Keycode::D => {
                        if snake.direction != 'Q' {
                            if map[snake.y][snake.x + 1] != WALL_CHAR && map[snake.y][snake.x + 1] != SNAKE_TAILS_CHAR {
                                snake.direction = 'D';
                            } else {
                                is_game_done = true;
                            }
                        }
                    }
                    Keycode::Q => {
                        if snake.direction != 'D' {
                            if map[snake.y][snake.x - 1] != WALL_CHAR && map[snake.y][snake.x - 1] != SNAKE_TAILS_CHAR {
                                snake.direction = 'Q';
                            } else {
                                is_game_done = true;
                            }
                        }
                    }
                    Keycode::S => {
                        if snake.direction != 'Z' {
                            if map[snake.y + 1][snake.x] != WALL_CHAR && map[snake.y + 1][snake.x] != SNAKE_TAILS_CHAR {
                                snake.direction = 'S';
                            } else {
                                is_game_done = true;
                            }
                        }
                    }
                    Keycode::Z => {
                        if snake.direction != 'S' {
                            if map[snake.y - 1][snake.x] != WALL_CHAR && map[snake.y - 1][snake.x] != SNAKE_TAILS_CHAR {
                                snake.direction = 'Z';
                            } else {
                                is_game_done = true;
                            }
                        }
                    }
                    _ => {}
                }
            }

            map[snake.y][snake.x] = ' ';

            last_snake_position_x = snake.x;
            last_snake_position_y = snake.y;

            match snake.direction {
                'D' => {
                    snake.x += 1;
                }
                'Q' => {
                    snake.x -= 1;
                }
                'S' => {
                    snake.y += 1;
                }
                'Z' => {
                    snake.y -= 1;
                }
                _ => {}
            }

            if map[snake.y][snake.x] == APPLE_CHAR {
                let _ = &snake.tails.push(Tails {
                    x: snake.x,
                    y: snake.y,
                    icon: SNAKE_TAILS_CHAR,
                });
                place_apple(&mut map);
                points += POINT;
            }

            if snake.tails.len() > 0 {
                let mut previous_x = 0;
                let mut previous_y = 0;
                let mut old_x;
                let mut old_y;
                for (tail_index, tail) in snake.tails.iter_mut().enumerate() {
                    map[tail.y][tail.x] = ' ';
                    match tail_index {
                        0 => {
                            previous_x = tail.x;
                            previous_y = tail.y;
                            tail.x = last_snake_position_x;
                            tail.y = last_snake_position_y;
                        }
                        _ => {
                            old_x = tail.x;
                            old_y = tail.y;
                            tail.x = previous_x;
                            tail.y = previous_y;
                            previous_x = old_x;
                            previous_y = old_y;
                        }
                    }
                    map[tail.y][tail.x] = tail.icon;
                }
            }
            map[snake.y][snake.x] = snake.icon;


            //Display map
            for column in &map {
                for cell in column {
                    match *cell {
                        APPLE_CHAR => {
                            print!("{}", cell.to_string().red());
                        }
                        SNAKE_CHAR => {
                            print!("{}", cell.to_string().bright_yellow());
                        }
                        SNAKE_TAILS_CHAR => {
                            print!("{}", cell.to_string().yellow());
                        }
                        WALL_CHAR => {
                            print!("{}", cell.to_string().green());
                        }
                        _ => {
                            print!("{}", cell);
                        }
                    }
                }
                println!();
            }
            println!("points: {}", points.to_string().magenta());

            thread::sleep(Duration::from_millis(100));
            clearscreen::clear().expect("failed to clear screen");
        };
    }
}

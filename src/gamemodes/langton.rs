use macroquad::prelude::{Color, KeyCode};
use macroquad::prelude::{get_fps, is_key_pressed};
use std::collections::{BTreeMap, HashMap};

#[derive(Clone, PartialEq, Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    // Convert the enum to a numerical value
    pub fn as_index(&self) -> i8 {
        match self {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }
    }

    // Convert a numerical value back to the enum
    pub fn from_index(index: i8) -> Self {
        match index {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => panic!("Invalid index for Direction"),
        }
    }

    // Rotate the direction by a given number of steps
    pub fn rotate(&self, steps: isize) -> Self {
        let index = self.as_index() as isize;
        let new_index = (index + steps).rem_euclid(4);
        Direction::from_index(new_index as i8)
    }

    // Cycle the direction to the left or right
    pub fn cycle_direction(&self, turn: &Direction) -> Direction {
        match turn {
            Direction::Right => self.rotate(1), // Turn clockwise
            Direction::Left => self.rotate(-1), // Turn counterclockwise
            _ => panic!("Invalid turn direction"),
        }
    }
}

#[derive(Debug)]
pub struct Ant {
    pub x: i32,
    pub y: i32,
    pub direction: Direction,
}

impl Ant {
    pub fn place_ant(x: i32, y: i32, direction: Direction) -> Ant {
        Ant {
            x,
            y,
            direction: direction,
        }
    }
}

#[derive(Clone)]
pub struct Rule {
    directions: Vec<Direction>,
    colors: Vec<Color>,
}

impl Rule {
    // Constructor method as an associated function
    pub fn new(directions: &str, colors: (u32, u32)) -> Self {
        let number_of_steps = directions.len();
        Rule {
            directions: Rule::convert_directions(directions),
            colors: Rule::generate_gradient(colors.0, colors.1, number_of_steps),
        }
    }

    // Convert a string to a vector of Path
    fn convert_directions(directions: &str) -> Vec<Direction> {
        directions
            .chars()
            .map(|c| match c {
                'R' => Direction::Right,
                'L' => Direction::Left,
                _ => panic!("Invalid direction: {}", c),
            })
            .collect()
    }

    // Generate gradient function
    fn generate_gradient(start: u32, end: u32, steps: usize) -> Vec<Color> {
        let start_r = ((start >> 16) & 0xFF) as f32;
        let start_g = ((start >> 8) & 0xFF) as f32;
        let start_b = (start & 0xFF) as f32;

        let end_r = ((end >> 16) & 0xFF) as f32;
        let end_g = ((end >> 8) & 0xFF) as f32;
        let end_b = (end & 0xFF) as f32;

        (0..steps)
            .map(|i| {
                let t = i as f32 / (steps as f32 - 1.0);
                let r = ((1.0 - t) * start_r + t * end_r) as u8;
                let g = ((1.0 - t) * start_g + t * end_g) as u8;
                let b = ((1.0 - t) * start_b + t * end_b) as u8;

                Color::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0)
            })
            .collect()
    }

    pub fn get_rule_direction(&self, position: usize) -> &Direction {
        &self.directions[position]
    }

    pub fn get_rule_color(&self, position: usize) -> &Color {
        &self.colors[position]
    }
}

pub struct Gamestate {
    grid: HashMap<(i32, i32), u8>,
    ants: BTreeMap<(i32, i32), Vec<Ant>>,
    paused: bool,
    iteration: u128,
    speed: u32,
    update_speed: u64,
    cursor_size: (u8, u8),
    max_cursor_size: u8,
    selected_rule: usize,
    rules: Vec<Rule>,
}

impl Gamestate {
    pub fn new() -> Gamestate {
        Gamestate {
            grid: HashMap::new(),
            ants: BTreeMap::new(),
            paused: true,
            iteration: 0,
            speed: 1,
            update_speed: 0,
            cursor_size: (1, 1),
            max_cursor_size: 10,
            selected_rule: 0,
            // 0 -> Right, 1 -> Left
            rules: vec![
                Rule::new("RL", (0x00000, 0xAAAAAA)),            // Classic Rules
                Rule::new("LRL", (0x005524, 0x2bb25a)),          // Lettuce
                Rule::new("RLLLLLRRL", (0x260511, 0x95097e)),    // Amethyst Cube
                Rule::new("RRLLLRLLLRRR", (0x000021, 0x06d7b4)), // Saphyre Triangle
                Rule::new("RRLL", (0x120021, 0xFF00AA)),         // Brain
                Rule::new("LLRRRLRLRLLR", (0x333300, 0xFFFF00)), // Yellow Highway
                Rule::new("RLLR", (0x00AAAA, 0xFF5500)),         // Cubic crystal
                Rule::new("RRLLRR", (0xDAF7A6, 0x581845)),       // Mini Brain
                Rule::new("LRRLRL", (0xFFC300, 0xFF5733)),       // Pollen
                Rule::new("RRRLLLL", (0x11998E, 0x3B5998)),      // Ocean
                Rule::new("RLLLRRR", (0x00FFFF, 0xFF00FF)),      // Cubic Crystal
            ],
        }
    }

    pub fn get_grid(&self) -> &HashMap<(i32, i32), u8> {
        &self.grid
    }

    pub fn set_grid_value(&mut self, key: (i32, i32), value: u8) {
        self.grid.insert(key, value);
    }

    pub fn clear_grid(&mut self) {
        self.grid.clear();
    }

    pub fn clear_ants(&mut self) {
        self.ants.clear();
    }

    pub fn add_ants(&mut self, position: (i32, i32)) {
        for x in 0..self.cursor_size.0 {
            for y in 0..self.cursor_size.1 {
                let ant =
                    Ant::place_ant(position.0 + x as i32, position.1 + y as i32, Direction::Up);
                self.ants
                    .entry((ant.x, ant.y))
                    .or_insert_with(Vec::new)
                    .push(ant);
            }
        }
    }

    pub fn get_ants_in_region(&self, min_x: i32, max_x: i32, min_y: i32, max_y: i32) -> Vec<&Ant> {
        // Perform a range query on the coordinates in the BTreeMap
        self.ants
            .range((min_x, min_y)..=(max_x, max_y)) // BTreeMap range query
            .flat_map(|(&(x, y), ants)| {
                // Filter ants based on both x and y coordinates
                if x >= min_x && x <= max_x && y >= min_y && y <= max_y {
                    ants.iter()
                } else {
                    [].iter()
                }
            })
            .collect()
    }

    pub fn get_total_ants(&self) -> usize {
        self.ants
            .values()
            .map(|ants_in_cell| ants_in_cell.len())
            .sum()
    }

    pub fn get_total_visible_ants(&self, visible_range: (i32, i32, i32, i32)) -> usize {
        self.get_ants_in_region(
            visible_range.0,
            visible_range.2,
            visible_range.1,
            visible_range.3,
        )
        .len()
    }

    pub fn reset(&mut self) {
        self.grid.clear();
        self.ants.clear();
        self.iteration = 0;
    }

    pub fn get_pause_state(&self) -> bool {
        self.paused
    }

    pub fn invert_pause_state(&mut self) {
        self.paused = !self.paused
    }

    pub fn set_pause_state(&mut self, value: bool) {
        self.paused = value;
    }

    pub fn get_iteration(&self) -> &u128 {
        &self.iteration
    }

    pub fn increment_iteration(&mut self, value: u32) {
        self.iteration += value as u128
    }

    pub fn get_speed(&self) -> u32 {
        self.speed
    }

    pub fn set_speed(&mut self, value: u32) {
        self.speed = value
    }

    pub fn get_update_speed(&self) -> &u64 {
        &self.update_speed
    }

    pub fn get_cursor_dimensions(&self) -> (u8, u8) {
        self.cursor_size
    }

    pub fn get_cursor_x(&self) -> u8 {
        self.cursor_size.0
    }

    pub fn get_cursor_y(&self) -> u8 {
        self.cursor_size.1
    }

    pub fn increment_cursor_x(&mut self, value: u8) {
        if self.cursor_size.0 + value < self.max_cursor_size {
            self.cursor_size.0 += value;
        } else {
            self.cursor_size.0 = self.max_cursor_size;
        }
    }

    pub fn increment_cursor_y(&mut self, value: u8) {
        if self.cursor_size.1 + value < self.max_cursor_size {
            self.cursor_size.1 += value;
        } else {
            self.cursor_size.1 = self.max_cursor_size;
        }
    }

    pub fn decrement_cursor_x(&mut self, value: u8) {
        match self.cursor_size.0.checked_sub(value) {
            Some(result) => self.cursor_size.0 = result,
            None => self.cursor_size.0 = 0,
        }
    }

    pub fn decrement_cursor_y(&mut self, value: u8) {
        match self.cursor_size.1.checked_sub(value) {
            Some(result) => self.cursor_size.1 = result,
            None => self.cursor_size.1 = 0,
        }
    }

    pub fn select_rule(&mut self, rule_number: usize) {
        if rule_number < self.rules.len() {
            self.selected_rule = rule_number
        } else {
            println!("Attempted to select a rule out of range !")
        }
    }

    pub fn get_rule(&self) -> Rule {
        self.rules[self.selected_rule].clone()
    }

    pub fn get_rule_length(&self) -> u8 {
        self.rules[self.selected_rule].directions.len() as u8
    }

    pub fn update(&mut self, number_of_iterations: u32) {
        self.update_speed = 0;
        if !self.paused || is_key_pressed(KeyCode::F) {
            let rule = self.get_rule();
            let rule_length: u8 = self.get_rule_length();
            self.update_speed = self.speed as u64 * get_fps() as u64;

            for _ in 0..number_of_iterations {
                let mut new_ants: BTreeMap<(i32, i32), Vec<Ant>> = BTreeMap::new();

                // Process all ants and determine new positions
                for (pos, ants) in std::mem::take(&mut self.ants) {
                    let current_state = *self.grid.get(&pos).unwrap_or(&0);
                    let new_state = (current_state + 1) % rule_length;
                    self.set_grid_value(pos, new_state); // Directly update the grid instead of collecting changes

                    for mut ant in ants {
                        // Rotate ant direction
                        let current_rule_state = rule.get_rule_direction(current_state as usize);
                        ant.direction = ant.direction.cycle_direction(current_rule_state);

                        // Move ant
                        match ant.direction {
                            Direction::Up => ant.y -= 1,
                            Direction::Right => ant.x += 1,
                            Direction::Down => ant.y += 1,
                            Direction::Left => ant.x -= 1,
                        }

                        // Insert the ant into its new position in the BTreemap
                        new_ants
                            .entry((ant.x, ant.y))
                            .or_insert_with(Vec::new)
                            .push(ant);
                    }
                }

                // Update the ants collection with the new positions
                self.ants = new_ants;
            }

            self.increment_iteration(number_of_iterations);
        }
    }
}

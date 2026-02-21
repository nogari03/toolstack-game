use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Car {
    pub x: f64,
    pub y: f64,
    pub speed: f64,
    pub speed_x: f64,
    pub speed_y: f64,
    pub angle: f64, // in radians
}

#[derive(Serialize, Deserialize, Clone, Debug, Copy, PartialEq)]
pub enum Tile {
    Grass,
    Road,
    Wall,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameState {
    pub car: Car,
    pub track: Vec<Vec<Tile>>,
    pub width: usize,
    pub height: usize,
    pub is_game_over: bool,
}

#[wasm_bindgen]
pub struct RacingGame {
    state: GameState,
}

// Track configuration
const TRACK_WIDTH: usize = 20;
const TRACK_HEIGHT: usize = 15;
const TILE_SIZE: f64 = 40.0; // Assume frontend renders each tile as 40x40 pixels

// Car physics constants
const ACCELERATION: f64 = 0.2;
const MAX_SPEED: f64 = 8.0;
const FRICTION: f64 = 0.95;
const OFF_ROAD_FRICTION: f64 = 0.8;
const MIN_SPEED_TO_TURN: f64 = 0.5;
const TURN_SPEED: f64 = 0.08;
const BRAKE_POWER: f64 = 0.4;

#[wasm_bindgen]
impl RacingGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> RacingGame {
        let track = Self::generate_track();
        
        // Find starting position (simple hardcode for now, on a road tile)
        let default_x = 4.0 * TILE_SIZE + TILE_SIZE / 2.0;
        let default_y = 12.0 * TILE_SIZE + TILE_SIZE / 2.0;

        let car = Car {
            x: default_x,
            y: default_y,
            speed: 0.0,
            speed_x: 0.0,
            speed_y: 0.0,
            angle: -std::f64::consts::FRAC_PI_2, // Pointing UP initially
        };

        RacingGame {
            state: GameState {
                car,
                track,
                width: TRACK_WIDTH,
                height: TRACK_HEIGHT,
                is_game_over: false,
            }
        }
    }

    fn generate_track() -> Vec<Vec<Tile>> {
        let mut track = vec![vec![Tile::Grass; TRACK_WIDTH]; TRACK_HEIGHT];
        
        // Define a simple oval track
        // Top straight
        for x in 4..16 {
            track[2][x] = Tile::Road;
            track[3][x] = Tile::Road;
        }
        // Bottom straight
        for x in 4..16 {
            track[11][x] = Tile::Road;
            track[12][x] = Tile::Road;
        }
        // Left curve
        for y in 4..11 {
            track[y][2] = Tile::Road;
            track[y][3] = Tile::Road;
        }
        // Corner pieces
        track[3][3] = Tile::Road;
        track[11][3] = Tile::Road;
        
        // Right curve
        for y in 4..11 {
            track[y][16] = Tile::Road;
            track[y][17] = Tile::Road;
        }
        track[3][16] = Tile::Road;
        track[11][16] = Tile::Road;
        
        // Outer Walls
        for x in 0..TRACK_WIDTH {
            track[0][x] = Tile::Wall;
            track[TRACK_HEIGHT - 1][x] = Tile::Wall;
        }
        for y in 0..TRACK_HEIGHT {
            track[y][0] = Tile::Wall;
            track[y][TRACK_WIDTH - 1] = Tile::Wall;
        }

        track
    }

    pub fn get_state_json(&self) -> String {
        serde_json::to_string(&self.state).unwrap()
    }

    // Input actions
    pub fn update(&mut self, up: bool, down: bool, left: bool, right: bool) {
        if self.state.is_game_over {
            return;
        }

        let mut current_acceleration = 0.0;

        if up {
            current_acceleration = ACCELERATION;
        } else if down {
            current_acceleration = -BRAKE_POWER;
        }

        // Apply acceleration
        self.state.car.speed += current_acceleration;

        // Apply friction
        let current_tile = self.get_tile_at(self.state.car.x, self.state.car.y);
        let current_friction = match current_tile {
            Tile::Grass => OFF_ROAD_FRICTION,
            _ => FRICTION,
        };
        
        self.state.car.speed *= current_friction;

        // Cap speed
        let actual_max_speed = match current_tile {
            Tile::Grass => MAX_SPEED * 0.4, // Slower on grass
            _ => MAX_SPEED,
        };
        
        if self.state.car.speed > actual_max_speed {
            self.state.car.speed = actual_max_speed;
        }
        if self.state.car.speed < - actual_max_speed * 0.5 { // Reverse speed limit
            self.state.car.speed = -actual_max_speed * 0.5;
        }

        // Stop completely if speed is very low and no input
        if self.state.car.speed.abs() < 0.1 && current_acceleration == 0.0 {
            self.state.car.speed = 0.0;
        }

        // Turning
        if self.state.car.speed.abs() > MIN_SPEED_TO_TURN {
            let turn_factor = if self.state.car.speed > 0.0 { 1.0 } else { -1.0 }; // Reverse turning direction when reversing
            if left {
                self.state.car.angle -= TURN_SPEED * turn_factor;
            }
            if right {
                self.state.car.angle += TURN_SPEED * turn_factor;
            }
        }

        // Update position based on angle and speed
        self.state.car.x += self.state.car.speed * self.state.car.angle.cos();
        self.state.car.y += self.state.car.speed * self.state.car.angle.sin();

        // Boundary / Wall collision
        let next_tile = self.get_tile_at(self.state.car.x, self.state.car.y);
        if next_tile == Tile::Wall {
            // Simple bounce or stop
            self.state.car.speed = -self.state.car.speed * 0.5; // Bounce back a bit
            // Move back out of wall
            self.state.car.x -= self.state.car.speed * self.state.car.angle.cos();
            self.state.car.y -= self.state.car.speed * self.state.car.angle.sin();
        }
    }

    fn get_tile_at(&self, x: f64, y: f64) -> Tile {
        let grid_x = (x / TILE_SIZE).floor() as i32;
        let grid_y = (y / TILE_SIZE).floor() as i32;

        if grid_x < 0 || grid_x >= TRACK_WIDTH as i32 || grid_y < 0 || grid_y >= TRACK_HEIGHT as i32 {
            return Tile::Wall;
        }

        self.state.track[grid_y as usize][grid_x as usize]
    }
}

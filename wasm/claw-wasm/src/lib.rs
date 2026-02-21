use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum ClawState {
    Idle,
    Dropping,
    Grabbing,
    Raising,
    Returning,
    Releasing,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Toy {
    pub id: u32,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub color: String,
    pub value: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameState {
    pub claw_x: f64,
    pub claw_y: f64,
    pub claw_state: ClawState,
    pub score: u32,
    pub toys: Vec<Toy>,
    pub grabbed_toy_id: Option<u32>,
    pub width: f64,
    pub height: f64,
}

#[wasm_bindgen]
pub struct ClawGame {
    state: GameState,
    grab_timer: f64,
    drop_x: f64,
}

const CLAW_SPEED: f64 = 4.0;
const CLAW_DROP_SPEED: f64 = 5.0;
const CLAW_WIDTH: f64 = 60.0;
const CLAW_HEIGHT: f64 = 80.0; // The actual grabber size
const HOME_X: f64 = 80.0; // Drop off hole X
const HOME_Y: f64 = 50.0; // Resting top Y

#[wasm_bindgen]
impl ClawGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ClawGame {
        let mut toys = Vec::new();
        let colors = ["#ef4444", "#3b82f6", "#10b981", "#f59e0b", "#8b5cf6", "#ec4899"];
        
        let mut id_counter = 0;
        // Generate some random-ish looking toys on the floor
        for i in 0..15 {
            let color = colors[i % colors.len()].to_string();
            let mut x = 200.0 + (i as f64 * 35.0) % 550.0;
            let mut y = 550.0 - ((i / 4) as f64 * 45.0);
            
            // simple pseudo-random offsets
            x += ((i * 13) % 40) as f64 - 20.0;
            y += ((i * 7) % 20) as f64 - 10.0;
            
            toys.push(Toy {
                id: id_counter,
                x,
                y,
                width: 40.0,
                height: 40.0,
                color,
                value: 100 * ((i % 5) as u32 + 1),
            });
            id_counter += 1;
        }

        ClawGame {
            state: GameState {
                claw_x: HOME_X,
                claw_y: HOME_Y,
                claw_state: ClawState::Idle,
                score: 0,
                toys,
                grabbed_toy_id: None,
                width: 800.0,
                height: 600.0,
            },
            grab_timer: 0.0,
            drop_x: HOME_X,
        }
    }

    pub fn update(&mut self, left: bool, right: bool, drop: bool) {
        match self.state.claw_state {
            ClawState::Idle => {
                if left {
                    self.state.claw_x -= CLAW_SPEED;
                }
                if right {
                    self.state.claw_x += CLAW_SPEED;
                }
                // Limit horizontal movement (keep away from drop hole mostly)
                if self.state.claw_x < HOME_X { self.state.claw_x = HOME_X; }
                if self.state.claw_x > self.state.width - CLAW_WIDTH { self.state.claw_x = self.state.width - CLAW_WIDTH; }

                if drop && self.state.claw_x > 150.0 { // Can only drop in the play area
                    self.state.claw_state = ClawState::Dropping;
                    self.drop_x = self.state.claw_x;
                }
            }
            ClawState::Dropping => {
                self.state.claw_y += CLAW_DROP_SPEED;
                
                // Check collision with floor or toys
                let mut hit = false;
                if self.state.claw_y >= self.state.height - CLAW_HEIGHT {
                    hit = true;
                } else {
                    for toy in &self.state.toys {
                        // Simple bounding box intersection check for hitting a toy from above
                        if self.state.claw_x < toy.x + toy.width &&
                           self.state.claw_x + CLAW_WIDTH > toy.x &&
                           self.state.claw_y + CLAW_HEIGHT > toy.y {
                            hit = true;
                            break;
                        }
                    }
                }

                if hit {
                    self.state.claw_state = ClawState::Grabbing;
                    self.grab_timer = 30.0; // wait ~0.5s (assuming ~60fps)
                }
            }
            ClawState::Grabbing => {
                self.grab_timer -= 1.0;
                if self.grab_timer <= 0.0 {
                    // Try to grab a toy
                    let mut grabbed = None;
                    for toy in &self.state.toys {
                        // Check if center of claw is close to center of toy
                        let claw_center_x = self.state.claw_x + CLAW_WIDTH / 2.0;
                        let toy_center_x = toy.x + toy.width / 2.0;
                        
                        if (claw_center_x - toy_center_x).abs() < 25.0 && 
                           self.state.claw_y + CLAW_HEIGHT >= toy.y {
                            grabbed = Some(toy.id);
                            break;
                        }
                    }

                    if let Some(id) = grabbed {
                        self.state.grabbed_toy_id = Some(id);
                    }
                    self.state.claw_state = ClawState::Raising;
                }
            }
            ClawState::Raising => {
                self.state.claw_y -= CLAW_DROP_SPEED;
                
                // Move grabbed toy with claw
                if let Some(id) = self.state.grabbed_toy_id {
                    if let Some(toy) = self.state.toys.iter_mut().find(|t| t.id == id) {
                        toy.x = self.state.claw_x + CLAW_WIDTH / 2.0 - toy.width / 2.0;
                        toy.y = self.state.claw_y + CLAW_HEIGHT - toy.height;
                    }
                }

                if self.state.claw_y <= HOME_Y {
                    self.state.claw_y = HOME_Y;
                    self.state.claw_state = ClawState::Returning;
                }
            }
            ClawState::Returning => {
                if self.state.claw_x > HOME_X {
                    self.state.claw_x -= CLAW_SPEED;
                } else {
                    self.state.claw_x = HOME_X;
                    self.state.claw_state = ClawState::Releasing;
                    self.grab_timer = 20.0;
                }

                // Move grabbed toy with claw
                if let Some(id) = self.state.grabbed_toy_id {
                    if let Some(toy) = self.state.toys.iter_mut().find(|t| t.id == id) {
                        toy.x = self.state.claw_x + CLAW_WIDTH / 2.0 - toy.width / 2.0;
                        toy.y = self.state.claw_y + CLAW_HEIGHT - toy.height;
                    }
                }
            }
            ClawState::Releasing => {
                self.grab_timer -= 1.0;
                
                if let Some(id) = self.state.grabbed_toy_id {
                    // Drop toy down the hole
                    if let Some(toy) = self.state.toys.iter_mut().find(|t| t.id == id) {
                        toy.y += CLAW_DROP_SPEED * 2.0;
                    }
                }

                if self.grab_timer <= 0.0 {
                    if let Some(id) = self.state.grabbed_toy_id {
                        // Toy went down the hole, increase score and remove it
                        let idx = self.state.toys.iter().position(|t| t.id == id).unwrap();
                        self.state.score += self.state.toys[idx].value;
                        self.state.toys.remove(idx);
                        self.state.grabbed_toy_id = None;
                    }
                    self.state.claw_state = ClawState::Idle;
                }
            }
        }
    }

    pub fn get_state_json(&self) -> String {
        serde_json::to_string(&self.state).unwrap()
    }
}

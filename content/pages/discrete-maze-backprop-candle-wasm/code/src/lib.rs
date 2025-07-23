use candle_core::{DType, Device, IndexOp, Result, Tensor};
use candle_nn::{Optimizer, VarBuilder, VarMap};
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::SmallRng;
use std::collections::{HashSet, VecDeque};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// Action constants
const ACTION_UP: usize = 0;
const ACTION_RIGHT: usize = 1;
const ACTION_DOWN: usize = 2;
const ACTION_LEFT: usize = 3;
const ACTION_NOOP: usize = 4;
const NUM_ACTIONS: usize = 5;

/// Differentiable maze navigation with soft agent position
pub struct DifferentiableMaze {
    /// Walls tensor [H, W, NUM_ACTIONS] - walls[i,j,a] is true if action a from (i,j) is blocked
    pub walls: Tensor,
    /// Transition matrices [NUM_ACTIONS, N, N] - T[a, from, to]
    pub transition_matrices: Tensor,
    /// Maze dimensions
    pub height: usize,
    pub width: usize,
    pub n_cells: usize,
}

impl DifferentiableMaze {
    /// Create maze from boolean grid (true = wall)
    pub fn new(raw_maze: &[Vec<bool>], device: &Device) -> Result<Self> {
        let height = raw_maze.len();
        let width = raw_maze[0].len();
        let n_cells = height * width;

        // Convert to tensor
        let maze_flat: Vec<f32> = raw_maze
            .iter()
            .flat_map(|row| row.iter().map(|&b| if b { 1.0 } else { 0.0 }))
            .collect();
        let maze_tensor = Tensor::from_vec(maze_flat, (height, width), device)?;

        // Create walls tensor
        let walls = Self::make_wall_tensor(&maze_tensor, device)?;

        // Create transition matrices
        let transition_matrices = Self::make_transition_matrices(&walls, height, width, device)?;

        Ok(Self {
            walls,
            transition_matrices,
            height,
            width,
            n_cells,
        })
    }

    /// Convert (i,j) coordinates to flat index
    pub fn coords_to_index(&self, i: usize, j: usize) -> usize {
        i * self.width + j
    }

    /// Create walls tensor from maze
    fn make_wall_tensor(maze: &Tensor, device: &Device) -> Result<Tensor> {
        let (h, w) = maze.dims2()?;
        let mut walls = vec![0.0f32; h * w * NUM_ACTIONS];

        // Get maze data
        let maze_data = maze.to_vec2::<f32>()?;

        for i in 0..h {
            for j in 0..w {
                let idx = (i * w + j) * NUM_ACTIONS;

                // Boundary walls
                if i == 0 {
                    walls[idx + ACTION_UP] = 1.0;
                } // Can't go up
                if j == w - 1 {
                    walls[idx + ACTION_RIGHT] = 1.0;
                } // Can't go right
                if i == h - 1 {
                    walls[idx + ACTION_DOWN] = 1.0;
                } // Can't go down
                if j == 0 {
                    walls[idx + ACTION_LEFT] = 1.0;
                } // Can't go left

                // Noop is never blocked
                walls[idx + ACTION_NOOP] = 0.0;

                // Walls from obstacles
                if maze_data[i][j] > 0.5 {
                    // This cell is a wall - block all movement actions
                    walls[idx + ACTION_UP] = 1.0;
                    walls[idx + ACTION_RIGHT] = 1.0;
                    walls[idx + ACTION_DOWN] = 1.0;
                    walls[idx + ACTION_LEFT] = 1.0;
                    // Note: NOOP remains unblocked even in wall cells

                    // Block neighbors from entering
                    if i > 0 {
                        walls[((i - 1) * w + j) * NUM_ACTIONS + ACTION_DOWN] = 1.0;
                    }
                    if i < h - 1 {
                        walls[((i + 1) * w + j) * NUM_ACTIONS + ACTION_UP] = 1.0;
                    }
                    if j > 0 {
                        walls[(i * w + j - 1) * NUM_ACTIONS + ACTION_RIGHT] = 1.0;
                    }
                    if j < w - 1 {
                        walls[(i * w + j + 1) * NUM_ACTIONS + ACTION_LEFT] = 1.0;
                    }
                }
            }
        }

        Tensor::from_vec(walls, (h, w, NUM_ACTIONS), device)
    }

    /// Create transition matrices
    fn make_transition_matrices(
        walls: &Tensor,
        h: usize,
        w: usize,
        device: &Device,
    ) -> Result<Tensor> {
        let n = h * w;
        let mut transitions = vec![0.0f32; NUM_ACTIONS * n * n];
        let walls_data = walls.to_vec3::<f32>()?;

        for i in 0..h {
            for j in 0..w {
                let from_idx = i * w + j;

                // Movement actions
                let neighbors = [
                    (ACTION_UP, (i.wrapping_sub(1), j)),
                    (ACTION_RIGHT, (i, j + 1)),
                    (ACTION_DOWN, (i + 1, j)),
                    (ACTION_LEFT, (i, j.wrapping_sub(1))),
                ];

                for &(a, (ni, nj)) in neighbors.iter() {
                    let blocked = ni >= h || nj >= w || walls_data[i][j][a] > 0.5;
                    let to_idx = if blocked { from_idx } else { ni * w + nj };

                    // T[a, from, to] = 1.0
                    transitions[a * n * n + from_idx * n + to_idx] = 1.0;
                }

                // Noop action: always stay in same cell
                transitions[ACTION_NOOP * n * n + from_idx * n + from_idx] = 1.0;
            }
        }

        Tensor::from_vec(transitions, (NUM_ACTIONS, n, n), device)
    }

    /// Compute feasibility mask for current state
    pub fn feasibility_mask(&self, state: &Tensor) -> Result<Tensor> {
        // Reshape state to (H, W)
        let occupied = state.reshape((self.height, self.width))?;

        // occupied[i,j] > 0 AND not walls[i,j,a]
        let occupied_expanded = occupied.unsqueeze(2)?; // (H, W, 1)
        let occupied_broadcast =
            occupied_expanded.broadcast_as((self.height, self.width, NUM_ACTIONS))?;

        // Mask is true where we have probability mass AND no wall
        let zero = Tensor::full(0.0f32, (), occupied_broadcast.device())?;
        let has_mass = occupied_broadcast.broadcast_gt(&zero)?;

        // Get walls as boolean (walls > 0.5 since they're stored as 0.0 or 1.0)
        let threshold = Tensor::full(0.5f32, (), self.walls.device())?;
        let walls_bool = self.walls.broadcast_gt(&threshold)?;

        // Compute logical AND: has_mass AND (NOT walls)
        // Convert to f32 for computation
        let has_mass_f32 = has_mass.to_dtype(DType::F32)?;
        let walls_f32 = walls_bool.to_dtype(DType::F32)?;
        let ones = Tensor::ones_like(&walls_f32)?;
        let walls_not_f32 = ones.sub(&walls_f32)?; // NOT operation

        // AND operation via multiplication
        let result = has_mass_f32.mul(&walls_not_f32)?;

        // Convert back to boolean (> 0 since result is 0.0 or 1.0)
        let zero_threshold = Tensor::zeros_like(&result)?;
        result.broadcast_gt(&zero_threshold)
    }

    /// Masked softmax over actions with temperature
    pub fn masked_softmax(
        &self,
        logits: &Tensor,
        mask: &Tensor,
        temperature: f32,
    ) -> Result<Tensor> {
        // Expand logits to (1, 1, NUM_ACTIONS) then broadcast to (H, W, NUM_ACTIONS)
        let logits_expanded = logits.reshape((1, 1, NUM_ACTIONS))?.broadcast_as((
            self.height,
            self.width,
            NUM_ACTIONS,
        ))?;

        // Apply temperature scaling
        let temp = Tensor::full(temperature, (), logits.device())?;
        let scaled_logits = logits_expanded.broadcast_div(&temp)?;

        // Set -inf on masked positions
        let large_neg = Tensor::full(
            -1e30f32,
            (self.height, self.width, NUM_ACTIONS),
            logits.device(),
        )?;
        let masked_logits = mask.where_cond(&scaled_logits, &large_neg)?;

        // Softmax along action dimension
        candle_nn::ops::softmax(&masked_logits, 2)
    }

    /// One step of state evolution
    pub fn state_step(&self, state: &Tensor, pi: &Tensor) -> Result<Tensor> {
        // state: (N,), pi: (H, W, NUM_ACTIONS)
        // Flatten pi to (N, NUM_ACTIONS)
        let pi_flat = pi.reshape((self.n_cells, NUM_ACTIONS))?;

        // Compute weighted state for each action
        let state_expanded = state.unsqueeze(1)?; // (N, 1)
        let weighted = state_expanded.broadcast_mul(&pi_flat)?; // (N, NUM_ACTIONS)

        // Use matmul to implement einsum('ank,na->k', T, weighted)
        // We need to sum over actions and source cells
        // Reshape weighted to (NUM_ACTIONS, N, 1) and T to (NUM_ACTIONS, N, N)
        // Then for each action, do matmul and sum
        let mut next_state = Tensor::zeros(self.n_cells, state.dtype(), state.device())?;

        for a in 0..NUM_ACTIONS {
            // Get transition matrix for action a: (N, N)
            let t_a = self.transition_matrices.i(a)?;

            // Get weighted probabilities for action a: (N,)
            let weighted_a = weighted.i((.., a))?;

            // Compute contribution: T[a] @ weighted_a
            // t_a is (from, to), weighted_a is (from)
            // Result is (to)
            let contribution = t_a.t()?.matmul(&weighted_a.unsqueeze(1)?)?.squeeze(1)?;
            next_state = next_state.add(&contribution)?;
        }

        Ok(next_state)
    }

    /// Rollout for T timesteps - returns final state only to preserve gradients
    pub fn rollout(
        &self,
        init_state: &Tensor,
        logits_seq: &Tensor,
        temperature: f32,
    ) -> Result<Tensor> {
        let mut current_state = init_state.clone();

        let t_steps = logits_seq.dim(0)?;

        for t in 0..t_steps {
            // Get logits for this timestep
            let logits_t = logits_seq.i(t)?;

            // Compute action probabilities with masking
            let mask = self.feasibility_mask(&current_state)?;
            let pi = self.masked_softmax(&logits_t, &mask, temperature)?;

            // Update state (no clone to preserve gradients)
            current_state = self.state_step(&current_state, &pi)?;
        }

        Ok(current_state)
    }
}

/// Simple model wrapper to hold parameters
struct MazePolicy {
    logits_seq: Tensor,
}

impl MazePolicy {
    fn new(vs: VarBuilder, t_horizon: usize) -> Result<Self> {
        let logits_seq = vs.get_with_hints(
            (t_horizon, NUM_ACTIONS),
            "logits",
            candle_nn::Init::Randn {
                mean: 0.0,
                stdev: 0.1,
            },
        )?;
        Ok(Self { logits_seq })
    }

    fn forward(&self) -> Result<&Tensor> {
        Ok(&self.logits_seq)
    }
}

/// Generate a andom maze with guaranteed path from start to goal
pub fn generate_random_maze(width: usize, height: usize, wall_probability: f32) -> Vec<Vec<bool>> {
    let mut rng = SmallRng::from_seed([43u8; 32]);
    let mut maze = vec![vec![false; width]; height];

    // Randomly place walls
    for i in 0..height {
        for j in 0..width {
            // Keep start and goal free
            if (i == 0 && j == 0) || (i == height - 1 && j == width - 1) {
                continue;
            }

            if rng.random::<f32>() < wall_probability {
                maze[i][j] = true;
            }
        }
    }

    // Ensure path exists from start to goal using BFS
    while !has_path(&maze, (0, 0), (height - 1, width - 1)) {
        // Remove a random wall
        let i = rng.random_range(0..height);
        let j = rng.random_range(0..width);
        if maze[i][j] && !(i == 0 && j == 0) && !(i == height - 1 && j == width - 1) {
            maze[i][j] = false;
        }
    }

    maze
}

/// Create Adam optimizer parameters with given learning rate
fn create_adam_params(lr: f32) -> candle_nn::ParamsAdamW {
    candle_nn::ParamsAdamW {
        lr: lr.into(),
        ..Default::default()
    }
}

/// Check if path exists between two points using BFS
fn has_path(maze: &[Vec<bool>], start: (usize, usize), goal: (usize, usize)) -> bool {
    let height = maze.len();
    let width = maze[0].len();

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some((i, j)) = queue.pop_front() {
        if (i, j) == goal {
            return true;
        }

        // Check all 4 neighbors
        for (di, dj) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;

            if ni >= 0 && ni < height as i32 && nj >= 0 && nj < width as i32 {
                let ni = ni as usize;
                let nj = nj as usize;

                if !maze[ni][nj] && !visited.contains(&(ni, nj)) {
                    visited.insert((ni, nj));
                    queue.push_back((ni, nj));
                }
            }
        }
    }

    false
}

/// Training session that maintains state between steps
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct TrainingSession {
    maze: DifferentiableMaze,
    model: MazePolicy,
    optimizer: candle_nn::AdamW,
    #[allow(dead_code)]
    varmap: VarMap,
    init_state: Tensor,
    start_idx: usize,
    goal_idx: usize,
    width: usize,
    height: usize,
    step: usize,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl TrainingSession {
    /// Create a new training session
    pub fn new(
        width: u32,
        height: u32,
        wall_probability: f32,
        learning_rate: f32,
    ) -> Option<TrainingSession> {
        let width = width as usize;
        let height = height as usize;
        let device = Device::Cpu;

        // Generate random maze
        let raw_maze = generate_random_maze(width, height, wall_probability);

        let maze = DifferentiableMaze::new(&raw_maze, &device).ok()?;

        // Start and goal
        let start_idx = maze.coords_to_index(0, 0);
        let goal_idx = maze.coords_to_index(height - 1, width - 1);

        // Initial state (one-hot at start)
        let mut init_state_vec = vec![0.0f32; maze.n_cells];
        init_state_vec[start_idx] = 1.0;
        let init_state = Tensor::from_vec(init_state_vec, maze.n_cells, &device).ok()?;

        // Trainable parameters - scale horizon with maze size
        let t_horizon = ((width + height) as f32 * 1.5) as usize;
        let varmap = VarMap::new();
        let vs = VarBuilder::from_varmap(&varmap, DType::F32, &device);

        // Create model
        let model = MazePolicy::new(vs, t_horizon).ok()?;

        // Optimizer
        let adam_params = create_adam_params(learning_rate);
        let optimizer = candle_nn::AdamW::new(varmap.all_vars(), adam_params).ok()?;

        Some(TrainingSession {
            maze,
            model,
            optimizer,
            varmap,
            init_state,
            start_idx,
            goal_idx,
            width,
            height,
            step: 0,
        })
    }

    /// Get maze info: [width, height, start_idx, goal_idx, ...maze_data]
    pub fn get_maze_info(&self) -> Vec<f32> {
        let mut result = vec![
            self.width as f32,
            self.height as f32,
            self.start_idx as f32,
            self.goal_idx as f32,
        ];

        // Add maze structure (we need to reconstruct from walls tensor)
        // This is a bit hacky but works
        for i in 0..self.height {
            for j in 0..self.width {
                // Check if this cell is a wall by looking at movement restrictions
                let _idx = self.maze.coords_to_index(i, j);
                // A cell is a wall if all movement actions from it are blocked
                // (except NOOP which is never blocked)
                let is_wall = if let Ok(walls_data) = self.maze.walls.to_vec3::<f32>() {
                    walls_data[i][j][ACTION_UP] > 0.5
                        && walls_data[i][j][ACTION_RIGHT] > 0.5
                        && walls_data[i][j][ACTION_DOWN] > 0.5
                        && walls_data[i][j][ACTION_LEFT] > 0.5
                } else {
                    false
                };
                result.push(if is_wall { 1.0 } else { 0.0 });
            }
        }

        result
    }

    /// Run one training step and return current state probabilities + loss
    pub fn step(&mut self) -> Vec<f32> {
        // Get current logits
        let logits_seq = match self.model.forward() {
            Ok(l) => l,
            Err(_) => return vec![],
        };

        // Rollout to get final state
        let final_state = match self.maze.rollout(&self.init_state, &logits_seq, 1.0) {
            Ok(s) => s,
            Err(_) => return vec![],
        };

        // Get state probabilities as vec
        let state_probs = match final_state.to_vec1::<f32>() {
            Ok(v) => v,
            Err(_) => return vec![],
        };

        // Loss = negative probability at goal
        let goal_prob = match final_state.i(self.goal_idx) {
            Ok(p) => p,
            Err(_) => return vec![],
        };
        let loss = match goal_prob.neg() {
            Ok(l) => l,
            Err(_) => return vec![],
        };

        // Get loss value
        let loss_val = match loss.to_scalar::<f32>() {
            Ok(v) => v,
            Err(_) => return vec![],
        };

        // Backward and update
        if let Err(_) = self.optimizer.backward_step(&loss) {
            return vec![];
        }

        self.step += 1;

        // Return state probabilities + loss
        let mut result = state_probs;
        result.push(loss_val);
        result
    }

    /// Get current step number
    pub fn get_step(&self) -> u32 {
        self.step as u32
    }
}

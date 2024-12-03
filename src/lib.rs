//! Hello, This library creates a small and easy space invaders clone.
//! It have a frame to render on
//! A player to control and shoot 
//! A set of invaders to destroy

pub mod frame;
pub mod invaders;
pub mod player;
pub mod render;
pub mod shot;

/// Number of Rows in the frame
pub const NUM_ROWS: usize = 20;
/// Number of Columns in the frame
pub const NUM_COLS: usize = 40;

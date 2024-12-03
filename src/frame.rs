use crate::{NUM_COLS, NUM_ROWS};
/// Holds the entire frame to be rendered
pub type Frame = Vec<Vec<&'static str>>;

/// Create a new frame with empty spaces
/// Number of rows are `NUM_ROWS`
/// and number of columns are `NUM_COLS`
pub fn new_frame() -> Frame {
    let mut cols = Vec::with_capacity(NUM_COLS);

    for _ in 0..NUM_COLS {
        cols.push(vec![" "; NUM_ROWS]);
    }

    cols
}

/// Define drawable trait so that everything can be drawn to display
pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}

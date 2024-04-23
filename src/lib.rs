#![warn(clippy::all, rust_2018_idioms)]
use std::collections::HashMap;

mod app;
pub use app::BinaryTreeApp;

// mod tree;

struct Binary {
    width: usize,
    bits: usize,
    current_width: usize,
}

impl Binary {
    pub fn new(width: usize) -> Self {
        Self {
            width,
            bits: 0,
            current_width: 0,
        }
    }
}

impl Iterator for Binary {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_width > self.width {
            return None;
        }
        let out = format!(
            "{bits:0width$b}",
            bits = self.bits,
            width = self.current_width
        );
        self.bits += 1;
        if self.bits == usize::pow(2, self.current_width as u32) {
            self.current_width += 1;
            self.bits = 0;
        }
        Some(out)
    }
}

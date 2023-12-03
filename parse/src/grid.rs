use std::fmt::Debug;

use miette::{Diagnostic, Result};
use thiserror::Error;

/// x is the column, y is the row
#[derive(Debug)]
pub struct Grid<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

#[derive(Copy, Clone)]
pub enum Axis {
    X,
    Y,
}

impl Debug for Axis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Axis::X => write!(f, "x"),
            Axis::Y => write!(f, "y"),
        }
    }
}

#[derive(Error, Debug, Diagnostic)]
pub enum GridError {
    #[diagnostic(code(parse::grid::bounds_error))]
    #[error("out of bounds index in {axis:?} axis, {axis:?}={index:?} (width: {width:?}, height: {height:?})")]
    BoundsError {
        index: usize,
        width: usize,
        height: usize,
        axis: Axis,
    },
}

impl<T> Grid<T> {
    pub fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn validate(&self, x: usize, y: usize) -> Result<(), GridError> {
        if x >= self.width {
            Err(GridError::BoundsError {
                index: x,
                width: self.width,
                height: self.height,
                axis: Axis::X,
            })
        } else if y >= self.height {
            Err(GridError::BoundsError {
                index: y,
                width: self.width,
                height: self.height,
                axis: Axis::Y,
            })
        } else {
            Ok(())
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Result<&T> {
        self.validate(x, y)?;
        Ok(&self.data[self.index(x, y)])
    }
}

pub fn parse_grid<T>(input: &str, map_fn: impl Fn(char) -> T) -> Result<Grid<T>> {
    let mut data = Vec::new();
    let mut width = 0;
    let mut height = 0;
    for line in input.lines() {
        height += 1;
        width = 0;
        for c in line.chars() {
            width += 1;
            data.push(map_fn(c));
        }
    }
    Ok(Grid {
        data,
        width,
        height,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_grid() {
        let input = indoc! {r#"
            abc
            def
            ghi
        "#};
        let grid = parse_grid(input, |c| c).unwrap();
        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 3);
        assert_eq!(grid.data.len(), 9);
        assert_eq!(grid.get(0, 0).unwrap(), &'a');
        assert_eq!(grid.get(1, 0).unwrap(), &'b');
        assert_eq!(grid.get(2, 0).unwrap(), &'c');
        assert_eq!(grid.get(0, 1).unwrap(), &'d');
        assert_eq!(grid.get(1, 1).unwrap(), &'e');
        assert_eq!(grid.get(2, 1).unwrap(), &'f');
        assert_eq!(grid.get(0, 2).unwrap(), &'g');
        assert_eq!(grid.get(1, 2).unwrap(), &'h');
        assert_eq!(grid.get(2, 2).unwrap(), &'i');
    }

    #[test]
    fn test_grid_out_of_bounds() {
        let input = indoc! {r#"
            abc
            def
            ghi
        "#};
        let grid = parse_grid(input, |c| c).unwrap();
        assert_eq!(
            grid.get(3, 0).unwrap_err().to_string(),
            "out of bounds index in x axis, x=3 (width: 3, height: 3)"
        );
        assert_eq!(
            grid.get(0, 3).unwrap_err().to_string(),
            "out of bounds index in y axis, y=3 (width: 3, height: 3)"
        );
    }
}

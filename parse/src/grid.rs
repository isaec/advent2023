use std::{collections::HashMap, fmt::Debug, hash::Hash};

use miette::{Diagnostic, Result};
use petgraph::graphmap::GraphMap;
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

#[derive(Debug)]
pub struct Neighbors {
    up: Option<(usize, usize)>,
    down: Option<(usize, usize)>,
    left: Option<(usize, usize)>,
    right: Option<(usize, usize)>,

    up_left: Option<(usize, usize)>,
    up_right: Option<(usize, usize)>,
    down_left: Option<(usize, usize)>,
    down_right: Option<(usize, usize)>,
}

#[derive(Debug)]
pub enum Relationship {
    Orthogonal,
    Diagonal,
    Adjacent,
}

impl Neighbors {
    pub fn iter(&self, relation: Relationship) -> impl Iterator<Item = (usize, usize)> {
        macro_rules! iter_chain {
            ($($iter:expr),*) => {
                None.into_iter()
                    $(.chain($iter.into_iter()))*
            };
        }

        match relation {
            Relationship::Orthogonal => {
                iter_chain! {
                    self.up,
                    self.down,
                    self.left,
                    self.right,
                    None,
                    None,
                    None,
                    None
                }
            }
            Relationship::Diagonal => {
                iter_chain! {
                    self.up_left,
                    self.up_right,
                    self.down_left,
                    self.down_right,
                    None,
                    None,
                    None,
                    None
                }
            }
            Relationship::Adjacent => {
                iter_chain! {
                    self.up,
                    self.down,
                    self.left,
                    self.right,
                    self.up_left,
                    self.up_right,
                    self.down_left,
                    self.down_right
                }
            }
        }
    }
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

    pub fn get_tuple(&self, (x, y): (usize, usize)) -> Result<&T> {
        self.get(x, y)
    }

    pub fn build_lookup(&self) -> HashMap<T, Vec<(usize, usize)>>
    where
        T: Eq + Hash + Copy,
    {
        let mut lookup = HashMap::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let tile = self.get(x, y).expect("valid index");
                lookup.entry(*tile).or_insert_with(Vec::new).push((x, y));
            }
        }
        lookup
    }

    pub fn get_neighbors(&self, x: usize, y: usize) -> Result<Neighbors> {
        macro_rules! cond_tuple {
            ($cond:expr => ($x:expr, $y:expr)) => {
                if $cond {
                    Some(($x, $y))
                } else {
                    None
                }
            };
        }

        self.validate(x, y)?;

        Ok(Neighbors {
            up: cond_tuple! {y > 0 => (x, y - 1)},
            down: cond_tuple! {y < self.height - 1 => (x, y + 1)},
            left: cond_tuple! {x > 0 => (x - 1, y)},
            right: cond_tuple! {x < self.width - 1 => (x + 1, y)},

            up_left: cond_tuple! {y > 0 && x > 0 => (x - 1, y - 1)},
            up_right: cond_tuple! {y > 0 && x < self.width - 1 => (x + 1, y - 1)},
            down_left: cond_tuple! {y < self.height - 1 && x > 0 => (x - 1, y + 1)},
            down_right: cond_tuple! {y < self.height - 1 && x < self.width - 1 => (x + 1, y + 1)},
        })
    }

    pub fn build_graph<E, Ty>(
        &self,
        edge_map_fn: impl Fn(T, T) -> Option<E>,
    ) -> GraphMap<T, E, Ty> {
        todo!()
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

    #[test]
    fn test_grid_parse_enum() {
        #[derive(Debug, PartialEq)]
        enum Tile {
            Empty,
            Wall,
            N(usize),
        }

        let input = indoc! {r#"
            ###
            #1#
            #..
        "#};

        let grid = parse_grid(input, |c| match c {
            '#' => Tile::Wall,
            '.' => Tile::Empty,
            c => Tile::N(c.to_digit(10).unwrap() as usize),
        });

        assert_eq!(
            grid.unwrap().data,
            vec![
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::N(1),
                Tile::Wall,
                Tile::Wall,
                Tile::Empty,
                Tile::Empty,
            ]
        );
    }

    #[test]
    fn test_grid_build_lookup() {
        let input = indoc! {r#"
            ###
            #1#
            #..
        "#};

        let grid = parse_grid(input, |c| c).unwrap();
        let lookup = grid.build_lookup();
        assert_eq!(lookup.len(), 3);
        assert_eq!(lookup.get(&'#').unwrap().len(), 6);
        assert_eq!(lookup.get(&'1').unwrap().len(), 1);
        assert_eq!(lookup.get(&'.').unwrap().len(), 2);

        let validate_lookup = |char: char| {
            let coords = lookup.get(&char).unwrap();
            for (x, y) in coords {
                assert_eq!(grid.get(*x, *y).unwrap(), &char);
            }
        };

        validate_lookup('#');
        validate_lookup('1');
        validate_lookup('.');
    }

    #[test]
    fn test_grid_get_neighbors() {
        let input = indoc! {r#"
            #####
            #...#
            #...#
            #####
        "#};

        let grid = parse_grid(input, |c| c).unwrap();

        assert_eq!(grid.width, 5);
        assert_eq!(grid.height, 4);

        {
            let neighbors = grid.get_neighbors(0, 0).unwrap();
            assert_eq!(neighbors.up, None);
            assert_eq!(neighbors.down, Some((0, 1)));
            assert_eq!(neighbors.left, None);
            assert_eq!(neighbors.right, Some((1, 0)));

            assert_eq!(neighbors.up_left, None);
            assert_eq!(neighbors.up_right, None);
            assert_eq!(neighbors.down_left, None);
            assert_eq!(neighbors.down_right, Some((1, 1)));
        }

        {
            let neighbors = grid.get_neighbors(1, 1).unwrap();
            assert_eq!(neighbors.up, Some((1, 0)));
            assert_eq!(neighbors.down, Some((1, 2)));
            assert_eq!(neighbors.left, Some((0, 1)));
            assert_eq!(neighbors.right, Some((2, 1)));

            assert_eq!(neighbors.up_left, Some((0, 0)));
            assert_eq!(neighbors.up_right, Some((2, 0)));
            assert_eq!(neighbors.down_left, Some((0, 2)));
            assert_eq!(neighbors.down_right, Some((2, 2)));
        }

        {
            let neighbors = grid.get_neighbors(4, 3).unwrap();
            assert_eq!(neighbors.up, Some((4, 2)));
            assert_eq!(neighbors.down, None);
            assert_eq!(neighbors.left, Some((3, 3)));
            assert_eq!(neighbors.right, None);

            assert_eq!(neighbors.up_left, Some((3, 2)));
            assert_eq!(neighbors.up_right, None);
            assert_eq!(neighbors.down_left, None);
            assert_eq!(neighbors.down_right, None);
        }
    }
}

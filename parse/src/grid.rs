use std::{collections::HashMap, fmt::Debug, hash::Hash, iter};

use miette::{Diagnostic, Result};

use petgraph::graphmap::GraphMap;
use thiserror::Error;

#[derive(Clone, PartialEq, Eq, Hash)]
/// x is the column, y is the row
pub struct Grid<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "width={}, height={} {{", self.width, self.height)?;
        let dbg_str = self
            .data
            .iter()
            .map(|tile| format!("{tile:?}"))
            .collect::<Vec<_>>();

        let max_len = dbg_str.iter().map(String::len).max().unwrap_or(0);

        dbg_str
            .chunks(self.width)
            .map(|row| {
                row.iter()
                    .map(|s| format!("{s:max_len$}"))
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .enumerate()
            .map(|(y, row)| format!(" {y}\t| {row}\n"))
            .chain(std::iter::once("}".to_string()))
            .try_for_each(|row| write!(f, "{row}"))
    }
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

#[derive(Debug, Clone, Copy)]
pub enum Relationship {
    /// ``+`` pattern
    Orthogonal,
    /// ``x`` pattern
    Diagonal,
    /// ``+`` and ``x`` pattern
    Adjacent,
}

impl Neighbors {
    pub fn iter(&self, relation: &Relationship) -> impl Iterator<Item = (usize, usize)> {
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
    #[must_use]
    pub fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    #[must_use]
    pub fn reverse_index(&self, i: usize) -> (usize, usize) {
        (i % self.width, i / self.width)
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

    fn unchecked_get(&self, x: usize, y: usize) -> &T {
        &self.data[self.index(x, y)]
    }

    pub fn get_tuple(&self, (x, y): (usize, usize)) -> Result<&T> {
        self.get(x, y)
    }

    #[must_use] pub fn build_lookup(&self) -> HashMap<T, Vec<(usize, usize)>>
    where
        T: Eq + Hash + Copy,
    {
        self.iter().fold(HashMap::new(), |mut acc, ((x, y), t)| {
            acc.entry(*t).or_default().push((x, y));
            acc
        })
    }

    pub fn lookup(&self, value: T) -> Vec<(usize, usize)>
    where
        T: Eq + Hash + Copy,
    {
        self.iter()
            .filter(|(_, t)| **t == value)
            .map(|((x, y), _)| (x, y))
            .collect()
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
        relation: &Relationship,
        edge_map_fn: impl Fn((T, (usize, usize)), (T, (usize, usize))) -> Option<E>,
    ) -> GraphMap<(usize, usize), E, Ty>
    where
        T: Eq + Hash + Copy + Ord,
        Ty: petgraph::EdgeType,
    {
        let mut graph = GraphMap::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let tile = self.get(x, y).expect("valid index");
                let cord = (x, y);
                if !graph.contains_node(cord) {
                    graph.add_node(cord);
                }
                let neighbors = self.get_neighbors(x, y).expect("valid index");
                for neighbor in neighbors.iter(relation) {
                    let neighbor_tile = self.get_tuple(neighbor).expect("valid index");
                    if let Some(edge) = edge_map_fn((*tile, cord), (*neighbor_tile, neighbor)) {
                        graph.add_edge(cord, neighbor, edge);
                    }
                }
            }
        }
        graph
    }

    pub fn raycast_from(
        &self,
        (x, y): (usize, usize),
        direction: (isize, isize),
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        let (mut x, mut y) = (x as isize, y as isize);
        let (dx, dy) = direction;
        iter::from_fn(move || {
            x += dx;
            y += dy;
            let coord = (x as usize, y as usize);
            if self.validate(coord.0, coord.1).is_ok() {
                Some(coord)
            } else {
                None
            }
        })
    }

    pub fn slide_while(
        &mut self,
        (x, y): (usize, usize),
        direction: (isize, isize),
        predicate: impl Fn((usize, usize), &T) -> bool,
        replacement: T,
    ) -> Result<()>
    where
        T: Clone,
    {
        self.validate(x, y)?;
        if let Some((new_x, new_y)) = self
            .raycast_from((x, y), direction)
            .take_while(|(x, y)| predicate((*x, *y), self.unchecked_get(*x, *y)))
            .last()
        {
            let old = self.get(x, y)?.clone();
            self.set(x, y, replacement);
            self.set(new_x, new_y, old);
        }

        Ok(())
    }

    #[must_use]
    pub fn compute_columns(&self) -> Vec<Vec<&T>> {
        (0..self.width)
            .map(|x| {
                (0..self.height)
                    .map(|y| self.get(x, y).expect("valid index"))
                    .collect()
            })
            .collect()
    }

    #[must_use]
    pub fn compute_rows(&self) -> Vec<Vec<&T>> {
        (0..self.height)
            .map(|y| {
                (0..self.width)
                    .map(|x| self.get(x, y).expect("valid index"))
                    .collect()
            })
            .collect()
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        let i = self.index(x, y);
        self.data[i] = value;
    }

    pub fn clone_set(&self, x: usize, y: usize, value: T) -> Self
    where
        T: Clone,
    {
        let mut clone = self.clone();
        clone.set(x, y, value);
        clone
    }

    pub fn replace_at(&mut self, x: usize, y: usize, map_fn: impl FnOnce(T) -> T) {
        let i = self.index(x, y);
        replace_with::replace_with_or_abort(&mut self.data[i], map_fn);
    }

    pub fn clone_replace_at(&self, x: usize, y: usize, map_fn: impl FnOnce(T) -> T) -> Self
    where
        T: Clone,
    {
        let mut clone = self.clone();
        clone.replace_at(x, y, map_fn);
        clone
    }

    pub fn iter(&self) -> impl Iterator<Item = ((usize, usize), &T)> {
        self.data
            .iter()
            .enumerate()
            .map(|(i, t)| (self.reverse_index(i), t))
    }
}

pub fn parse_grid<T>(input: &str, map_fn: impl Fn(char) -> T) -> Result<Grid<T>> {
    let mut data = Vec::with_capacity(input.len());
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

#[macro_export]
macro_rules! Tile {
    ($($name:ident = $value:expr),* ,  $(@$number_name:ident($number_type:ty))? $(,)?) => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub enum Tile {
            $($name,)*
            $($number_name($number_type),)*
        }

        impl TryFrom<char> for Tile {
            type Error = miette::Report;

            fn try_from(c: char) -> std::result::Result<Self, Self::Error> {
                match c {
                    $($value => Ok(Tile::$name),)*
                    $(
                    _ if c.is_digit(10) => Ok(Tile::$number_name(c.to_digit(10).unwrap() as $number_type)),
                    )?
                    _ => Err(miette::Report::msg(format!("None of [{patterns}] match '{c}'", patterns = stringify!($($value),*)))),
                }
            }
        }

        impl core::fmt::Debug for Tile {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Tile::$name => write!(f, "{v}", v = $value),)*
                    $(Tile::$number_name(n) => write!(f, "{n}", n = n),)?
                }
            }
        }

        impl Tile {
            #[track_caller]
            pub fn parse_grid(input: &str) -> Result<Grid<Tile>> {
                use miette::WrapErr;
                let mut data = Vec::with_capacity(input.len());
                let mut width = 0;
                let mut height = 0;
                for line in input.lines() {
                    height += 1;
                    width = 0;
                    for c in line.chars() {
                        width += 1;
                        data.push(Tile::try_from(c)
                            .wrap_err(format!("y={y} \"{line}\"", y = height - 1, line = line))
                            .wrap_err(format!("'{c}' at ({x}, {y}) called from {line}",
                                x = width - 1,
                                y = height - 1,
                                line = std::panic::Location::caller()
                            ))?
                        );
                    }
                }

                Ok(Grid {
                    data,
                    width,
                    height,
                })
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use std::{iter::zip};

    use super::*;
    use indoc::indoc;
    use petgraph::{algo::astar, Undirected};
    use proptest::prelude::*;

    #[test]
    fn grid() {
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
    fn grid_out_of_bounds() {
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
    fn grid_parse_enum() {
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
    fn grid_build_lookup() {
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
    fn grid_get_neighbors() {
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

    #[test]
    fn neighbors_iter() {
        let input = indoc! {r#"
            #####
            #...#
            #...#
            #####
        "#};

        let grid = parse_grid(input, |c| c).unwrap();

        let neighbors = grid.get_neighbors(1, 1).unwrap();
        let iter = neighbors.iter(&Relationship::Orthogonal);
        assert_eq!(
            iter.collect::<Vec<_>>(),
            vec![(1, 0), (1, 2), (0, 1), (2, 1)]
        );

        let neighbors = grid.get_neighbors(1, 1).unwrap();
        let iter = neighbors.iter(&Relationship::Diagonal);
        assert_eq!(
            iter.collect::<Vec<_>>(),
            vec![(0, 0), (2, 0), (0, 2), (2, 2)]
        );

        let neighbors = grid.get_neighbors(1, 1).unwrap();
        let iter = neighbors.iter(&Relationship::Adjacent);
        assert_eq!(
            iter.collect::<Vec<_>>(),
            vec![
                (1, 0),
                (1, 2),
                (0, 1),
                (2, 1),
                (0, 0),
                (2, 0),
                (0, 2),
                (2, 2)
            ]
        );

        let neighbors = grid.get_neighbors(0, 0).unwrap();
        let iter = neighbors.iter(&Relationship::Adjacent);
        assert_eq!(iter.collect::<Vec<_>>(), vec![(0, 1), (1, 0), (1, 1)]);
    }

    #[test]
    fn grid_build_graph() {
        let input = indoc! {r#"
            ###########
            #....#....#
            #....#....#
            #....#....#
            #.........#
            ###########
        "#};

        #[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
        enum Tile {
            Wall,
            Empty,
        }

        let grid = parse_grid(input, |c| match c {
            '#' => Tile::Wall,
            '.' => Tile::Empty,
            _ => panic!("invalid tile"),
        })
        .expect("valid grid");

        assert_eq!(grid.width, 11);
        assert_eq!(grid.height, 6);

        let graph = grid.build_graph::<u64, Undirected>(
            &Relationship::Orthogonal,
            |(a, _), (b, _)| match (a, b) {
                (Tile::Empty, Tile::Empty) => Some(1),
                _ => None,
            },
        );

        dbg!(&graph);

        assert_eq!(graph.node_count(), 66);

        let path = astar(
            &graph,
            (1, 1),
            |finish| finish == (9, 4),
            |(_, _, n)| *dbg!(n),
            |_| 0,
        )
        .expect("path exists");

        assert_eq!(
            path,
            (
                11,
                vec![
                    (1, 1),
                    (1, 2),
                    (1, 3),
                    (1, 4),
                    (2, 4),
                    (3, 4),
                    (4, 4),
                    (5, 4),
                    (6, 4),
                    (7, 4),
                    (8, 4),
                    (9, 4)
                ]
            )
        );
    }

    #[test]
    fn grid_debug() {
        let input = indoc! {r#"
            abc
            def
            ghi
        "#};
        let grid = parse_grid(input, |c| c).unwrap();
        dbg!(&grid);
        assert_eq!(
            format!("{grid:?}"),
            "width=3, height=3 {\n 0\t| 'a' 'b' 'c'\n 1\t| 'd' 'e' 'f'\n 2\t| 'g' 'h' 'i'\n}"
        );
    }

    #[test]
    fn grid_debug_enum() {
        #[derive(Debug)]
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
        })
        .expect("valid grid");

        dbg!(&grid);

        assert_eq!(
            format!("{grid:?}"),
            "width=3, height=3 {\n 0\t| Wall  Wall  Wall \n 1\t| Wall  N(1)  Wall \n 2\t| Wall  Empty Empty\n}"
        );
    }

    #[test]
    fn reverse_index() {
        let input = indoc! {r#"
            abc
            def
            ghi
        "#};
        let grid = parse_grid(input, |c| c).unwrap();
        assert_eq!(grid.reverse_index(0), (0, 0));
        assert_eq!(grid.data[0], 'a');
        assert_eq!(grid.reverse_index(1), (1, 0));
        assert_eq!(grid.data[1], 'b');
        assert_eq!(grid.reverse_index(2), (2, 0));
        assert_eq!(grid.data[2], 'c');
        assert_eq!(grid.reverse_index(3), (0, 1));
        assert_eq!(grid.data[3], 'd');
        assert_eq!(grid.reverse_index(4), (1, 1));
        assert_eq!(grid.reverse_index(5), (2, 1));
        assert_eq!(grid.reverse_index(6), (0, 2));
        assert_eq!(grid.reverse_index(7), (1, 2));
        assert_eq!(grid.reverse_index(8), (2, 2));
    }

    #[test]
    fn tile_macro_parses_correctly() {
        Tile! {
            Empty = '.',
            Wall = '#',
            N1 = '1',
            N2 = '2',
            N3 = '3',
            N4 = '4',
            N5 = '5',
            N6 = '6',
            N7 = '7',
            N8 = '8',
            N9 = '9',
        }

        let input = indoc! {r#"
            ###
            #1#
            #..
        "#};

        let grid = Tile::parse_grid(input).unwrap();

        assert_eq!(
            grid.data,
            vec![
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::N1,
                Tile::Wall,
                Tile::Wall,
                Tile::Empty,
                Tile::Empty,
            ]
        );
    }

    #[test]
    fn tile_macro_fails_parsing_with_diagnostic() {
        Tile! {
            Rock = '#',
            Ash = '.',
        }

        let input = indoc! {r#"
            ####
            #.1#
            ##..
        "#};

        let (line, column, grid) = (line!(), column!() + 16, Tile::parse_grid(input));

        let err = grid.unwrap_err();

        for (actual, expected) in zip(
            err.chain().map(std::string::ToString::to_string),
            vec![
                format!("'1' at (2, 1) called from parse/src/grid.rs:{line}:{column}").as_str(),
                "y=1 \"#.1#\"",
                "None of [\'#\', \'.\'] match '1'",
            ],
        ) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn tile_macro_debug() {
        Tile! {
            Empty = '.',
            Wall = '#',
        }

        let input = indoc! {r#"
            ###
            #.#
            #..
        "#};

        let grid = Tile::parse_grid(input).unwrap();

        dbg!(&grid);

        assert_eq!(
            format!("{grid:?}"),
            "width=3, height=3 {\n 0\t| # # #\n 1\t| # . #\n 2\t| # . .\n}"
        );
    }

    #[test]
    fn tile_macro_with_numbers_debug() {
        Tile! {
            Empty = '.',
            Wall = '#',
            @Number(u8)
        }

        let input = indoc! {r#"
            ###1
            #.#2
            #..3
        "#};

        let grid = Tile::parse_grid(input).unwrap();

        dbg!(&grid);

        assert_eq!(
            format!("{grid:?}"),
            "width=4, height=3 {\n 0\t| # # # 1\n 1\t| # . # 2\n 2\t| # . . 3\n}"
        );
    }

    #[test]
    fn tile_macro_generates_number_parser() {
        Tile! {
            Empty = '.',
            Wall = '#',
            @Number(u64)
        }

        let input = indoc! {r#"
            ###
            #1#
            #..
        "#};

        let grid = Tile::parse_grid(input).unwrap();

        assert_eq!(
            grid.data,
            vec![
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Wall,
                Tile::Number(1),
                Tile::Wall,
                Tile::Wall,
                Tile::Empty,
                Tile::Empty,
            ]
        );
    }

    // proptest

    fn arbitrary_grid(width: usize, height: usize) -> impl Strategy<Value = Grid<&'static str>> {
        let data = prop::sample::select(
            &[
                "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p",
                "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
            ][..],
        );

        let width = 2..width;
        let height = 2..height;

        (width, height, Just(data))
            .prop_flat_map(|(width, height, data)| {
                let data = prop::collection::vec(data, width * height);
                (Just(width), Just(height), data)
            })
            .prop_map(|(width, height, data)| Grid {
                data,
                width,
                height,
            })
    }

    fn arbitrary_grid_with_index(
        width: usize,
        height: usize,
    ) -> impl Strategy<Value = (Grid<&'static str>, (usize, usize))> {
        let grid = arbitrary_grid(width, height);
        grid.prop_flat_map(|grid| {
            let width = 0..grid.width;
            let height = 0..grid.height;
            (Just(grid), (width, height))
        })
    }

    fn empty_grid(width: usize, height: usize) -> impl Strategy<Value = Grid<&'static str>> {
        let width = 2..width;
        let height = 2..height;

        (width, height).prop_map(|(width, height)| Grid {
            data: Vec::new(),
            width,
            height,
        })
    }

    fn empty_grid_with_index(
        width: usize,
        height: usize,
    ) -> impl Strategy<Value = (Grid<&'static str>, (usize, usize))> {
        empty_grid(width, height).prop_flat_map(|grid| {
            let width = 0..grid.width;
            let height = 0..grid.height;
            (Just(grid), (width, height))
        })
    }

    proptest! {
        #[test]
        fn grid_index_reverse_index((grid, (x, y)) in empty_grid_with_index(100_000, 100_000)) {
            let i = grid.index(x, y);
            let (x2, y2) = grid.reverse_index(i);
            assert_eq!((x, y), (x2, y2));
        }

        #[test]
        fn get_columns_matches_manual_iteration(grid in arbitrary_grid(100, 100)) {
            let columns = grid.compute_columns();
            for x in 0..grid.width {
                for y in 0..grid.height {
                    assert_eq!(grid.get(x, y).unwrap(), columns[x][y]);
                }
            }
        }

        #[test]
        fn get_rows_matches_manual_iteration(grid in arbitrary_grid(100, 100)) {
            let rows = grid.compute_rows();
            for y in 0..grid.height {
                for x in 0..grid.width {
                    assert_eq!(grid.get(x, y).unwrap(), rows[y][x]);
                }
            }
        }
    }
}

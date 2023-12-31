use std::ops::Range;

use itertools::Itertools;
use miette::Result;
use miette_pretty::Pretty;
use parse::QuickRegex;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part2(input).unwrap());
}

#[derive(Debug)]
struct Map {
    dest_range_start: i64,
    source_range_start: i64,
    range_length: i64,
}

#[derive(Debug)]
enum MapLine {
    Map(Map),
    MapType(String),
}

#[derive(Debug)]
struct Maps {
    seed_to_soil: Vec<Map>,
    soil_to_fertilizer: Vec<Map>,
    fertilizer_to_water: Vec<Map>,
    water_to_light: Vec<Map>,
    light_to_temperature: Vec<Map>,
    temperature_to_humidity: Vec<Map>,
    humidity_to_location: Vec<Map>,
}

fn parse(input: &str) -> Result<(Vec<Range<i64>>, Maps)> {
    let (seeds, maps) = input.split_once("\n\n").pretty()?;

    let seeds = seeds
        .get_digits()?
        .iter()
        .tuples()
        .map(|(a, b)| *a..(*a + *b))
        .collect();

    let maps = maps
        .lines()
        .filter(|line| !line.is_empty())
        // group by map type
        .map(|line| {
            if line.contains("map:") {
                return Ok(MapLine::MapType(line.get_match("[\\w\\-]+")?.to_string()));
            }
            let nums = line.get_digits()?;
            Ok(MapLine::Map(Map {
                dest_range_start: nums[0],
                source_range_start: nums[1],
                range_length: nums[2],
            }))
        })
        .collect::<Result<Vec<_>>>()?;

    let mut seed_to_soil = Vec::new();
    let mut soil_to_fertilizer = Vec::new();
    let mut fertilizer_to_water = Vec::new();
    let mut water_to_light = Vec::new();
    let mut light_to_temperature = Vec::new();
    let mut temperature_to_humidity = Vec::new();
    let mut humidity_to_location = Vec::new();

    let mut current_map = &mut seed_to_soil;
    for map in maps {
        match map {
            MapLine::MapType(map_type) => match map_type.as_str() {
                "seed-to-soil" => current_map = &mut seed_to_soil,
                "soil-to-fertilizer" => current_map = &mut soil_to_fertilizer,
                "fertilizer-to-water" => current_map = &mut fertilizer_to_water,
                "water-to-light" => current_map = &mut water_to_light,
                "light-to-temperature" => current_map = &mut light_to_temperature,
                "temperature-to-humidity" => current_map = &mut temperature_to_humidity,
                "humidity-to-location" => current_map = &mut humidity_to_location,
                _ => unimplemented!("unknown map type: {}", map_type),
            },
            MapLine::Map(map) => current_map.push(map),
        }
    }

    Ok((
        seeds,
        Maps {
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        },
    ))
}

fn apply_map(seed: i64, map: &Vec<Map>) -> i64 {
    let mut result = seed;
    for map in map {
        if result >= map.source_range_start && result < map.source_range_start + map.range_length {
            result = map.dest_range_start + (result - map.source_range_start);
            break;
        }
    }
    result
}
fn lookup_final_location(maps: &Maps, seed: i64) -> i64 {
    let mut result = seed;
    result = apply_map(result, &maps.seed_to_soil);
    result = apply_map(result, &maps.soil_to_fertilizer);
    result = apply_map(result, &maps.fertilizer_to_water);
    result = apply_map(result, &maps.water_to_light);
    result = apply_map(result, &maps.light_to_temperature);
    result = apply_map(result, &maps.temperature_to_humidity);
    result = apply_map(result, &maps.humidity_to_location);
    result
}

pub fn part2(input: &str) -> Result<i64> {
    let parsed = parse(input)?;
    dbg!(parsed.0.len());
    dbg!(parsed.1.seed_to_soil.len());
    parsed
        .0
        .iter()
        .map(|seeds| {
            dbg!(&seeds);
            seeds
                .clone()
                .into_par_iter()
                .map(|seed| lookup_final_location(&parsed.1, seed))
                .min()
                .expect("min should exist")
        })
        .min()
        .pretty()
}

#[cfg(test)]
mod part2_tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn example() {
        let input = indoc! {r#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#};
        assert_eq!(part2(input).expect("part2 should return Ok"), 46);
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input).expect("part2 should return Ok"), 26829166);
    }
}

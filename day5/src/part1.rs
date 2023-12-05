use miette::Result;
use miette_pretty::Pretty;
use parse::QuickRegex;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(part1(input).unwrap());
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

fn parse(input: &str) -> Result<(Vec<i64>, Maps)> {
    let (seeds, maps) = input.split_once("\n\n").pretty()?;

    let seeds = seeds.get_digits()?;

    let maps = maps
        .lines()
        .filter(|line| !line.is_empty())
        // group by map type
        .map(|line| {
            if line.contains("map:") {
                return Ok(MapLine::MapType(line.get_match("[\\w\\-]+")?.to_string()));
            }
            let nums = line.get_digits()?;
            dbg!(&line);
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

pub fn part1(input: &str) -> Result<i64> {
    let parsed = parse(input)?;
    let apply_map = |seed: i64, map: &Vec<Map>| {
        let mut result = seed;
        for map in map {
            if result >= map.source_range_start
                && result < map.source_range_start + map.range_length
            {
                result = map.dest_range_start + (result - map.source_range_start);
                break;
            }
        }
        result
    };
    let lookup_final_location = |seed: i64| {
        let mut result = seed;
        result = apply_map(result, &parsed.1.seed_to_soil);
        result = apply_map(result, &parsed.1.soil_to_fertilizer);
        result = apply_map(result, &parsed.1.fertilizer_to_water);
        result = apply_map(result, &parsed.1.water_to_light);
        result = apply_map(result, &parsed.1.light_to_temperature);
        result = apply_map(result, &parsed.1.temperature_to_humidity);
        result = apply_map(result, &parsed.1.humidity_to_location);
        result
    };
    let locations = parsed
        .0
        .iter()
        .map(|seed| lookup_final_location(*seed))
        .collect::<Vec<_>>();
    dbg!(&locations);
    locations.iter().min().pretty().copied()
}

#[cfg(test)]
mod part1_tests {
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
        assert_eq!(part1(input).expect("part1 should return Ok"), 35);
    }

    #[test]
    fn input() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input).expect("part1 should return Ok"), 0);
    }
}

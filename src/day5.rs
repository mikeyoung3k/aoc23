pub fn run(file: &str) -> (i64, i64) {
    let data = read_file(file);
    (pt1(&data),pt2_2(&data))
}

#[derive(Debug,Default)]
struct Mappings {
    seeds: Vec<i64>,
    all_maps: Vec<Map>,
}


fn read_file(file: &str) -> Mappings {
    let file = std::fs::read_to_string(file).expect("File not found");
    let mut file_lines = file.lines();
    
    let seeds = file_lines.next().expect("Should have a seeds line").split_whitespace().skip(1).map(|x| x.parse::<i64>().expect("Failed to parse seed")).collect::<Vec<i64>>();
    let _ = file_lines.nth(1);
    
    let seed_to_soil = parse_map(&mut file_lines);
    let soil_to_fertilizer = parse_map(&mut file_lines);
    let fertilizer_to_water = parse_map(&mut file_lines);
    let water_to_light = parse_map(&mut file_lines);
    let light_to_temperature = parse_map(&mut file_lines);
    let temperature_to_humidity = parse_map(&mut file_lines);
    let humidity_to_location = parse_map(&mut file_lines);
    
    let all_maps = vec!(seed_to_soil,soil_to_fertilizer,fertilizer_to_water,water_to_light,light_to_temperature,temperature_to_humidity,humidity_to_location);
    
    Mappings{
        seeds,
        all_maps,
    }
    
}

#[derive(Debug,Default)]
struct Map {
    src: Vec<(i64,i64)>,
    diff: Vec<i64>,
}

impl Map {
    fn get(&self,val: i64) -> i64 {
        for (i,(st,fin)) in self.src.iter().enumerate() {
            if val >= *st && val <= *fin {
                return val + self.diff[i];
            }
        }
        val
    }
    fn revget(&self,val: i64) -> i64 {
        for (i,d) in self.diff.iter().enumerate() {
            let srcval = val - *d;
            let (st,fin) = self.src[i];
            if srcval >= st && srcval <= fin {
                return srcval
            }
        }
        val
    }
}

fn parse_map(lines: &mut std::str::Lines) ->Map {
    let mut diff = Vec::new();
    let mut src = Vec::new();
    while let Some(x) = lines.next() {
        if x.is_empty() {
            break
        }
        let vals = x.split_whitespace().map(|v| v.parse::<i64>().expect("Failed to parse map value")).collect::<Vec<i64>>();
        diff.push(vals[0] - vals[1]);
        src.push((vals[1], vals[1] + vals[2]-1)); // src + diff = dest  -> diff = dest-src
    }
    let _ = lines.next();

    Map{
        src,
        diff,
    }
}

fn pt1(mapping: &Mappings) -> i64 {
    mapping.seeds.iter().map(|seed| seed_to_location(seed,&mapping)).min().expect("Failed to find min")
}

fn pt2(mapping: &Mappings) -> i64 {
    seeds_from_mapping(mapping).iter().map(|seed| seed_to_location(seed,&mapping)).min().expect("Failed to find min")
}

fn seed_to_location(seed: &i64, mapping: &Mappings) -> i64 {
    mapping.all_maps.iter().fold(*seed, |acc,map| map.get(acc))
}

fn seeds_from_mapping(mapping: &Mappings) -> Vec<i64> {
    let strt = mapping.seeds.iter().step_by(2);
    let lens = mapping.seeds[1..].iter().step_by(2);
    let rngs = strt.zip(lens);

    let mut res = Vec::new();

    for (&s,&l) in rngs {
        res.append(
            &mut (s..s+l-1).collect::<Vec<i64>>()
        )
    }
    res
}

fn pt2_2(mapping: &Mappings) -> i64 {
    for loc in 0..i64::MAX {
        if is_seed(loc, mapping) {
            return loc
        }
    }
    panic!("Couldn't find a seed!")
}

fn is_seed(loc: i64, mapping: &Mappings) -> bool {
    let seed = mapping.all_maps.iter().rev().fold(loc, |acc,map| map.revget(acc));
    
    let strt = mapping.seeds.iter().step_by(2);
    let lens = mapping.seeds[1..].iter().step_by(2);
    let rngs = strt.zip(lens);

    for (&s,&l) in rngs {
        if seed >= s && seed <= s+l-1 {
            return true
        }
    }
    false
}
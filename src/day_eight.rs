pub const COORDINATES: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Coordinate {
    x: usize,
    y: usize,
    z: usize,
}

impl Coordinate {
    pub fn euclidean_distance(&self, other: &Coordinate) -> f64 {
        // https://en.wikipedia.org/wiki/Euclidean_distance
        let dx = (self.x as isize - other.x as isize) as f64;
        let dy = (self.y as isize - other.y as isize) as f64;
        let dz = (self.z as isize - other.z as isize) as f64;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

pub struct DayEight {
    junction_boxes: HashMap<Coordinate, usize>,
    total_connections: usize,
    circuit_tracker: HashMap<usize, Vec<Coordinate>>,
    handled_pairs: Vec<(Coordinate, Coordinate)>,
}

impl DayEight {
    pub fn new(input: &str) -> Self {
        let mut junction_boxes = HashMap::new();
        for line in input.lines() {
            let parts: Vec<&str> = line.split(',').collect();
            let x = parts[0].trim().parse::<usize>().unwrap();
            let y = parts[1].trim().parse::<usize>().unwrap();
            let z = parts[2].trim().parse::<usize>().unwrap();
            let coord = Coordinate { x, y, z };
            junction_boxes.insert(coord, 0);
        }

        DayEight {
            junction_boxes,
            total_connections: 0,
            circuit_tracker: HashMap::new(),
            handled_pairs: Vec::new(),
        }
    }

    pub fn find_closest_two_boxes(&self) -> (Coordinate, Coordinate) {
        let mut closest_pair = None;
        let mut closest_distance = f64::MAX;

        for (coord_a, coord_a_id) in self.junction_boxes.iter() {
            for (coord_b, coord_b_id) in self.junction_boxes.iter() {
                if coord_a_id == coord_b_id && *coord_a_id != 0 {
                    continue; // Same circuit, skip
                }
                if coord_a != coord_b {
                    let distance = coord_a.euclidean_distance(coord_b);
                    if distance < closest_distance {
                        closest_distance = distance;
                        closest_pair = Some((coord_a.clone(), coord_b.clone()));
                    }
                }
            }
        }

        closest_pair.expect("No closest pair found")
    }

    pub fn print_junctions(&self) {
        let sorted_by_size: Vec<(&usize, &Vec<Coordinate>)> = {
            let mut vec: Vec<(&usize, &Vec<Coordinate>)> = self.circuit_tracker.iter().collect();
            vec.sort_by_key(|&(_id, coords)| usize::MAX - coords.len());
            vec
        };
        for (circuit_id, coords) in sorted_by_size.iter() {
            println!("Circuit ID {}: {} boxes", circuit_id, coords.len());
            for coord in coords.iter() {
                println!("  - {:?}", coord);
            }
        }
        // Print unassigned boxes
        let unassigned: Vec<&Coordinate> = self
            .junction_boxes
            .iter()
            .filter_map(|(coord, id)| if *id == 0 { Some(coord) } else { None })
            .collect();
        println!("Unassigned boxes ({}):", unassigned.len());
    }

    pub fn find_closest_boxes(&mut self, _times: usize) {
        // Ugly but whatever
        let mut iteration = 0;
        loop {
            iteration += 1;
            let (coord_a, closest) = self.find_closest_two_boxes();

            self.handled_pairs.push((coord_a.clone(), closest.clone()));

            println!(
                "Iteration {}: Closest boxes are {:?} and {:?}",
                iteration + 1,
                coord_a,
                closest
            );

            // While looping the values might have changed (Since we are looping over a copy)
            let own_id = *self.junction_boxes.get(&coord_a).unwrap();
            let closest_id = *self.junction_boxes.get(&closest).unwrap();

            if own_id == 0 && closest_id == 0 {
                self.total_connections += 1;
                let new_id = self.total_connections;
                self.junction_boxes.insert(coord_a.clone(), new_id);
                self.junction_boxes.insert(closest.clone(), new_id);
                self.circuit_tracker
                    .insert(new_id, vec![coord_a.clone(), closest.clone()]);
                println!(
                    "Creating new circuit id {} with {:?} and {:?}",
                    new_id, coord_a, closest
                );
            } else if own_id == closest_id {
                println!(
                    "Skipping {:?} since own id {} is same as closest id {}",
                    coord_a, own_id, closest_id
                );
                continue;
            } else if own_id == 0 && closest_id != 0 {
                self.junction_boxes.insert(coord_a.clone(), closest_id);
                self.circuit_tracker
                    .get_mut(&closest_id)
                    .unwrap()
                    .push(coord_a.clone());
                println!(
                    "Adding {:?} to circuit id {}. New size: {}",
                    coord_a,
                    closest_id,
                    self.circuit_tracker.get(&closest_id).unwrap().len()
                );
            } else if own_id != 0 && closest_id == 0 {
                self.junction_boxes.insert(closest.clone(), own_id);
                self.circuit_tracker
                    .get_mut(&own_id)
                    .unwrap()
                    .push(closest.clone());
                println!(
                    "Adding {:?} to circuit id {}. New size: {}",
                    closest,
                    own_id,
                    self.circuit_tracker.get(&own_id).unwrap().len()
                );
            } else {
                // Both have ids, need to merge
                let (keep_id, merge_id) = if own_id < closest_id {
                    (own_id, closest_id)
                } else {
                    (closest_id, own_id)
                };

                println!(
                    "Merging circuits {} and {} into {}",
                    own_id, closest_id, keep_id
                );

                println!(
                    "Merging Coords: {:?} into circuit id {}",
                    self.circuit_tracker.get(&merge_id).unwrap(),
                    keep_id
                );

                let merge_coords = self.circuit_tracker.remove(&merge_id).unwrap();
                for coord in merge_coords.iter() {
                    self.junction_boxes.insert(coord.clone(), keep_id);
                }
                self.circuit_tracker
                    .get_mut(&keep_id)
                    .unwrap()
                    .extend(merge_coords);

                println!(
                    "Merging circuit id {} into {}. New size: {}",
                    merge_id,
                    keep_id,
                    self.circuit_tracker.get(&keep_id).unwrap().len()
                );
            }
            //self.print_junctions();
        }
    }
}

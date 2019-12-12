use regex::Regex;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Moon {
    position: Vec<i32>,
    velocity: Vec<i32>
}

fn apply_gravity(moon1: &mut Moon, moon2: &mut Moon) {
    for i in 0..3 {
        let pos1 = moon1.position[i];
        let pos2 = moon2.position[i];
        let mut delta = 0;

        if pos1 < pos2 {
            delta = 1;
        } else if pos1 > pos2 {
            delta = -1
        }
        moon1.velocity[i] += delta;
        moon2.velocity[i] -= delta;
    }
}

fn main() {
    let text = "<x=14, y=4, z=5>
    <x=12, y=10, z=8>
    <x=1, y=7, z=-10>
    <x=16, y=-5, z=3>".to_string();
    let re = Regex::new(r"<x=(.*), y=(.*), z=(.*)>").unwrap();
    let mut moons: Vec<Moon> = Vec::new();

    for cap in re.captures_iter(&text) {
        let position: Vec<i32> = vec!(cap[1].parse::<i32>().unwrap(),
                                      cap[2].parse::<i32>().unwrap(),
                                      cap[3].parse::<i32>().unwrap());
        let velocity: Vec<i32> = vec![0, 0, 0];
        let moon: Moon = Moon { position, velocity};
        moons.push(moon);
    }

    // println!("{:?}", moons);

    // let mut permutations = Vec::new();
    // heap_recursive(&mut moons, |permutation| {
    //     permutations.push(permutation.to_vec())
    // });

    for iteration in 1..=1000 {

        let it = (0..moons.len()).tuple_combinations();

        println!("Iteration: {}", iteration);

        for (idx1, idx2) in it.collect_vec() {
            // println!("{:?}", idx1);
            let mut moon1 = moons[idx1 as usize].clone();
            let mut moon2 = moons[idx2 as usize].clone();

            // println!("{}", moon1.muh());

            // apply_gravity(&mut moon1, &mut moon2);
            for i in 0..3 {
                let pos1 = moon1.position[i];
                let pos2 = moon2.position[i];
                let mut delta = 0;

                if pos1 < pos2 {
                    delta = 1;
                } else if pos1 > pos2 {
                    delta = -1
                }
                moon1.velocity[i] += delta;
                moon2.velocity[i] -= delta;
            }

            moons[idx1 as usize] = moon1;
            moons[idx2 as usize] = moon2;
        }

        // println!("{:?}", moons);

        for mut moon in &mut moons {
            let mut temp: Vec<i32> = vec!(0, 0, 0);
            for i in 0..3 {
                temp[i] = moon.position[i] + moon.velocity[i];
            }
            moon.position = temp;
        }

        // println!("{:?}", moons);

        let mut total: i32 = 0;
        for mut moon in &mut moons {
            let potential: i32 = moon.position.iter().map(|x| x.abs()).sum();
            let kinetic: i32 = moon.velocity.iter().map(|x| x.abs()).sum();
            total += potential * kinetic;
        }

        println!("{}", total);
    }
}

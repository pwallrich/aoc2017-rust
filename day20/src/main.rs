use std::{collections::HashMap, fs, hash::Hash};

use regex::Regex;

fn main() {
    let input =
        fs::read_to_string("day20/input.txt").expect("Should have been able to read the file");
    // let input: String = "p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>\np=<-4,0,0>, v=<2,0,0>, a=<0,0,0>\np=<-2,0,0>, v=<1,0,0>, a=<0,0,0>\np=<3,0,0>, v=<-1,0,0>, a=<0,0,0>".to_string();

    let re = Regex::new(
        r"p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>",
    )
    .unwrap();
    let mut particles: Vec<Particle> = Vec::new();
    for (_, [px, py, pz, vx, vy, vz, ax, ay, az]) in re.captures_iter(&input).map(|c| c.extract()) {
        particles.push(Particle {
            position: Point3D {
                x: px.parse::<i32>().unwrap(),
                y: py.parse::<i32>().unwrap(),
                z: pz.parse::<i32>().unwrap(),
            },
            velocity: Point3D {
                x: vx.parse::<i32>().unwrap(),
                y: vy.parse::<i32>().unwrap(),
                z: vz.parse::<i32>().unwrap(),
            },
            acc: Point3D {
                x: ax.parse::<i32>().unwrap(),
                y: ay.parse::<i32>().unwrap(),
                z: az.parse::<i32>().unwrap(),
            },
        })
    }
    // println!("{:?}", particles);

    // find min a in manhattan distance
    let distances: (usize, i32) = particles
        .iter()
        .map(|x| x.acc.x.abs() + x.acc.y.abs() + x.acc.z.abs())
        .enumerate()
        .min_by(|x, y| x.1.cmp(&y.1))
        .unwrap();
    println!("minmum will be {}", distances.0);
    let mut count = 0;
    for i in 0..1_000_000 {
        if i % 1_000 == 0 {
            println!("iter: {count}");
        }
        count += 1;
        let mut next_pos: HashMap<Point3D, Vec<usize>> = HashMap::new();
        let mut idx: usize = 0;
        for particle in &mut particles {
            particle.velocity.x += particle.acc.x;
            particle.velocity.y += particle.acc.y;
            particle.velocity.z += particle.acc.z;

            particle.position.x += particle.velocity.x;
            particle.position.y += particle.velocity.y;
            particle.position.z += particle.velocity.z;

            if next_pos.contains_key(&particle.position) {
                next_pos.get_mut(&particle.position).unwrap().push(idx);
            } else {
                next_pos.insert(particle.position, Vec::from([idx]));
            }
            idx += 1;
        }
        particles = particles
            .iter()
            .enumerate()
            .filter(|x| next_pos[&x.1.position].len() == 1)
            .map(|x| *x.1)
            .collect();

        // println!("{:?}", particles);
    }
    println!("{:?}", particles.len())
}
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Particle {
    position: Point3D,
    velocity: Point3D,
    acc: Point3D,
}
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

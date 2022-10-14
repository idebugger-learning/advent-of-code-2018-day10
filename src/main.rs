use std::{collections::HashSet, fmt::Display};

use regex::Regex;

#[derive(Debug)]
struct Point {
    position: (i64, i64),
    velocity: (i64, i64),
}

fn main() {
    // let input = include_str!("./data/input_example.txt");
    let input = include_str!("./data/input.txt");

    let mut points = parse_input(input);

    println!("Points: {:?}", points);
    println!("Points amount: {}", points.len());

    let steps = run_simulation(&mut points);

    println!("Simulation took {} steps", steps);
    print_points(&points);
}

fn print_points(points: &Vec<Point>) {
    let points_positions = points
        .iter()
        .map(|point| point.position)
        .collect::<HashSet<_>>();
    let padding = 2;
    let max_x = points_positions
        .iter()
        .max_by_key(|position| position.0)
        .unwrap()
        .0;
    let min_x = points_positions
        .iter()
        .min_by_key(|position| position.0)
        .unwrap()
        .0;
    let max_y = points_positions
        .iter()
        .max_by_key(|position| position.1)
        .unwrap()
        .1;
    let min_y = points_positions
        .iter()
        .min_by_key(|position| position.1)
        .unwrap()
        .1;

    println!();
    for y in (min_y - padding)..=(max_y + padding) {
        for x in (min_x - padding)..=(max_x + padding) {
            if points_positions.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn run_simulation(points: &mut Vec<Point>) -> u64 {
    // The simulation stops on local minimum score found
    let mut old_score = calc_score(&points);
    println!("Points initial score: {}", old_score);

    let mut steps = 0;
    loop {
        steps += 1;
        step_forward(points);
        let new_score = calc_score(&points);
        if steps % 100 == 0 {
            println!("Points score after {} step: {}", steps, new_score);
        }

        if new_score <= old_score {
            old_score = new_score;
        } else {
            step_backward(points);
            steps -= 1;
            break;
        }
    }
    steps
}

fn step_forward(points: &mut Vec<Point>) {
    for point in points {
        point.position.0 += point.velocity.0;
        point.position.1 += point.velocity.1;
    }
}

fn step_backward(points: &mut Vec<Point>) {
    for point in points {
        point.position.0 -= point.velocity.0;
        point.position.1 -= point.velocity.1;
    }
}

fn calc_score(points: &Vec<Point>) -> u64 {
    let mut sum_of_distances = 0u64;
    for point_x in points {
        for point_y in points {
            let distance = (point_x.position.0 - point_y.position.0).abs() as u64
                + (point_x.position.1 - point_y.position.1).abs() as u64;
            sum_of_distances += distance;
        }
    }
    sum_of_distances
}

fn parse_input(input: &str) -> Vec<Point> {
    let row_regexp =
        Regex::new(r"^position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>$").unwrap();

    input
        .split('\n')
        .inspect(|row| println!("Row: {}", row))
        .map(|row| row_regexp.captures(row).unwrap())
        .map(|captures| Point {
            position: (
                captures[1].parse::<i64>().unwrap(),
                captures[2].parse::<i64>().unwrap(),
            ),
            velocity: (
                captures[3].parse::<i64>().unwrap(),
                captures[4].parse::<i64>().unwrap(),
            ),
        })
        .collect()
}

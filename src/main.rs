use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;
use std::collections::VecDeque;



const QUEUE_LIMIT: usize = 1000;

fn path_sum(path: &[usize], distance_matrix: &[Vec<f64>]) -> f64 {
    path.iter()
        .zip(path.iter().skip(1))
        .map(|(a, b)| distance_matrix[*a][*b])
        .sum()
}

fn bfs(distance_matrix: Vec<Vec<f64>>) -> (VecDeque<Vec<usize>>, f64, Vec<usize>){
    let node_count = distance_matrix.len();
    let mut best_path = vec![0];
    let mut best_distance = std::f64::MAX;

    let mut queue: VecDeque<Vec<usize>> = VecDeque::new();

    while let Some(path) = queue.pop_front() {
        if path.len() == node_count {
            let distance = path_sum(&path, &distance_matrix);
            if distance < best_distance {
                best_distance = distance;
                best_path = path;
            }
        } else {
            for next in 0..node_count {
                if !path.contains(&next) {
                    let mut new_path = path.clone();
                    new_path.push(next);
                    if path_sum(&new_path, &distance_matrix) < best_distance && queue.len() < QUEUE_LIMIT {
                        queue.push_back(new_path);
                    }
                    if queue.len() >= QUEUE_LIMIT {
                        break;
                    }
                }
            }
        }
    } 
    (queue, best_distance, best_path)
}

fn tsp(distance_matrix: Vec<Vec<f64>>, thread_count: usize) -> (Vec<usize>, f64) {
    let node_count = distance_matrix.len();
    let mut best_path = vec![0; node_count];
    let mut best_distance = std::f64::MAX;

    let queue = Arc::new(Mutex::new(Vec::<Vec<usize>>::new()));
    let path_stacks = Arc::new(Mutex::new(Vec::<Vec<usize>>::new()));

    for i in 0..thread_count {
        let queue = queue.clone();
        let path_stacks = path_stacks.clone();
        thread::spawn(move || {
            let mut path_stack = Vec::new();
            path_stacks.lock().unwrap().push(path_stack);
            while let Some(path) = queue.lock().unwrap().pop() {
                let distance = path.iter().zip(path.iter().skip(1)).map(|(a, b)| distance_matrix[*a][*b]).sum();
                if distance < best_distance {
                    best_distance = distance;
                    best_path = path;
                }
                let last = *path.last().unwrap();
                for next in 0..node_count {
                    if !path.contains(&next) {
                        let mut new_path = path.clone();
                        new_path.push(next);
                        queue.lock().unwrap().push(new_path);
                    }
                }
            }
        });
    }

    let mut path = vec![0];
    queue.lock().unwrap().push(path);
    while thread_count != path_stacks.lock().unwrap().len() {}

    (best_path, best_distance)
}

fn main() {
    let file = File::open("data/datasets/FIVE/five_d.txt").unwrap();
    let reader = BufReader::new(file);
    let distance_matrix: Vec<Vec<f64>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();
    let (best_path, best_distance) = tsp(distance_matrix, 8);
    println!("Best path: {:?}", best_path);
    println!("Best distance: {}", best_distance);
}

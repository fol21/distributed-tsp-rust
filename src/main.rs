use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::collections::VecDeque;

use crossbeam_channel::unbounded;

extern crate num_cpus;

const QUEUE_LIMIT: usize = 100000;

fn path_sum(path: &[usize], distance_matrix: &[Vec<f64>]) -> f64 {
    path.iter()
        .zip(path.iter().skip(1))
        .map(|(a, b)| distance_matrix[*a][*b])
        .sum()
}

fn print_path(path: &[usize]) {
    for i in 0..path.len() {
        if i != path.len() - 1 {
            print!("{}->", path[i]);
        } else {
            print!("{}", path[i]);
        }
    }
    println!();
}

fn print_stack(stack: &Vec<Vec<usize>>)
{
    for i in 0..stack.len() {
        for j in 0..stack[i].len() {
            if j == stack[i].len() - 1 {
                print!("{}", stack[i][j]);
            } else {
                print!("{} ", stack[i][j]);
            }
        }
        print!(" || ");
    }
    println!();
}

fn print_matrix(distance_matrix: &[Vec<f64>]) {
    for i in 0..distance_matrix.len() {
        for j in 0..distance_matrix.len() {
            if j == distance_matrix.len() - 1 {
                print!("{}", distance_matrix[i][j]);
            } else {
                print!("{} ", distance_matrix[i][j]);
            }
        }
        println!();
    }
}

fn bfs(distance_matrix: &Vec<Vec<f64>>, size: usize) -> (VecDeque<Vec<usize>>, f64, Option<Vec<usize>>){
    let node_count = distance_matrix.len();
    let mut best_path: Option<Vec<usize>> = None;
    let mut best_distance = std::f64::MAX;
    let mut queue: VecDeque<Vec<usize>> = VecDeque::from(vec![vec![]]);

    let mut ready = false;
    while  !queue.is_empty() && !ready {
        let path = queue.pop_front().unwrap();
        if path.len() == node_count {
            let distance = path_sum(&path, &distance_matrix);
            if distance < best_distance {
                best_distance = distance;
                best_path = Some(path);
            }
        } else {
            for next in 0..node_count {
                if !path.contains(&next) {
                    let mut new_path = path.clone();
                    new_path.push(next);
                    if path_sum(&new_path, &distance_matrix) < best_distance {
                        queue.push_back(new_path);
                    }
                    if queue.len() >= size || queue.len() >= QUEUE_LIMIT {
                        ready = true;
                        break;
                    }
                }
            }
        }
    } 
    (queue, best_distance, best_path)
}

fn tsp(distance_matrix: &Vec<Vec<f64>>, queue_size: usize) -> (Option<Vec<usize>>, f64) {
    let node_count = distance_matrix.len();
    let (mut queue, best_distance, best_path) = bfs(distance_matrix, queue_size);
    let mut path_stacks = vec![Vec::<Vec<usize>>::new(); num_cpus::get()];

    let mut _c: usize = 0;
    while !queue.is_empty() {
        let mut path = queue.pop_front().unwrap();
        path_stacks[_c % num_cpus::get()].push(path);
        // print_stack(&stacks[_c % num_cpus::get()]);
        _c += 1;
    }
    
    let mut best_distance = std::f64::MAX;
    let mut best_path: Option<Vec<usize>> = None;
    
    let (distance_otx, distance_orx) = unbounded::<f64>();
    let (path_otx, path_orx) = unbounded::<Vec<usize>>();

    let mut join_handles = vec![];
    for stack in path_stacks.iter() {
        let (distance_tx, distance_rx) = (distance_otx.clone(), distance_orx.clone());
        let (path_tx, path_rx) = (path_otx.clone(), path_orx.clone());
        
        let mut _s = Arc::new(Mutex::new(stack.clone()));
        let _d = Arc::new(distance_matrix.clone());
        join_handles.push(thread::spawn(move || {
            let mut best_distance = std::f64::MAX;
            let mut best_path: Option<Vec<usize>> = None;
            while let Some(path) = _s.lock().unwrap().pop() {
                let distance = path_sum(&path, &_d);
                let _aux = distance_rx.recv().unwrap();
                if _aux < best_distance {
                    best_distance = _aux;
                }
                if distance < best_distance {
                    best_distance = distance;
                    best_path = Some(path);
                    distance_tx.send(best_distance.clone()).unwrap();
                    path_tx.send(best_path.clone().unwrap()).unwrap();
                } else {
                    for next in 0..node_count {
                        if !path.contains(&next) {
                            let mut new_path = path.clone();
                            new_path.push(next);
                            _s.lock().unwrap().push(new_path);
                        }
                    }
                }
            }
            distance_tx.send(best_distance).unwrap();
        }));
    }
    for handle in join_handles {
        handle.join().unwrap();
    }
    best_distance = distance_orx.recv().unwrap();
    best_path = Some(path_orx.recv().unwrap());

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
    // print_matrix(&distance_matrix);

    // let (mut queue, best_distance, best_path) = 
    //     bfs(&distance_matrix, distance_matrix.len());
    let (best_path, best_distance) = tsp(&distance_matrix, num_cpus::get());

    println!("Best path: {:?}", best_path);
    if let Some(path) = best_path {
        println!("Best distance: {}", if path.len() < distance_matrix.len() {"-".to_string()} else {best_distance.to_string()});
    }
}

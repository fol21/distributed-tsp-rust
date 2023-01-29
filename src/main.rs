extern crate num_cpus;
pub mod lib;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread::{self};
use std::collections::VecDeque;

use lib::utils::{
    print_path_distance,
    path_sum,
    QUEUE_LIMIT
};

fn bfs(distance_matrix: &Vec<Vec<f64>>, size: usize) -> (VecDeque<Vec<usize>>, f64, Option<Vec<usize>>){
    let node_count = distance_matrix.len();
    let validated_size = std::cmp::max(node_count, size);
    let mut best_path: Option<Vec<usize>> = None;
    let mut best_distance = std::f64::MAX;
    let mut queue: VecDeque<Vec<usize>> = VecDeque::from(vec![vec![]]);

    let mut ready = false;
    while  !queue.is_empty() && !ready {
        let path = queue.pop_front().unwrap();
        if path.len() == node_count {
            let distance = path_sum(&path, &distance_matrix);
            if distance < best_distance {
                best_distance = distance + distance_matrix[path[path.len()-1]][path[0]];
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
                    if queue.len() >= validated_size || queue.len() >= QUEUE_LIMIT {
                        ready = true;
                        break;
                    }
                }
            }
        }
    } 
    (queue, best_distance, best_path)
}

fn tsp(distance_matrix: &Vec<Vec<f64>>, queue_size: usize, num_threads: usize) -> (Option<Vec<usize>>, f64) {
    let node_count = distance_matrix.len();
    let (mut queue, best_distance, best_path) = bfs(distance_matrix, queue_size);
    let mut path_stacks = vec![Vec::<Vec<usize>>::new(); num_threads];

    let mut _c: usize = 0;
    while !queue.is_empty() {
        let path = queue.pop_front().unwrap();
        path_stacks[_c % num_cpus::get()].push(path);
        // print_stack(&stacks[_c % num_cpus::get()]);
        _c += 1;
    }
    
    let distance_rc = Arc::new(Mutex::new(best_distance));
    let path_rc= Arc::new(Mutex::new(best_path));
    let matrix_rc = Arc::new(distance_matrix.clone());
    
    let mut join_handles = vec![];
    let mut _counter = 0;
    for stack in path_stacks.iter() {
        let distance_rc = Arc::clone(&distance_rc);
        let path_rc = Arc::clone(&path_rc);
        let matrix_rc = Arc::clone(&matrix_rc);
        let mut _s = Mutex::new(stack.clone());

        join_handles.push(thread::spawn(move || {
            let _d = matrix_rc;
            while _s.get_mut().unwrap().is_empty() == false {
                let path = _s.get_mut().unwrap().pop().unwrap();
                if path.len() == node_count {
                    let mut distance_rc = distance_rc.lock().unwrap();
                    let distance = path_sum(&path, &*_d);
                    if distance < *distance_rc {
                        println!("Thread {} found new best: {}, stack size: {}", _counter.clone(), distance, _s.get_mut().unwrap().len());
                        *distance_rc = distance;
                        {
                            let mut path_rc = path_rc.lock().unwrap();
                            *path_rc = Some(path);
                        }
                    }   
                } else {
                    for next in 0..node_count {
                        if !path.contains(&next) {
                            let mut new_path = path.clone();
                            new_path.push(next);
                            let d = distance_rc.lock().unwrap();
                            if path_sum(&new_path, &_d) < *d {
                                _s.get_mut().unwrap().push(new_path);
                            }
                        }
                    }
                }
            }
            println!("...Thread {} finished", _counter.clone());
        }));
        _counter += 1;
    }
    for handle in join_handles {
        handle.join().unwrap();
    }
    println!("### All threads finished ###");

    let mut best_path = path_rc.lock().unwrap().clone().unwrap();
    best_path.push(best_path[0]);
    let _ref = &best_path;
    let _back_to_start = distance_matrix[_ref[_ref.len()-2]][_ref[0]];
    let best_distance = *distance_rc.lock().unwrap() + _back_to_start;

    (Some(best_path), best_distance)
}

fn main() {
    let file = File::open("data/datasets/SP11/sp11_d.txt").unwrap();
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
    let (best_path, best_distance) = tsp(&distance_matrix, num_cpus::get(), num_cpus::get());

    print!("Best path: ");
    if let Some(path) = best_path {
        print_path_distance(&path, &distance_matrix);
        println!("Best distance: {}", if path.len() < distance_matrix.len() {"-".to_string()} else {best_distance.to_string()});
    }
}

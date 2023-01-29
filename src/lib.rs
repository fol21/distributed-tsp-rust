pub mod utils {
    pub const QUEUE_LIMIT: usize = 100000;

    pub fn path_sum(path: &[usize], distance_matrix: &[Vec<f64>]) -> f64 {
        path.iter()
            .zip(path.iter().skip(1))
            .map(|(a, b)| distance_matrix[*a][*b])
            .sum()
    }
    
    pub fn print_path(path: &[usize]) {
        for i in 0..path.len() {
            if i != path.len() - 1 {
                print!("{}->", path[i] + 1);
            } else {
                print!("{}", path[i] + 1);
            }
        }
        println!();
    }
    pub fn print_path_distance(path: &[usize], distance_matrix: &[Vec<f64>]) {
        for i in 0..path.len() {
            if i != path.len() - 1 {
                print!("({} |{}|)->", path[i] + 1, path_sum(&path[0..i+1], distance_matrix));
            } else {
                let _sum = path_sum(path, distance_matrix);
                print!("({} |{}|)", path[i] + 1, _sum);
            }
        }
        println!();
    }
    
    pub fn print_stack(stack: &Vec<Vec<usize>>) {
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
    
    pub fn print_matrix(distance_matrix: &[Vec<f64>]) {
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
}
use rand::prelude::*;
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Cell {
    Wall,
    Path,
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

fn generate_maze(width: usize, height: usize) -> Vec<Vec<Cell>> {
    let mut maze = vec![vec![Cell::Wall; width]; height];
    let mut stack = VecDeque::new();
    let mut rng = thread_rng();

    // Random starting point
    let start = Point::new(rng.gen_range(0..width / 2) * 2, rng.gen_range(0..height / 2) * 2);
    maze[start.y][start.x] = Cell::Path;
    stack.push_back(start);

    while let Some(current) = stack.pop_back() {
        let mut neighbors = Vec::new();

        for &(dx, dy) in &[(0, 2), (2, 0), (0, -2), (-2, 0)] {
            let nx = current.x as isize + dx;
            let ny = current.y as isize + dy;

            if nx > 0 && nx < (width as isize - 1) && ny > 0 && ny < (height as isize - 1) {
                let nx = nx as usize;
                let ny = ny as usize;
                if maze[ny][nx] == Cell::Wall {
                    neighbors.push(Point::new(nx, ny));
                }
            }
        }

        if !neighbors.is_empty() {
            stack.push_back(current);

            let chosen = neighbors[rng.gen_range(0..neighbors.len())];
            maze[chosen.y][chosen.x] = Cell::Path;

            // Knock down the wall between
            maze[(chosen.y + current.y) / 2][(chosen.x + current.x) / 2] = Cell::Path;

            stack.push_back(chosen);
        }
    }

    maze
}

fn print_maze(maze: &[Vec<Cell>]) {
    // Top border
    println!("{}", "██".repeat(maze[0].len()));

    for row in maze {
        print!("██"); // Left border
        for &cell in row {
            print!("{}", if cell == Cell::Path { "  " } else { "██" });
        }
        println!("██"); // Right border
    }

    // Bottom border
    println!("{}", "██".repeat(maze[0].len()));
}

fn main() {
    let width = 41; // Should be odd
    let height = 21; // Should be odd
    let maze = generate_maze(width, height);
    print_maze(&maze);
}

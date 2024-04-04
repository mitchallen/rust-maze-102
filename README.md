rust-maze-102
==

I asked ChatGTP to write rust code to generate a maze with a border

## Usage

```sh
make run
```

## ChatGPT conversation


To add a border around the maze, you'll need to adjust the maze generation slightly and modify the `print_maze` function to include printing the border. Here's how you can do it:

- **Generate Maze with Border**: Instead of starting the maze generation with a grid fully initialized as `Wall`, you will ensure that the outermost cells are always walls. However, since the algorithm already treats the outer bounds as walls by not allowing movement beyond them, you just need to adjust the printing function.
- **Print Maze with Border**: Modify the `print_maze` function to print a border around the entire maze. This involves adding a top and bottom border and ensuring that each row of the maze also starts and ends with a border.

Here's the updated Rust program with these modifications:

```rust
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
```

This code will generate a maze with the specified dimensions and print it to the console, surrounded by a solid border. The top and bottom borders are printed before and after the maze rows, and each row starts and ends with a border to complete the enclosure. Adjust the `width` and `height` in the `main` function for different maze sizes, remembering that they should be odd numbers to ensure proper maze generation and border alignment.
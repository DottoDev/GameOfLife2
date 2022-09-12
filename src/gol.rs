use ndarray::prelude::*;
use ndarray::Array;
extern crate crossbeam;
extern crate queues;
extern crate threadpool;
use crossbeam::queue::SegQueue;
use rand::Rng;
use std::sync::Arc;
use threadpool::ThreadPool;

//Size must be dividable by 2
// pub static HEIGHT: usize = 8;
// pub static WIDTH: usize = HEIGHT;
// pub static HEIGHT_OFFSET: isize = (HEIGHT / 2) as isize;
// pub static WIDTH_OFFSET: isize = (WIDTH / 2) as isize;

static POP_MIN: u32 = 2;
static POP_MAX: u32 = 5;

pub static HEIGHT: usize = 399 + 1;
pub static WIDTH: usize = 399 + 1;
pub static NUMBER_THREADS: usize = 32;

// static GOL_MIN_X: usize = 1;
static GOL_MIN_Y: usize = 1;
static GOL_MAX_X: usize = HEIGHT - 2;
static GOL_MAX_Y: usize = WIDTH - 2;

//fn x_position(position: isize) -> usize {
//    (position + WIDTH_OFFSET) as usize
//}

//fn y_position(position: isize) -> usize {
//    (position + HEIGHT_OFFSET) as usize
//}

#[derive(Clone, Debug, Copy)]
pub struct Cell_Action {
    x: usize,
    y: usize,
    new_age: u32,
    new_value: u32,
}

#[derive(Clone, Debug, Copy)]
pub struct Cell {
    value: u32,
    age: u32,
}

#[derive(Clone, Debug)]
pub struct Grid {
    grid: Array<Cell, Ix2>,
    height: usize,
    width: usize,
}

impl Cell {
    pub fn new() -> Self {
        Cell { value: 0, age: 0 }
    }

    pub fn new_with_value(value: u32) -> Self {
        Cell { value, age: 0 }
    }

    pub fn get_value(cell: &Cell) -> u32 {
        cell.value
    }
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            grid: Array::<Cell, Ix2>::from_elem((HEIGHT, WIDTH), Cell { value: 0, age: 1 }),
            height: HEIGHT,
            width: WIDTH,
        }
    }
    pub fn set_value(x: usize, y: usize, value: Cell, grid: &mut Grid) {
        grid.grid
            //.slice_mut(s![x_position(x), y_position(y)])
            .slice_mut(s![x, y])
            .fill(value);
    }

    pub fn get_value(x: usize, y: usize, grid: &Grid) -> Cell {
        //return grid.grid[[x_position(x) as usize, y_position(y) as usize]];
        return grid.grid[[x as usize, y as usize]];
    }
}

pub fn randomize_grid(grid: &mut Grid) {
    let mut rng = rand::thread_rng();
    for x in 2..(WIDTH - 1 - 2) {
        for y in 2..(HEIGHT - 1 - 2) {
            Grid::set_value(x, y, Cell::new_with_value(rng.gen_range(0..=1)), grid)
        }
    }
}

// pub fn print_grid(grid: &mut Grid) {
//     for y in (0 - HEIGHT_OFFSET)..=(0 + HEIGHT_OFFSET) {
//         for x in (0 - WIDTH_OFFSET)..=(0 + WIDTH_OFFSET) {
//             let cell = Grid::get_value(x, y, grid);
//             let value = Cell::get_value(&cell);
//             match value {
//                 1 => {
//                     print!("{}", "0".red());
//                 }
//                 _ => {
//                     print!("{}", "0".blue());
//                 }
//             }
//         }
//         println!("")
//     }
// }
pub fn gol_algorithm_multithreaded(
    x_pos: usize,
    y_pos: usize,
    grid: &Grid,
    action_queue: Arc<SegQueue<Cell_Action>>,
) {
    let mut neighbours_amount: u32 = 0;
    let mut neighbours_value: u32 = 0;
    let mut neighbours_age_sum: u32 = 0;
    let mut neighbours_age_average: u32 = 0;
    let own_cell: Cell = Grid::get_value(x_pos, y_pos, &grid);

    for x in (-2..=2 as isize) {
        for y in -2..=2 as isize {
            if (x, y) != (0, 0) {
                let cell: Cell = Grid::get_value(
                    (x_pos as isize + x) as usize,
                    (y_pos as isize + y) as usize,
                    &grid,
                );
                if cell.value != 0 {
                    neighbours_amount += 1;
                    neighbours_value += cell.value;
                    neighbours_age_sum += cell.age;
                }
            }
        }
    }

    if neighbours_amount != 0 {
        neighbours_age_average = neighbours_age_sum / neighbours_amount;
    }

    if neighbours_amount <= POP_MIN || neighbours_amount > POP_MAX {
        if own_cell.value != 0 {
            action_queue.push(Cell_Action {
                x: x_pos,
                y: y_pos,
                new_age: 0,
                new_value: 0,
            });
        }
    } else {
        if own_cell.value == 0 {
            action_queue.push(Cell_Action {
                x: x_pos,
                y: y_pos,
                new_age: 0,
                new_value: 1,
            });
        }
    }
}

pub fn gol_multithreaded(grid: &Grid, action_queue: Arc<SegQueue<Cell_Action>>) {
    //let iter_x_axis = ((0 - WIDTH_OFFSET + 2)..(0 + WIDTH_OFFSET - 1)).into_iter();
    let iter_x_axis = (2..(WIDTH - 1 - 2)).into_iter();
    //let iter_y_axis = ((0 - HEIGHT_OFFSET + 2)..(0 + HEIGHT_OFFSET - 1)).into_iter();
    let iter_y_axis = (2..(HEIGHT - 1 - 2)).into_iter();

    let pool = ThreadPool::with_name("calculation workers".to_owned(), NUMBER_THREADS);

    for x in iter_x_axis.clone() {
        let t_action_queue = action_queue.clone();
        let t_x = x.clone();
        let t_grid = grid.clone();
        let t_iter_y_axis = iter_y_axis.clone();
        pool.execute(move || {
            for y in t_iter_y_axis {
                let t_t_action_queue = t_action_queue.clone();
                let t_y = y.clone();
                gol_algorithm_multithreaded(t_x, t_y, &t_grid, t_t_action_queue);
            }
        });
    }

    pool.join();
}
pub fn run_gol(action_queue: &Arc<SegQueue<Cell_Action>>, grid: &mut Grid) {
    while !action_queue.is_empty() {
        let action: Cell_Action = action_queue.pop().unwrap();
        Grid::set_value(
            action.x,
            action.y,
            Cell {
                value: action.new_value,
                age: 0,
            },
            grid,
        );
    }
}

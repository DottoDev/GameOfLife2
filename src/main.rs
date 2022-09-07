use crossbeam::queue::SegQueue;
use gameOfLife::gol::Grid;
use plotters::prelude::*;
use progress_bar::*;
use std::io;
use std::process::ExitCode;
use std::sync::Arc;
use std::time::*;
use std::{thread, time};

use crate::gol::Cell;

pub mod gol;
pub mod threads;

fn main() -> ExitCode {
    let mut graph_y_range: usize = 10;
    let mut graph_x_range: usize = 10;
    while gol::HEIGHT > graph_y_range {
        graph_y_range *= 10;
    }

    while gol::WIDTH > graph_x_range {
        graph_x_range *= 10;
    }

    graph_x_range = 801;
    graph_y_range = 801;

    let drawing_area = BitMapBackend::gif("/home/s34m/Pictures/gol.gif", (1920, 1080), 2_000)
        .unwrap()
        .into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&drawing_area)
        .build_cartesian_2d(0..graph_x_range, 0..graph_y_range)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    let mut main_grid: gol::Grid = gol::Grid::new();
    println!("Initialized");

    loop {
        println!("Please input: ");
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let input: isize = line.trim().parse().expect("Wanted a number");

        match input {
            1 => {
                let mut line = String::new();
                println!("Input x axis");
                io::stdin()
                    .read_line(&mut line)
                    .expect("Failed to read line");
                let x: isize = line.trim().parse().expect("Wanted a number");

                line = "".to_string();
                println!("Input y axis");
                io::stdin()
                    .read_line(&mut line)
                    .expect("Failed to read line");
                let y: isize = line.trim().parse().expect("Wanted a number");

                line = "".to_string();
                println!("Input value");
                io::stdin()
                    .read_line(&mut line)
                    .expect("Failed to read line");
                let value: u32 = line.trim().parse().expect("Wanted a number");

                gol::Grid::set_value(x, y, Cell::new_with_value(value), &mut main_grid)
            }

            2 => {
                let mut living_cells: Vec<(usize, usize)> = Vec::new();

                for y in (0 - gol::HEIGHT_OFFSET)..(0 + gol::HEIGHT_OFFSET) {
                    for x in (0 - gol::WIDTH_OFFSET)..(0 + gol::WIDTH_OFFSET) {
                        let cell = gol::Grid::get_value(x as isize, y as isize, &main_grid);
                        let value = gol::Cell::get_value(&cell);
                        match value {
                            1 => {
                                let cooridnates = (
                                    (x + gol::WIDTH_OFFSET) as usize,
                                    (y + gol::HEIGHT_OFFSET) as usize,
                                );

                                living_cells.push(cooridnates);
                            }
                            _ => {}
                        }
                    }
                }

                chart
                    .draw_series(
                        living_cells
                            .iter()
                            .map(|point| Circle::new(*point, 5, &BLUE)),
                    )
                    .unwrap();
                drawing_area.present().unwrap();

                gol::print_grid(&mut main_grid);
            }

            3 => {
                gol::randomize_grid(&mut main_grid);
            }

            4 => {
                let action_queue = Arc::new(SegQueue::<gol::Cell_Action>::new());
                println!("Input amount of runs");
                line = "".to_string();
                io::stdin()
                    .read_line(&mut line)
                    .expect("Failed to read line");
                let amount: usize = line.trim().parse().expect("Wanted a number");

                init_progress_bar(amount);
                set_progress_bar_action(
                    "Loading",
                    progress_bar::Color::Blue,
                    progress_bar::Style::Bold,
                );

                let now = Instant::now();
                for _ in 0..amount {
                    gol::gol_multithreaded(&main_grid, action_queue.clone());
                    gol::run_gol(&action_queue, &mut main_grid);
                    inc_progress_bar();
                }

                finalize_progress_bar();

                let elapsed_time = now.elapsed();
                println!(
                    "Running function took {} seconds, doing an average of {} cycles per second",
                    elapsed_time.as_secs(),
                    amount / (elapsed_time.as_secs() as usize)
                );

                println!("Done");
            }

            5 => {
                let action_queue = Arc::new(SegQueue::<gol::Cell_Action>::new());

                let sleep_time = time::Duration::from_millis(500);
                println!("Input amount of runs");
                line = "".to_string();
                io::stdin()
                    .read_line(&mut line)
                    .expect("Failed to read line");
                let amount: u32 = line.trim().parse().expect("Wanted a number");
                let now = Instant::now();
                for _ in 0..amount {
                    gol::gol_multithreaded(&main_grid, action_queue.clone());
                    gol::run_gol(&action_queue, &mut main_grid);

                    gol::print_grid(&mut main_grid);
                    thread::sleep(sleep_time);
                }

                let elapsed_time = now.elapsed();
                println!("Running function took {} seconds.", elapsed_time.as_secs());

                println!("Done");
            }

            6 => {
                let action_queue = Arc::new(SegQueue::<gol::Cell_Action>::new());
                println!("Input amount of runs");
                line = "".to_string();
                io::stdin()
                    .read_line(&mut line)
                    .expect("Failed to read line");
                let amount: usize = line.trim().parse().expect("Wanted a number");

                init_progress_bar(amount);
                set_progress_bar_action(
                    "Loading",
                    progress_bar::Color::Blue,
                    progress_bar::Style::Bold,
                );

                let now = Instant::now();
                for _ in 0..amount {
                    gol::gol_multithreaded(&main_grid, action_queue.clone());
                    gol::run_gol(&action_queue, &mut main_grid);

                    {
                        drawing_area.fill(&WHITE).unwrap();
                        let mut living_cells: Vec<(usize, usize)> = Vec::new();

                        for y in (0 - gol::HEIGHT_OFFSET)..(0 + gol::HEIGHT_OFFSET) {
                            for x in (0 - gol::WIDTH_OFFSET)..(0 + gol::WIDTH_OFFSET) {
                                let cell = gol::Grid::get_value(x as isize, y as isize, &main_grid);
                                let value = gol::Cell::get_value(&cell);
                                match value {
                                    1 => {
                                        let cooridnates = (
                                            (x + gol::WIDTH_OFFSET) as usize,
                                            (y + gol::HEIGHT_OFFSET) as usize,
                                        );

                                        living_cells.push(cooridnates);
                                    }
                                    _ => {}
                                }
                            }
                        }

                        chart
                            .draw_series(
                                living_cells
                                    .iter()
                                    .map(|point| Circle::new(*point, 1, &BLUE)),
                            )
                            .unwrap();
                        drawing_area.present().unwrap();
                    }
                    inc_progress_bar();
                }

                finalize_progress_bar();

                let elapsed_time = now.elapsed();
                println!(
                    "Running function took {} seconds, doing an average of {} cycles per second",
                    elapsed_time.as_secs(),
                    amount / (elapsed_time.as_secs() as usize)
                );

                println!("Done");
            }
            9 => {
                println!("Shutting down");
                return ExitCode::SUCCESS;
            }

            _ => {}
        }
    }
}

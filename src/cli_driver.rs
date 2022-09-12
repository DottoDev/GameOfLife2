use crossbeam::queue::SegQueue;
use csv::Writer;
use plotters::coord::types::RangedCoordusize;
use plotters::coord::Shift;
use plotters::prelude::*;
use progress_bar::*;
use std::io;
use std::sync::Arc;
use std::time::*;
extern crate pbr;
use pbr::ProgressBar;
use std::fs::File;

use crate::gol;

pub fn print(
    chart: &mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordusize, RangedCoordusize>>,
    main_grid: &mut gol::Grid,
    drawing_area: &DrawingArea<BitMapBackend, Shift>,
    wtr: &mut Writer<File>,
) {
    let mut living_cells: Vec<(usize, usize)> = Vec::new();

    for x in 2..(gol::WIDTH - 1 - 2) {
        for y in 2..(gol::HEIGHT - 1 - 2) {
            let cell = gol::Grid::get_value(x, y, &main_grid);
            let value = gol::Cell::get_value(&cell);
            match value {
                1 => {
                    let cooridnates = (x, y);

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
                .map(|point| Pixel::new(*point, crate::COLOR_FOREGROUND)),
        )
        .unwrap();
    drawing_area.present().unwrap();
}

pub fn run_without_print(main_grid: &mut gol::Grid) {
    let action_queue = Arc::new(SegQueue::<gol::Cell_Action>::new());
    println!("Input amount of runs");
    let mut line = "".to_string();
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
        gol::run_gol(&action_queue, main_grid);
        inc_progress_bar();
    }

    finalize_progress_bar();

    //let elapsed_time = now.elapsed();
    // println!(
    //     "Running function took {} seconds, doing an average of {} cycles per second",
    //     elapsed_time.as_secs(),
    //     amount / (elapsed_time.as_secs() as usize);

    println!("Done");
}

pub fn run_with_print(
    chart: &mut ChartContext<BitMapBackend, Cartesian2d<RangedCoordusize, RangedCoordusize>>,
    main_grid: &mut gol::Grid,
    drawing_area: &DrawingArea<BitMapBackend, Shift>,
) {
    let action_queue = Arc::new(SegQueue::<gol::Cell_Action>::new());
    println!("Input amount of runs");
    let mut line = "".to_string();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let amount: usize = line.trim().parse().expect("Wanted a number");

    let mut pb = ProgressBar::new(amount as u64);
    pb.format("╢▌▌░╟");

    // init_progress_bar(amount);
    // set_progress_bar_action(
    //     "Loading",
    //     progress_bar::Color::Blue,
    //     progress_bar::Style::Bold,
    // );

    let now = Instant::now();
    for _ in 0..amount {
        gol::gol_multithreaded(&main_grid, action_queue.clone());
        gol::run_gol(&action_queue, main_grid);

        {
            drawing_area.fill(crate::COLOR_BACKGROUND).unwrap();
            let mut living_cells: Vec<(usize, usize)> = Vec::new();

            for x in 2..(gol::WIDTH - 1 - 3) {
                for y in 2..(gol::HEIGHT - 1 - 3) {
                    let cell = gol::Grid::get_value(x, y, &main_grid);
                    let value = gol::Cell::get_value(&cell);
                    match value {
                        1 => {
                            let cooridnates = (x, y);

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
                        .map(|point| Pixel::new(*point, crate::COLOR_FOREGROUND)),
                )
                .unwrap();
            drawing_area.present().unwrap();
        }
        pb.inc();
    }

    pb.finish_println("Done\n");

    let elapsed_time = now.elapsed();
    println!("Running function took {} seconds", elapsed_time.as_secs());

    println!("Done");
}

pub fn run_x_times(main_grid: &mut gol::Grid, amount: usize) {
    let action_queue = Arc::new(SegQueue::<gol::Cell_Action>::new());

    for _ in 0..amount {
        gol::gol_multithreaded(&main_grid, action_queue.clone());
        gol::run_gol(&action_queue, main_grid);
    }
}

pub fn get_living_cells(main_grid: &gol::Grid) -> Vec<(usize, usize)> {
    let action_queue = Arc::new(SegQueue::<gol::Cell_Action>::new());
    gol::gol_multithreaded(&main_grid, action_queue.clone());

    let mut living_cells: Vec<(usize, usize)> = Vec::new();

    for x in 2..(gol::WIDTH - 1 - 3) {
        for y in 2..(gol::HEIGHT - 1 - 3) {
            let cell = gol::Grid::get_value(x, y, &main_grid);
            let value = gol::Cell::get_value(&cell);
            match value {
                1 => {
                    let cooridnates = (x, y);

                    living_cells.push(cooridnates);
                }
                _ => {}
            }
        }
    }
    return living_cells;
}
pub fn get_living_cells_as_f64(main_grid: &gol::Grid) -> Vec<(f64, f64)> {
    let action_queue = Arc::new(SegQueue::<gol::Cell_Action>::new());
    gol::gol_multithreaded(&main_grid, action_queue.clone());

    let mut living_cells: Vec<(f64, f64)> = Vec::new();

    for x in 2..(gol::WIDTH - 1 - 3) {
        for y in 2..(gol::HEIGHT - 1 - 3) {
            let cell = gol::Grid::get_value(x, y, &main_grid);
            let value = gol::Cell::get_value(&cell);
            match value {
                1 => {
                    let cooridnates = (x as f64, y as f64);

                    living_cells.push(cooridnates);
                }
                _ => {}
            }
        }
    }
    return living_cells;
}

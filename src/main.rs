use plotters::prelude::*;
use std::io;
use std::process::ExitCode;
extern crate pbr;

pub mod cli_driver;
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

    graph_x_range = 200;
    graph_y_range = 200;

    let drawing_area = BitMapBackend::gif("/home/s34m/Pictures/gol.gif", (1920, 1080), 2_000)
        .unwrap()
        .into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&drawing_area)
        .build_cartesian_2d(0..graph_x_range, 0..graph_y_range)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    let mut main_grid: gol::Grid = gol::Grid::new();
    println!("Initialized \n");

    loop {
        println!("1: Print");
        println!("2: Randomize");
        println!("3: Run without printing");
        println!("4: Run with printing\n");

        println!("Please input: ");
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let input: isize = line.trim().parse().expect("Wanted a number");

        match input {
            1 => {
                cli_driver::print(&mut chart, &mut main_grid, &drawing_area);
            }

            2 => {
                gol::randomize_grid(&mut main_grid);
            }

            3 => {
                cli_driver::run_without_print(&mut main_grid);
            }

            4 => {
                cli_driver::run_with_print(&mut chart, &mut main_grid, &drawing_area);
            }
            9 => {
                println!("Shutting down");
                return ExitCode::SUCCESS;
            }

            _ => {}
        }
    }
}

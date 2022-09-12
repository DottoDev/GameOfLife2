use csv::Writer;
use plotters::prelude::*;
use std::process::ExitCode;
extern crate pbr;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::thread::sleep;
use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::Style,
    symbols,
    text::Span,
    widgets::{Axis, Block, Borders, Chart, Dataset, Widget},
    Terminal,
};

pub mod cli_driver;
pub mod gol;
pub mod threads;

pub static COLOR_FOREGROUND: &RGBColor = &GREEN;
pub static COLOR_BACKGROUND: &RGBColor = &BLACK;

fn main() -> Result<(), io::Error> {
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

    let mut wtr = Writer::from_path("data.csv").unwrap();
    wtr.write_record(&["x_axis", "y_axis", "value"]).unwrap();

    let drawing_area = BitMapBackend::gif("/home/s34m/Pictures/gol.gif", (1920, 1080), 2_000)
        .unwrap()
        .into_drawing_area();
    drawing_area.fill(&BLACK).unwrap();

    let mut chart = ChartBuilder::on(&drawing_area)
        .build_cartesian_2d(0..graph_x_range, 0..graph_y_range)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    let mut main_grid: gol::Grid = gol::Grid::new();

    gol::randomize_grid(&mut main_grid);
    cli_driver::run_x_times(&mut main_grid, 1000);

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        cli_driver::run_x_times(&mut main_grid, 1);
        let data = cli_driver::get_living_cells_as_f64(&main_grid);
        let dataset = vec![Dataset::default()
            .name("data")
            .marker(symbols::Marker::Dot)
            .graph_type(tui::widgets::GraphType::Scatter)
            .style(Style::default().fg(tui::style::Color::Green))
            .data(&data)];

        terminal.draw(|f| {
            let size = f.size();
            let chart = Chart::new(dataset)
                .x_axis(
                    Axis::default()
                        .title(Span::styled(
                            "X Axis",
                            Style::default().fg(tui::style::Color::Red),
                        ))
                        .style(Style::default().fg(tui::style::Color::White))
                        .bounds([0.0, 400.0])
                        .labels(
                            ["0.0", "1.0", "400.0"]
                                .iter()
                                .cloned()
                                .map(Span::from)
                                .collect(),
                        ),
                )
                .y_axis(
                    Axis::default()
                        .title(Span::styled(
                            "Y Axis",
                            Style::default().fg(tui::style::Color::Red),
                        ))
                        .style(Style::default().fg(tui::style::Color::White))
                        .bounds([0.0, 400.0])
                        .labels(
                            ["0.0", "1.0", "400.0"]
                                .iter()
                                .cloned()
                                .map(Span::from)
                                .collect(),
                        ),
                );
            f.render_widget(chart, size);
        });
        sleep(Duration::from_secs_f64(0.5));
    }
    // println!("Initialized \n");
    // loop {
    //     println!("1: Print");
    //     println!("2: Randomize");
    //     println!("3: Run without printing");
    //     println!("4: Run with printing\n");

    //     println!("Please input: ");
    //     let mut line = String::new();
    //     io::stdin()
    //         .read_line(&mut line)
    //         .expect("Failed to read line");
    //     let input: isize = line.trim().parse().expect("Wanted a number");

    // match input {
    //     1 => {
    //         cli_driver::print(&mut chart, &mut main_grid, &drawing_area, &mut wtr);
    //     }

    //     2 => {
    //         gol::randomize_grid(&mut main_grid);
    //     }

    //     3 => {
    //         cli_driver::run_without_print(&mut main_grid);
    //     }

    //     4 => {
    //         cli_driver::run_with_print(&mut chart, &mut main_grid, &drawing_area);
    //     }
    //     5 => {
    //         for x in 2..(gol::WIDTH - 1 - 3) {
    //             for y in 2..(gol::HEIGHT - 1 - 3) {
    //                 let cell = gol::Grid::get_value(x, y, &main_grid);
    //                 let value = gol::Cell::get_value(&cell);
    //                 match value {
    //                     1 => {
    //                         wtr.write_record(&[
    //                             x.to_string(),
    //                             y.to_sggtring(),
    //                             value.to_string(),
    //                         ])
    //                         .unwrap();
    //                     }
    //                     _ => {}
    //                 }
    //             }
    //         }
    //     }

    //     9 => {
    //         wtr.flush().unwrap();
    //         println!("Shutting down");
    //         return ExitCode::SUCCESS;
    //     }

    //     _ => {}
    // }
    //}
    //
    thread::sleep(Duration::from_secs(10));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

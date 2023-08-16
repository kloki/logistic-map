use clap::Parser;
use plotters::prelude::*;
use rand::{thread_rng, Rng};
use std::error::Error;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Start value x-axis
    start_x: f64,
    /// End value x-axis
    end_x: f64,
    /// Start value x-axis
    start_y: f64,
    /// End value y-axis
    end_y: f64,
    #[arg(short, long, default_value = "./output.png")]
    output_file: String,
    #[arg(long, default_value_t = 1000)]
    width: u32,
    #[arg(long, default_value_t = 500)]
    height: u32,
}
fn main() {
    let args = Args::parse();
    graph(
        args.start_x,
        args.end_x,
        args.start_y,
        args.end_y,
        args.width,
        args.height,
        args.output_file,
    )
    .unwrap();
}

fn run(r: f64, init: f64, iterations: usize) -> Vec<(f64, f64)> {
    let mut result = vec![(r, init); iterations];
    for i in 1..iterations {
        result[i].1 = r * result[i - 1].1 * (1f64 - result[i - 1].1);
    }
    result
}

fn get_data_set(start: f64, end: f64, width: u32) -> Vec<(f64, f64)> {
    let mut data_set: Vec<(f64, f64)> = vec![];
    let step = (end - start) / width as f64;
    let mut r = start - step;
    let mut rng = thread_rng();
    for _ in 0..width {
        r = r + step;
        if r < 0.0 || r > 4.0 {
            break;
        } else if r < 1.0 {
            data_set.push((r, 0.))
        } else if r < 2.0 {
            let new_run = run(r, rng.gen(), 200);
            data_set.push(*new_run.last().unwrap());
        } else if r < 3.0 {
            let mut new_run = run(r, rng.gen(), 400);
            data_set.append(&mut new_run.drain(397..).collect::<Vec<_>>());
        } else {
            let mut new_run = run(r, rng.gen(), 800);
            data_set.append(&mut new_run.drain(500..).collect::<Vec<_>>());
        }
    }

    data_set
}

fn graph(
    start: f64,
    end: f64,
    start_y: f64,
    end_y: f64,
    width: u32,
    height: u32,
    file_name: String,
) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(&file_name, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Bifucation diagram", ("sans-serif", 20))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .set_label_area_size(LabelAreaPosition::Right, 10)
        .build_cartesian_2d(start..end, start_y..end_y)?;

    chart.configure_mesh().disable_mesh().x_desc("r").draw()?;

    let data_set = get_data_set(start, end, width);

    chart
        .draw_series(
            data_set
                .iter()
                .map(|point| Pixel::new(*point, Into::<ShapeStyle>::into(&BLACK).filled())),
        )
        .unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::graph;

    #[test]
    fn test_basic() {
        graph(0.0, 4.0, 0.0, 1.0, 100, 50, "test.png".to_string()).unwrap();
    }
}

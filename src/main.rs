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
    /// Start value y-axis
    start_y: f64,
    /// End value y-axis
    end_y: f64,
    #[arg(long, default_value_t = 500)]
    /// Determines the amount of data points rendered. Play with this on high zoom levels.
    density: usize,
    #[arg(short, long, default_value = "./output.png")]
    /// Name of png file
    output_file: String,
    #[arg(long, default_value_t = 1000)]
    /// Width of png file in pixels.
    width: u32,
    /// Height of png file in pixels.
    #[arg(long, default_value_t = 800)]
    height: u32,
}
fn main() {
    let args = Args::parse();
    graph(
        args.start_x,
        args.end_x,
        args.start_y,
        args.end_y,
        args.density,
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

fn get_data_set(start: f64, end: f64, steps: u32, density: usize) -> Vec<(f64, f64)> {
    let mut data_set: Vec<(f64, f64)> = vec![];
    let step = (end - start) / steps as f64;
    let mut r = start - step;
    let mut rng = thread_rng();
    for _ in 0..steps {
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
            data_set.append(&mut new_run.drain(398..).collect::<Vec<_>>());
        } else {
            let mut new_run = run(r, rng.gen(), density * 2);
            data_set.append(&mut new_run.drain(density..).collect::<Vec<_>>());
        }
    }

    data_set
}

fn graph(
    start: f64,
    end: f64,
    start_y: f64,
    end_y: f64,
    density: usize,
    width: u32,
    height: u32,
    file_name: String,
) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(&file_name, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Logistic map", ("sans-serif", 20))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .set_label_area_size(LabelAreaPosition::Right, 10)
        .build_cartesian_2d(start..end, start_y..end_y)?;

    chart.configure_mesh().disable_mesh().x_desc("r").draw()?;

    //base the amount of steps on the image width
    let data_set = get_data_set(start, end, width, density);

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
        graph(0.0, 4.0, 0.0, 1.0, 500, 100, 50, "test.png".to_string()).unwrap();
    }
}

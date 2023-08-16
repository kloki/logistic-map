use clap::Parser;
use plotters::prelude::*;
use std::error::Error;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Start value
    start: f64,
    /// End value{
    end: f64,
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
        args.start,
        args.end,
        args.width,
        args.height,
        args.output_file,
    )
    .unwrap();
}

fn graph(
    start: f64,
    end: f64,
    width: u32,
    height: u32,
    file_name: String,
) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(&file_name, (width, height)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Bifucation diagram", ("sans-serif", 20))
        .set_label_area_size(LabelAreaPosition::Left, 100)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .set_label_area_size(LabelAreaPosition::Right, 100)
        .build_cartesian_2d((start as isize)..(end as isize), 0..1)?;

    chart.configure_mesh().x_desc("r").draw()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::graph;

    #[test]
    fn test_basic() {
        graph(1., 3., 1000, 500, "test.png".to_string()).unwrap();
    }
}

use std::process::Command;
use std::process::CommandArgs;

use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;
use plotters::prelude::*;

const COUNT: u64 = 1000;

fn main() -> anyhow::Result<()> {
    let values = (0..COUNT)
        .into_iter()
        .map(|i| {
            if i % 100 == 0 {
                println!("progress: {}", i as f64 / COUNT as f64);
            }

            let bytes = Command::new("hexdump")
                .args(["-v", "-e", r#"/1 "%02x""#, "-n", "64", "/dev/urandom"])
                .output()
                .expect("failed to execute process")
                .stdout;

            let integer = BigUint::from_bytes_le(&bytes);
            integer.to_f64().expect("Failed to convert to f64")
        })
        .collect::<Vec<_>>();

    let root = BitMapBackend::new("target/plot.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .caption("BigInt Plot", ("sans-serif", 50))
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..values.len(), min..max)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        values.into_iter().enumerate().map(|(idx, val)| (idx, val)),
        &RED,
    ))?;

    root.present()?;
    Ok(())
}

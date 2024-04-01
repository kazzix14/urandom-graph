use std::collections::HashMap;
use std::process::Command;
use std::process::CommandArgs;

use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;
use num_traits::FromPrimitive;
use plotters::prelude::*;

const COUNT: u64 = 100000;

fn main() -> anyhow::Result<()> {
    let bytes_list = (0..COUNT)
        .into_iter()
        .map(|i| {
            if i % 100 == 0 {
                println!("progress: {}", i as f64 / COUNT as f64);
            }

            let out = Command::new("hexdump")
                .args(["-v", "-e", r#"/1 " %02x""#, "-n", "64", "/dev/urandom"])
                .output()
                .expect("failed to execute process")
                .stdout;

            let hex_list = String::from_utf8_lossy(&out).to_string();
            let bytes = hex_list
                .split_whitespace()
                .filter_map(|s| u8::from_str_radix(s, 16).ok())
                .collect::<Vec<_>>();

            //let integer = BigUint::from_bytes_le(&bytes);
            //integer.to_f64().expect("Failed to convert to f64")
            bytes
        })
        .collect::<Vec<_>>();

    let frequencies = count_byte_frequencies(&bytes_list);
    let _ = plot_byte_distribution(frequencies);

    Ok(())
}

fn count_byte_frequencies(bytes: &[Vec<u8>]) -> HashMap<u8, usize> {
    let mut frequencies = HashMap::new();
    for byte in bytes {
        let key = remap(byte);
        *frequencies.entry(key).or_insert(0usize) += 1usize;
    }
    frequencies
}

fn remap(value: &[u8]) -> u8 {
    let a = ((BigUint::from_bytes_le(&value) * BigUint::from_u32(u8::MAX as u32).unwrap())
        / BigUint::from_bytes_le(&[255; 64]));

    let b = a.to_f32().unwrap();
    b.round() as u8
}

fn plot_byte_distribution(
    frequencies: HashMap<u8, usize>,
) -> Result<(), Box<dyn std::error::Error>> {
    let root_area =
        BitMapBackend::new("target/byte_distribution.png", (640, 480)).into_drawing_area();
    root_area.fill(&WHITE)?;
    let max_frequency = frequencies.values().copied().max().unwrap_or(0) as i64;

    let mut chart = ChartBuilder::on(&root_area)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Byte Distribution", ("sans-serif", 40))
        .build_cartesian_2d(0usize..(u8::MAX as usize), 0..(max_frequency as usize))?;

    chart.configure_mesh().draw()?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.filled())
            .data(frequencies.into_iter().map(|(value, freq)| (value, freq))),
    )?;

    Ok(())
}

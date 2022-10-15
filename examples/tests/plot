use std::fs;

use plotters::prelude::*;

const OUT_FILE_NAME: &'static str = "plotters-doc-data/histogram_new.png";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents =
        fs::read_to_string("Data/height.txt").expect("Should have been able to read the file");

    let lines: Vec<String> = contents.split("\n").map(|s: &str| s.to_string()).collect();

    let mut contents: Vec<f64> = Vec::new();

    for line in lines {
        let heights: Vec<String> = line
            .split_whitespace()
            .map(|s: &str| s.to_string())
            .collect();

        for height in heights {
            contents.push(height.trim().parse::<f64>().unwrap())
        }
    }

    /*
    println!(
        "Mean: {}, Deviation: {} len: {}",
        mean(&contents),
        deviation(&contents),
        contents.len()
    );
    */

    println!(
        "Mean: {}, Deviation: {} len: {}",
        mean(&contents),
        deviation(&contents),
        contents.len()
    );

    contents = rMean(contents);

    contents = einheitvariant(contents);

    println!(
        "Mean: {}, Deviation: {} len: {}",
        mean(&contents),
        deviation(&contents),
        contents.len()
    );

    let root = BitMapBackend::new(OUT_FILE_NAME, (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Height Distribution", ("sans-serif", 50.0))
        .build_cartesian_2d(0..(contents.len() - 1), 0.0..1500.0)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(&WHITE.mix(0.3))
        .y_desc("Count")
        .x_desc("Height in cm")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    chart.draw_series(
        AreaSeries::new(
            (0..).zip(data.iter()).map(|(x, y)| (x, *y)),
            0.0,
            &RED.mix(0.2),
        )
        .border_style(&RED),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}

fn mean(v: &Vec<f64>) -> f64 {
    let nnumbers = v.len() as f64;
    let mut sum = 0.0;

    for n in v {
        sum += *n;
    }

    sum / nnumbers
}

fn deviation(v: &Vec<f64>) -> f64 {
    let nnumbers = (v.len() - 1) as f64;
    let mut sum = 0.0;
    let avg = mean(v);

    for n in v {
        sum += (*n - avg).powi(2);
    }

    (sum / nnumbers).sqrt()
}

fn rMean(v: Vec<f64>) -> Vec<f64> {
    let mean = mean(&v);
    let mut new: Vec<f64> = Vec::new();

    for n in v {
        new.push(n - mean);
    }

    new
}

fn einheitvariant(v: Vec<f64>) -> Vec<f64> {
    let sd = deviation(&v);
    let mut new: Vec<f64> = Vec::new();

    for n in v {
        new.push(n / sd);
    }

    new
}

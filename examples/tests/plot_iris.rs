use plotters::prelude::*;
use std::fs;

const OUT_FILE_NAME: &'static str = "plotters-doc-data/iris.svg";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents =
        fs::read_to_string("Data/iris.txt").expect("Should have been able to read the file");

    let lines: Vec<String> = contents.split("\n").map(|s: &str| s.to_string()).collect();

    let mut contents_setosa: Vec<(f64, f64, f64)> = Vec::new();
    let mut contents_virginica: Vec<(f64, f64, f64)> = Vec::new();
    let mut contents_versicolor: Vec<(f64, f64, f64)> = Vec::new();

    for line in lines {
        let contents: Vec<String> = line.split(",").map(|s: &str| s.to_string()).collect();

        let dimensions: (f64, f64, f64) = (
            contents[0].parse::<f64>().unwrap(),
            contents[1].parse::<f64>().unwrap(),
            contents[2].parse::<f64>().unwrap(),
        );
        let flower = contents[4].as_str().trim();

        match flower {
            "setosa" => {
                contents_setosa.push(dimensions);
            }
            "versicolor" => {
                contents_versicolor.push(dimensions);
            }
            "virginica" => {
                contents_virginica.push(dimensions);
            }
            _ => {
                println!("Error");
            }
        };
    }

    let area = SVGBackend::new(OUT_FILE_NAME, (1024, 760)).into_drawing_area();

    area.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&area)
        .caption("Scatter Demo", ("sans-serif", 40))
        .build_cartesian_3d(
            (4.0..8.0).step(0.1),
            (2.0..4.3).step(0.1),
            (1.0..6.5).step(0.1),
        )?;

    chart.with_projection(|mut pb| {
        pb.pitch = 0.7;
        pb.yaw = 1.0;
        pb.scale = 0.7;
        pb.into_matrix()
    });

    chart
        .configure_axes()
        .light_grid_style(BLACK.mix(0.15))
        .max_light_lines(3)
        .draw()?;

    chart
        .draw_series(
            contents_setosa
                .iter()
                .map(|point| TriangleMarker::new(*point, 5, &BLUE)),
        )
        .unwrap();

    chart
        .draw_series(
            contents_versicolor
                .iter()
                .map(|point| Circle::new(*point, 5, &RED)),
        )
        .unwrap();

    chart
        .draw_series(
            contents_virginica
                .iter()
                .map(|point| Cross::new(*point, 5, &GREEN)),
        )
        .unwrap();

    area.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);
    Ok(())
}

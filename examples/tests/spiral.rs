use plotters::prelude::*;
use neuralnet::data::*;

const OUT_FILE_NAME: &'static str = "plotters-doc-data/spiral.png";
fn main() {
    
    let data = spiral(30,3);

    let DATA1 = data.0[0..29].to_vec();
    let DATA2 = data.0[30..59].to_vec();
    let DATA3 = data.0[60..89].to_vec();

    let root_area = BitMapBackend::new(OUT_FILE_NAME, (600, 400))
    .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Scatter Demo", ("sans-serif", 40))
        .build_cartesian_2d(-0.25..0.25, -0.25..0.25)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(DATA1.iter().map(|point| Circle::new(*point, 5, &BLUE)))
    .unwrap();

    ctx.draw_series(DATA2.iter().map(|point| Circle::new(*point, 5, &RED)))
        .unwrap();

    ctx.draw_series(DATA3.iter().map(|point| Circle::new(*point, 5, &GREEN)))
        .unwrap();
}


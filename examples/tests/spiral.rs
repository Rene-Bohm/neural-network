use plotters::prelude::*;
use neuralnet::data::*;

const OUT_FILE_NAME: &'static str = "plotters-doc-data/spiral.png";
fn main() {

    let samples = 200 as usize;

    let classes = 3 as usize;
    
    let data = spiral(samples,classes);

    let DATA1 = get_n(&data.0, 0, samples-1);
    //let DATA1:Vec<(f64,f64)> = DATA1.iter().map(|x| ((x.0)*2.0,(x.0)*10.0) ).collect();

    println!("{:?}", DATA1);
    println!("");

    let DATA2 = get_n(&data.0, samples, 2*samples-1);
    //let DATA2:Vec<(f64,f64)> = DATA2.iter().map(|x| ((x.0)*20.0,(x.0)*10.0) ).collect();
    println!("{:?}", DATA2);
    println!("");
    
    let DATA3 = get_n(&data.0, 2*samples, 3*samples-1);
    //let DATA3:Vec<(f64,f64)> = DATA3.iter().map(|x| ((x.0)*2.0,(x.0)*10.0) ).collect();
    println!("{:?}", DATA3);
    println!("");

    let root_area = BitMapBackend::new(OUT_FILE_NAME, (600, 400))
    .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Scatter Demo", ("sans-serif", 40))
        .build_cartesian_2d(-1.0..1.0, -1.0..1.0)
        .unwrap();


    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(DATA1.iter().map(|point| Circle::new(*point, 2, &BLUE)))
    .unwrap();

    ctx.draw_series(DATA2.iter().map(|point| Circle::new(*point, 2, &RED)))
        .unwrap();

    ctx.draw_series(DATA3.iter().map(|point| Circle::new(*point, 2, &GREEN)))
        .unwrap();
}


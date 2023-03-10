use plotters::prelude::*;

pub fn plot_simulation() {
    let area = Area::new(10, 10.0, true);

    let lenght = (area.particles.len() * area.particles[0].len()) as f64;

    const OUT_FILE_NAME: &str = "output.png";
    let root = BitMapBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();
    root.fill(&WHITE).expect("Panic!!");

    let mut chart = ChartBuilder::on(&root)
        .caption("Pepe :)", ("serif", 50).into_font())
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(0..100, 0..100)
        .expect("Panicked!!");

    chart.configure_mesh().draw().unwrap();

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);
}

use std::collections::HashSet;

use plotters::prelude::*;

use crate::cell_index_method::CellIndexMethod;

pub fn plot_cell_index_method(cell_index_method: &CellIndexMethod, neighbors: &Vec<HashSet<u32>>) {
    const OUT_FILE_NAME: &str = "output.png";
    let root = BitMapBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();
    root.fill(&WHITE).expect("Panic!!");

    let mut chart = ChartBuilder::on(&root)
        .caption("Pepe :)", ("serif", 50).into_font())
        .margin(15)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(
            0..(cell_index_method.get_length() * 1.1) as u32,
            0..cell_index_method.get_length() as u32,
        )
        .expect("Panicked!!");

    chart
        .configure_mesh()
        .disable_mesh()
        .x_desc("X")
        .y_desc("Y")
        .x_label_formatter(&|x| format!("{:.1}", x))
        .y_label_formatter(&|y| format!("{:.1}", y))
        .light_line_style(&WHITE.mix(0.3))
        .draw()
        .expect("Panic!!");

    // Configure cell size
    let mesh_interval = cell_index_method.get_length() / cell_index_method.get_m() as f64;
    let mesh_interval = mesh_interval as usize;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);
}

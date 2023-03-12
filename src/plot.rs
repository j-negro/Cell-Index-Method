use plotters::prelude::*;
use plotters::series::LineSeries;

use crate::neighbors::{cell_index_method::CellIndexMethod, ParticleNeighbors};

pub fn plot_cell_index_method(cell_index_method: &CellIndexMethod, neighbors: &ParticleNeighbors) {
    const OUT_FILE_NAME: &str = "output.png";
    let drawing_area = BitMapBackend::new(OUT_FILE_NAME, (1024, 768)).into_drawing_area();
    drawing_area.fill(&WHITE).expect("Panic!!");

    let mut chart_context = ChartBuilder::on(&drawing_area)
        .caption("Pepe :)", ("serif", 50).into_font())
        .margin(10)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(
            0f64..cell_index_method.get_length() as f64,
            0f64..cell_index_method.get_length() as f64,
        )
        .expect("Panicked!!");

    chart_context
        .configure_mesh()
        .disable_mesh()
        .x_desc("X")
        .y_desc("Y")
        .x_label_formatter(&|x| format!("{:.1}", x))
        .y_label_formatter(&|y| format!("{:.1}", y))
        .draw()
        .expect("Panic!!");

    // Configure cell size
    let mesh_interval = cell_index_method.get_length() / cell_index_method.get_m() as f64;

    // Draw cells
    for i in 0..cell_index_method.get_m() {
        for j in 0..cell_index_method.get_m() {
            let x = i as f64 * mesh_interval;
            let y = j as f64 * mesh_interval;
            // Draw horizontal line on y * mesh_interval and vertical line on x * mesh_interval
            chart_context
                .draw_series(LineSeries::new(
                    vec![(x, y), (x + mesh_interval, y)],
                    &BLACK,
                ))
                .expect("Panic!!");
            chart_context
                .draw_series(LineSeries::new(
                    vec![(x, y), (x, y + mesh_interval)],
                    &BLACK,
                ))
                .expect("Panic!!");
        }
    }
    chart_context
        .draw_series(LineSeries::new(
            vec![
                (cell_index_method.get_length(), 0.0),
                (
                    cell_index_method.get_length(),
                    cell_index_method.get_length(),
                ),
            ],
            &BLACK,
        ))
        .expect("Panic!!");
    chart_context
        .draw_series(LineSeries::new(
            vec![
                (0.0, cell_index_method.get_length()),
                (
                    cell_index_method.get_length(),
                    cell_index_method.get_length(),
                ),
            ],
            &BLACK,
        ))
        .expect("Panic!!");

    drawing_area
        .present()
        .expect("Unable to write result to file");

    // Draw particles
    let particles = cell_index_method.get_cells().iter().flatten();
    chart_context
        .draw_series(particles.map(|particle| {
            let (x, y) = particle.get_coordinates();
            let radius = particle.get_radius() as f64;
            let color = if particle.get_id() == neighbors.get_particle_id() {
                &RED
            } else {
                if neighbors.contains(&particle.get_id()) {
                    &GREEN
                } else {
                    &BLUE
                }
            };
            dbg!(radius);
            Circle::new((x, y), radius, color.filled())
        }))
        .unwrap();

    println!("Result has been saved to {}", OUT_FILE_NAME);
}

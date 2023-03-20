use anyhow::Result;
use plotters::prelude::*;
use plotters::series::LineSeries;

use crate::CellIndexParticle;
use neighbors::{cell_index_method::CellIndexMethod, Particle, ParticleNeighbors};

const MARGIN_SIZE: u32 = 10;
const LABEL_AREA_SIZE: u32 = 40;
const FONT_SIZE: u32 = 50;
const HEIGHT: u32 = 1000;
const WIDTH: u32 = 1000;

pub fn plot_cell_index_method(
    cell_index_method: &CellIndexMethod<CellIndexParticle>,
    neighbors: &ParticleNeighbors,
    path: &str,
) -> Result<()> {
    const OUT_FILE_NAME: &str = "output.png";
    let drawing_area = BitMapBackend::new(OUT_FILE_NAME, (WIDTH, HEIGHT)).into_drawing_area();
    drawing_area.fill(&WHITE)?;

    let mut chart_context = ChartBuilder::on(&drawing_area)
        .caption("Cell Index Method", ("serif", FONT_SIZE).into_font())
        .margin(MARGIN_SIZE)
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_SIZE)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_SIZE)
        .build_cartesian_2d(
            0f64..cell_index_method.get_length(),
            0f64..cell_index_method.get_length(),
        )?;

    chart_context
        .configure_mesh()
        .disable_mesh()
        .x_desc("X")
        .y_desc("Y")
        .x_label_formatter(&|x| format!("{:.1}", x))
        .y_label_formatter(&|y| format!("{:.1}", y))
        .draw()?;

    // Configure cell size
    let mesh_interval = cell_index_method.get_length() / cell_index_method.get_m() as f64;

    // Draw cells
    for i in 0..cell_index_method.get_m() {
        for j in 0..cell_index_method.get_m() {
            let x = i as f64 * mesh_interval;
            let y = j as f64 * mesh_interval;
            // Draw horizontal line on y * mesh_interval and vertical line on x * mesh_interval
            chart_context.draw_series(LineSeries::new(
                vec![(x, y), (x + mesh_interval, y)],
                &BLACK,
            ))?;
            chart_context.draw_series(LineSeries::new(
                vec![(x, y), (x, y + mesh_interval)],
                &BLACK,
            ))?;
        }
    }
    chart_context.draw_series(LineSeries::new(
        vec![
            (cell_index_method.get_length(), 0.0),
            (
                cell_index_method.get_length(),
                cell_index_method.get_length(),
            ),
        ],
        &BLACK,
    ))?;
    chart_context.draw_series(LineSeries::new(
        vec![
            (0.0, cell_index_method.get_length()),
            (
                cell_index_method.get_length(),
                cell_index_method.get_length(),
            ),
        ],
        &BLACK,
    ))?;

    // Draw particles
    let particles = cell_index_method.get_cells().iter().flatten();
    chart_context.draw_series(particles.map(|particle| {
        let (x, y) = particle.get_coordinates();
        let color = if particle.get_id() == neighbors.get_particle_id() {
            &RED
        } else if neighbors.contains(&particle.get_id()) {
            &GREEN
        } else {
            &BLUE
        };
        Circle::new((x, y), 5, color.filled())
    }))?;

    drawing_area.present()?;

    println!("Result has been saved to {}", path);

    Ok(())
}

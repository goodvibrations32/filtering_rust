use plotters::prelude::*;
pub fn my_first_plot()->Result<(), Box<dyn std::error::Error>>{
    let root = BitMapBackend::new("plotters_rust_data/image.png",
                                  (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("put the title here")
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartatian_2d(-1f32..1f32, -0.1f32..1f32)?;
    chart.configure_mesh().draw?;
    chart
        .configure_series_labels()
        .backround_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
    root.present()?;
    Ok(())
}

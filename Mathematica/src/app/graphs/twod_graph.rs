use plotters::prelude::*;

pub fn graph(equation: &str) {
        
    match create_graph(equation) {
        None => {
            std::process::Command::new("feh")
                .arg("-F")
                .arg("images/0.1.png")
                .spawn()
                .expect("ls command failed to start");
                println!("Image Rendered")
        },
        _ => println!("File Could Not Load"),
    }
}

fn create_graph(equation: &str) -> Option<()> {
    let root_drawing_area = BitMapBackend::new("images/graph.png", (1024, 600))
        .into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root_drawing_area)
        //.build_cartesian_2d(-3.14..3.14, -1.2..1.2)
        .caption("Figure Sample", ("Arial", 30))
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(-6.28..6.28, -1.2..1.2)
        .unwrap();

    let expr: meval::Expr = equation.parse().unwrap();
    let func = expr.bind("x").unwrap();

    chart.draw_series(LineSeries::new(
        (-628..628).map(|x| x as f64 / 100.0).map(|x| (x, func(x))),
        &RED
    )).unwrap();
    chart.configure_mesh().draw().unwrap();
    None
}


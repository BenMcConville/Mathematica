use plotters::prelude::*;
use std::io;
use super::graph_gui;

enum InputMode  {
    Normal,
    Editing
}
/*
struct App  {
    input: String,
    input_mode: InputMode,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
        }
    }
}*/

pub fn graph() {
    println!("Enter Equation: \n");
    let mut equation = String::new();
    let mut app = graph_gui::App::default(); //Possibly Add correct equation detector
    graph_gui::user_input(&mut app);
    println!("{:?}", app);
    //io::stdin().read_line(&mut equation)
    //    .expect("Failed to Read Equation");
    
    match create_graph(&app.input) {
        None => {
            std::process::Command::new("feh")
                .arg("-F")
                .arg("images/graph.png")
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
    println!("E1");
    let func = expr.bind("x");//.unwrap();
    match func  {
        Ok(_) => (),
        _ => return Some(()),
    }
    let func = func.unwrap();
    chart.draw_series(LineSeries::new(
        (-628..628).map(|x| x as f64 / 100.0).map(|x| (x, func(x))),
        &RED
    )).unwrap();
    chart.configure_mesh().draw().unwrap();
    None
}




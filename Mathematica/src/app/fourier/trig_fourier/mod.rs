pub mod fourier_gui;
pub mod trigCalc;

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

pub fn fourier_setup() {
    println!("Enter Equation: \n");
    let mut equation = String::new();
    let mut app = fourier_gui::App::default(); //Possibly Add correct equation detector
    fourier_gui::user_input(&mut app);
    
    println!("{:?}", app);
}



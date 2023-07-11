pub mod main_page;
pub mod graphs;
pub mod fourier;

pub fn run()    {
    let mut run_program = true;
    let mut directory = String::from("~/");
    directory.push_str("next/"); 
    for i in directory.split("/")   {
        println!("{}", i);
    }
    while run_program   {
        match main_page::run()  {
            Ok(0) => match graphs::graph_setup()    {
                Ok(1) => println!("Ok"),
                _ => println!("NOK"),
            },
            Ok(1) => match fourier::fourier_setup() {
                Ok(1) => println!("Ok"),
                _ => println!("NOK"),
            },
            //Ok(num) => println!("{}", num),
            _ => run_program = false,
        }
    }
}

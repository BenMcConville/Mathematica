pub mod main_page;
pub mod graphs;

pub fn run()    {
    let mut directory = String::from("~/");
    directory.push_str("next/"); 
    for i in directory.split("/")   {
        println!("{}", i);
    }
    match main_page::run()  {
        Ok(99) => println!("Error"),
        Ok(0) => match graphs::graph_setup()    {
            Ok(1) => println!("Ok"),
            _ => println!("NOK"),
        },
        Ok(num) => println!("{}", num),
        _ => println!("nothing"),
    }
}

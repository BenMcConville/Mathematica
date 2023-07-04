pub mod main_page;

pub fn run()    {
    let mut directory = String::from("~/");
    directory.push_str("next/"); 
    for i in directory.split("/")   {
        println!("{}", i);
    }
    match main_page::run()  {
        Ok(99) => println!("Error"),
        Ok(num) => println!("{}", num),
        _ => println!("nothing"),
    }
}

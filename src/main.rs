extern crate clap; 
use clap::App; 
use std::process::Command;

fn main() { 
    let matches = App::new("mypipe")
        .version("1.0")
        .author("Pauline")

        .arg(
            clap::Arg::with_name("in")
                .takes_value(true)
                .long("in") 
                .requires("out")
        )
        .arg(
            clap::Arg::with_name("out")
                .takes_value(true)
                .long("out") 
        )
    .get_matches();  

    if matches.is_present("in") {
        process_func(matches.value_of("in").unwrap().to_string(), matches.value_of("out").unwrap().to_string());
    }
    else{
        println!("Invalid input ! Expected mypipe --in <in> --out <out>");
    }

}

fn process_func(cmd1 : String, cmd2 : String){
    let output_fortune = 
        Command::new(cmd1)
                .output()
                .expect("failed to execute process");

    let fortune_message = String::from_utf8_lossy(&output_fortune.stdout).to_string();
    
    let output_cowsay =
        Command::new(cmd2)
            .arg(fortune_message)
            .output()
            .expect("failed to execute process");
    println!("{}", String::from_utf8_lossy(&output_cowsay.stdout));
}
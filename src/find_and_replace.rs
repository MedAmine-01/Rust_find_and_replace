use text_colorizer::*;
use std::env;
use std::fs;
use regex;
use regex::Regex;

#[derive(Debug)]
#[allow(dead_code)]

struct Arguments{
    pattern: String,
    replace: String,
    input_file: String,
    output_file: String,
}


fn print_help(){
    eprintln!("{} - replace a string with a new string", "Find and Replace".green().bold());
    eprintln!("Usage <target String> <replacement String> <Input file> <Output file>");
}

fn parse_args()-> Arguments{
    let args : Vec<String> = env::args().skip(1).collect();
    if args.len() != 4 {
        eprintln!("{}: Wrong number of arguments given expected 4 got {}", "Error".red().bold(), args.len());
        print_help();
        std::process::exit(1);
    }
    Arguments{
        pattern: args[0].clone(),
        replace: args[1].clone(),
        input_file: args[2].clone(),
        output_file: args[3].clone(),
    }
}

fn replace(target: &str, replace:&str, data :&str)->Result<String, regex::Error>{
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(data, replace).to_string())
}

fn read_and_write(args: Arguments){
    let data  = match fs::read_to_string(&args.input_file){
        Ok(f)=> f,
        Err(error)=>{
            eprintln!("{} failed to read from file {}: {:?}","Error".red().bold(), args.input_file, error);
            std::process::exit(1);
        }
    };
    let replaced_data = match replace(&args.pattern, &args.replace, &data){
        Ok(t)=>t,
        Err(error)=>{
            eprintln!("{} failed to replace text: {:?}","Error".red().bold(), error);
            std::process::exit(1);
        }
    };

    //fs::File::create(&args.output_file).expect("Creating output file failed");
    match fs::write(&args.output_file, &replaced_data) {
        Ok(_)=>{},
        Err(error)=>{
            eprintln!("{} failed to write to file {}: {:?}","Error".red().bold(), args.output_file, error);
            std::process::exit(1);
        }
    }

}

pub fn run(){
    let args = parse_args();
    println!("{:?}", args);
    read_and_write(args);

}
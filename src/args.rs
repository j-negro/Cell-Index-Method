use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Cell Index Method", author, version, about)]
pub struct Cli {
    #[arg(default_value_t = String::from("./static.txt"))]
    pub static_input_path: String,
    #[arg(default_value_t = String::from("./dynamic.txt"))]
    pub dynamic_input_path: String,
    #[arg(default_value_t = String::from("./output.txt"))]
    pub output_path: String,
    #[arg(short, long)]
    pub brute_force: bool,
    #[arg(short, long)]
    pub periodic: bool,
}

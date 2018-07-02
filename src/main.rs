extern crate handlebars;
extern crate serde_json;
#[macro_use]
extern crate structopt;

use std::{
    path::PathBuf,
    fs
};
use handlebars::Handlebars;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "jrtr", about = "Render Handlebars templates with data from context json file.")]
struct Opt {
    /// Input context file
    #[structopt(short = "c", long = "context", parse(from_os_str))]
    context: PathBuf,
    /// Input template file
    #[structopt(short = "t", long = "template", parse(from_os_str))]
    template: PathBuf,
    /// Output file, stdout if not present
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();
    let reg = Handlebars::new();
    let ctx = fs::read_to_string(opt.context).expect("Unable to read context file");
    let ctx: serde_json::Value = serde_json::from_str(&ctx).expect("Malformed context json");
    let tpl = fs::read_to_string(opt.template).expect("Unable to read template file");
    let rendered = reg.render_template(&tpl, &ctx).unwrap();
    match opt.output {
        Some(output) => {
            eprintln!("This will write to file in the future. Right now not available!");
            std::process::exit(-1);
        },
        None => {
            print!("{}", rendered);
        }
    }
}


pub mod models;
pub mod services;

use models::{ create_react_component_args::CreateReactComponentArgs, components::Component };
use clap::Parser;

fn main() {
   let args = CreateReactComponentArgs::parse();
   let folder_name = format!("{}/{}", args.output.to_str().unwrap(), args.name);
   Component::build(&args.name, &folder_name);
}

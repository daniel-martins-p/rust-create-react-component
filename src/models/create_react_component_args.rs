use clap:: {Parser};

#[derive(Debug, Parser)]
#[clap(version = "0.0.1", about = "Scafolds a component folder structure with a index export file, the component file and the unit test file", author = "Daniel Martins <mp.daniel.414@gmail.com")]
pub struct CreateReactComponentArgs {
  /// Component Name - Which will be used as the component name and the folder name
  #[arg(short = 'n', long = "name", required = true)]
  pub name: String,
  /// (Optional) Component Path - Which will be used as the full component path
  #[arg(short = 'o', long = "output", required = false, default_value = ".")]
  pub output: std::path::PathBuf,
}

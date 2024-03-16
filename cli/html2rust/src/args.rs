use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
pub struct Args {
    /// Input file.
    #[clap(short = 'f', long = "file")]
    pub input_file_path: Option<PathBuf>,

    /// Output function or type name.
    #[clap(short = 'n', long = "name")]
    pub function_name: Option<String>,

    /// Add no mangle declaration for use in FFI/static loading.
    #[clap(short = 'm', long)]
    pub no_mangle: bool,

    /// Add whitespace between elements.
    #[clap(short = 'w', long)]
    pub whitespace: bool,
}

impl Args {
    /// Convenience method to load CLI arguments.
    pub fn load() -> Self {
        <Args as clap::Parser>::parse()
    }
}

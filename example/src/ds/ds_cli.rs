use clap::Parser;

#[derive(Parser)]
#[command(name = "example")]
#[command(version = "0.1.0")]
#[command(
    about = "dimsys cli tools example demo",
    long_about = "Use dimsys libary, make a CLI Side Project"
)]
#[command(propagate_version = true)]
#[command(next_line_help = true)]
pub struct Cli {
    #[arg(short, long)]
    #[arg(default_value = "all")]
    /// Choose need output a information data!
    /// (You can output: cpu,gpu,disk,memory,network,power,system,desktop,environment)
    pub output: Option<String>,
}

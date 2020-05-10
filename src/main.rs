use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)] // do we need Debug here?
#[structopt(name = "seg")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Sub,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "Static Email Generator")]
enum Sub {
    /// Generates all available emails using [SOURCE] folder
    #[structopt(display_order = 1)]
    Generate {
        /// Source template folder
        #[structopt(short, long)]
        source: PathBuf,

        /// Output email folder
        #[structopt(short, long)]
        destination: PathBuf,
    },

    /// Sends a test mail to preview
    #[structopt(display_order = 2)]
    Send {
        /// Email template file
        #[structopt(short, long)]
        template: PathBuf,

        /// Email subject
        #[structopt(short, long)]
        subject: String,

        /// Receiver email
        #[structopt(short, long)]
        email: String,
    },
}

fn main() {
    let opt = Opt::from_args();
    println!("Hello, world!");
    println!("Params: {:#?}", opt);
}

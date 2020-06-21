use meyl;
use meyl::template::{generate_all_templates, ErrorKind};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "meyl")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Sub,
}

#[derive(StructOpt)]
#[structopt(about = "Static Email Generator")]
enum Sub {
    /// Create a new email template directory
    #[structopt(display_order = 1)]
    New {
        /// Email template directory
        #[structopt(short, long)]
        source: PathBuf,
    },

    /// Generates all available emails using [SOURCE] directory
    #[structopt(display_order = 2)]
    Generate {
        /// Source template directory
        #[structopt(short, long)]
        source: PathBuf,

        /// Output email directory
        #[structopt(short, long)]
        destination: PathBuf,
    },

    /// Sends a test mail to preview
    #[structopt(display_order = 3)]
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

fn create_new_directory(path: &PathBuf) -> Result<(), ErrorKind> {
    if path.exists() {
        let msg = format!(
            "{:?} already exists. Try a different one or remove the directory.",
            path
        );
        return Err(ErrorKind::InvalidDirectory(msg));
    }
    // TODO 1: create <path>/src/ dir
    // TODO 2: create <path>/dst/ dir
    // TODO 3: create <path>/src/example/ with config.toml, subject.html, body.html, and body_text.html
    // TODO 4: create <path>/src/style.css file
    // TODO 5: create <path>/src/base.html file
    Ok(())
}

fn main() {
    let opt = Opt::from_args();
    match opt.cmd {
        Sub::New { source } => match create_new_directory(&source) {
            Ok(_) => println!(
                "Your new email template directory is created in {:?}",
                source
            ),
            Err(e) => eprintln!("{}", e),
        },
        Sub::Generate {
            source,
            destination,
        } => match generate_all_templates(source, destination) {
            Ok(_) => println!("All templates are generated successfully."),
            Err(e) => eprintln!("{:?}", e),
        },
        Sub::Send {
            template,
            subject,
            email,
        } => {
            println!("command is send.");
            println!(
                "template: {:?}, subject: {}, email: {}",
                template, subject, email
            );
        }
    }
}

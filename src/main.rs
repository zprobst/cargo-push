/// We want to support a commands like:
///
/// cargo push commit  -m "hello world" -t feature --story SVC-1111
mod commit;
use commit::{perform_commit, ConventionalCommitType};

#[macro_use]
extern crate lazy_static;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "cargo push",
    about = "A tool to abstract git and enforce consistency of commits and releases using the semver and conventional commits standards"
)]
enum Opt {
    Commit {
        #[structopt(short = "m", long = "message")]
        message: String,
        #[structopt(short = "t", long = "type")]
        commit_type: Option<ConventionalCommitType>,
        #[structopt(short = "s", long = "story")]
        ticket: Option<String>,
    },
    Version {
        tag: Option<String>,
    },
}

fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::Commit {
            message,
            commit_type,
            ticket,
        } => {
            perform_commit(message, commit_type, ticket).expect("Failed to perform commit");
        }
        _ => {
            println!("Not yet supported");
        }
    };
}

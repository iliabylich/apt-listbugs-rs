mod error;
mod input;
mod log;
mod soap;

use crate::{error::AppError, input::get_input, log::log, soap::get_bugs, soap::list_bugs};
use std::io::{BufRead, Write};

fn main() {
    if let Err(err) = try_main() {
        eprintln!("[apt-listbugs-rs] Error, aborting:\n{err}");
        std::process::exit(1);
    }
}

fn try_main() -> Result<(), AppError> {
    let packages_list = get_input()?;
    println!("Checking packages {}", packages_list.join(" "));

    let bug_numbers = get_bugs(packages_list)?;
    if bug_numbers.is_empty() {
        println!("No bugs found!");
        std::process::exit(0);
    } else {
        println!("Found bugs: {}", bug_numbers.join(", "));
    }

    let bugs = list_bugs(bug_numbers)?;
    log!("Got bugs {bugs:?}");

    for bug in bugs {
        println!();
        println!("\x1b[0;31m{} bug in {}\x1b[0m", bug.severity, bug.package);
        println!("{}", bug.subject);
        println!(
            "URL: https://bugs.debian.org/cgi-bin/bugreport.cgi?bug={}",
            bug.id
        );
        println!();
    }

    if !can_proceed() {
        println!("[apt-listbugs-rs] Aborting");
        std::process::exit(1);
    }

    Ok(())
}

fn can_proceed() -> bool {
    let mut proceed: Option<bool> = None;
    while proceed.is_none() {
        print!("\x1b[0;31mDo you still want to install them? y/n\x1b[0m ");
        std::io::stdout().flush().unwrap();

        let line = std::io::stdin().lock().lines().next().unwrap();

        match line {
            Ok(line) if line == "y" || line == "Y" => proceed = Some(true),
            Ok(line) if line == "n" || line == "N" => proceed = Some(false),
            _ => {}
        }
    }
    proceed.unwrap()
}

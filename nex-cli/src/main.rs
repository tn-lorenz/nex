use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::process;

use colored::Colorize;
use strip_ansi_escapes::strip;

fn read_file(path: &Path) -> io::Result<String> {
    fs::read_to_string(path)
}

fn main() -> io::Result<()> {
    let mut args = env::args();
    let program = args.next().unwrap_or_else(|| "nex".to_string());

    let command = match args.next() {
        Some(cmd) => cmd,
        None => {
            eprintln!("Usage: {} <command> [args]", program);
            process::exit(1);
        }
    };

    match command.as_str() {
        "run" => {
            // run <filename>
            let filename = match args.next() {
                Some(f) => f,
                None => {
                    eprintln!("Usage: {} run <filename>", program);
                    process::exit(1);
                }
            };

            match read_file(Path::new(&filename)) {
                Ok(content) => println!("File content:\n{}", content),
                Err(e) => {
                    eprintln!("Error reading file '{}': {}", filename, e);
                    process::exit(1);
                }
            }
        }
        "build" => {
            println!("Building project...");
        }
        "help" => {
            println!(
                "{} {} {} {}",
                "Usage:".green().bold(),
                "nex".cyan().bold(),
                "[OPTIONS]".cyan(),
                "[COMMAND]".cyan()
            );

            println!(" ");

            let options = [
                ("-V, --version", "Print version info and exit"),
                ("--list", "List installed commands"),
                ("...", "See all commands with --list"),
            ];

            println!("{}", "Options:".green().bold());
            print_columns(&options, Some("--list"));

            println!(" ");

            let commands = [
                ("run, r", "Run the current package"),
                ("build, b", "Compile the current package"),
                (
                    "check, c",
                    "Analyze the current package and report errors, but don't build object files",
                ),
                ("clean", "Remove the target directory"),
                (
                    "help, h",
                    "Print this message or the help of the given subcommand",
                ),
            ];

            println!("{}", "Commands:".green().bold());

            print_columns(&commands, None);
        }
        _ => {
            let help_msg = "help: a command with a similar name does not exist :P\nhelp: view all available commands with `nex help`"
            .trim_start();

            eprintln!(
                "{}{} {}\n\n{}",
                "error".red().bold(),
                ":".bold(),
                format!("unknown command: `{}`", command),
                help_msg
            );
            process::exit(1);
        }
    }

    Ok(())
}

fn visible_len(s: &str) -> usize {
    strip(s).unwrap_or_default().len()
}

fn print_columns(items: &[(&str, &str)], highlight: Option<&str>) {
    // maximale sichtbare Länge der linken Spalte
    let col_width = items
        .iter()
        .map(|(opt, _)| visible_len(opt))
        .max()
        .unwrap_or(0)
        + 2;

    for (opt, desc) in items {
        // Kommata grau, Buchstaben cyan+bold
        let parts: Vec<String> = opt
            .split(',')
            .map(|p| {
                p.trim()
                    .chars()
                    .map(|c| {
                        if c.is_alphabetic() || c.is_digit(10) || c == '-' {
                            c.to_string().cyan().bold().to_string()
                        } else {
                            c.to_string()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("")
            })
            .collect();

        let opt_colored = parts
            .iter()
            .enumerate()
            .map(|(i, part)| {
                if i > 0 {
                    format!("{}", ",".truecolor(150, 150, 150)) + " " + part
                } else {
                    part.clone()
                }
            })
            .collect::<Vec<_>>()
            .join("");

        // Beschreibung: optional Highlight
        let desc_colored = if let Some(h) = highlight {
            desc.replace(h, &h.cyan().to_string())
        } else {
            desc.to_string()
        };

        // ANSI-freundliche Ausrichtung
        let padding = col_width.saturating_sub(visible_len(&opt_colored));
        println!("  {}{}{}", opt_colored, " ".repeat(padding), desc_colored);
    }
}

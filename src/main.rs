use clap::{builder::Str, command, Arg, ArgAction, ArgMatches, Command};
use log::{debug, error, LevelFilter};

mod puzzles;

fn main() {
    let matches = command!()
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .add_puzzle_commands()
        .get_matches();

    let debug: bool = *matches.get_one("debug").unwrap_or(&false);
    init_logging(debug);

    run_command(matches.subcommand().unwrap());
}

fn init_logging(debug: bool) {
    let level = if debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Warn
    };
    pretty_env_logger::formatted_builder()
        .filter_level(level)
        .init();
}

fn run_command(command: (&str, &ArgMatches)) {
    match command {
        ("all", _) => run_all_puzzles(),
        ("day", args) => run_day_command(args),
        _ => error!("Use the 'all' or 'day' command to run one or more puzzles"),
    }
}

fn run_all_puzzles() {
    for puzzle in puzzles::puzzles() {
        debug!("Running day '{}' part one:", puzzle.name());
        puzzle.run_part_one();
        debug!("Running day '{}' part two:", puzzle.name());
        puzzle.run_part_two();
    }
}

fn run_day_command(args: &ArgMatches) {
    match args.subcommand() {
        Some((day_name, args)) => match args.subcommand() {
            Some(("part", args)) => match args.subcommand() {
                Some((part_name, _)) => run_day_puzzle(day_name, part_name),
                _ => error!("Missing the part name"),
            },
            Some((unexpected_command, _)) => error!("unhandled command '{}'", unexpected_command),
            _ => error!("Missing the part"),
        },
        _ => error!("Missing the day name"),
    }
}

fn run_day_puzzle(name: &str, part: &str) {
    if let Some(puzzle) = puzzles::get_puzzle(name) {
        if part == "one" || part == "both" {
            debug!("Running day '{}' part one:", puzzle.name());
            puzzle.run_part_one();
        }
        if part == "two" || part == "both" {
            debug!("Running day '{}' part two:", puzzle.name());
            puzzle.run_part_two();
        }
    }
}

trait AddPuzzlesCommands {
    fn add_puzzle_commands(self) -> Self;
}

impl AddPuzzlesCommands for Command {
    fn add_puzzle_commands(mut self) -> Self {
        let mut day_command = command!("day")
            .disable_help_subcommand(true)
            .subcommand_required(true)
            .about("Runs a specified day's puzzle");
        for (index, name) in puzzles::puzzle_names().iter().enumerate() {
            let alias = Str::from((index + 1).to_string());
            day_command = day_command.subcommand(
                command!(name)
                    .disable_help_subcommand(true)
                    .subcommand_required(true)
                    .about(format!("Run day {name}"))
                    .alias(&alias)
                    .subcommand(
                        command!("part")
                            .disable_help_subcommand(true)
                            .subcommand_required(true)
                            .subcommand(command!("both").about("Runs both day's puzzle parts"))
                            .subcommand(
                                command!("one")
                                    .about("Runs the day's puzzle part one")
                                    .alias("1"),
                            )
                            .subcommand(
                                command!("two")
                                    .about("Runs the day's puzzle part two")
                                    .alias("2"),
                            ),
                    ),
            );
        }

        self = self.subcommand_required(true).subcommand(
            command!("all")
                .disable_help_subcommand(true)
                .about("Runs all puzzles"),
        );
        self = self.subcommand(day_command);
        self
    }
}

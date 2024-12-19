use clap::{Arg, ArgAction, ArgMatches, Command};

mod controller;

use controller::Controller;

pub fn handle_tournament_command(tournament_matches: &ArgMatches) {
    let bots = tournament_matches
        .get_many::<String>("bots")
        .expect("Required argument: bots")
        .map(|s| s.to_string())
        .collect();

    let rounds = tournament_matches
        .get_one::<u32>("rounds")
        .copied()
        .expect("Required argument: rounds");

    let board_start = tournament_matches
        .get_one::<String>("board-start")
        .map(|s| s.to_string());

    let custom_board_start = tournament_matches
        .get_one::<String>("custom-board-start")
        .map(|s| s.to_string());

    let pgn = tournament_matches
        .get_one::<String>("pgn")
        .map(|s| s.to_string());

    let export_txt = tournament_matches.get_flag("export-txt");
    let export_pgn = tournament_matches.get_flag("export-pgn");
    let verbose = tournament_matches.get_flag("verbose");

    let output_dir = tournament_matches
        .get_one::<String>("output-dir")
        .map(|s| s.to_string())
        .unwrap_or_else(|| ".".to_string());

    let controller = Controller::new(
        bots,
        rounds,
        board_start,
        custom_board_start,
        pgn,
        export_txt,
        export_pgn,
        verbose,
        output_dir,
    );

    controller.run();
}

fn cli_commands() {
    let bots_arg = Arg::new("bots")
        .help("List of bots to compete in the tournament")
        .required(true)
        .action(ArgAction::Append)
        .value_name("BOT")
        .num_args(1..);

    let rounds_arg = Arg::new("rounds")
        .long("rounds")
        .help("Number of rounds to play")
        .default_value("1")
        .value_parser(clap::value_parser!(u32));

    let board_start_arg = Arg::new("board-start")
        .long("board-start")
        .help("Start position of the board (FEN string)")
        .value_name("FEN");

    let custom_board_start_arg = Arg::new("custom-board-start")
        .long("custom-board-start")
        .help("Custom start position for the board (string)")
        .value_name("CUSTOM_FEN");

    let pgn_arg = Arg::new("pgn")
        .long("pgn")
        .help("Path to a PGN file to use for the tournament")
        .value_name("PGN_FILE");

    let export_txt_arg = Arg::new("export-txt")
        .long("export-txt")
        .help("Export tournament results as a TXT file")
        .action(ArgAction::SetTrue);

    let export_pgn_arg = Arg::new("export-pgn")
        .long("export-pgn")
        .help("Export tournament results as a PGN file")
        .action(ArgAction::SetTrue);

    let verbose_arg = Arg::new("verbose")
        .long("verbose")
        .help("Print verbose output")
        .action(ArgAction::SetTrue);

    let output_dir_arg = Arg::new("output-dir")
        .long("output-dir")
        .help("Directory to save the output files")
        .default_value(".");

    let tournament_command = Command::new("tournament")
        .about("Run a chess bot tournament")
        .arg(bots_arg)
        .arg(rounds_arg)
        .arg(board_start_arg)
        .arg(custom_board_start_arg)
        .arg(pgn_arg)
        .arg(export_txt_arg)
        .arg(export_pgn_arg)
        .arg(verbose_arg)
        .arg(output_dir_arg);

    let matches = Command::new("chess")
        .version("1.0")
        .about("Chess bot tournament!")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(tournament_command)
        .get_matches();

    match matches.subcommand() {
        Some(("tournament", tournament_matches)) => {
            handle_tournament_command(tournament_matches);
        }
        _ => unreachable!(),
    }
}

fn main() {
    cli_commands();
}

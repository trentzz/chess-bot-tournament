
pub struct Controller {
    bots: Vec<String>,
    rounds: u32,
    board_start: Option<String>,
    custom_board_start: Option<String>,
    pgn: Option<String>,
    export_txt: bool,
    export_pgn: bool,
    verbose: bool,
    output_dir: String,
}

impl Controller {
    pub fn new(
        bots: Vec<String>,
        rounds: u32,
        board_start: Option<String>,
        custom_board_start: Option<String>,
        pgn: Option<String>,
        export_txt: bool,
        export_pgn: bool,
        verbose: bool,
        output_dir: String,
    ) -> Self {
        Self {
            bots,
            rounds,
            board_start,
            custom_board_start,
            pgn,
            export_txt,
            export_pgn,
            verbose,
            output_dir,
        }
    }

    pub fn run(&self) {
        if self.verbose {
            println!("Starting tournament with the following configuration:");
            println!("Bots: {:?}", self.bots);
            println!("Rounds: {}", self.rounds);
            println!("Board Start: {:?}", self.board_start);
            println!("Custom Board Start: {:?}", self.custom_board_start);
            println!("PGN: {:?}", self.pgn);
            println!("Export TXT: {}", self.export_txt);
            println!("Export PGN: {}", self.export_pgn);
            println!("Output Directory: {}", self.output_dir);
        }


        if self.verbose {
            println!("Tournament completed.");
        }
    }
}

extern crate clap;
use clap::App;

mod board;
mod app;

const VERSION_STR: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let app = App::new("rs2048")
        .version(VERSION_STR)
        .author("shinkwhek")
        .about("A 2048 clone written in Rust.");

    app.get_matches();

    loop {
        let mut board = board::Board::new();
        board.render();
        board.render_parameter_box();
        app::Update::update(&mut board);

        if board.fin() {
            break;
        }
    }

    println!("bye bye.");
}

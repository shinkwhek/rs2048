extern crate rustyline;
use self::rustyline::error:: ReadlineError;
use self::rustyline::Editor;


extern crate ansi_term;
use self::ansi_term::{Colour};

extern crate rand;
use self::rand::random;

#[derive(Clone, PartialEq)]
pub enum Status {
    Nothing,
    Exist,
}

#[derive(Clone)]
pub struct Tile {
    pub score: usize,
    pub status: Status,
}

impl Tile {

    pub fn new(score: usize, status: Status) -> Tile {
        Tile {
            score: score,
            status: status
        }
    }

    pub fn get_score(&self) -> usize {
        self.score
    }

    pub fn is_exist(&self) -> bool {
        self.status == Status::Exist
    }

    pub fn is_nothing(&self) -> bool {
        self.status == Status::Nothing
    }
}

pub struct Board {
    pub score: usize,
    pub tiles: Vec<Tile>,
}

impl Board {

    pub fn new() -> Board {
        let mut board = Board {
            score: 0,
            tiles: vec![Tile::new(2, Status::Nothing); 16],
        };
        board.gen();
        board.gen();
        board.sum_score();
        board
    }

    pub fn gen(&mut self) {
        loop {
            let n = random::<usize>() % 16; 
            if self.get_tile(n).is_none()  {
                let score = if random::<u32>() % 10 == 0 {
                    4
                } else {
                    2
                };
                self.tiles[n] = Tile::new(score, Status::Exist);
                break;
            }
        }
    }

    fn color(n: usize) -> Colour {
        match n {
            2     => Colour::RGB(234,222,207),
            4     => Colour::RGB(231,218,187),
            8     => Colour::RGB(238,162,102),
            16    => Colour::RGB(240,129,81),
            32    => Colour::RGB(240,101,77),
            64    => Colour::RGB(240,70,46),
            128   => Colour::RGB(230,197,94),
            256   => Colour::RGB(230,194,79),
            512   => Colour::RGB(230,189,64),
            1024  => Colour::RGB(230,186,50),
            2048  => Colour::RGB(230,182,37),
            _     => Colour::RGB(251,0,25),
        }
    }

    fn format_line(&self, n: usize, t: Option<usize>) -> String {
        let score = self.tiles[n].get_score();
        let color = Board::color(score); 
        if self.tiles[n].is_exist() {
            if t.is_none() {
                return Colour::White.on(color).fg(Colour::White).paint("        ").to_string();
            } else {
                return
                    Colour::White.on(color)
                    .fg(
                        if color == Colour::RGB(234,222,207) || color == Colour::RGB(231,218,187) {
                            Colour::RGB(100,91,83)
                        } else {
                            Colour::White
                        }
                       ).bold().paint(format!("{0:^8}", score)).to_string();
            }

        } else {
            Colour::White.paint("        ").to_string()
        }
    }

    pub fn render(&self) {
        println!("{0}{1:\u{2500}>8}{2}{1:\u{2500}>8}{2}{1:\u{2500}>8}{2}{1:\u{2500}>8}{3}",
                 "\u{250c}","\u{2500}","\u{252c}","\u{2510}");

        for i in 0..4 {
            println!("{0}{1}{0}{2}{0}{3}{0}{4}{0}", "\u{2502}",
                     Board::format_line(&self, 4*i, None),
                     Board::format_line(&self, 4*i+1, None),
                     Board::format_line(&self, 4*i+2, None),
                     Board::format_line(&self, 4*i+3, None),
                     );
            println!("{0}{1}{0}{2}{0}{3}{0}{4}{0}", "\u{2502}",
                     Board::format_line(&self, 4*i, None),
                     Board::format_line(&self, 4*i+1, None),
                     Board::format_line(&self, 4*i+2, None),
                     Board::format_line(&self, 4*i+3, None),
                     ); 
            println!("{0}{1:^8}{0}{2:^8}{0}{3:^8}{0}{4:^8}{0}", "\u{2502}",
                     Board::format_line(&self, 4*i, Some(4*i)),
                     Board::format_line(&self, 4*i+1, Some(1+(4*i))),
                     Board::format_line(&self, 4*i+2, Some(2+(4*i))),
                     Board::format_line(&self, 4*i+3, Some(3+(4*i))),
                     );
            println!("{0}{1}{0}{2}{0}{3}{0}{4}{0}","\u{2502}",
                     Board::format_line(&self, 4*i, None),
                     Board::format_line(&self, 4*i+1, None),
                     Board::format_line(&self, 4*i+2, None),
                     Board::format_line(&self, 4*i+3, None),
                     );
            if i != 3 {
                println!("{0}{1:\u{2500}>8}{2}{1:\u{2500}>8}{2}{1:\u{2500}>8}{2}{1:\u{2500}>8}{3}",
                         "\u{251c}","\u{2500}","\u{253c}","\u{2524}");
            } else {
                println!("{0}{1:\u{2500}>8}{2}{1:\u{2500}>8}{2}{1:\u{2500}>8}{2}{1:\u{2500}>8}{3}",
                         "\u{2514}","\u{2500}","\u{2534}","\u{2518}");
            }

        }
    }

    pub fn get_tile(&self, n: usize) -> Option<&Tile> {
        if self.tiles[n].is_exist() {
            return Some(&self.tiles[n]);
        }
        None
    }

    fn get_score(&self) -> usize {
        self.score
    }

    fn sum_score(&mut self) {
        self.score = 0;
        for tile in &self.tiles {
            if tile.is_exist() {
                self.score += tile.get_score();
            }
        }
    }


    pub fn render_parameter_box(&mut self) {
        self.sum_score();

        println!("{} {}", Colour::White.bold().paint("Score:"), self.get_score());

        println!("\n {2}{1}{0} type command!! {3}{4}{5}\n{6}\n{7}\n{8}\n{9}\n{10}\n\n",
                 Colour::Red.bold().paint("<"),
                 Colour::Yellow.bold().paint("<"),
                 Colour::Green.bold().paint("<"),
                 Colour::Red.bold().paint(">"),
                 Colour::Yellow.bold().paint(">"),
                 Colour::Green.bold().paint(">"),
                 Colour::White.paint("              (up)  "),
                 Colour::White.paint("               w"),
                 Colour::White.paint("     (left) a     d (right)"),
                 Colour::White.paint("               s"),
                 Colour::White.paint("             (down)")
                );
    }

    pub fn fin(&mut self) -> bool {
        self.sum_score();
        
        println!("{}", Colour::Yellow.bold().paint("Game over!!!"));
        println!("{} {}", Colour::White.bold().underline().paint("Total score:"), self.get_score());

        let mut r1 = Editor::<()>::new();

        loop {
            let readline = r1.readline("continue? [Y/n]: ");
            match readline {
                Ok(line) => {
                    match &*line.to_string() {
                        "y" | "Y" => { return false; },
                        "n"       => { return true; },
                        _         => {
                            println!("type y or n");
                            continue;
                        },
                    }
                },
                Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                    return true;

                },
                Err(err) => {
                    println!("Error: {:?}", err);
                    return true;
                },
            }
        }
    }
}

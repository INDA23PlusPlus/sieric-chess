use chess::*;
use std::io;
use itertools::Either;

fn print_board(board: &[ChessPiece; 64], rev: bool) {
    use ChessPiece::*;
    use ChessColor::*;

    fn c(col: &ChessColor) -> String {
        return if *col == Wh { String::from("\x1b[34m") }
               else { String::from("\x1b[31m") };
    }

    let range = if rev {
        Either::Right((0..8).rev())
    } else {
        Either::Left(0..8)
    };
    for y in range {
        print!("{} ", 8-y);
        for x in 0..8 {
            match &board[56-y*8 + x] {
                P(col) => print!("{}P\x1b[m", c(col)),
                R(col) => print!("{}R\x1b[m", c(col)),
                N(col) => print!("{}N\x1b[m", c(col)),
                B(col) => print!("{}B\x1b[m", c(col)),
                Q(col) => print!("{}Q\x1b[m", c(col)),
                K(col) => print!("{}K\x1b[m", c(col)),
                None => print!("."),
            };
        }
        println!();
    }
    println!("  abcdefgh");
}

fn dump_moves(moves: &Vec<ChessMove>) {
    for (i, mv) in moves.iter().enumerate() {
        println!("{i}: {}", mv.algebraic());
    }
}

fn main() {
    let mut game = ChessGame::new();

    loop {
        let moves = game.find_moves();
        dump_moves(&moves);
        print_board(game.get_board(), game.turn == ChessColor::Bl);
        let mut inp = String::new();

        let _ = io::stdin().read_line(&mut inp);
        match inp.trim().parse::<usize>() {
            Ok(i) => {
                game.apply_move(&moves[i]);
                game.switch_turn();
            },
            _ => break,
        }
    }
}

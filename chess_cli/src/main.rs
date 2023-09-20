use chess::*;
use std::io::{self, Write};
use itertools::Either;

/**
 * Print a representation of the provided board using ANSI colors to show which
 * side pieces belong to. Blue is white and red is black. Optionally reversed
 * when `rev` is [true].
 */
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

/**
 * Print moves and their indices
 */
fn dump_moves(moves: &Vec<ChessMove>) {
    for (i, mv) in moves.iter().enumerate() {
        println!("{i}: {}", mv.algebraic());
    }
}

fn main() {
    /* create the game */
    let mut game = ChessGame::new();

    /* loop until the game is over */
    while !game.is_ended() {
        /* get all legal moves in a Vec */
        let moves = game.get_legal_moves(&game.turn);

        /* print moves, the check state, and the board
         * (reversed on blacks turn) */
        dump_moves(&moves);
        if game.is_check() {
            println!("In check!");
        }
        print_board(game.get_board(), game.turn == ChessColor::Bl);

        /* take input from the user (index into the moves Vec) */
        print!("Move: ");
        io::stdout().flush().expect("Could not flush stdout");
        let mut inp = String::new();
        let _ = io::stdin().read_line(&mut inp);

        match inp.trim().parse::<usize>() {
            Ok(i) => {
                if i < moves.len() {
                    /* apply the specified move */
                    game.apply_move(&moves[i]);
                    /* switch turn */
                    game.switch_turn();
                }
            },
            _ => (),
        }
    }

    if game.is_checkmate() {
        /* print the player who made the last move, i.e. the opposite of
         * `game.turn` */
        println!("{} checkmate", if game.turn == ChessColor::Wh {
            "Black"
        } else {
            "White"
        });
    } else {
        /* if the game is over and it was not checkmate it is trivially
         * stalemate, which could also be checked using `game.is_stalemate()` */
        println!("Stalemate");
    }
}

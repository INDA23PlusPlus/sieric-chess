use chess::{self, ChessGame, ChessPiece, ChessColor};

fn print_board(board: &[ChessPiece; 64]) {
    use ChessPiece::*;
    use ChessColor::*;

    fn c(col: &ChessColor) -> String {
        return if *col == White { String::from("\x1b[34m") }
               else { String::from("\x1b[31m") };
    }

    for y in 0..8 {
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
}

fn main() {
    let game = ChessGame::new();

    println!("{:?}", game.find_moves());

    print_board(game.get_board());
}

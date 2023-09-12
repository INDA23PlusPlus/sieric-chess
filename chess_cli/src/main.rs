use chess::*;

fn print_board(board: &[ChessPiece; 64]) {
    use ChessPiece::*;
    use ChessColor::*;

    fn c(col: &ChessColor) -> String {
        return if *col == Wh { String::from("\x1b[34m") }
               else { String::from("\x1b[31m") };
    }

    for y in 0..8 {
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
    for mv in moves {
        println!("{}", mv.algebraic());
    }
}

fn main() {
    let game = ChessGame::new();

    dump_moves(&game.find_moves());
    print_board(game.get_board());
}

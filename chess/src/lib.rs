#[derive(Debug,PartialEq,Eq,Clone)]
pub enum ChessColor {
    White,
    Black,
}

impl ChessColor {
    fn dir(&self) -> isize {
        return if *self == ChessColor::White { 1 } else { -1 };
    }
}

#[derive(Debug,PartialEq,Eq)]
pub enum ChessPiece {
    None,
    /* true means white, false means black */
    P(ChessColor), /* pawn */
    R(ChessColor), /* rook */
    N(ChessColor), /* knight */
    B(ChessColor), /* bishop */
    Q(ChessColor), /* queen */
    K(ChessColor), /* king */
}

impl ChessPiece {
    pub fn unwrap(&self) -> ChessColor {
        use ChessPiece::*;
        return match self {
            P(val) => val.clone(),
            R(val) => val.clone(),
            N(val) => val.clone(),
            B(val) => val.clone(),
            Q(val) => val.clone(),
            K(val) => val.clone(),
            _ => panic!("Trying to unwrap None piece"),
        };
    }
}

#[derive(Debug)]
pub struct ChessGame {
    board: [ChessPiece; 64],
    turn: ChessColor,
}

#[derive(Debug,PartialEq,Eq)]
#[allow(dead_code)]
pub struct ChessMove {
    origin: usize,
    target: usize,
    captures: bool,
    promotes: Option<ChessPiece>,
    en_passant: bool,
}

impl ChessMove {
    pub fn to(origin: usize, target: usize) -> ChessMove {
        return ChessMove {
            origin, target,
            captures: false,
            promotes: None,
            en_passant: false,
        };
    }

    pub fn captures(origin: usize, target: usize) -> ChessMove {
        return ChessMove {
            origin, target,
            captures: true,
            promotes: None,
            en_passant: false,
        };
    }
}

impl ChessGame {
    pub fn new() -> ChessGame {
        use ChessPiece::*;
        use ChessColor::*;

        let board: [ChessPiece; 64] = [
            R(White), N(White), B(White), Q(White), K(White), B(White), N(White), R(White),
            P(White), P(White), P(White), P(White), P(White), P(White), P(White), P(White),
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            P(Black), P(Black), P(Black), P(Black), P(Black), P(Black), P(Black), P(Black),
            R(Black), N(Black), B(Black), Q(Black), K(Black), B(Black), N(Black), R(Black),
        ];

        return ChessGame { board, turn: White };
    }

    pub fn load_board(&mut self, board: [ChessPiece; 64]) {
        self.board = board;
    }

    pub fn get_board(&self) -> &[ChessPiece; 64] {
        return &self.board;
    }

    fn step(&self, i: usize, dx: isize, dy: isize) -> Option<usize> {
        let rdy: isize = dy * self.turn.dir();
        let x = (i % 8) as isize + dx;
        let y = (i / 8) as isize + rdy;

        return if 0 <= x && x <= 7 && 0 <= y && y <= 7 {
            Some((8*y+x) as usize)
        } else {
            None
        };
    }

    fn collides(&self, i: usize) -> bool {
        return self.board[i] != ChessPiece::None;
    }

    fn collides_opponent(&self, i: usize) -> bool {
        return self.board[i] != ChessPiece::None
            && self.board[i].unwrap() == ChessColor::Black;
    }

    fn pawn_moves(&self, i: usize, out: &mut Vec<ChessMove>) {
        /* forward step */
        let (collides, pos) = match self.step(i, 0, 1) {
            Some(t) => (self.collides(t), t),
            _ => (true, 0),
        };

        if !collides {
            out.push(ChessMove::to(i, pos));
            if (i < 16 && i > 7) || (i < 56 && i > 47) {
                match self.step(i, 0, 2) {
                    Some(t) => if !self.collides(t) {
                        out.push(ChessMove::to(i, t));
                    },
                    _ => (),
                }
            }
        }

        /* captures */
        match self.step(i, 1, 1) {
            Some(t) => if self.collides_opponent(t) {
                out.push(ChessMove::captures(i, t));
            },
            _ => (),
        }

        match self.step(i, -1, 1) {
            Some(t) => if self.collides_opponent(t) {
                out.push(ChessMove::captures(i, t));
            },
            _ => (),
        }
    }

    fn rook_moves(&self, i: usize, out: &mut Vec<ChessMove>) {
        // out.push(ChessMove::to(i, i+1));
    }

    fn knight_moves(&self, i: usize, out: &mut Vec<ChessMove>) {
        // out.push(ChessMove::to(i, i+1));
    }

    fn bishop_moves(&self, i: usize, out: &mut Vec<ChessMove>) {
        // out.push(ChessMove::to(i, i+1));
    }

    fn queen_moves(&self, i: usize, out: &mut Vec<ChessMove>) {
        // out.push(ChessMove::to(i, i+1));
    }

    fn king_moves(&self, i: usize, out: &mut Vec<ChessMove>) {
        // out.push(ChessMove::to(i, i+1));
    }

    pub fn find_moves(&self) -> Vec<ChessMove> {
        use ChessPiece::*;
        let mut out: Vec<ChessMove> = Vec::new();
        let it = self.board.iter()
                           .enumerate()
                           .filter(|(_, x)| **x != None
                                   && x.unwrap() == self.turn);
        for (i, piece) in it {
            match piece {
                P(_) => self.pawn_moves(i, &mut out),
                R(_) => self.rook_moves(i, &mut out),
                N(_) => self.knight_moves(i, &mut out),
                B(_) => self.bishop_moves(i, &mut out),
                Q(_) => self.queen_moves(i, &mut out),
                K(_) => self.king_moves(i, &mut out),
                _ => unreachable!(),
            };
        }

        return out;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literally_redundant() {
        use ChessPiece::*;
        use ChessColor::*;

        let game = ChessGame::new();
        assert_eq!(*game.get_board(), [
            R(White), N(White), B(White), Q(White), K(White), B(White), N(White), R(White),
            P(White), P(White), P(White), P(White), P(White), P(White), P(White), P(White),
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            P(Black), P(Black), P(Black), P(Black), P(Black), P(Black), P(Black), P(Black),
            R(Black), N(Black), B(Black), Q(Black), K(Black), B(Black), N(Black), R(Black),
        ]);
    }

    #[test]
    fn pawn_moves() {
        use ChessPiece::*;
        use ChessColor::*;

        let mut game = ChessGame::new();
        game.load_board([
            None,     None, None,     None, None,     None,     None, None,
            P(White), None, P(White), None, P(White), P(White), None, None,
            None,     None, None,     None, P(Black), None,     None, None,
            P(White), None, None,     None, None,     None,     None, None,
            None,     None, None,     None, None,     None,     None, None,
            None,     None, None,     None, None,     None,     None, None,
            None,     None, P(White), None, None,     None,     None, None,
            None,     None, None,     None, None,     None,     None, None,
        ]);

        /* Make this not depend on order somehow */
        assert_eq!(game.find_moves(), vec!(
            ChessMove::to(8, 16),
            ChessMove::to(10, 18),
            ChessMove::to(10, 26),
            ChessMove::to(13, 21),
            ChessMove::to(13, 29),
            ChessMove::captures(13, 20),
            ChessMove::to(24, 32),
            ChessMove::to(50, 58),
        ));
    }
}

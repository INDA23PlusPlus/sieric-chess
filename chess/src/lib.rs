#[derive(Debug,Hash,PartialEq,Eq,Clone)]
pub enum ChessColor {
    Wh,
    Bl,
}

impl ChessColor {
    fn dir(&self) -> isize {
        return if *self == ChessColor::Wh { 1 } else { -1 };
    }

    fn opposite(&self) -> ChessColor {
        use ChessColor::*;
        return if *self == Wh { Bl } else { Wh };
    }
}

#[derive(Debug,Clone,Hash,PartialEq,Eq)]
pub enum ChessPiece {
    None,
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

    pub fn str(&self) -> String {
        use ChessPiece::*;

        return String::from(match self {
            P(_) => "",
            R(_) => "R",
            N(_) => "N",
            B(_) => "B",
            Q(_) => "Q",
            K(_) => "K",
            None => "_",
        });
    }

    pub fn to(&self, origin: usize, target: usize) -> ChessMove {
        return ChessMove::to(self.clone(), origin, target);
    }

    pub fn captures(&self, origin: usize, target: usize) -> ChessMove {
        return ChessMove::captures(self.clone(), origin, target);
    }
}

#[derive(Debug,Hash,PartialEq,Eq)]
#[allow(dead_code)]
pub struct ChessMove {
    piece: ChessPiece,
    origin: usize,
    target: usize,
    captures: bool,
    promotes: Option<ChessPiece>,
    en_passant: bool,
}


impl ChessMove {
    pub fn to(piece: ChessPiece, origin: usize, target: usize) -> ChessMove {
        return ChessMove {
            piece, origin, target,
            captures: false,
            promotes: None,
            en_passant: false,
        };
    }

    pub fn captures(piece: ChessPiece, origin: usize, target: usize) -> ChessMove {
        return ChessMove {
            piece, origin, target,
            captures: true,
            promotes: None,
            en_passant: false,
        };
    }

    pub fn algebraic(&self) -> String {
        let piece = self.piece.str();
        let file1 = char::from(97 + (self.origin % 8) as u8);
        let rank1 = self.origin / 8 + 1;
        let file2 = char::from(97 + (self.target % 8) as u8);
        let rank2 = self.target / 8 + 1;
        let captures = if self.captures {"x"} else {""};
        let ep = if self.en_passant {" e.p."} else {""};
        let promotes = match &self.promotes {
            Some(p) => format!(" ({})", p.str()),
            None => String::from(""),
        };
        return format!("{piece}{file1}{rank1}{captures}{file2}{rank2}{ep}{promotes}");
    }
}

#[derive(Debug)]
pub struct ChessGame {
    board: [ChessPiece; 64],
    pub turn: ChessColor,
}

impl ChessGame {
    pub fn new() -> ChessGame {
        use ChessPiece::*;
        use ChessColor::*;

        let board: [ChessPiece; 64] = [
            R(Wh), N(Wh), B(Wh), Q(Wh), K(Wh), B(Wh), N(Wh), R(Wh),
            P(Wh), P(Wh), P(Wh), P(Wh), P(Wh), P(Wh), P(Wh), P(Wh),
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            P(Bl), P(Bl), P(Bl), P(Bl), P(Bl), P(Bl), P(Bl), P(Bl),
            R(Bl), N(Bl), B(Bl), Q(Bl), K(Bl), B(Bl), N(Bl), R(Bl),
        ];

        return ChessGame { board, turn: Wh };
    }

    pub fn load_board(&mut self, board: [ChessPiece; 64]) {
        self.board = board;
    }

    pub fn get_board(&self) -> &[ChessPiece; 64] {
        return &self.board;
    }

    pub fn apply_move(&mut self, mv: &ChessMove) {
        if mv.piece != self.board[mv.origin] {
            eprintln!("Illegal move");
            return;
        }

        self.board[mv.target] = match &mv.promotes {
            Some(p) => p.clone(),
            _ => self.board[mv.origin].clone(),
        };
        self.board[mv.origin] = ChessPiece::None;

        if mv.en_passant {
            self.board[mv.target.wrapping_add_signed(-8*self.turn.dir())]
                = ChessPiece::None;
        }
    }

    pub fn switch_turn(&mut self) {
        self.turn = self.turn.opposite();
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

    fn step_real(&self, i: usize, dx: isize, dy: isize) -> Option<usize> {
        let x = (i % 8) as isize + dx;
        let y = (i / 8) as isize + dy;

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
            && self.board[i].unwrap() == self.turn.opposite();
    }

    fn pawn_moves(&self, i: usize, out: &mut Vec<ChessMove>) {
        let piece = &self.board[i];

        /* forward step */
        let (collides, pos) = match self.step(i, 0, 1) {
            Some(t) => (self.collides(t), t),
            _ => (true, 0),
        };

        if !collides {
            out.push(piece.to(i, pos));
            if (i < 16 && i > 7) || (i < 56 && i > 47) {
                match self.step(i, 0, 2) {
                    Some(t) => if !self.collides(t) {
                        out.push(piece.to(i, t));
                    },
                    _ => (),
                }
            }
        }

        /* captures */
        match self.step(i, 1, 1) {
            Some(t) => if self.collides_opponent(t) {
                out.push(piece.captures(i, t));
            },
            _ => (),
        }

        match self.step(i, -1, 1) {
            Some(t) => if self.collides_opponent(t) {
                out.push(piece.captures(i, t));
            },
            _ => (),
        }
    }

    fn rook_moves(&self, i: usize, out: &mut Vec<ChessMove>) {
        let piece = &self.board[i];

        for j in 1..7 {
            match self.step_real(i, 0, j) {
                Some(t) => if self.collides_opponent(t) {
                    out.push(piece.captures(i, t));
                    break;
                } else if !self.collides(t) {
                    out.push(piece.to(i, t));
                } else {
                    break;
                },
                _ => break,
            }
        }

        for j in 1..7 {
            match self.step_real(i, j, 0) {
                Some(t) => if self.collides_opponent(t) {
                    out.push(piece.captures(i, t));
                    break;
                } else if !self.collides(t) {
                    out.push(piece.to(i, t));
                } else {
                    break;
                },
                _ => break,
            }
        }

        for j in 1..7 {
            match self.step_real(i, 0, -j) {
                Some(t) => if self.collides_opponent(t) {
                    out.push(piece.captures(i, t));
                    break;
                } else if !self.collides(t) {
                    out.push(piece.to(i, t));
                } else {
                    break;
                },
                _ => break,
            }
        }

        for j in 1..7 {
            match self.step_real(i, -j, 0) {
                Some(t) => if self.collides_opponent(t) {
                    out.push(piece.captures(i, t));
                    break;
                } else if !self.collides(t) {
                    out.push(piece.to(i, t));
                } else {
                    break;
                },
                _ => break,
            }
        }
    }

    fn knight_moves(&self, i: usize, out: &mut Vec<ChessMove>) {
        let piece = &self.board[i];

        let targets = [
            self.step_real(i, 1, 2),
            self.step_real(i, -1, 2),
            self.step_real(i, 1, -2),
            self.step_real(i, -1, -2),
            self.step_real(i, 2, 1),
            self.step_real(i, -2, 1),
            self.step_real(i, 2, -1),
            self.step_real(i, -2, -1),
        ];

        for t in targets {
            match t {
                Some(t) => if self.collides_opponent(t) {
                    out.push(piece.captures(i, t));
                } else if !self.collides(t) {
                    out.push(piece.to(i, t));
                },
                _ => (),
            }
        }
    }

    fn bishop_moves(&self, i: usize, out: &mut Vec<ChessMove>) {
        let piece = &self.board[i];

        for j in 1..7 {
            match self.step_real(i, j, j) {
                Some(t) => if self.collides_opponent(t) {
                    out.push(piece.captures(i, t));
                    break;
                } else if !self.collides(t) {
                    out.push(piece.to(i, t));
                } else {
                    break;
                },
                _ => break,
            }
        }

        for j in 1..7 {
            match self.step_real(i, -j, j) {
                Some(t) => if self.collides_opponent(t) {
                    out.push(piece.captures(i, t));
                    break;
                } else if !self.collides(t) {
                    out.push(piece.to(i, t));
                } else {
                    break;
                },
                _ => break,
            }
        }

        for j in 1..7 {
            match self.step_real(i, j, -j) {
                Some(t) => if self.collides_opponent(t) {
                    out.push(piece.captures(i, t));
                    break;
                } else if !self.collides(t) {
                    out.push(piece.to(i, t));
                } else {
                    break;
                },
                _ => break,
            }
        }

        for j in 1..7 {
            match self.step_real(i, -j, -j) {
                Some(t) => if self.collides_opponent(t) {
                    out.push(piece.captures(i, t));
                    break;
                } else if !self.collides(t) {
                    out.push(piece.to(i, t));
                } else {
                    break;
                },
                _ => break,
            }
        }
    }

    fn queen_moves(&self, i: usize, out: &mut Vec<ChessMove>) {
        self.rook_moves(i, out);
        self.bishop_moves(i, out);
    }

    fn king_moves(&self, i: usize, out: &mut Vec<ChessMove>) {
        let piece = &self.board[i];

        let targets = [
            self.step_real(i, 0, 1),
            self.step_real(i, 1, 1),
            self.step_real(i, 1, 0),
            self.step_real(i, 1, -1),
            self.step_real(i, 0, -1),
            self.step_real(i, -1, -1),
            self.step_real(i, -1, 0),
            self.step_real(i, -1, 1),
        ];

        for t in targets {
            match t {
                Some(t) => if self.collides_opponent(t) {
                    out.push(piece.captures(i, t));
                } else if !self.collides(t) {
                    out.push(piece.to(i, t));
                },
                _ => (),
            }
        }
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
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn literally_redundant() {
        use ChessPiece::*;
        use ChessColor::*;

        let game = ChessGame::new();
        assert_eq!(*game.get_board(), [
            R(Wh), N(Wh), B(Wh), Q(Wh), K(Wh), B(Wh), N(Wh), R(Wh),
            P(Wh), P(Wh), P(Wh), P(Wh), P(Wh), P(Wh), P(Wh), P(Wh),
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            P(Bl), P(Bl), P(Bl), P(Bl), P(Bl), P(Bl), P(Bl), P(Bl),
            R(Bl), N(Bl), B(Bl), Q(Bl), K(Bl), B(Bl), N(Bl), R(Bl),
        ]);
    }

    #[test]
    fn pawn_moves() {
        use ChessPiece::*;
        use ChessColor::*;

        let mut game = ChessGame::new();
        game.load_board([
            None,  None, None,  None, None,  None,  None, None,
            P(Wh), None, P(Wh), None, P(Wh), P(Wh), None, None,
            None,  None, None,  None, P(Bl), None,  None, None,
            P(Wh), None, None,  None, None,  None,  None, None,
            None,  None, None,  None, None,  None,  None, None,
            None,  None, None,  None, None,  None,  None, None,
            None,  None, P(Wh), None, None,  None,  None, None,
            None,  None, None,  None, None,  None,  None, None,
        ]);

        /* Make this not depend on order somehow */
        let moves: HashSet<ChessMove> = game.find_moves().into_iter().collect();
        assert_eq!(moves, HashSet::from([
            ChessMove::to(P(Wh), 8, 16),
            ChessMove::to(P(Wh), 10, 18),
            ChessMove::to(P(Wh), 10, 26),
            ChessMove::to(P(Wh), 13, 21),
            ChessMove::to(P(Wh), 13, 29),
            ChessMove::to(P(Wh), 24, 32),
            ChessMove::captures(P(Wh), 13, 20),
            ChessMove::to(P(Wh), 50, 58),
        ]));
    }

    #[test]
    fn rook_moves() {
        use ChessPiece::*;
        use ChessColor::*;

        let mut game = ChessGame::new();
        game.load_board([
            None, None, None, None,  None,  None, None, None,
            None, None, None, None,  None,  None, None, None,
            None, None, None, None,  None,  None, None, None,
            None, None, None, R(Wh), P(Wh), None, None, None,
            None, None, None, None,  None,  None, None, None,
            None, None, None, None,  None,  None, None, None,
            None, None, None, B(Bl), None,  None, None, None,
            None, None, None, None,  None,  None, None, None,
        ]);

        let moves: HashSet<ChessMove> = game.find_moves().into_iter().collect();
        assert_eq!(moves, HashSet::from([
            ChessMove::to(P(Wh), 28, 36),

            ChessMove::to(R(Wh), 27, 19),
            ChessMove::to(R(Wh), 27, 11),
            ChessMove::to(R(Wh), 27, 3),

            ChessMove::to(R(Wh), 27, 35),
            ChessMove::to(R(Wh), 27, 43),
            ChessMove::captures(R(Wh), 27, 51),

            ChessMove::to(R(Wh), 27, 26),
            ChessMove::to(R(Wh), 27, 25),
            ChessMove::to(R(Wh), 27, 24),
        ]));
    }

    #[test]
    fn knight_moves() {
        use ChessPiece::*;
        use ChessColor::*;

        let mut game = ChessGame::new();
        game.load_board([
            None, None, None,  None,  None,  None, None, None,
            None, None, N(Wh), None,  None,  None, None, None,
            None, None, None,  None,  P(Wh), None, None, None,
            None, None, None,  B(Bl), None,  None, None, None,
            None, None, None,  None,  None,  None, None, None,
            None, None, None,  None,  None,  None, None, None,
            None, None, None,  None,  None,  None, None, None,
            None, None, None,  None,  None,  None, None, None,
        ]);

        let moves: HashSet<ChessMove> = game.find_moves().into_iter().collect();
        assert_eq!(moves, HashSet::from([
            ChessMove::to(N(Wh), 10, 0),
            ChessMove::to(N(Wh), 10, 4),
            ChessMove::to(N(Wh), 10, 16),
            ChessMove::to(N(Wh), 10, 25),
            ChessMove::captures(N(Wh), 10, 27),

            ChessMove::to(P(Wh), 20, 28),
            ChessMove::captures(P(Wh), 20, 27),
        ]));
    }

    #[test]
    fn bishop_moves() {
        use ChessPiece::*;
        use ChessColor::*;

        let mut game = ChessGame::new();
        game.load_board([
            None,  None, None, None,  None,  None, None, None,
            None,  None, None, None,  None,  None, None, None,
            None,  None, None, None,  None,  None, None, None,
            None,  None, None, B(Wh), None,  None, None, None,
            None,  None, None, None,  P(Wh), None, None, None,
            None,  None, None, None,  None,  None, None, None,
            B(Bl), None, None, None,  None,  None, None, None,
            None,  None, None, None,  None,  None, None, None,
        ]);

        let moves: HashSet<ChessMove> = game.find_moves().into_iter().collect();
        assert_eq!(moves, HashSet::from([
            ChessMove::to(P(Wh), 36, 44),

            ChessMove::to(B(Wh), 27, 18),
            ChessMove::to(B(Wh), 27, 9),
            ChessMove::to(B(Wh), 27, 0),

            ChessMove::to(B(Wh), 27, 34),
            ChessMove::to(B(Wh), 27, 41),
            ChessMove::captures(B(Wh), 27, 48),

            ChessMove::to(B(Wh), 27, 20),
            ChessMove::to(B(Wh), 27, 13),
            ChessMove::to(B(Wh), 27, 6),
        ]));
    }

    #[test]
    fn queen_moves() {
        use ChessPiece::*;
        use ChessColor::*;

        let mut game = ChessGame::new();
        game.load_board([
            None,  None, None, None,  None,  None, None,  None,
            None,  None, None, None,  None,  None, None,  None,
            None,  None, None, P(Wh), None,  None, None,  None,
            None,  None, None, Q(Wh), None,  None, B(Bl), None,
            None,  None, None, None,  P(Wh), None, None,  None,
            None,  None, None, None,  None,  None, None,  None,
            B(Bl), None, None, None,  None,  None, None,  None,
            None,  None, None, None,  None,  None, None,  None,
        ]);

        let moves: HashSet<ChessMove> = game.find_moves().into_iter().collect();
        assert_eq!(moves, HashSet::from([
            ChessMove::to(P(Wh), 36, 44),

            ChessMove::to(Q(Wh), 27, 18),
            ChessMove::to(Q(Wh), 27, 9),
            ChessMove::to(Q(Wh), 27, 0),

            ChessMove::to(Q(Wh), 27, 34),
            ChessMove::to(Q(Wh), 27, 41),
            ChessMove::captures(Q(Wh), 27, 48),

            ChessMove::to(Q(Wh), 27, 20),
            ChessMove::to(Q(Wh), 27, 13),
            ChessMove::to(Q(Wh), 27, 6),

            ChessMove::to(Q(Wh), 27, 26),
            ChessMove::to(Q(Wh), 27, 25),
            ChessMove::to(Q(Wh), 27, 24),

            ChessMove::to(Q(Wh), 27, 28),
            ChessMove::to(Q(Wh), 27, 29),
            ChessMove::captures(Q(Wh), 27, 30),

            ChessMove::to(Q(Wh), 27, 35),
            ChessMove::to(Q(Wh), 27, 43),
            ChessMove::to(Q(Wh), 27, 51),
            ChessMove::to(Q(Wh), 27, 59),
        ]));
    }

    #[test]
    fn king_moves() {
        use ChessPiece::*;
        use ChessColor::*;

        let mut game = ChessGame::new();
        game.load_board([
            None, None, None,  None,  None, None, None, None,
            None, None, None,  None,  None, None, None, None,
            None, None, None,  P(Wh), None, None, None, None,
            None, None, B(Bl), K(Wh), None, None, None, None,
            None, None, None,  None,  None, None, None, None,
            None, None, None,  None,  None, None, None, None,
            None, None, None,  None,  None, None, None, None,
            None, None, None,  None,  None, None, None, None,
        ]);

        let moves: HashSet<ChessMove> = game.find_moves().into_iter().collect();
        assert_eq!(moves, HashSet::from([
            ChessMove::to(K(Wh), 27, 18),
            ChessMove::to(K(Wh), 27, 20),
            ChessMove::to(K(Wh), 27, 28),
            ChessMove::to(K(Wh), 27, 36),
            ChessMove::to(K(Wh), 27, 35),
            ChessMove::to(K(Wh), 27, 34),
            ChessMove::captures(K(Wh), 27, 26),

            ChessMove::captures(P(Wh), 19, 26),
        ]));
    }

    #[test]
    fn algebraic() {
        use ChessPiece::*;
        use ChessColor::*;

        assert_eq!(ChessMove::to(P(Wh), 4, 12).algebraic(), "e1e2");
        assert_eq!(ChessMove::captures(P(Wh), 4, 11).algebraic(), "e1xd2");
        assert_eq!(ChessMove::to(R(Wh), 7, 23).algebraic(), "Rh1h3");
        let mut m1 = ChessMove::to(P(Wh), 55, 63);
        m1.promotes = Some(Q(Wh));
        assert_eq!(m1.algebraic(), "h7h8 (Q)");
        let mut m2 = ChessMove::captures(P(Wh), 32, 41);
        m2.en_passant = true;
        assert_eq!(m2.algebraic(), "a5xb6 e.p.");
        m2.promotes = Some(B(Wh));
        assert_eq!(m2.algebraic(), "a5xb6 e.p. (B)");
    }
}

#[derive(Debug,PartialEq,Eq)]
pub enum ChessState {
    Normal,
    Check,
}

#[derive(Debug,Copy,Clone,Hash,PartialEq,Eq)]
pub enum ChessColor {
    Wh = 0,
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

#[derive(Debug,Copy,Clone,Hash,PartialEq,Eq)]
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
    pub fn color(&self) -> Option<ChessColor> {
        use ChessPiece::*;
        return match self {
            P(val) => Some(*val),
            R(val) => Some(*val),
            N(val) => Some(*val),
            B(val) => Some(*val),
            Q(val) => Some(*val),
            K(val) => Some(*val),
            _ => Option::None,
        };
    }

    pub fn unwrap(&self) -> ChessColor {
        match self.color() {
            Some(val) => val,
            _ => panic!("Trying to unwrap None piece"),
        }
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

    pub fn captures(&self, origin: usize, target: usize, captures: ChessPiece) -> ChessMove {
        return ChessMove::captures(self.clone(), origin, target, captures);
    }
}

#[derive(Debug,Copy,Clone,Hash,PartialEq,Eq)]
pub struct ChessMove {
    piece: ChessPiece,
    origin: usize,
    target: usize,
    captures: Option<ChessPiece>,
    promotes: Option<ChessPiece>,

    /* these could really be one enum but whatever */
    en_passant: bool,
    castles: bool,
}


impl ChessMove {
    pub fn to(piece: ChessPiece, origin: usize, target: usize) -> ChessMove {
        return ChessMove {
            piece, origin, target,
            captures: None,
            promotes: None,
            en_passant: false,
            castles: false,
        };
    }

    pub fn captures(piece: ChessPiece, origin: usize, target: usize, captures: ChessPiece) -> ChessMove {
        return ChessMove {
            piece, origin, target,
            captures: Some(captures),
            promotes: None,
            en_passant: false,
            castles: false,
        };
    }

    pub fn algebraic(&self) -> String {
        let piece = self.piece.str();
        let file1 = char::from(97 + (self.origin % 8) as u8);
        let rank1 = self.origin / 8 + 1;
        let file2 = char::from(97 + (self.target % 8) as u8);
        let rank2 = self.target / 8 + 1;
        let captures = if self.captures != None {"x"} else {""};
        let ep = if self.en_passant {" e.p."} else {""};
        let promotes = match &self.promotes {
            Some(p) => format!("({})", p.str()),
            None => String::from(""),
        };
        return format!("{piece}{file1}{rank1}{captures}{file2}{rank2}{promotes}{ep}");
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ChessGame {
    board: [ChessPiece; 64],
    temp_board: [ChessPiece; 64],
    can_castle_k: [bool; 2],
    can_castle_q: [bool; 2],
    can_castle_now_k: [bool; 2],
    can_castle_now_q: [bool; 2],
    en_passant_loc: [Option<(usize, usize)>; 2],
    next_moves: [Vec<ChessMove>; 2],
    pub turn: ChessColor,
    pub state: ChessState,
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

        let mut game = ChessGame {
            board,
            temp_board: [None; 64],
            can_castle_k: [true; 2],
            can_castle_q: [true; 2],
            can_castle_now_k: [false; 2],
            can_castle_now_q: [false; 2],
            en_passant_loc: [Option::None; 2],
            next_moves: [Vec::new(), Vec::new()],
            turn: Wh,
            state: ChessState::Normal
        };
        /* HACK: calculate initial game state by doing nothing */
        game.apply_move(&ChessMove::to(None, 16, 16), true);

        return game;
    }

    pub fn load_board(&mut self, board: [ChessPiece; 64]) {
        self.board = board;
        /* HACK: calculate game state by doing nothing */
        self.apply_move(&ChessMove::to(ChessPiece::None, 16, 16), true);
    }

    pub fn get_board(&self) -> &[ChessPiece; 64] {
        return &self.board;
    }

    pub fn switch_turn(&mut self) {
        self.turn = self.turn.opposite();
    }

    pub fn apply_move(&mut self, mv: &ChessMove, real: bool) {
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

        /* ignore lasting effects of non-real moves
         * eg. calls from `apply_temp_move` */
        if !real {
            return;
        }

        /* check which squares can en passant next turn */
        let en_passant_target = (mv.origin + mv.target) / 2;
        /* jank way to check for pawn */
        if mv.piece.str() == "" &&
            (mv.origin as isize - mv.target as isize).abs() == 16 {
            match self.step_real(mv.target, 1, 0) {
                Some(loc) => self.en_passant_loc[0]
                    = Some((loc, en_passant_target)),
                _ => self.en_passant_loc[0] = None,
            }
            match self.step_real(mv.target, -1, 0) {
                Some(loc) => self.en_passant_loc[1]
                    = Some((loc, en_passant_target)),
                _ => self.en_passant_loc[1] = None,
            }
        } else {
            self.en_passant_loc = [None; 2];
        }

        /* check castle eligibility */
        match mv.piece {
            ChessPiece::K(side) => {
                self.can_castle_k[side as usize] = false;
                self.can_castle_q[side as usize] = false;
            },
            ChessPiece::R(side) => {
                if mv.origin == 0 || mv.origin == 56 {
                    self.can_castle_q[side as usize] = false;
                } else if mv.origin == 7 || mv.origin == 63 {
                    self.can_castle_k[side as usize] = false;
                }
            }
            _ => (),
        }

        /* update possible moves for next turn */
        self.next_moves[ChessColor::Wh as usize]
            = self.find_legal_moves(&ChessColor::Wh);
        self.next_moves[ChessColor::Bl as usize]
            = self.find_legal_moves(&ChessColor::Bl);

        /* TODO: place in move generation and save as "next state?"
         * Would be useful for algebraic notation. */
        if self.next_moves[self.turn as usize].iter().any(|x| x.captures == Some(ChessPiece::K(self.turn.opposite()))) {
            self.state = ChessState::Check;
        } else {
            self.state = ChessState::Normal;
        }
    }

    fn mv_captures(&self, origin: usize, target: usize) -> ChessMove {
        return ChessMove::captures(self.board[origin], origin, target, self.board[target]);
    }

    fn mv_en_passant(&self, origin: usize, target: usize) -> ChessMove {
        let piece = self.board[origin];
        let mut mv = ChessMove::to(piece, origin, target);
        mv.captures = match piece.color() {
            Some(col) => Some(ChessPiece::P(col.opposite())),
            _ => None,
        };
        mv.en_passant = true;

        return mv;
    }

    fn apply_temp_move(&mut self, mv: &ChessMove) {
        self.temp_board = self.board;
        self.apply_move(mv, false);
    }

    fn restore_temp_move(&mut self) {
        self.board = self.temp_board;
    }

    fn step(&self, i: usize, dx: isize, dy: isize, side: &ChessColor) -> Option<usize> {
        let rdy: isize = dy * side.dir();
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

    fn collides_opponent(&self, i: usize, side: &ChessColor) -> bool {
        return self.board[i] != ChessPiece::None
            && self.board[i].unwrap() == side.opposite();
    }

    fn pawn_moves(&self, i: usize, out: &mut Vec<ChessMove>, side: &ChessColor) {
        let piece = &self.board[i];

        /* forward step */
        let (collides, pos) = match self.step(i, 0, 1, side) {
            Some(t) => (self.collides(t), t),
            _ => (true, 0),
        };

        if !collides {
            out.push(piece.to(i, pos));
            if (i < 16 && i > 7) || (i < 56 && i > 47) {
                match self.step(i, 0, 2, side) {
                    Some(t) => if !self.collides(t) {
                        out.push(piece.to(i, t));
                    },
                    _ => (),
                }
            }
        }

        /* captures */
        match self.step(i, 1, 1, side) {
            Some(t) => if self.collides_opponent(t, side) {
                out.push(self.mv_captures(i, t));
            },
            _ => (),
        }

        match self.step(i, -1, 1, side) {
            Some(t) => if self.collides_opponent(t, side) {
                out.push(self.mv_captures(i, t));
            },
            _ => (),
        }

        /* en passant */
        for loc in self.en_passant_loc {
            match loc {
                Some((origin, target)) => if i == origin {
                    out.push(self.mv_en_passant(origin, target))
                },
                _ => (),
            }
        }
    }

    fn rook_moves(&self, i: usize, out: &mut Vec<ChessMove>, side: &ChessColor) {
        let piece = &self.board[i];

        for j in 1..7 {
            match self.step_real(i, 0, j) {
                Some(t) => if self.collides_opponent(t, side) {
                    out.push(self.mv_captures(i, t));
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
                Some(t) => if self.collides_opponent(t, side) {
                    out.push(self.mv_captures(i, t));
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
                Some(t) => if self.collides_opponent(t, side) {
                    out.push(self.mv_captures(i, t));
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
                Some(t) => if self.collides_opponent(t, side) {
                    out.push(self.mv_captures(i, t));
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

    fn knight_moves(&self, i: usize, out: &mut Vec<ChessMove>, side: &ChessColor) {
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
                Some(t) => if self.collides_opponent(t, side) {
                    out.push(self.mv_captures(i, t));
                } else if !self.collides(t) {
                    out.push(piece.to(i, t));
                },
                _ => (),
            }
        }
    }

    fn bishop_moves(&self, i: usize, out: &mut Vec<ChessMove>, side: &ChessColor) {
        let piece = &self.board[i];

        for j in 1..7 {
            match self.step_real(i, j, j) {
                Some(t) => if self.collides_opponent(t, side) {
                    out.push(self.mv_captures(i, t));
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
                Some(t) => if self.collides_opponent(t, side) {
                    out.push(self.mv_captures(i, t));
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
                Some(t) => if self.collides_opponent(t, side) {
                    out.push(self.mv_captures(i, t));
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
                Some(t) => if self.collides_opponent(t, side) {
                    out.push(self.mv_captures(i, t));
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

    fn queen_moves(&self, i: usize, out: &mut Vec<ChessMove>, side: &ChessColor) {
        self.rook_moves(i, out, side);
        self.bishop_moves(i, out, side);
    }

    fn king_moves(&self, i: usize, out: &mut Vec<ChessMove>, side: &ChessColor) {
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
                Some(t) => if self.collides_opponent(t, side) {
                    out.push(self.mv_captures(i, t));
                } else if !self.collides(t) {
                    out.push(piece.to(i, t));
                },
                _ => (),
            }
        }
    }

    fn find_moves(&self, side: &ChessColor) -> Vec<ChessMove> {
        use ChessPiece::*;
        let mut out: Vec<ChessMove> = Vec::new();
        let it = self.board.iter()
                           .enumerate()
                           .filter(|(_, x)| **x != None
                                   && x.unwrap() == *side);
        for (i, piece) in it {
            match piece {
                P(_) => self.pawn_moves(i, &mut out, side),
                R(_) => self.rook_moves(i, &mut out, side),
                N(_) => self.knight_moves(i, &mut out, side),
                B(_) => self.bishop_moves(i, &mut out, side),
                Q(_) => self.queen_moves(i, &mut out, side),
                K(_) => self.king_moves(i, &mut out, side),
                _ => unreachable!(),
            };
        }

        return out;
    }

    pub fn is_move_legal(&mut self, side: &ChessColor, mv: &ChessMove) -> bool {
        self.apply_temp_move(&mv);
        let result = self.find_moves(&side.opposite())
                         .iter().all(|x| x.captures
                                     != Some(ChessPiece::K(*side)));
        self.restore_temp_move();
        return result;
    }

    fn find_legal_moves(&mut self, side: &ChessColor) -> Vec<ChessMove> {
        let moves = self.find_moves(side);
        let mut out: Vec<ChessMove> = Vec::new();
        for mv in moves.iter() {
            if self.is_move_legal(side, mv) {
                out.push(*mv);
            }
        }

        return out;
    }

    pub fn get_legal_moves(&self, side: &ChessColor) -> Vec<ChessMove> {
        /* I hate the fact that I have to clone here becase the borrow checker
         * got angry with me */
        return self.next_moves[*side as usize].clone();
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
        let moves: HashSet<ChessMove> = game.get_legal_moves(&game.turn).into_iter().collect();
        assert_eq!(moves, HashSet::from([
            ChessMove::to(P(Wh), 8, 16),
            ChessMove::to(P(Wh), 10, 18),
            ChessMove::to(P(Wh), 10, 26),
            ChessMove::to(P(Wh), 13, 21),
            ChessMove::to(P(Wh), 13, 29),
            ChessMove::to(P(Wh), 24, 32),
            ChessMove::captures(P(Wh), 13, 20, P(Bl)),
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

        let moves: HashSet<ChessMove> = game.get_legal_moves(&game.turn).into_iter().collect();
        assert_eq!(moves, HashSet::from([
            ChessMove::to(P(Wh), 28, 36),

            ChessMove::to(R(Wh), 27, 19),
            ChessMove::to(R(Wh), 27, 11),
            ChessMove::to(R(Wh), 27, 3),

            ChessMove::to(R(Wh), 27, 35),
            ChessMove::to(R(Wh), 27, 43),
            ChessMove::captures(R(Wh), 27, 51, B(Bl)),

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

        let moves: HashSet<ChessMove> = game.get_legal_moves(&game.turn).into_iter().collect();
        assert_eq!(moves, HashSet::from([
            ChessMove::to(N(Wh), 10, 0),
            ChessMove::to(N(Wh), 10, 4),
            ChessMove::to(N(Wh), 10, 16),
            ChessMove::to(N(Wh), 10, 25),
            ChessMove::captures(N(Wh), 10, 27, B(Bl)),

            ChessMove::to(P(Wh), 20, 28),
            ChessMove::captures(P(Wh), 20, 27, B(Bl)),
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

        let moves: HashSet<ChessMove> = game.get_legal_moves(&game.turn).into_iter().collect();
        assert_eq!(moves, HashSet::from([
            ChessMove::to(P(Wh), 36, 44),

            ChessMove::to(B(Wh), 27, 18),
            ChessMove::to(B(Wh), 27, 9),
            ChessMove::to(B(Wh), 27, 0),

            ChessMove::to(B(Wh), 27, 34),
            ChessMove::to(B(Wh), 27, 41),
            ChessMove::captures(B(Wh), 27, 48, B(Bl)),

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

        let moves: HashSet<ChessMove> = game.get_legal_moves(&game.turn).into_iter().collect();
        assert_eq!(moves, HashSet::from([
            ChessMove::to(P(Wh), 36, 44),

            ChessMove::to(Q(Wh), 27, 18),
            ChessMove::to(Q(Wh), 27, 9),
            ChessMove::to(Q(Wh), 27, 0),

            ChessMove::to(Q(Wh), 27, 34),
            ChessMove::to(Q(Wh), 27, 41),
            ChessMove::captures(Q(Wh), 27, 48, B(Bl)),

            ChessMove::to(Q(Wh), 27, 20),
            ChessMove::to(Q(Wh), 27, 13),
            ChessMove::to(Q(Wh), 27, 6),

            ChessMove::to(Q(Wh), 27, 26),
            ChessMove::to(Q(Wh), 27, 25),
            ChessMove::to(Q(Wh), 27, 24),

            ChessMove::to(Q(Wh), 27, 28),
            ChessMove::to(Q(Wh), 27, 29),
            ChessMove::captures(Q(Wh), 27, 30, B(Bl)),

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

        let moves: HashSet<ChessMove> = game.get_legal_moves(&game.turn).into_iter().collect();
        assert_eq!(moves, HashSet::from([
            ChessMove::to(K(Wh), 27, 18),
            ChessMove::to(K(Wh), 27, 20),
            ChessMove::to(K(Wh), 27, 28),
            ChessMove::to(K(Wh), 27, 36),
            ChessMove::to(K(Wh), 27, 34),
            ChessMove::captures(K(Wh), 27, 26, B(Bl)),

            ChessMove::captures(P(Wh), 19, 26, B(Bl)),
        ]));
    }

    #[test]
    fn checkmate() {
        use ChessPiece::*;
        use ChessColor::*;

        let mut game = ChessGame::new();
        game.load_board([
            None, None, None,  K(Bl), None,  None,  None, None,
            None, None, None,  None,  None,  None,  None, None,
            None, None, None,  None,  None,  R(Wh), None, None,
            None, None, None,  None,  R(Wh), None,  None, None,
            None, None, R(Wh), None,  None,  None,  None, None,
            None, None, None,  None,  None,  None,  None, None,
            None, None, None,  None,  None,  None,  None, None,
            None, None, None,  None,  None,  None,  None, None,
        ]);
        game.apply_move(&ChessMove::to(R(Wh), 21, 19), true);
        game.switch_turn();

        assert_eq!(game.state, ChessState::Check);
        let turn = game.turn;
        assert_eq!(game.get_legal_moves(&turn), Vec::new());
    }

    #[test]
    fn stalemate() {
        use ChessPiece::*;
        use ChessColor::*;

        let mut game = ChessGame::new();
        game.load_board([
            None, None, None,  K(Bl), None,  None,  None, None,
            None, None, None,  None,  None,  None,  None, None,
            None, None, None,  None,  None,  R(Wh), None, None,
            None, None, None,  None,  R(Wh), None,  None, None,
            None, None, R(Wh), None,  None,  None,  None, None,
            None, None, None,  None,  None,  None,  None, None,
            None, None, None,  None,  None,  None,  None, None,
            None, None, None,  None,  None,  None,  None, None,
        ]);
        game.apply_move(&ChessMove::to(R(Wh), 21, 13), true);
        game.switch_turn();

        assert_eq!(game.state, ChessState::Normal);
        let turn = game.turn;
        assert_eq!(game.get_legal_moves(&turn), Vec::new());
    }

    #[test]
    fn en_passant() {
        use ChessPiece::*;
        use ChessColor::*;

        let mut game = ChessGame::new();
        game.load_board([
            None,  None,  None, None,  None,  None,  None, None,
            P(Wh), None,  None, None,  P(Wh), None,  None, None,
            None,  None,  None, None,  None,  None,  None, None,
            None,  P(Bl), None, P(Bl), None,  P(Bl), None, None,
            None,  None,  None, None,  None,  None,  None, None,
            None,  None,  None, None,  None,  None,  None, None,
            None,  None,  None, None,  None,  None,  None, None,
            None,  None,  None, None,  None,  None,  None, None,
        ]);
        {
            game.apply_move(&ChessMove::to(P(Wh), 8, 24), true);
            let moves = game.get_legal_moves(&Bl);
            assert!(moves.contains(&game.mv_en_passant(25, 16)));
        }

        {
            game.apply_move(&ChessMove::to(P(Wh), 12, 28), true);
            let moves = game.get_legal_moves(&Bl);
            assert!(moves.contains(&game.mv_en_passant(27, 20)));
            assert!(moves.contains(&game.mv_en_passant(29, 20)));

            assert!(!moves.contains(&game.mv_en_passant(25, 16)));
        }
    }

    #[test]
    fn algebraic() {
        use ChessPiece::*;
        use ChessColor::*;

        assert_eq!(ChessMove::to(P(Wh), 4, 12).algebraic(), "e1e2");
        assert_eq!(ChessMove::captures(P(Wh), 4, 11, P(Bl)).algebraic(), "e1xd2");
        assert_eq!(ChessMove::to(R(Wh), 7, 23).algebraic(), "Rh1h3");
        let mut m1 = ChessMove::to(P(Wh), 55, 63);
        m1.promotes = Some(Q(Wh));
        assert_eq!(m1.algebraic(), "h7h8(Q)");
        let mut m2 = ChessMove::captures(P(Wh), 32, 41, P(Bl));
        m2.en_passant = true;
        assert_eq!(m2.algebraic(), "a5xb6 e.p.");
        m2.en_passant = false;
        m2.promotes = Some(B(Wh));
        assert_eq!(m2.algebraic(), "a5xb6(B)");
    }
}

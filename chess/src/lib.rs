#[derive(Debug,PartialEq,Eq)]
enum ChessState {
    Normal,
    Check,
}

/**
 * Represents one color in chess. Commonly used as indices in arrays when
 * converted to [usize].
 */
#[derive(Debug,Copy,Clone,Hash,PartialEq,Eq)]
pub enum ChessColor {
    Wh = 0,
    Bl,
}

impl ChessColor {
    fn dir(&self) -> isize {
        return if *self == ChessColor::Wh { 1 } else { -1 };
    }

    /**
     * Returns the opposite [ChessColor].
     */
    pub fn opposite(&self) -> ChessColor {
        use ChessColor::*;
        return if *self == Wh { Bl } else { Wh };
    }
}

/**
 * Representation of one chess piece as an enum. Uses common shorthands for
 * pieces in algebraic notation, with pawns being `P`.
 *
 * All pieces except the `None` piece have an associated color.
 */
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
    /**
     * Gets the color of the [ChessPiece] if it isn't [ChessPiece::None].
     * Otherwise return [None].
     */
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

    fn str(&self) -> String {
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

    fn to(&self, origin: usize, target: usize) -> ChessMove {
        return ChessMove::to(self.clone(), origin, target);
    }

    fn captures(&self, origin: usize, target: usize, captures: ChessPiece) -> ChessMove {
        return ChessMove::captures(self.clone(), origin, target, captures);
    }
}

/**
 * Representation of one move in chess.
 */
#[derive(Debug,Copy,Clone,Hash,PartialEq,Eq)]
pub struct ChessMove {
    /**
     * The piece which moves.
     */
    pub piece: ChessPiece,
    /**
     * The original position of the piece.
     */
    pub origin: usize,
    /**
     * The target position of the piece.
     */
    pub target: usize,
    /**
     * The piece being captured or [ChessPiece::None].
     */
    pub captures: ChessPiece,
    /**
     * A piece to replace the current one with after the move or
     * [ChessPiece::None]. Only used for pawn promotions.
     */
    pub promotes: ChessPiece,

    /* these could really be one enum but whatever */
    pub en_passant: bool,
    pub castles: bool,
}


impl ChessMove {
    fn to(piece: ChessPiece, origin: usize, target: usize) -> ChessMove {
        return ChessMove {
            piece, origin, target,
            captures: ChessPiece::None,
            promotes: ChessPiece::None,
            en_passant: false,
            castles: false,
        };
    }

    fn captures(piece: ChessPiece, origin: usize, target: usize, captures: ChessPiece) -> ChessMove {
        return ChessMove {
            piece, origin, target,
            captures: captures,
            promotes: ChessPiece::None,
            en_passant: false,
            castles: false,
        };
    }

    /**
     * Returns the move in algebraic notation (sort of).
     *
     * Does not contain information about checks, and contains redundant
     * information about piece locations.
     */
    pub fn algebraic(&self) -> String {
        if self.castles {
            return String::from(if self.target as isize - self.origin as isize == -2 {
                "O-O-O"
            } else {
                "O-O"
            });
        }

        let piece = self.piece.str();
        let file1 = char::from(b'a' + (self.origin % 8) as u8);
        let rank1 = self.origin / 8 + 1;
        let file2 = char::from(b'a' + (self.target % 8) as u8);
        let rank2 = self.target / 8 + 1;
        let captures = if self.captures != ChessPiece::None {"x"} else {""};
        let ep = if self.en_passant {" e.p."} else {""};
        let promotes = if self.promotes == ChessPiece::None {
            String::from("")
        } else {
            format!("({})", self.promotes.str())
        };

        return format!("{piece}{file1}{rank1}{captures}{file2}{rank2}{promotes}{ep}");
    }
}

/**
 * Representation of one game of chess
 */
#[derive(Debug)]
pub struct ChessGame {
    board: [ChessPiece; 64],
    temp_board: [ChessPiece; 64],
    can_castle_k: [bool; 2],
    can_castle_q: [bool; 2],
    can_castle_now_k: [bool; 2],
    can_castle_now_q: [bool; 2],
    en_passant_loc: [Option<(usize, usize)>; 2],
    next_moves: [Vec<ChessMove>; 2],
    state: ChessState,
    /**
     * The color whose turn it currently is. Can be modified in place, but the
     * helper function [ChessGame::switch_turn] exists to swap it.
     */
    pub turn: ChessColor,
}

impl ChessGame {
    /**
     * Create a new [ChessGame] with the default chess board.
     */
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
        game.apply_move(&ChessMove::to(None, 16, 16));

        return game;
    }

    /**
     * Returns an immutable reference to the current board. Index 0 is `a1` and
     * the array follows rank-major order up to `h8`.
     */
    pub fn get_board(&self) -> &[ChessPiece; 64] {
        return &self.board;
    }

    /**
     * Load a custom board into the game. Disables castling for both players,
     * which can be turned back on using [ChessGame::set_castle_eligibility] or
     * [ChessGame::set_all_castle_eligibility]. Note that castling only works
     * correctly when the king is placed in its normal location.
     */
    pub fn load_board(&mut self, board: [ChessPiece; 64]) {
        self.board = board;

        /* disable castling after loading arbitrary boards */
        self.can_castle_k = [false; 2];
        self.can_castle_q = [false; 2];

        /* HACK: calculate game state by doing nothing */
        self.apply_move(&ChessMove::to(ChessPiece::None, 16, 16));
    }

    /**
     * Sets the eligibility to castle for one player (`side`) on either queens
     * or kings side. When `queens` is true set eligibility for queens side
     * castling to `state`, otherwise do the same for kings side.
     *
     * This could be useful after running [ChessGame::load_board] with custom
     * boards.
     */
    pub fn set_castle_eligibility(&mut self, side: &ChessColor, queens: bool, state: bool) {
        if queens {
            self.can_castle_q[*side as usize] = state;
        } else {
            self.can_castle_k[*side as usize] = state;
        }

        /* HACK: update game state by doing nothing */
        self.apply_move(&ChessMove::to(ChessPiece::None, 0, 0));
    }

    /**
     * Sets the castle eligibility for both black and white on `kings` and
     * `queens` side. The arguments are arrays where the indices are
     * [ChessColor]s as [usize].
     *
     * This could be useful after running [ChessGame::load_board] with custom
     * boards.
     */
    pub fn set_all_castle_eligibility(&mut self, kings: [bool; 2], queens: [bool; 2]) {
        self.can_castle_q = queens;
        self.can_castle_k = kings;

        /* HACK: update game state by doing nothing */
        self.apply_move(&ChessMove::to(ChessPiece::None, 0, 0));
    }

    /**
     * Switches the turn.
     */
    pub fn switch_turn(&mut self) {
        self.turn = self.turn.opposite();
    }

    /**
     * Plays the provided move (`mv`). This does not automatically switch the
     * turn, which must be done using [ChessGame::switch_turn].
     */
    pub fn apply_move(&mut self, mv: &ChessMove) -> bool {
        return self.apply_move_internal(mv, true);
    }

    fn apply_move_internal(&mut self, mv: &ChessMove, real: bool) -> bool {
        /* HACK: Allow moves of None to update game state */
        if mv.piece != ChessPiece::None {
            if mv.piece != self.board[mv.origin] {
                eprintln!("Illegal move: board:{:?} move:{:?}",
                          self.board[mv.origin], mv);
                return false;
            }

            self.board[mv.target] = if mv.promotes == ChessPiece::None {
                mv.piece
            } else {
                mv.promotes
            };
            self.board[mv.origin] = ChessPiece::None;

            if mv.en_passant {
                self.board[mv.target.wrapping_add_signed(-8*self.turn.dir())]
                    = ChessPiece::None;
            }

            if mv.castles {
                let queens = mv.target as isize - mv.origin as isize == -2;
                let rook_origin = mv.origin.wrapping_add_signed(if queens {-4} else {3});
                let rook_target = (mv.target + mv.origin)/2;
                self.board[rook_target] = self.board[rook_origin];
                self.board[rook_origin] = ChessPiece::None;
            }
        }

        /* ignore lasting effects of non-real moves
         * eg. calls from `apply_temp_move` */
        if !real {
            return true;
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

        /* check caste eligibility for next turn */
        self.can_castle_now_q[ChessColor::Wh as usize]
            = self.board[1] == ChessPiece::None
            && self.board[2] == ChessPiece::None
            && self.board[3] == ChessPiece::None;
        self.can_castle_now_k[ChessColor::Wh as usize]
            = self.board[5] == ChessPiece::None
            && self.board[6] == ChessPiece::None;
        self.can_castle_now_q[ChessColor::Bl as usize]
            = self.board[57] == ChessPiece::None
            && self.board[58] == ChessPiece::None
            && self.board[59] == ChessPiece::None;
        self.can_castle_now_k[ChessColor::Bl as usize]
            = self.board[61] == ChessPiece::None
            && self.board[62] == ChessPiece::None;

        for mv in self.next_moves[ChessColor::Wh as usize].iter() {
            if mv.target == 58 || mv.target == 59 || mv.target == 60 {
                self.can_castle_now_q[ChessColor::Bl as usize] = false;
            }
            if mv.target == 60 || mv.target == 61 || mv.target == 62 {
                self.can_castle_now_k[ChessColor::Bl as usize] = false;
            }
        }
        for mv in self.next_moves[ChessColor::Bl as usize].iter() {
            if mv.target == 2 || mv.target == 3 || mv.target == 4 {
                self.can_castle_now_q[ChessColor::Wh as usize] = false;
            }
            if mv.target == 4 || mv.target == 5 || mv.target == 6 {
                self.can_castle_now_k[ChessColor::Wh as usize] = false;
            }
        }

        /* update possible moves again since
         * castle eligibility may have changed */
        self.next_moves[ChessColor::Wh as usize]
            = self.find_legal_moves(&ChessColor::Wh);
        self.next_moves[ChessColor::Bl as usize]
            = self.find_legal_moves(&ChessColor::Bl);

        /* TODO: place in move generation and save as "next state?"
         * Would be useful for algebraic notation. */
        if self.next_moves[self.turn as usize].iter().any(|x| x.captures == ChessPiece::K(self.turn.opposite())) {
            self.state = ChessState::Check;
        } else {
            self.state = ChessState::Normal;
        }

        return true;
    }

    fn mv_promotion(&self, mv: ChessMove) -> Vec<ChessMove> {
        let col = match mv.piece.color() {
            Some(col) => col,
            _ => return Vec::new(),
        };

        if mv.target > 55 || mv.target < 8 {
            use ChessPiece::*;

            let mut out: Vec<ChessMove> = Vec::new();
            for p in [R(col), N(col), B(col), Q(col)] {
                let mut new_mv = mv;
                new_mv.promotes = p;
                out.push(new_mv);
            }
            return out;
        } else {
            return vec![mv];
        }
    }

    fn mv_captures(&self, origin: usize, target: usize) -> ChessMove {
        return self.board[origin].captures(origin, target, self.board[target]);
    }

    fn mv_en_passant(&self, origin: usize, target: usize) -> ChessMove {
        let piece = self.board[origin];
        let mut mv = ChessMove::to(piece, origin, target);
        mv.captures = match piece.color() {
            Some(col) => ChessPiece::P(col.opposite()),
            _ => ChessPiece::None,
        };
        mv.en_passant = true;

        return mv;
    }

    fn mv_castle(&self, side: &ChessColor, queens: bool) -> ChessMove {
        use ChessPiece::*;
        use ChessColor::*;

        let king = if *side == Wh {4} else {60};
        let mut mv = ChessMove::to(K(*side), king, if queens {king - 2} else {king + 2});
        mv.castles = true;
        return mv;
    }

    fn apply_temp_move(&mut self, mv: &ChessMove) {
        self.temp_board = self.board;
        self.apply_move_internal(mv, false);
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
        return match self.board[i].color() {
            Some(col) => col == side.opposite(),
            _ => false
        }
    }

    fn pawn_moves(&self, i: usize, out: &mut Vec<ChessMove>, side: &ChessColor) {
        let piece = &self.board[i];

        /* forward step */
        let (collides, pos) = match self.step(i, 0, 1, side) {
            Some(t) => (self.collides(t), t),
            _ => (true, 0),
        };

        if !collides {
            out.extend(self.mv_promotion(piece.to(i, pos)));
            if (i < 16 && i > 7) || (i < 56 && i > 47) {
                match self.step(i, 0, 2, side) {
                    Some(t) => if !self.collides(t) {
                        /* NOTE: can never be a promotion */
                        out.push(piece.to(i, t));
                    },
                    _ => (),
                }
            }
        }

        /* captures */
        for pos in [self.step(i, 1, 1, side), self.step(i, -1, 1, side)] {
            match pos {
                Some(t) => if self.collides_opponent(t, side) {
                    out.extend(self.mv_promotion(self.mv_captures(i, t)));
                },
                _ => (),
            }
        }

        /* en passant */
        for loc in self.en_passant_loc {
            match loc {
                Some((origin, target)) => if i == origin {
                    /* NOTE: can never be a promotion */
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
                                   && x.color().unwrap() == *side);
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

        if self.can_castle_k[*side as usize] && self.can_castle_now_k[*side as usize] {
            out.push(self.mv_castle(side, false));
        }
        if self.can_castle_q[*side as usize] && self.can_castle_now_q[*side as usize] {
            out.push(self.mv_castle(side, true));
        }

        return out;
    }

    fn is_move_legal(&mut self, side: &ChessColor, mv: &ChessMove) -> bool {
        self.apply_temp_move(&mv);
        let result = self.find_moves(&side.opposite())
                         .iter().all(|x| x.captures
                                     != ChessPiece::K(*side));
        self.restore_temp_move();
        return result;
    }

    fn find_legal_moves(&mut self, side: &ChessColor) -> Vec<ChessMove> {
        return self.find_moves(side)
                   .iter()
                   .filter(|mv| self.is_move_legal(side, mv))
                   .copied()
                   .collect();
    }

    /**
     * Gets all legal moves for one `side`. This does not have to be the side
     * whose turn it is right now.
     *
     * Will usually be called like this: `game.get_legal_moves(&game.turn);`
     */
    pub fn get_legal_moves(&self, side: &ChessColor) -> Vec<ChessMove> {
        /* I hate the fact that I have to clone here becase the borrow checker
         * got angry with me */
        return self.next_moves[*side as usize].clone();
    }

    /**
     * Returns [true] if the game is over.
     */
    pub fn is_ended(&self) -> bool {
        return self.next_moves[self.turn as usize].is_empty();
    }

    /**
     * Returns [true] if the current side is in check.
     */
    pub fn is_check(&self) -> bool {
        return self.state == ChessState::Check;
    }

    /**
     * Returns [true] if the game is over in checkmate.
     */
    pub fn is_checkmate(&self) -> bool {
        return self.is_ended() && self.is_check();
    }

    /**
     * Returns [true] if the game is over in stalemate.
     */
    pub fn is_stalemate(&self) -> bool {
        return self.is_ended() && !self.is_check();
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
            None,  None, P(Wh), None, None,  None,  None, None,
            None,  None, None,  None, None,  None,  None, None,
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
            ChessMove::to(P(Wh), 42, 50),
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
        game.apply_move(&ChessMove::to(R(Wh), 21, 19));
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
        game.apply_move(&ChessMove::to(R(Wh), 21, 13));
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
            game.apply_move(&ChessMove::to(P(Wh), 8, 24));
            let moves = game.get_legal_moves(&Bl);
            assert!(moves.contains(&game.mv_en_passant(25, 16)));
        }

        {
            game.apply_move(&ChessMove::to(P(Wh), 12, 28));
            let moves = game.get_legal_moves(&Bl);
            assert!(moves.contains(&game.mv_en_passant(27, 20)));
            assert!(moves.contains(&game.mv_en_passant(29, 20)));

            assert!(!moves.contains(&game.mv_en_passant(25, 16)));
        }
    }

    #[test]
    fn castling_1() {
        use ChessPiece::*;
        use ChessColor::*;

        let mut game = ChessGame::new();
        game.load_board([
            R(Wh), N(Wh), None, None, K(Wh), None, None, R(Wh),
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, K(Bl), None, None, None,
        ]);
        game.set_all_castle_eligibility([true; 2], [true; 2]);

        let moves = game.get_legal_moves(&Wh);
        assert!(moves.contains(&game.mv_castle(&Wh, false)));
        assert!(!moves.contains(&game.mv_castle(&Wh, true)));
    }

    #[test]
    fn castling_2() {
        use ChessPiece::*;
        use ChessColor::*;

        let mut game = ChessGame::new();
        game.load_board([
            R(Wh), None, None, None, K(Wh), None, None, R(Wh),
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, Q(Bl), None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, K(Bl), None, None, None,
        ]);
        game.set_all_castle_eligibility([true; 2], [true; 2]);

        let moves = game.get_legal_moves(&Wh);
        assert!(moves.contains(&game.mv_castle(&Wh, false)));
        assert!(!moves.contains(&game.mv_castle(&Wh, true)));

        game.apply_move(&game.mv_castle(&Wh, false));
        assert_eq!(game.get_board(), &[
            R(Wh), None, None, None, None, R(Wh), K(Wh), None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, Q(Bl), None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, K(Bl), None, None, None,
        ]);
    }

    #[test]
    fn castling_3() {
        use ChessPiece::*;
        use ChessColor::*;

        let mut game = ChessGame::new();
        game.load_board([
            None, None, None, None, K(Wh), None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, Q(Wh), None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            R(Bl), None, None, None, K(Bl), None, None, R(Bl),
        ]);
        game.set_all_castle_eligibility([true; 2], [true; 2]);

        let moves = game.get_legal_moves(&Bl);
        assert!(!moves.contains(&game.mv_castle(&Bl, false)));
        assert!(!moves.contains(&game.mv_castle(&Bl, true)));
    }

    #[test]
    fn castling_4() {
        use ChessPiece::*;
        use ChessColor::*;

        let mut game = ChessGame::new();
        game.load_board([
            None, None, None, None, K(Wh), None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, Q(Wh), None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            R(Bl), None, None, None, K(Bl), None, None, R(Bl),
        ]);
        game.set_all_castle_eligibility([true; 2], [true; 2]);

        let moves = game.get_legal_moves(&Bl);
        assert!(!moves.contains(&game.mv_castle(&Bl, false)));
        assert!(moves.contains(&game.mv_castle(&Bl, true)));

        game.apply_move(&game.mv_castle(&Bl, true));
        assert_eq!(game.get_board(), &[
            None, None, None, None, K(Wh), None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, Q(Wh), None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, K(Bl), R(Bl), None, None, None, R(Bl),
        ]);
    }

    #[test]
    fn promotion() {
        use ChessPiece::*;
        use ChessColor::*;

        let mut game = ChessGame::new();
        game.load_board([
            None, None, None, None, K(Wh), None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, P(Wh), None,
            None, None, None, None, K(Bl), B(Bl), None, R(Bl),
        ]);

        let moves: HashSet<ChessMove> = game.get_legal_moves(&game.turn).into_iter().collect();
        {
            let moves2: HashSet<ChessMove>
                = game.mv_promotion(ChessMove::to(P(Wh), 54, 62)).into_iter().collect();
            assert!(moves.is_superset(&moves2))
        }
        {
            let moves2: HashSet<ChessMove>
                = game.mv_promotion(ChessMove::captures(P(Wh), 54, 61, B(Bl))).into_iter().collect();
            assert!(moves.is_superset(&moves2))
        }
        {
            let moves2: HashSet<ChessMove>
                = game.mv_promotion(ChessMove::captures(P(Wh), 54, 63, R(Bl))).into_iter().collect();
            assert!(moves.is_superset(&moves2))
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
        m1.promotes = Q(Wh);
        assert_eq!(m1.algebraic(), "h7h8(Q)");
        let mut m2 = ChessMove::captures(P(Wh), 32, 41, P(Bl));
        m2.en_passant = true;
        assert_eq!(m2.algebraic(), "a5xb6 e.p.");
        m2.en_passant = false;
        m2.promotes = B(Wh);
        assert_eq!(m2.algebraic(), "a5xb6(B)");
    }
}

use std::fmt;
use std::ascii::AsciiExt;
use std::char;
use std::ops;
use std::str;

use square::Square;

pub use self::Color::{Black, White};
pub use self::Role::{Pawn, Knight, Bishop, Rook, Queen, King};

/// `White` or `Black`.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn from_char(ch: char) -> Option<Color> {
        match ch {
            'w' => Some(Color::White),
            'b' => Some(Color::Black),
            _ => None
        }
    }

    pub fn from_bool(white: bool) -> Color {
        if white { Color::White } else { Color::Black }
    }

    pub fn fold<T>(self, white: T, black: T) -> T {
        match self {
            Color::White => white,
            Color::Black => black
        }
    }

    pub fn is_white(self) -> bool { self == Color::White }
    pub fn is_black(self) -> bool { self == Color::Black }

    pub fn char(self) -> char { self.fold('w', 'b') }

    pub const fn pawn(self)   -> Piece { Pawn.of(self) }
    pub const fn knight(self) -> Piece { Knight.of(self) }
    pub const fn bishop(self) -> Piece { Bishop.of(self) }
    pub const fn rook(self)   -> Piece { Rook.of(self) }
    pub const fn queen(self)  -> Piece { Queen.of(self) }
    pub const fn king(self)   -> Piece { King.of(self) }
}

impl ops::Not for Color {
    type Output = Color;

    fn not(self) -> Color {
        self.fold(Color::Black, Color::White)
    }
}

/// Piece types: `Pawn`, `Knight`, `Bishop`, `Rook`, `Queen`, `King`.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Role {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Role {
    pub fn from_char(ch: char) -> Option<Role> {
        match ch {
            'p' => Some(Role::Pawn),
            'n' => Some(Role::Knight),
            'b' => Some(Role::Bishop),
            'r' => Some(Role::Rook),
            'q' => Some(Role::Queen),
            'k' => Some(Role::King),
            _ => None
        }
    }

    pub const fn of(self, color: Color) -> Piece {
        Piece { color, role: self }
    }

    pub fn char(self) -> char {
        match self {
            Role::Pawn =>   'p',
            Role::Knight => 'n',
            Role::Bishop => 'b',
            Role::Rook =>   'r',
            Role::Queen =>  'q',
            Role::King =>   'k'
        }
    }
}

pub const ROLES: [Role; 6] = [Pawn, Knight, Bishop, Rook, Queen, King];

/// A piece with `Color` and `Role`.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Piece {
    pub color: Color,
    pub role: Role,
}

impl Piece {
    pub fn char(&self) -> char {
        self.color.fold(self.role.char().to_ascii_uppercase(), self.role.char())
    }

    pub fn from_char(ch: char) -> Option<Piece> {
        if ch == ch.to_ascii_lowercase() {
            Role::from_char(ch).map(|role| role.of(Color::Black))
        } else {
            Role::from_char(ch.to_ascii_lowercase()).map(|role| role.of(Color::White))
        }
    }
}

/// Information about a move.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Move {
    Normal { role: Role, from: Square, capture: Option<Role>, to: Square, promotion: Option<Role> },
    EnPassant { from: Square, to: Square },
    Castle { king: Square, rook: Square },
    Put { role: Role, to: Square },
    Null
}

impl Move {
    pub fn to_uci(&self) -> Uci {
        match *self {
            Move::Normal { from, to, promotion, .. } =>
                Uci::Normal { from, to, promotion },
            Move::EnPassant { from, to, .. } =>
                Uci::Normal { from, to, promotion: None },
            Move::Castle { king, rook } =>
                Uci::Normal { from: king, to: rook, promotion: None },  // Chess960-style
            Move::Put { role, to } =>
                Uci::Put { role, to },
            Move::Null =>
                Uci::Null
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Move::Normal { role, from, capture, to, promotion } => {
                if role != Role::Pawn {
                    try!(write!(f, "{}", role.char().to_ascii_uppercase()));
                }

                try!(write!(f, "{}{}{}", from, if capture.is_some() { 'x' } else { '-' }, to));

                if let Some(p) = promotion {
                    try!(write!(f, "={}", p.char().to_ascii_uppercase()));
                }

                Ok(())
            },
            Move::EnPassant { from, to, .. } => {
                write!(f, "{}x{}", from, to)
            },
            Move::Castle { king, rook } => {
                if king < rook {
                    write!(f, "O-O")
                } else {
                    write!(f, "O-O-O")
                }
            },
            Move::Put { role, to } => {
                write!(f, "{}@{}", role.char().to_ascii_uppercase(), to)
            },
            Move::Null => {
                write!(f, "--")
            }
        }
    }
}

/// A move as represented in the UCI protocol.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Uci {
    Normal { from: Square, to: Square, promotion: Option<Role> },
    Put { role: Role, to: Square },
    Null
}

impl str::FromStr for Uci {
    type Err = ();

    fn from_str(uci: &str) -> Result<Uci, ()> {
        if uci.len() < 4 || uci.len() > 5 {
            return Err(())
        }

        match (Square::from_str(&uci[0..2]), Square::from_str(&uci[2..4]), uci.chars().nth(4)) {
            (Ok(from), Ok(to), Some(promotion)) =>
                return Role::from_char(promotion).map(|role| {
                    Uci::Normal { from, to, promotion: Some(role) }
                }).ok_or(()),
            (Ok(from), Ok(to), None) =>
                return Ok(Uci::Normal { from, to, promotion: None }),
            _ => ()
        }

        match (uci.chars().nth(0), uci.chars().nth(1), Square::from_str(&uci[2..4])) {
            (Some(piece), Some('@'), Ok(to)) =>
                return Role::from_char(piece.to_ascii_lowercase()).map(|role| {
                    Uci::Put { role, to }
                }).ok_or(()),
            _ => ()
        }

        if uci == "0000" {
            return Ok(Uci::Null)
        }

        Err(())
    }
}

impl fmt::Display for Uci {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Uci::Normal { from, to, promotion: None } =>
                write!(f, "{}{}", from, to),
            Uci::Normal { from, to, promotion: Some(promotion) } =>
                write!(f, "{}{}{}", from, to, promotion.char()),
            Uci::Put { to, role } =>
                write!(f, "{}@{}", role.char().to_ascii_uppercase(), to),
            Uci::Null =>
                write!(f, "0000")
        }
    }
}

/// A players Crazyhouse pocket.
#[derive(Clone, Default)]
pub struct Pocket {
    pub pawns: u8,
    pub knights: u8,
    pub bishops: u8,
    pub rooks: u8,
    pub queens: u8,
    pub kings: u8,
}

impl Pocket {
    pub fn by_role(&self, role: Role) -> u8 {
        match role {
            Role::Pawn   => self.pawns,
            Role::Knight => self.knights,
            Role::Bishop => self.bishops,
            Role::Rook   => self.rooks,
            Role::Queen  => self.queens,
            Role::King   => self.kings,
        }
    }

    pub fn mut_by_role(&mut self, role: Role) -> &mut u8 {
        match role {
            Role::Pawn   => &mut self.pawns,
            Role::Knight => &mut self.knights,
            Role::Bishop => &mut self.bishops,
            Role::Rook   => &mut self.rooks,
            Role::Queen  => &mut self.queens,
            Role::King   => &mut self.kings,
        }
    }
}

/// Crazyhouse pockets for both sides, holding captured pieces.
#[derive(Clone, Default)]
pub struct Pockets {
    pub white: Pocket,
    pub black: Pocket,
}

impl Pockets {
    pub fn by_color(&self, color: Color) -> &Pocket {
        color.fold(&self.white, &self.black)
    }

    pub fn mut_by_color(&mut self, color: Color) -> &mut Pocket {
        color.fold(&mut self.white, &mut self.black)
    }

    pub fn by_piece(&self, piece: Piece) -> u8 {
        self.by_color(piece.color).by_role(piece.role)
    }

    pub fn mut_by_piece(&mut self, piece: Piece) -> &mut u8 {
        self.mut_by_color(piece.color).mut_by_role(piece.role)
    }
}

impl fmt::Display for Pockets {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for color in &[White, Black] {
            for role in &ROLES {
                let piece = Piece { color: *color, role: *role };
                try!(write!(f, "{}", piece.char().to_string().repeat(self.by_piece(piece) as usize)));
            }
        }
        Ok(())
    }
}

/// The number of checks the respective side needs to give in order to win
/// (in a game of Three-check).
#[derive(Clone)]
pub struct RemainingChecks {
    pub white: u8,
    pub black: u8,
}

impl Default for RemainingChecks {
    fn default() -> RemainingChecks {
        RemainingChecks { white: 3, black: 3 }
    }
}

impl RemainingChecks {
    pub fn by_color(&self, color: Color) -> u8 {
        color.fold(self.white, self.black)
    }

    pub fn mut_by_color(&mut self, color: Color) -> &mut u8 {
        color.fold(&mut self.white, &mut self.black)
    }
}

impl fmt::Display for RemainingChecks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}+{}", self.white, self.black)
    }
}

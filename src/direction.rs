use enumflags2::{BitFlags, bitflags, make_bitflags};

use crate::geometry::CoordinateOffset;

/// Directions in the world plane.
#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
  Left = 1 << 0,
  Right = 1 << 1,
  Up = 1 << 2,
  Down = 1 << 3,
}

/// Flags representing a combination of directions.
/// Can be used for connectivity or sprite variants
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub struct DirectionFlags(BitFlags<Direction>);

const OFFSETS: [(Direction, CoordinateOffset); 4] = [
  (Direction::Left, CoordinateOffset::new(-1, 0)),
  (Direction::Right, CoordinateOffset::new(1, 0)),
  (Direction::Up, CoordinateOffset::new(0, -1)),
  (Direction::Down, CoordinateOffset::new(0, 1)),
];

impl From<Direction> for DirectionFlags {
  fn from(direction: Direction) -> DirectionFlags {
    DirectionFlags(BitFlags::from_flag(direction))
  }
}

impl From<u8> for DirectionFlags {
  fn from(bits: u8) -> DirectionFlags {
    DirectionFlags(BitFlags::from_bits_truncate(bits))
  }
}

impl Direction {
  pub fn opposite(self) -> Direction {
    match self {
      Direction::Left => Direction::Right,
      Direction::Right => Direction::Left,
      Direction::Up => Direction::Down,
      Direction::Down => Direction::Up,
    }
  }
}

impl DirectionFlags {
  pub const NONE: DirectionFlags = DirectionFlags(BitFlags::EMPTY);
  pub const HORIZONTAL: DirectionFlags = DirectionFlags(make_bitflags!(Direction::{Left | Right}));
  pub const VERTICAL: DirectionFlags = DirectionFlags(make_bitflags!(Direction::{Up | Down}));

  pub fn contains(self, direction: Direction) -> bool {
    self.0.contains(direction)
  }

  pub fn iter_direction_offset_pairs(self) -> impl Iterator<Item = (Direction, CoordinateOffset)> {
    OFFSETS.iter().copied().filter_map(move |(dir, offset)| {
      if self.0.contains(dir) {
        Some((dir, offset))
      } else {
        None
      }
    })
  }

  pub fn iter_offsets(self) -> impl Iterator<Item = CoordinateOffset> {
    self.iter_direction_offset_pairs().map(|(_, o)| o)
  }
}

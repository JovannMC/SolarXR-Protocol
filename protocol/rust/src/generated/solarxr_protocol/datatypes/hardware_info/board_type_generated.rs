// automatically generated by the FlatBuffers compiler, do not modify
// @generated
extern crate alloc;
extern crate flatbuffers;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::mem;
use core::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_BOARD_TYPE: u16 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_BOARD_TYPE: u16 = 12;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_BOARD_TYPE: [BoardType; 13] = [
  BoardType::UNKNOWN,
  BoardType::SLIMEVR_LEGACY,
  BoardType::SLIMEVR_DEV,
  BoardType::NODEMCU,
  BoardType::CUSTOM,
  BoardType::WROOM32,
  BoardType::WEMOSD1MINI,
  BoardType::TTGO_TBASE,
  BoardType::ESP01,
  BoardType::SLIMEVR,
  BoardType::LOLIN_C3_MINI,
  BoardType::BEETLE32C3,
  BoardType::ES32C3DEVKITM1,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct BoardType(pub u16);
#[allow(non_upper_case_globals)]
impl BoardType {
  pub const UNKNOWN: Self = Self(0);
  pub const SLIMEVR_LEGACY: Self = Self(1);
  pub const SLIMEVR_DEV: Self = Self(2);
  pub const NODEMCU: Self = Self(3);
  pub const CUSTOM: Self = Self(4);
  pub const WROOM32: Self = Self(5);
  pub const WEMOSD1MINI: Self = Self(6);
  pub const TTGO_TBASE: Self = Self(7);
  pub const ESP01: Self = Self(8);
  pub const SLIMEVR: Self = Self(9);
  pub const LOLIN_C3_MINI: Self = Self(10);
  pub const BEETLE32C3: Self = Self(11);
  pub const ES32C3DEVKITM1: Self = Self(12);

  pub const ENUM_MIN: u16 = 0;
  pub const ENUM_MAX: u16 = 12;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::UNKNOWN,
    Self::SLIMEVR_LEGACY,
    Self::SLIMEVR_DEV,
    Self::NODEMCU,
    Self::CUSTOM,
    Self::WROOM32,
    Self::WEMOSD1MINI,
    Self::TTGO_TBASE,
    Self::ESP01,
    Self::SLIMEVR,
    Self::LOLIN_C3_MINI,
    Self::BEETLE32C3,
    Self::ES32C3DEVKITM1,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::UNKNOWN => Some("UNKNOWN"),
      Self::SLIMEVR_LEGACY => Some("SLIMEVR_LEGACY"),
      Self::SLIMEVR_DEV => Some("SLIMEVR_DEV"),
      Self::NODEMCU => Some("NODEMCU"),
      Self::CUSTOM => Some("CUSTOM"),
      Self::WROOM32 => Some("WROOM32"),
      Self::WEMOSD1MINI => Some("WEMOSD1MINI"),
      Self::TTGO_TBASE => Some("TTGO_TBASE"),
      Self::ESP01 => Some("ESP01"),
      Self::SLIMEVR => Some("SLIMEVR"),
      Self::LOLIN_C3_MINI => Some("LOLIN_C3_MINI"),
      Self::BEETLE32C3 => Some("BEETLE32C3"),
      Self::ES32C3DEVKITM1 => Some("ES32C3DEVKITM1"),
      _ => None,
    }
  }
}
impl core::fmt::Debug for BoardType {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for BoardType {
  type Inner = Self;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<u16>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for BoardType {
    type Output = BoardType;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<u16>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for BoardType {
  type Scalar = u16;
  #[inline]
  fn to_little_endian(self) -> u16 {
    self.0.to_le()
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(v: u16) -> Self {
    let b = u16::from_le(v);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for BoardType {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    u16::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for BoardType {}

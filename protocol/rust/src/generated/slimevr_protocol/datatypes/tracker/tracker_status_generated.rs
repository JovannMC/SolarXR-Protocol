// automatically generated by the FlatBuffers compiler, do not modify
extern crate flatbuffers;
use std::mem;
use std::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum TrackerStatusOffset {}
#[derive(Copy, Clone, PartialEq)]

/// Contains all the relevant sensor data about a tracker. A tracker is anything that
/// provides kinematic data about a particular body part.
///
/// Trackers may be synthetic/computed or instead part of an actual hardware device.
pub struct TrackerStatus<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for TrackerStatus<'a> {
  type Inner = TrackerStatus<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> TrackerStatus<'a> {
  pub const VT_ROLE: flatbuffers::VOffsetT = 4;
  pub const VT_ORIENTATION: flatbuffers::VOffsetT = 6;
  pub const VT_POSITION: flatbuffers::VOffsetT = 8;
  pub const VT_RAW_ROT_VEL: flatbuffers::VOffsetT = 10;
  pub const VT_RAW_TRANS_ACCEL: flatbuffers::VOffsetT = 12;
  pub const VT_TEMP: flatbuffers::VOffsetT = 14;
  pub const VT_POLL_RATE: flatbuffers::VOffsetT = 16;
  pub const VT_MOUNTING_ORIENTATION: flatbuffers::VOffsetT = 18;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    TrackerStatus { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args TrackerStatusArgs<'args>
  ) -> flatbuffers::WIPOffset<TrackerStatus<'bldr>> {
    let mut builder = TrackerStatusBuilder::new(_fbb);
    if let Some(x) = args.mounting_orientation { builder.add_mounting_orientation(x); }
    if let Some(x) = args.poll_rate { builder.add_poll_rate(x); }
    if let Some(x) = args.temp { builder.add_temp(x); }
    if let Some(x) = args.raw_trans_accel { builder.add_raw_trans_accel(x); }
    if let Some(x) = args.raw_rot_vel { builder.add_raw_rot_vel(x); }
    if let Some(x) = args.position { builder.add_position(x); }
    if let Some(x) = args.orientation { builder.add_orientation(x); }
    if let Some(x) = args.role { builder.add_role(x); }
    builder.finish()
  }


  /// The user-assigned role of the tracker.
  #[inline]
  pub fn role(&self) -> Option<super::TrackerRole> {
    self._tab.get::<super::TrackerRole>(TrackerStatus::VT_ROLE, None)
  }
  #[inline]
  pub fn orientation(&self) -> Option<&'a super::math::Quat> {
    self._tab.get::<super::math::Quat>(TrackerStatus::VT_ORIENTATION, None)
  }
  /// Position, in meters
  #[inline]
  pub fn position(&self) -> Option<&'a super::math::Vec3f> {
    self._tab.get::<super::math::Vec3f>(TrackerStatus::VT_POSITION, None)
  }
  /// Raw rotational velocity, in euler angles
  #[inline]
  pub fn raw_rot_vel(&self) -> Option<&'a super::math::Vec3f> {
    self._tab.get::<super::math::Vec3f>(TrackerStatus::VT_RAW_ROT_VEL, None)
  }
  /// Raw translational acceleration, in m/s^2
  #[inline]
  pub fn raw_trans_accel(&self) -> Option<&'a super::math::Vec3f> {
    self._tab.get::<super::math::Vec3f>(TrackerStatus::VT_RAW_TRANS_ACCEL, None)
  }
  /// Temperature in degrees celsius
  #[inline]
  pub fn temp(&self) -> Option<f32> {
    self._tab.get::<f32>(TrackerStatus::VT_TEMP, None)
  }
  /// average samples per second
  #[inline]
  pub fn poll_rate(&self) -> Option<f32> {
    self._tab.get::<f32>(TrackerStatus::VT_POLL_RATE, None)
  }
  /// The orientation of the tracker when mounted on the body
  #[inline]
  pub fn mounting_orientation(&self) -> Option<&'a super::math::Quat> {
    self._tab.get::<super::math::Quat>(TrackerStatus::VT_MOUNTING_ORIENTATION, None)
  }
}

impl flatbuffers::Verifiable for TrackerStatus<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<super::TrackerRole>("role", Self::VT_ROLE, false)?
     .visit_field::<super::math::Quat>("orientation", Self::VT_ORIENTATION, false)?
     .visit_field::<super::math::Vec3f>("position", Self::VT_POSITION, false)?
     .visit_field::<super::math::Vec3f>("raw_rot_vel", Self::VT_RAW_ROT_VEL, false)?
     .visit_field::<super::math::Vec3f>("raw_trans_accel", Self::VT_RAW_TRANS_ACCEL, false)?
     .visit_field::<f32>("temp", Self::VT_TEMP, false)?
     .visit_field::<f32>("poll_rate", Self::VT_POLL_RATE, false)?
     .visit_field::<super::math::Quat>("mounting_orientation", Self::VT_MOUNTING_ORIENTATION, false)?
     .finish();
    Ok(())
  }
}
pub struct TrackerStatusArgs<'a> {
    pub role: Option<super::TrackerRole>,
    pub orientation: Option<&'a super::math::Quat>,
    pub position: Option<&'a super::math::Vec3f>,
    pub raw_rot_vel: Option<&'a super::math::Vec3f>,
    pub raw_trans_accel: Option<&'a super::math::Vec3f>,
    pub temp: Option<f32>,
    pub poll_rate: Option<f32>,
    pub mounting_orientation: Option<&'a super::math::Quat>,
}
impl<'a> Default for TrackerStatusArgs<'a> {
  #[inline]
  fn default() -> Self {
    TrackerStatusArgs {
      role: None,
      orientation: None,
      position: None,
      raw_rot_vel: None,
      raw_trans_accel: None,
      temp: None,
      poll_rate: None,
      mounting_orientation: None,
    }
  }
}

pub struct TrackerStatusBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> TrackerStatusBuilder<'a, 'b> {
  #[inline]
  pub fn add_role(&mut self, role: super::TrackerRole) {
    self.fbb_.push_slot_always::<super::TrackerRole>(TrackerStatus::VT_ROLE, role);
  }
  #[inline]
  pub fn add_orientation(&mut self, orientation: &super::math::Quat) {
    self.fbb_.push_slot_always::<&super::math::Quat>(TrackerStatus::VT_ORIENTATION, orientation);
  }
  #[inline]
  pub fn add_position(&mut self, position: &super::math::Vec3f) {
    self.fbb_.push_slot_always::<&super::math::Vec3f>(TrackerStatus::VT_POSITION, position);
  }
  #[inline]
  pub fn add_raw_rot_vel(&mut self, raw_rot_vel: &super::math::Vec3f) {
    self.fbb_.push_slot_always::<&super::math::Vec3f>(TrackerStatus::VT_RAW_ROT_VEL, raw_rot_vel);
  }
  #[inline]
  pub fn add_raw_trans_accel(&mut self, raw_trans_accel: &super::math::Vec3f) {
    self.fbb_.push_slot_always::<&super::math::Vec3f>(TrackerStatus::VT_RAW_TRANS_ACCEL, raw_trans_accel);
  }
  #[inline]
  pub fn add_temp(&mut self, temp: f32) {
    self.fbb_.push_slot_always::<f32>(TrackerStatus::VT_TEMP, temp);
  }
  #[inline]
  pub fn add_poll_rate(&mut self, poll_rate: f32) {
    self.fbb_.push_slot_always::<f32>(TrackerStatus::VT_POLL_RATE, poll_rate);
  }
  #[inline]
  pub fn add_mounting_orientation(&mut self, mounting_orientation: &super::math::Quat) {
    self.fbb_.push_slot_always::<&super::math::Quat>(TrackerStatus::VT_MOUNTING_ORIENTATION, mounting_orientation);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TrackerStatusBuilder<'a, 'b> {
    let start = _fbb.start_table();
    TrackerStatusBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<TrackerStatus<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for TrackerStatus<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("TrackerStatus");
      ds.field("role", &self.role());
      ds.field("orientation", &self.orientation());
      ds.field("position", &self.position());
      ds.field("raw_rot_vel", &self.raw_rot_vel());
      ds.field("raw_trans_accel", &self.raw_trans_accel());
      ds.field("temp", &self.temp());
      ds.field("poll_rate", &self.poll_rate());
      ds.field("mounting_orientation", &self.mounting_orientation());
      ds.finish()
  }
}
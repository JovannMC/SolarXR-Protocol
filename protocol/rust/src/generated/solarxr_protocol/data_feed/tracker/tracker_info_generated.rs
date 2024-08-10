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
pub enum TrackerInfoOffset {}
#[derive(Copy, Clone, PartialEq)]

/// Static description of a tracker
pub struct TrackerInfo<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for TrackerInfo<'a> {
  type Inner = TrackerInfo<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> TrackerInfo<'a> {
  pub const VT_IMU_TYPE: flatbuffers::VOffsetT = 4;
  pub const VT_BODY_PART: flatbuffers::VOffsetT = 6;
  pub const VT_POLL_RATE: flatbuffers::VOffsetT = 8;
  pub const VT_MOUNTING_ORIENTATION: flatbuffers::VOffsetT = 10;
  pub const VT_EDITABLE: flatbuffers::VOffsetT = 12;
  pub const VT_IS_COMPUTED: flatbuffers::VOffsetT = 14;
  pub const VT_IS_IMU: flatbuffers::VOffsetT = 16;
  pub const VT_DISPLAY_NAME: flatbuffers::VOffsetT = 18;
  pub const VT_CUSTOM_NAME: flatbuffers::VOffsetT = 20;
  pub const VT_ALLOW_DRIFT_COMPENSATION: flatbuffers::VOffsetT = 22;
  pub const VT_MOUNTING_RESET_ORIENTATION: flatbuffers::VOffsetT = 24;
  pub const VT_IS_HMD: flatbuffers::VOffsetT = 26;
  pub const VT_DATA_SUPPORT: flatbuffers::VOffsetT = 28;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    TrackerInfo { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args TrackerInfoArgs<'args>
  ) -> flatbuffers::WIPOffset<TrackerInfo<'bldr>> {
    let mut builder = TrackerInfoBuilder::new(_fbb);
    if let Some(x) = args.mounting_reset_orientation { builder.add_mounting_reset_orientation(x); }
    if let Some(x) = args.custom_name { builder.add_custom_name(x); }
    if let Some(x) = args.display_name { builder.add_display_name(x); }
    if let Some(x) = args.mounting_orientation { builder.add_mounting_orientation(x); }
    if let Some(x) = args.poll_rate { builder.add_poll_rate(x); }
    builder.add_imu_type(args.imu_type);
    builder.add_data_support(args.data_support);
    builder.add_is_hmd(args.is_hmd);
    builder.add_allow_drift_compensation(args.allow_drift_compensation);
    builder.add_is_imu(args.is_imu);
    builder.add_is_computed(args.is_computed);
    builder.add_editable(args.editable);
    builder.add_body_part(args.body_part);
    builder.finish()
  }


  #[inline]
  pub fn imu_type(&self) -> super::super::datatypes::hardware_info::ImuType {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<super::super::datatypes::hardware_info::ImuType>(TrackerInfo::VT_IMU_TYPE, Some(super::super::datatypes::hardware_info::ImuType::Other)).unwrap()}
  }
  /// The user-assigned role of the tracker.
  #[inline]
  pub fn body_part(&self) -> super::super::datatypes::BodyPart {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<super::super::datatypes::BodyPart>(TrackerInfo::VT_BODY_PART, Some(super::super::datatypes::BodyPart::NONE)).unwrap()}
  }
  /// average samples per second
  #[inline]
  pub fn poll_rate(&self) -> Option<&'a super::super::datatypes::HzF32> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<super::super::datatypes::HzF32>(TrackerInfo::VT_POLL_RATE, None)}
  }
  /// The orientation of the tracker when mounted on the body
  #[inline]
  pub fn mounting_orientation(&self) -> Option<&'a super::super::datatypes::math::Quat> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<super::super::datatypes::math::Quat>(TrackerInfo::VT_MOUNTING_ORIENTATION, None)}
  }
  /// Should the tracker's settings be editable by the user
  #[inline]
  pub fn editable(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(TrackerInfo::VT_EDITABLE, Some(false)).unwrap()}
  }
  /// Indicates if the tracker is computed (solved position and rotation)
  #[inline]
  pub fn is_computed(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(TrackerInfo::VT_IS_COMPUTED, Some(false)).unwrap()}
  }
  /// Indicates if the tracker is using an IMU for its tracking data
  #[inline]
  pub fn is_imu(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(TrackerInfo::VT_IS_IMU, Some(false)).unwrap()}
  }
  /// A human-friendly name to display as the name of the tracker.
  #[inline]
  pub fn display_name(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(TrackerInfo::VT_DISPLAY_NAME, None)}
  }
  /// name to display as the name of the tracker set by the user
  #[inline]
  pub fn custom_name(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(TrackerInfo::VT_CUSTOM_NAME, None)}
  }
  /// Whether to allow yaw drift compensation for this tracker or not.
  #[inline]
  pub fn allow_drift_compensation(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(TrackerInfo::VT_ALLOW_DRIFT_COMPENSATION, Some(false)).unwrap()}
  }
  /// Mounting Reset orientation overrides the current `mounting_orientation` of
  /// the tracker, this orientation is not saved and needs to be calculated
  /// each time the server is ran
  #[inline]
  pub fn mounting_reset_orientation(&self) -> Option<&'a super::super::datatypes::math::Quat> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<super::super::datatypes::math::Quat>(TrackerInfo::VT_MOUNTING_RESET_ORIENTATION, None)}
  }
  /// Indicates if the tracker is actually an HMD
  #[inline]
  pub fn is_hmd(&self) -> bool {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<bool>(TrackerInfo::VT_IS_HMD, Some(false)).unwrap()}
  }
  /// Indicates what type of data the tracker sends (note: it always ends up being rotation in the end)
  #[inline]
  pub fn data_support(&self) -> super::super::datatypes::hardware_info::TrackerDataSupport {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<super::super::datatypes::hardware_info::TrackerDataSupport>(TrackerInfo::VT_DATA_SUPPORT, Some(super::super::datatypes::hardware_info::TrackerDataSupport::OTHER)).unwrap()}
  }
}

impl flatbuffers::Verifiable for TrackerInfo<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<super::super::datatypes::hardware_info::ImuType>("imu_type", Self::VT_IMU_TYPE, false)?
     .visit_field::<super::super::datatypes::BodyPart>("body_part", Self::VT_BODY_PART, false)?
     .visit_field::<super::super::datatypes::HzF32>("poll_rate", Self::VT_POLL_RATE, false)?
     .visit_field::<super::super::datatypes::math::Quat>("mounting_orientation", Self::VT_MOUNTING_ORIENTATION, false)?
     .visit_field::<bool>("editable", Self::VT_EDITABLE, false)?
     .visit_field::<bool>("is_computed", Self::VT_IS_COMPUTED, false)?
     .visit_field::<bool>("is_imu", Self::VT_IS_IMU, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("display_name", Self::VT_DISPLAY_NAME, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("custom_name", Self::VT_CUSTOM_NAME, false)?
     .visit_field::<bool>("allow_drift_compensation", Self::VT_ALLOW_DRIFT_COMPENSATION, false)?
     .visit_field::<super::super::datatypes::math::Quat>("mounting_reset_orientation", Self::VT_MOUNTING_RESET_ORIENTATION, false)?
     .visit_field::<bool>("is_hmd", Self::VT_IS_HMD, false)?
     .visit_field::<super::super::datatypes::hardware_info::TrackerDataSupport>("data_support", Self::VT_DATA_SUPPORT, false)?
     .finish();
    Ok(())
  }
}
pub struct TrackerInfoArgs<'a> {
    pub imu_type: super::super::datatypes::hardware_info::ImuType,
    pub body_part: super::super::datatypes::BodyPart,
    pub poll_rate: Option<&'a super::super::datatypes::HzF32>,
    pub mounting_orientation: Option<&'a super::super::datatypes::math::Quat>,
    pub editable: bool,
    pub is_computed: bool,
    pub is_imu: bool,
    pub display_name: Option<flatbuffers::WIPOffset<&'a str>>,
    pub custom_name: Option<flatbuffers::WIPOffset<&'a str>>,
    pub allow_drift_compensation: bool,
    pub mounting_reset_orientation: Option<&'a super::super::datatypes::math::Quat>,
    pub is_hmd: bool,
    pub data_support: super::super::datatypes::hardware_info::TrackerDataSupport,
}
impl<'a> Default for TrackerInfoArgs<'a> {
  #[inline]
  fn default() -> Self {
    TrackerInfoArgs {
      imu_type: super::super::datatypes::hardware_info::ImuType::Other,
      body_part: super::super::datatypes::BodyPart::NONE,
      poll_rate: None,
      mounting_orientation: None,
      editable: false,
      is_computed: false,
      is_imu: false,
      display_name: None,
      custom_name: None,
      allow_drift_compensation: false,
      mounting_reset_orientation: None,
      is_hmd: false,
      data_support: super::super::datatypes::hardware_info::TrackerDataSupport::OTHER,
    }
  }
}

pub struct TrackerInfoBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> TrackerInfoBuilder<'a, 'b> {
  #[inline]
  pub fn add_imu_type(&mut self, imu_type: super::super::datatypes::hardware_info::ImuType) {
    self.fbb_.push_slot::<super::super::datatypes::hardware_info::ImuType>(TrackerInfo::VT_IMU_TYPE, imu_type, super::super::datatypes::hardware_info::ImuType::Other);
  }
  #[inline]
  pub fn add_body_part(&mut self, body_part: super::super::datatypes::BodyPart) {
    self.fbb_.push_slot::<super::super::datatypes::BodyPart>(TrackerInfo::VT_BODY_PART, body_part, super::super::datatypes::BodyPart::NONE);
  }
  #[inline]
  pub fn add_poll_rate(&mut self, poll_rate: &super::super::datatypes::HzF32) {
    self.fbb_.push_slot_always::<&super::super::datatypes::HzF32>(TrackerInfo::VT_POLL_RATE, poll_rate);
  }
  #[inline]
  pub fn add_mounting_orientation(&mut self, mounting_orientation: &super::super::datatypes::math::Quat) {
    self.fbb_.push_slot_always::<&super::super::datatypes::math::Quat>(TrackerInfo::VT_MOUNTING_ORIENTATION, mounting_orientation);
  }
  #[inline]
  pub fn add_editable(&mut self, editable: bool) {
    self.fbb_.push_slot::<bool>(TrackerInfo::VT_EDITABLE, editable, false);
  }
  #[inline]
  pub fn add_is_computed(&mut self, is_computed: bool) {
    self.fbb_.push_slot::<bool>(TrackerInfo::VT_IS_COMPUTED, is_computed, false);
  }
  #[inline]
  pub fn add_is_imu(&mut self, is_imu: bool) {
    self.fbb_.push_slot::<bool>(TrackerInfo::VT_IS_IMU, is_imu, false);
  }
  #[inline]
  pub fn add_display_name(&mut self, display_name: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(TrackerInfo::VT_DISPLAY_NAME, display_name);
  }
  #[inline]
  pub fn add_custom_name(&mut self, custom_name: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(TrackerInfo::VT_CUSTOM_NAME, custom_name);
  }
  #[inline]
  pub fn add_allow_drift_compensation(&mut self, allow_drift_compensation: bool) {
    self.fbb_.push_slot::<bool>(TrackerInfo::VT_ALLOW_DRIFT_COMPENSATION, allow_drift_compensation, false);
  }
  #[inline]
  pub fn add_mounting_reset_orientation(&mut self, mounting_reset_orientation: &super::super::datatypes::math::Quat) {
    self.fbb_.push_slot_always::<&super::super::datatypes::math::Quat>(TrackerInfo::VT_MOUNTING_RESET_ORIENTATION, mounting_reset_orientation);
  }
  #[inline]
  pub fn add_is_hmd(&mut self, is_hmd: bool) {
    self.fbb_.push_slot::<bool>(TrackerInfo::VT_IS_HMD, is_hmd, false);
  }
  #[inline]
  pub fn add_data_support(&mut self, data_support: super::super::datatypes::hardware_info::TrackerDataSupport) {
    self.fbb_.push_slot::<super::super::datatypes::hardware_info::TrackerDataSupport>(TrackerInfo::VT_DATA_SUPPORT, data_support, super::super::datatypes::hardware_info::TrackerDataSupport::OTHER);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TrackerInfoBuilder<'a, 'b> {
    let start = _fbb.start_table();
    TrackerInfoBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<TrackerInfo<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for TrackerInfo<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("TrackerInfo");
      ds.field("imu_type", &self.imu_type());
      ds.field("body_part", &self.body_part());
      ds.field("poll_rate", &self.poll_rate());
      ds.field("mounting_orientation", &self.mounting_orientation());
      ds.field("editable", &self.editable());
      ds.field("is_computed", &self.is_computed());
      ds.field("is_imu", &self.is_imu());
      ds.field("display_name", &self.display_name());
      ds.field("custom_name", &self.custom_name());
      ds.field("allow_drift_compensation", &self.allow_drift_compensation());
      ds.field("mounting_reset_orientation", &self.mounting_reset_orientation());
      ds.field("is_hmd", &self.is_hmd());
      ds.field("data_support", &self.data_support());
      ds.finish()
  }
}

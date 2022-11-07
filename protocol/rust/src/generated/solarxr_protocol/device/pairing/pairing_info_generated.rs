// automatically generated by the FlatBuffers compiler, do not modify
extern crate alloc;
extern crate flatbuffers;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::mem;
use core::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum PairingInfoOffset {}
#[derive(Copy, Clone, PartialEq)]

/// Broadcast by the device on startup to tell servers what this device supports,
/// and if it's already paired (i.e. if the server should show the popup).
pub struct PairingInfo<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for PairingInfo<'a> {
  type Inner = PairingInfo<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> PairingInfo<'a> {
  pub const VT_PAIRED_TO: flatbuffers::VOffsetT = 4;
  pub const VT_DISPLAY_NAME: flatbuffers::VOffsetT = 6;
  pub const VT_MODEL: flatbuffers::VOffsetT = 8;
  pub const VT_MANUFACTURER: flatbuffers::VOffsetT = 10;
  pub const VT_FIRMWARE_VERSION: flatbuffers::VOffsetT = 12;
  pub const VT_MCU_TYPE: flatbuffers::VOffsetT = 14;
  pub const VT_FEATURES: flatbuffers::VOffsetT = 16;
  pub const VT_SENSORS: flatbuffers::VOffsetT = 18;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    PairingInfo { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args PairingInfoArgs<'args>
  ) -> flatbuffers::WIPOffset<PairingInfo<'bldr>> {
    let mut builder = PairingInfoBuilder::new(_fbb);
    if let Some(x) = args.sensors { builder.add_sensors(x); }
    if let Some(x) = args.features { builder.add_features(x); }
    if let Some(x) = args.firmware_version { builder.add_firmware_version(x); }
    if let Some(x) = args.manufacturer { builder.add_manufacturer(x); }
    if let Some(x) = args.model { builder.add_model(x); }
    if let Some(x) = args.display_name { builder.add_display_name(x); }
    builder.add_paired_to(args.paired_to);
    builder.add_mcu_type(args.mcu_type);
    builder.finish()
  }


  /// If this tracker isn't paired to any server, this field should be `0`.
  #[inline]
  pub fn paired_to(&self) -> u32 {
    self._tab.get::<u32>(PairingInfo::VT_PAIRED_TO, Some(0)).unwrap()
  }
  #[inline]
  pub fn display_name(&self) -> &'a str {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(PairingInfo::VT_DISPLAY_NAME, None).unwrap()
  }
  #[inline]
  pub fn model(&self) -> &'a str {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(PairingInfo::VT_MODEL, None).unwrap()
  }
  #[inline]
  pub fn manufacturer(&self) -> &'a str {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(PairingInfo::VT_MANUFACTURER, None).unwrap()
  }
  #[inline]
  pub fn firmware_version(&self) -> &'a str {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(PairingInfo::VT_FIRMWARE_VERSION, None).unwrap()
  }
  #[inline]
  pub fn mcu_type(&self) -> super::super::datatypes::hardware_info::McuType {
    self._tab.get::<super::super::datatypes::hardware_info::McuType>(PairingInfo::VT_MCU_TYPE, Some(super::super::datatypes::hardware_info::McuType::Other)).unwrap()
  }
  #[inline]
  pub fn features(&self) -> flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<DeviceFeatureInfo<'a>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<DeviceFeatureInfo>>>>(PairingInfo::VT_FEATURES, None).unwrap()
  }
  #[inline]
  pub fn sensors(&self) -> flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<DeviceSensorInfo<'a>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<DeviceSensorInfo>>>>(PairingInfo::VT_SENSORS, None).unwrap()
  }
}

impl flatbuffers::Verifiable for PairingInfo<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<u32>("paired_to", Self::VT_PAIRED_TO, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("display_name", Self::VT_DISPLAY_NAME, true)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("model", Self::VT_MODEL, true)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("manufacturer", Self::VT_MANUFACTURER, true)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("firmware_version", Self::VT_FIRMWARE_VERSION, true)?
     .visit_field::<super::super::datatypes::hardware_info::McuType>("mcu_type", Self::VT_MCU_TYPE, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<DeviceFeatureInfo>>>>("features", Self::VT_FEATURES, true)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<DeviceSensorInfo>>>>("sensors", Self::VT_SENSORS, true)?
     .finish();
    Ok(())
  }
}
pub struct PairingInfoArgs<'a> {
    pub paired_to: u32,
    pub display_name: Option<flatbuffers::WIPOffset<&'a str>>,
    pub model: Option<flatbuffers::WIPOffset<&'a str>>,
    pub manufacturer: Option<flatbuffers::WIPOffset<&'a str>>,
    pub firmware_version: Option<flatbuffers::WIPOffset<&'a str>>,
    pub mcu_type: super::super::datatypes::hardware_info::McuType,
    pub features: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<DeviceFeatureInfo<'a>>>>>,
    pub sensors: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<DeviceSensorInfo<'a>>>>>,
}
impl<'a> Default for PairingInfoArgs<'a> {
  #[inline]
  fn default() -> Self {
    PairingInfoArgs {
      paired_to: 0,
      display_name: None, // required field
      model: None, // required field
      manufacturer: None, // required field
      firmware_version: None, // required field
      mcu_type: super::super::datatypes::hardware_info::McuType::Other,
      features: None, // required field
      sensors: None, // required field
    }
  }
}

pub struct PairingInfoBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> PairingInfoBuilder<'a, 'b> {
  #[inline]
  pub fn add_paired_to(&mut self, paired_to: u32) {
    self.fbb_.push_slot::<u32>(PairingInfo::VT_PAIRED_TO, paired_to, 0);
  }
  #[inline]
  pub fn add_display_name(&mut self, display_name: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(PairingInfo::VT_DISPLAY_NAME, display_name);
  }
  #[inline]
  pub fn add_model(&mut self, model: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(PairingInfo::VT_MODEL, model);
  }
  #[inline]
  pub fn add_manufacturer(&mut self, manufacturer: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(PairingInfo::VT_MANUFACTURER, manufacturer);
  }
  #[inline]
  pub fn add_firmware_version(&mut self, firmware_version: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(PairingInfo::VT_FIRMWARE_VERSION, firmware_version);
  }
  #[inline]
  pub fn add_mcu_type(&mut self, mcu_type: super::super::datatypes::hardware_info::McuType) {
    self.fbb_.push_slot::<super::super::datatypes::hardware_info::McuType>(PairingInfo::VT_MCU_TYPE, mcu_type, super::super::datatypes::hardware_info::McuType::Other);
  }
  #[inline]
  pub fn add_features(&mut self, features: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<DeviceFeatureInfo<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(PairingInfo::VT_FEATURES, features);
  }
  #[inline]
  pub fn add_sensors(&mut self, sensors: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<DeviceSensorInfo<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(PairingInfo::VT_SENSORS, sensors);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> PairingInfoBuilder<'a, 'b> {
    let start = _fbb.start_table();
    PairingInfoBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<PairingInfo<'a>> {
    let o = self.fbb_.end_table(self.start_);
    self.fbb_.required(o, PairingInfo::VT_DISPLAY_NAME,"display_name");
    self.fbb_.required(o, PairingInfo::VT_MODEL,"model");
    self.fbb_.required(o, PairingInfo::VT_MANUFACTURER,"manufacturer");
    self.fbb_.required(o, PairingInfo::VT_FIRMWARE_VERSION,"firmware_version");
    self.fbb_.required(o, PairingInfo::VT_FEATURES,"features");
    self.fbb_.required(o, PairingInfo::VT_SENSORS,"sensors");
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for PairingInfo<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("PairingInfo");
      ds.field("paired_to", &self.paired_to());
      ds.field("display_name", &self.display_name());
      ds.field("model", &self.model());
      ds.field("manufacturer", &self.manufacturer());
      ds.field("firmware_version", &self.firmware_version());
      ds.field("mcu_type", &self.mcu_type());
      ds.field("features", &self.features());
      ds.field("sensors", &self.sensors());
      ds.finish()
  }
}
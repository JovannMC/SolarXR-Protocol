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
pub enum AddUnknownDeviceRequestOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct AddUnknownDeviceRequest<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for AddUnknownDeviceRequest<'a> {
  type Inner = AddUnknownDeviceRequest<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> AddUnknownDeviceRequest<'a> {
  pub const VT_MAC_ADDRESS: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    AddUnknownDeviceRequest { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args AddUnknownDeviceRequestArgs<'args>
  ) -> flatbuffers::WIPOffset<AddUnknownDeviceRequest<'bldr>> {
    let mut builder = AddUnknownDeviceRequestBuilder::new(_fbb);
    if let Some(x) = args.mac_address { builder.add_mac_address(x); }
    builder.finish()
  }


  #[inline]
  pub fn mac_address(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(AddUnknownDeviceRequest::VT_MAC_ADDRESS, None)}
  }
}

impl flatbuffers::Verifiable for AddUnknownDeviceRequest<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("mac_address", Self::VT_MAC_ADDRESS, false)?
     .finish();
    Ok(())
  }
}
pub struct AddUnknownDeviceRequestArgs<'a> {
    pub mac_address: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for AddUnknownDeviceRequestArgs<'a> {
  #[inline]
  fn default() -> Self {
    AddUnknownDeviceRequestArgs {
      mac_address: None,
    }
  }
}

pub struct AddUnknownDeviceRequestBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> AddUnknownDeviceRequestBuilder<'a, 'b> {
  #[inline]
  pub fn add_mac_address(&mut self, mac_address: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(AddUnknownDeviceRequest::VT_MAC_ADDRESS, mac_address);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> AddUnknownDeviceRequestBuilder<'a, 'b> {
    let start = _fbb.start_table();
    AddUnknownDeviceRequestBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<AddUnknownDeviceRequest<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for AddUnknownDeviceRequest<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("AddUnknownDeviceRequest");
      ds.field("mac_address", &self.mac_address());
      ds.finish()
  }
}

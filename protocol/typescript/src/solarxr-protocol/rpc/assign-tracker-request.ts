// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';

import { BodyPart } from '../../solarxr-protocol/datatypes/body-part';
import { TrackerId, TrackerIdT } from '../../solarxr-protocol/datatypes/tracker-id';
import { Quat, QuatT } from '../../solarxr-protocol/datatypes/math/quat';


export class AssignTrackerRequest {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
__init(i:number, bb:flatbuffers.ByteBuffer):AssignTrackerRequest {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsAssignTrackerRequest(bb:flatbuffers.ByteBuffer, obj?:AssignTrackerRequest):AssignTrackerRequest {
  return (obj || new AssignTrackerRequest()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsAssignTrackerRequest(bb:flatbuffers.ByteBuffer, obj?:AssignTrackerRequest):AssignTrackerRequest {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new AssignTrackerRequest()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

trackerId(obj?:TrackerId):TrackerId|null {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? (obj || new TrackerId()).__init(this.bb!.__indirect(this.bb_pos + offset), this.bb!) : null;
}

bodyPosition():BodyPart {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? this.bb!.readUint8(this.bb_pos + offset) : BodyPart.NONE;
}

mountingRotation(obj?:Quat):Quat|null {
  const offset = this.bb!.__offset(this.bb_pos, 8);
  return offset ? (obj || new Quat()).__init(this.bb_pos + offset, this.bb!) : null;
}

static startAssignTrackerRequest(builder:flatbuffers.Builder) {
  builder.startObject(3);
}

static addTrackerId(builder:flatbuffers.Builder, trackerIdOffset:flatbuffers.Offset) {
  builder.addFieldOffset(0, trackerIdOffset, 0);
}

static addBodyPosition(builder:flatbuffers.Builder, bodyPosition:BodyPart) {
  builder.addFieldInt8(1, bodyPosition, BodyPart.NONE);
}

static addMountingRotation(builder:flatbuffers.Builder, mountingRotationOffset:flatbuffers.Offset) {
  builder.addFieldStruct(2, mountingRotationOffset, 0);
}

static endAssignTrackerRequest(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}


unpack(): AssignTrackerRequestT {
  return new AssignTrackerRequestT(
    (this.trackerId() !== null ? this.trackerId()!.unpack() : null),
    this.bodyPosition(),
    (this.mountingRotation() !== null ? this.mountingRotation()!.unpack() : null)
  );
}


unpackTo(_o: AssignTrackerRequestT): void {
  _o.trackerId = (this.trackerId() !== null ? this.trackerId()!.unpack() : null);
  _o.bodyPosition = this.bodyPosition();
  _o.mountingRotation = (this.mountingRotation() !== null ? this.mountingRotation()!.unpack() : null);
}
}

export class AssignTrackerRequestT {
constructor(
  public trackerId: TrackerIdT|null = null,
  public bodyPosition: BodyPart = BodyPart.NONE,
  public mountingRotation: QuatT|null = null
){}


pack(builder:flatbuffers.Builder): flatbuffers.Offset {
  const trackerId = (this.trackerId !== null ? this.trackerId!.pack(builder) : 0);

  AssignTrackerRequest.startAssignTrackerRequest(builder);
  AssignTrackerRequest.addTrackerId(builder, trackerId);
  AssignTrackerRequest.addBodyPosition(builder, this.bodyPosition);
  AssignTrackerRequest.addMountingRotation(builder, (this.mountingRotation !== null ? this.mountingRotation!.pack(builder) : 0));

  return AssignTrackerRequest.endAssignTrackerRequest(builder);
}
}
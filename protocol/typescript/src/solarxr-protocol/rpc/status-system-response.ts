// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';

import { StatusMessage, StatusMessageT } from '../../solarxr-protocol/rpc/status-message.js';


/**
 * Response containing all current valid statuses
 */
export class StatusSystemResponse implements flatbuffers.IUnpackableObject<StatusSystemResponseT> {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):StatusSystemResponse {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsStatusSystemResponse(bb:flatbuffers.ByteBuffer, obj?:StatusSystemResponse):StatusSystemResponse {
  return (obj || new StatusSystemResponse()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsStatusSystemResponse(bb:flatbuffers.ByteBuffer, obj?:StatusSystemResponse):StatusSystemResponse {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new StatusSystemResponse()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

currentStatuses(index: number, obj?:StatusMessage):StatusMessage|null {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? (obj || new StatusMessage()).__init(this.bb!.__indirect(this.bb!.__vector(this.bb_pos + offset) + index * 4), this.bb!) : null;
}

currentStatusesLength():number {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.__vector_len(this.bb_pos + offset) : 0;
}

static startStatusSystemResponse(builder:flatbuffers.Builder) {
  builder.startObject(1);
}

static addCurrentStatuses(builder:flatbuffers.Builder, currentStatusesOffset:flatbuffers.Offset) {
  builder.addFieldOffset(0, currentStatusesOffset, 0);
}

static createCurrentStatusesVector(builder:flatbuffers.Builder, data:flatbuffers.Offset[]):flatbuffers.Offset {
  builder.startVector(4, data.length, 4);
  for (let i = data.length - 1; i >= 0; i--) {
    builder.addOffset(data[i]!);
  }
  return builder.endVector();
}

static startCurrentStatusesVector(builder:flatbuffers.Builder, numElems:number) {
  builder.startVector(4, numElems, 4);
}

static endStatusSystemResponse(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createStatusSystemResponse(builder:flatbuffers.Builder, currentStatusesOffset:flatbuffers.Offset):flatbuffers.Offset {
  StatusSystemResponse.startStatusSystemResponse(builder);
  StatusSystemResponse.addCurrentStatuses(builder, currentStatusesOffset);
  return StatusSystemResponse.endStatusSystemResponse(builder);
}

unpack(): StatusSystemResponseT {
  return new StatusSystemResponseT(
    this.bb!.createObjList<StatusMessage, StatusMessageT>(this.currentStatuses.bind(this), this.currentStatusesLength())
  );
}


unpackTo(_o: StatusSystemResponseT): void {
  _o.currentStatuses = this.bb!.createObjList<StatusMessage, StatusMessageT>(this.currentStatuses.bind(this), this.currentStatusesLength());
}
}

export class StatusSystemResponseT implements flatbuffers.IGeneratedObject {
constructor(
  public currentStatuses: (StatusMessageT)[] = []
){}


pack(builder:flatbuffers.Builder): flatbuffers.Offset {
  const currentStatuses = StatusSystemResponse.createCurrentStatusesVector(builder, builder.createObjectOffsetList(this.currentStatuses));

  return StatusSystemResponse.createStatusSystemResponse(builder,
    currentStatuses
  );
}
}

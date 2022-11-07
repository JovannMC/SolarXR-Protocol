// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';

import { HardwareAddress, HardwareAddressT } from '../../solarxr-protocol/datatypes/hardware-info/hardware-address';
import { PingResponse, PingResponseT } from '../../solarxr-protocol/device/ping-response';
import { ServerBoundMessage, unionToServerBoundMessage, unionListToServerBoundMessage } from '../../solarxr-protocol/device/server-bound-message';
import { DeviceStatus, DeviceStatusT } from '../../solarxr-protocol/device/packets/device-status';
import { ImuMovement, ImuMovementT } from '../../solarxr-protocol/device/packets/imu-movement';
import { ImuStatus, ImuStatusT } from '../../solarxr-protocol/device/packets/imu-status';
import { PairingInfo, PairingInfoT } from '../../solarxr-protocol/device/pairing/pairing-info';
import { PairingResponse, PairingResponseT } from '../../solarxr-protocol/device/pairing/pairing-response';


export class ServerBoundMessageHeader {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
__init(i:number, bb:flatbuffers.ByteBuffer):ServerBoundMessageHeader {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsServerBoundMessageHeader(bb:flatbuffers.ByteBuffer, obj?:ServerBoundMessageHeader):ServerBoundMessageHeader {
  return (obj || new ServerBoundMessageHeader()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsServerBoundMessageHeader(bb:flatbuffers.ByteBuffer, obj?:ServerBoundMessageHeader):ServerBoundMessageHeader {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new ServerBoundMessageHeader()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

macAddress(obj?:HardwareAddress):HardwareAddress|null {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? (obj || new HardwareAddress()).__init(this.bb_pos + offset, this.bb!) : null;
}

reqRepType():ServerBoundMessage {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? this.bb!.readUint8(this.bb_pos + offset) : ServerBoundMessage.NONE;
}

reqRep<T extends flatbuffers.Table>(obj:any):any|null {
  const offset = this.bb!.__offset(this.bb_pos, 8);
  return offset ? this.bb!.__union(obj, this.bb_pos + offset) : null;
}

static startServerBoundMessageHeader(builder:flatbuffers.Builder) {
  builder.startObject(3);
}

static addMacAddress(builder:flatbuffers.Builder, macAddressOffset:flatbuffers.Offset) {
  builder.addFieldStruct(0, macAddressOffset, 0);
}

static addReqRepType(builder:flatbuffers.Builder, reqRepType:ServerBoundMessage) {
  builder.addFieldInt8(1, reqRepType, ServerBoundMessage.NONE);
}

static addReqRep(builder:flatbuffers.Builder, reqRepOffset:flatbuffers.Offset) {
  builder.addFieldOffset(2, reqRepOffset, 0);
}

static endServerBoundMessageHeader(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  builder.requiredField(offset, 4) // mac_address
  return offset;
}

static createServerBoundMessageHeader(builder:flatbuffers.Builder, macAddressOffset:flatbuffers.Offset, reqRepType:ServerBoundMessage, reqRepOffset:flatbuffers.Offset):flatbuffers.Offset {
  ServerBoundMessageHeader.startServerBoundMessageHeader(builder);
  ServerBoundMessageHeader.addMacAddress(builder, macAddressOffset);
  ServerBoundMessageHeader.addReqRepType(builder, reqRepType);
  ServerBoundMessageHeader.addReqRep(builder, reqRepOffset);
  return ServerBoundMessageHeader.endServerBoundMessageHeader(builder);
}

unpack(): ServerBoundMessageHeaderT {
  return new ServerBoundMessageHeaderT(
    (this.macAddress() !== null ? this.macAddress()!.unpack() : null),
    this.reqRepType(),
    (() => {
      let temp = unionToServerBoundMessage(this.reqRepType(), this.reqRep.bind(this));
      if(temp === null) { return null; }
      return temp.unpack()
  })()
  );
}


unpackTo(_o: ServerBoundMessageHeaderT): void {
  _o.macAddress = (this.macAddress() !== null ? this.macAddress()!.unpack() : null);
  _o.reqRepType = this.reqRepType();
  _o.reqRep = (() => {
      let temp = unionToServerBoundMessage(this.reqRepType(), this.reqRep.bind(this));
      if(temp === null) { return null; }
      return temp.unpack()
  })();
}
}

export class ServerBoundMessageHeaderT {
constructor(
  public macAddress: HardwareAddressT|null = null,
  public reqRepType: ServerBoundMessage = ServerBoundMessage.NONE,
  public reqRep: DeviceStatusT|ImuMovementT|ImuStatusT|PairingInfoT|PairingResponseT|PingResponseT|null = null
){}


pack(builder:flatbuffers.Builder): flatbuffers.Offset {
  const reqRep = builder.createObjectOffset(this.reqRep);

  return ServerBoundMessageHeader.createServerBoundMessageHeader(builder,
    (this.macAddress !== null ? this.macAddress!.pack(builder) : 0),
    this.reqRepType,
    reqRep
  );
}
}
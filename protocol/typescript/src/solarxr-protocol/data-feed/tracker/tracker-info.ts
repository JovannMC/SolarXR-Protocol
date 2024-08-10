// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';

import { BodyPart } from '../../../solarxr-protocol/datatypes/body-part.js';
import { HzF32, HzF32T } from '../../../solarxr-protocol/datatypes/hz-f32.js';
import { ImuType } from '../../../solarxr-protocol/datatypes/hardware-info/imu-type.js';
import { TrackerDataSupport } from '../../../solarxr-protocol/datatypes/hardware-info/tracker-data-support.js';
import { Quat, QuatT } from '../../../solarxr-protocol/datatypes/math/quat.js';


/**
 * Static description of a tracker
 */
export class TrackerInfo implements flatbuffers.IUnpackableObject<TrackerInfoT> {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):TrackerInfo {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsTrackerInfo(bb:flatbuffers.ByteBuffer, obj?:TrackerInfo):TrackerInfo {
  return (obj || new TrackerInfo()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsTrackerInfo(bb:flatbuffers.ByteBuffer, obj?:TrackerInfo):TrackerInfo {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new TrackerInfo()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

imuType():ImuType {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.readUint16(this.bb_pos + offset) : ImuType.Other;
}

/**
 * The user-assigned role of the tracker.
 */
bodyPart():BodyPart {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? this.bb!.readUint8(this.bb_pos + offset) : BodyPart.NONE;
}

/**
 * average samples per second
 */
pollRate(obj?:HzF32):HzF32|null {
  const offset = this.bb!.__offset(this.bb_pos, 8);
  return offset ? (obj || new HzF32()).__init(this.bb_pos + offset, this.bb!) : null;
}

/**
 * The orientation of the tracker when mounted on the body
 */
mountingOrientation(obj?:Quat):Quat|null {
  const offset = this.bb!.__offset(this.bb_pos, 10);
  return offset ? (obj || new Quat()).__init(this.bb_pos + offset, this.bb!) : null;
}

/**
 * Should the tracker's settings be editable by the user
 */
editable():boolean {
  const offset = this.bb!.__offset(this.bb_pos, 12);
  return offset ? !!this.bb!.readInt8(this.bb_pos + offset) : false;
}

/**
 * Indicates if the tracker is computed (solved position and rotation)
 */
isComputed():boolean {
  const offset = this.bb!.__offset(this.bb_pos, 14);
  return offset ? !!this.bb!.readInt8(this.bb_pos + offset) : false;
}

/**
 * Indicates if the tracker is using an IMU for its tracking data
 */
isImu():boolean {
  const offset = this.bb!.__offset(this.bb_pos, 16);
  return offset ? !!this.bb!.readInt8(this.bb_pos + offset) : false;
}

/**
 * A human-friendly name to display as the name of the tracker.
 */
displayName():string|null
displayName(optionalEncoding:flatbuffers.Encoding):string|Uint8Array|null
displayName(optionalEncoding?:any):string|Uint8Array|null {
  const offset = this.bb!.__offset(this.bb_pos, 18);
  return offset ? this.bb!.__string(this.bb_pos + offset, optionalEncoding) : null;
}

/**
 * name to display as the name of the tracker set by the user
 */
customName():string|null
customName(optionalEncoding:flatbuffers.Encoding):string|Uint8Array|null
customName(optionalEncoding?:any):string|Uint8Array|null {
  const offset = this.bb!.__offset(this.bb_pos, 20);
  return offset ? this.bb!.__string(this.bb_pos + offset, optionalEncoding) : null;
}

/**
 * Whether to allow yaw drift compensation for this tracker or not.
 */
allowDriftCompensation():boolean {
  const offset = this.bb!.__offset(this.bb_pos, 22);
  return offset ? !!this.bb!.readInt8(this.bb_pos + offset) : false;
}

/**
 * Mounting Reset orientation overrides the current `mounting_orientation` of
 * the tracker, this orientation is not saved and needs to be calculated
 * each time the server is ran
 */
mountingResetOrientation(obj?:Quat):Quat|null {
  const offset = this.bb!.__offset(this.bb_pos, 24);
  return offset ? (obj || new Quat()).__init(this.bb_pos + offset, this.bb!) : null;
}

/**
 * Indicates if the tracker is actually an HMD
 */
isHmd():boolean {
  const offset = this.bb!.__offset(this.bb_pos, 26);
  return offset ? !!this.bb!.readInt8(this.bb_pos + offset) : false;
}

/**
 * Indicates what type of data the tracker sends (note: it always ends up being rotation in the end)
 */
dataSupport():TrackerDataSupport {
  const offset = this.bb!.__offset(this.bb_pos, 28);
  return offset ? this.bb!.readUint8(this.bb_pos + offset) : TrackerDataSupport.OTHER;
}

static startTrackerInfo(builder:flatbuffers.Builder) {
  builder.startObject(13);
}

static addImuType(builder:flatbuffers.Builder, imuType:ImuType) {
  builder.addFieldInt16(0, imuType, ImuType.Other);
}

static addBodyPart(builder:flatbuffers.Builder, bodyPart:BodyPart) {
  builder.addFieldInt8(1, bodyPart, BodyPart.NONE);
}

static addPollRate(builder:flatbuffers.Builder, pollRateOffset:flatbuffers.Offset) {
  builder.addFieldStruct(2, pollRateOffset, 0);
}

static addMountingOrientation(builder:flatbuffers.Builder, mountingOrientationOffset:flatbuffers.Offset) {
  builder.addFieldStruct(3, mountingOrientationOffset, 0);
}

static addEditable(builder:flatbuffers.Builder, editable:boolean) {
  builder.addFieldInt8(4, +editable, +false);
}

static addIsComputed(builder:flatbuffers.Builder, isComputed:boolean) {
  builder.addFieldInt8(5, +isComputed, +false);
}

static addIsImu(builder:flatbuffers.Builder, isImu:boolean) {
  builder.addFieldInt8(6, +isImu, +false);
}

static addDisplayName(builder:flatbuffers.Builder, displayNameOffset:flatbuffers.Offset) {
  builder.addFieldOffset(7, displayNameOffset, 0);
}

static addCustomName(builder:flatbuffers.Builder, customNameOffset:flatbuffers.Offset) {
  builder.addFieldOffset(8, customNameOffset, 0);
}

static addAllowDriftCompensation(builder:flatbuffers.Builder, allowDriftCompensation:boolean) {
  builder.addFieldInt8(9, +allowDriftCompensation, +false);
}

static addMountingResetOrientation(builder:flatbuffers.Builder, mountingResetOrientationOffset:flatbuffers.Offset) {
  builder.addFieldStruct(10, mountingResetOrientationOffset, 0);
}

static addIsHmd(builder:flatbuffers.Builder, isHmd:boolean) {
  builder.addFieldInt8(11, +isHmd, +false);
}

static addDataSupport(builder:flatbuffers.Builder, dataSupport:TrackerDataSupport) {
  builder.addFieldInt8(12, dataSupport, TrackerDataSupport.OTHER);
}

static endTrackerInfo(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}


unpack(): TrackerInfoT {
  return new TrackerInfoT(
    this.imuType(),
    this.bodyPart(),
    (this.pollRate() !== null ? this.pollRate()!.unpack() : null),
    (this.mountingOrientation() !== null ? this.mountingOrientation()!.unpack() : null),
    this.editable(),
    this.isComputed(),
    this.isImu(),
    this.displayName(),
    this.customName(),
    this.allowDriftCompensation(),
    (this.mountingResetOrientation() !== null ? this.mountingResetOrientation()!.unpack() : null),
    this.isHmd(),
    this.dataSupport()
  );
}


unpackTo(_o: TrackerInfoT): void {
  _o.imuType = this.imuType();
  _o.bodyPart = this.bodyPart();
  _o.pollRate = (this.pollRate() !== null ? this.pollRate()!.unpack() : null);
  _o.mountingOrientation = (this.mountingOrientation() !== null ? this.mountingOrientation()!.unpack() : null);
  _o.editable = this.editable();
  _o.isComputed = this.isComputed();
  _o.isImu = this.isImu();
  _o.displayName = this.displayName();
  _o.customName = this.customName();
  _o.allowDriftCompensation = this.allowDriftCompensation();
  _o.mountingResetOrientation = (this.mountingResetOrientation() !== null ? this.mountingResetOrientation()!.unpack() : null);
  _o.isHmd = this.isHmd();
  _o.dataSupport = this.dataSupport();
}
}

export class TrackerInfoT implements flatbuffers.IGeneratedObject {
constructor(
  public imuType: ImuType = ImuType.Other,
  public bodyPart: BodyPart = BodyPart.NONE,
  public pollRate: HzF32T|null = null,
  public mountingOrientation: QuatT|null = null,
  public editable: boolean = false,
  public isComputed: boolean = false,
  public isImu: boolean = false,
  public displayName: string|Uint8Array|null = null,
  public customName: string|Uint8Array|null = null,
  public allowDriftCompensation: boolean = false,
  public mountingResetOrientation: QuatT|null = null,
  public isHmd: boolean = false,
  public dataSupport: TrackerDataSupport = TrackerDataSupport.OTHER
){}


pack(builder:flatbuffers.Builder): flatbuffers.Offset {
  const displayName = (this.displayName !== null ? builder.createString(this.displayName!) : 0);
  const customName = (this.customName !== null ? builder.createString(this.customName!) : 0);

  TrackerInfo.startTrackerInfo(builder);
  TrackerInfo.addImuType(builder, this.imuType);
  TrackerInfo.addBodyPart(builder, this.bodyPart);
  TrackerInfo.addPollRate(builder, (this.pollRate !== null ? this.pollRate!.pack(builder) : 0));
  TrackerInfo.addMountingOrientation(builder, (this.mountingOrientation !== null ? this.mountingOrientation!.pack(builder) : 0));
  TrackerInfo.addEditable(builder, this.editable);
  TrackerInfo.addIsComputed(builder, this.isComputed);
  TrackerInfo.addIsImu(builder, this.isImu);
  TrackerInfo.addDisplayName(builder, displayName);
  TrackerInfo.addCustomName(builder, customName);
  TrackerInfo.addAllowDriftCompensation(builder, this.allowDriftCompensation);
  TrackerInfo.addMountingResetOrientation(builder, (this.mountingResetOrientation !== null ? this.mountingResetOrientation!.pack(builder) : 0));
  TrackerInfo.addIsHmd(builder, this.isHmd);
  TrackerInfo.addDataSupport(builder, this.dataSupport);

  return TrackerInfo.endTrackerInfo(builder);
}
}

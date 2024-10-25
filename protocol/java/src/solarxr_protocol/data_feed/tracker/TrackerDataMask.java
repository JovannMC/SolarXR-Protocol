// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.data_feed.tracker;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

/**
 * A mask of the different components in `TrackerComponent`
 */
@SuppressWarnings("unused")
public final class TrackerDataMask extends Table {
  public static void ValidateVersion() { Constants.FLATBUFFERS_22_10_26(); }
  public static TrackerDataMask getRootAsTrackerDataMask(ByteBuffer _bb) { return getRootAsTrackerDataMask(_bb, new TrackerDataMask()); }
  public static TrackerDataMask getRootAsTrackerDataMask(ByteBuffer _bb, TrackerDataMask obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public TrackerDataMask __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public boolean info() { int o = __offset(4); return o != 0 ? 0!=bb.get(o + bb_pos) : false; }
  public boolean status() { int o = __offset(6); return o != 0 ? 0!=bb.get(o + bb_pos) : false; }
  public boolean rotation() { int o = __offset(8); return o != 0 ? 0!=bb.get(o + bb_pos) : false; }
  public boolean position() { int o = __offset(10); return o != 0 ? 0!=bb.get(o + bb_pos) : false; }
  public boolean rawAngularVelocity() { int o = __offset(12); return o != 0 ? 0!=bb.get(o + bb_pos) : false; }
  public boolean rawAcceleration() { int o = __offset(14); return o != 0 ? 0!=bb.get(o + bb_pos) : false; }
  public boolean temp() { int o = __offset(16); return o != 0 ? 0!=bb.get(o + bb_pos) : false; }
  public boolean linearAcceleration() { int o = __offset(18); return o != 0 ? 0!=bb.get(o + bb_pos) : false; }
  public boolean rotationReferenceAdjusted() { int o = __offset(20); return o != 0 ? 0!=bb.get(o + bb_pos) : false; }
  public boolean rotationIdentityAdjusted() { int o = __offset(22); return o != 0 ? 0!=bb.get(o + bb_pos) : false; }
  public boolean tps() { int o = __offset(24); return o != 0 ? 0!=bb.get(o + bb_pos) : false; }

  public static int createTrackerDataMask(FlatBufferBuilder builder,
      boolean info,
      boolean status,
      boolean rotation,
      boolean position,
      boolean rawAngularVelocity,
      boolean rawAcceleration,
      boolean temp,
      boolean linearAcceleration,
      boolean rotationReferenceAdjusted,
      boolean rotationIdentityAdjusted,
      boolean tps) {
    builder.startTable(11);
    TrackerDataMask.addTps(builder, tps);
    TrackerDataMask.addRotationIdentityAdjusted(builder, rotationIdentityAdjusted);
    TrackerDataMask.addRotationReferenceAdjusted(builder, rotationReferenceAdjusted);
    TrackerDataMask.addLinearAcceleration(builder, linearAcceleration);
    TrackerDataMask.addTemp(builder, temp);
    TrackerDataMask.addRawAcceleration(builder, rawAcceleration);
    TrackerDataMask.addRawAngularVelocity(builder, rawAngularVelocity);
    TrackerDataMask.addPosition(builder, position);
    TrackerDataMask.addRotation(builder, rotation);
    TrackerDataMask.addStatus(builder, status);
    TrackerDataMask.addInfo(builder, info);
    return TrackerDataMask.endTrackerDataMask(builder);
  }

  public static void startTrackerDataMask(FlatBufferBuilder builder) { builder.startTable(11); }
  public static void addInfo(FlatBufferBuilder builder, boolean info) { builder.addBoolean(0, info, false); }
  public static void addStatus(FlatBufferBuilder builder, boolean status) { builder.addBoolean(1, status, false); }
  public static void addRotation(FlatBufferBuilder builder, boolean rotation) { builder.addBoolean(2, rotation, false); }
  public static void addPosition(FlatBufferBuilder builder, boolean position) { builder.addBoolean(3, position, false); }
  public static void addRawAngularVelocity(FlatBufferBuilder builder, boolean rawAngularVelocity) { builder.addBoolean(4, rawAngularVelocity, false); }
  public static void addRawAcceleration(FlatBufferBuilder builder, boolean rawAcceleration) { builder.addBoolean(5, rawAcceleration, false); }
  public static void addTemp(FlatBufferBuilder builder, boolean temp) { builder.addBoolean(6, temp, false); }
  public static void addLinearAcceleration(FlatBufferBuilder builder, boolean linearAcceleration) { builder.addBoolean(7, linearAcceleration, false); }
  public static void addRotationReferenceAdjusted(FlatBufferBuilder builder, boolean rotationReferenceAdjusted) { builder.addBoolean(8, rotationReferenceAdjusted, false); }
  public static void addRotationIdentityAdjusted(FlatBufferBuilder builder, boolean rotationIdentityAdjusted) { builder.addBoolean(9, rotationIdentityAdjusted, false); }
  public static void addTps(FlatBufferBuilder builder, boolean tps) { builder.addBoolean(10, tps, false); }
  public static int endTrackerDataMask(FlatBufferBuilder builder) {
    int o = builder.endTable();
    return o;
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public TrackerDataMask get(int j) { return get(new TrackerDataMask(), j); }
    public TrackerDataMask get(TrackerDataMask obj, int j) {  return obj.__assign(__indirect(__element(j), bb), bb); }
  }
  public TrackerDataMaskT unpack() {
    TrackerDataMaskT _o = new TrackerDataMaskT();
    unpackTo(_o);
    return _o;
  }
  public void unpackTo(TrackerDataMaskT _o) {
    boolean _oInfo = info();
    _o.setInfo(_oInfo);
    boolean _oStatus = status();
    _o.setStatus(_oStatus);
    boolean _oRotation = rotation();
    _o.setRotation(_oRotation);
    boolean _oPosition = position();
    _o.setPosition(_oPosition);
    boolean _oRawAngularVelocity = rawAngularVelocity();
    _o.setRawAngularVelocity(_oRawAngularVelocity);
    boolean _oRawAcceleration = rawAcceleration();
    _o.setRawAcceleration(_oRawAcceleration);
    boolean _oTemp = temp();
    _o.setTemp(_oTemp);
    boolean _oLinearAcceleration = linearAcceleration();
    _o.setLinearAcceleration(_oLinearAcceleration);
    boolean _oRotationReferenceAdjusted = rotationReferenceAdjusted();
    _o.setRotationReferenceAdjusted(_oRotationReferenceAdjusted);
    boolean _oRotationIdentityAdjusted = rotationIdentityAdjusted();
    _o.setRotationIdentityAdjusted(_oRotationIdentityAdjusted);
    boolean _oTps = tps();
    _o.setTps(_oTps);
  }
  public static int pack(FlatBufferBuilder builder, TrackerDataMaskT _o) {
    if (_o == null) return 0;
    return createTrackerDataMask(
      builder,
      _o.getInfo(),
      _o.getStatus(),
      _o.getRotation(),
      _o.getPosition(),
      _o.getRawAngularVelocity(),
      _o.getRawAcceleration(),
      _o.getTemp(),
      _o.getLinearAcceleration(),
      _o.getRotationReferenceAdjusted(),
      _o.getRotationIdentityAdjusted(),
      _o.getTps());
  }
}


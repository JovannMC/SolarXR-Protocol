// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.rpc;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

@SuppressWarnings("unused")
public final class SetPauseTrackingRequest extends Table {
  public static void ValidateVersion() { Constants.FLATBUFFERS_22_10_26(); }
  public static SetPauseTrackingRequest getRootAsSetPauseTrackingRequest(ByteBuffer _bb) { return getRootAsSetPauseTrackingRequest(_bb, new SetPauseTrackingRequest()); }
  public static SetPauseTrackingRequest getRootAsSetPauseTrackingRequest(ByteBuffer _bb, SetPauseTrackingRequest obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public SetPauseTrackingRequest __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  /**
   * Pauses skeleton tracking if true, resumes skeleton tracking if false.
   */
  public boolean pauseTracking() { int o = __offset(4); return o != 0 ? 0!=bb.get(o + bb_pos) : false; }

  public static int createSetPauseTrackingRequest(FlatBufferBuilder builder,
      boolean pauseTracking) {
    builder.startTable(1);
    SetPauseTrackingRequest.addPauseTracking(builder, pauseTracking);
    return SetPauseTrackingRequest.endSetPauseTrackingRequest(builder);
  }

  public static void startSetPauseTrackingRequest(FlatBufferBuilder builder) { builder.startTable(1); }
  public static void addPauseTracking(FlatBufferBuilder builder, boolean pauseTracking) { builder.addBoolean(0, pauseTracking, false); }
  public static int endSetPauseTrackingRequest(FlatBufferBuilder builder) {
    int o = builder.endTable();
    return o;
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public SetPauseTrackingRequest get(int j) { return get(new SetPauseTrackingRequest(), j); }
    public SetPauseTrackingRequest get(SetPauseTrackingRequest obj, int j) {  return obj.__assign(__indirect(__element(j), bb), bb); }
  }
  public SetPauseTrackingRequestT unpack() {
    SetPauseTrackingRequestT _o = new SetPauseTrackingRequestT();
    unpackTo(_o);
    return _o;
  }
  public void unpackTo(SetPauseTrackingRequestT _o) {
    boolean _oPauseTracking = pauseTracking();
    _o.setPauseTracking(_oPauseTracking);
  }
  public static int pack(FlatBufferBuilder builder, SetPauseTrackingRequestT _o) {
    if (_o == null) return 0;
    return createSetPauseTrackingRequest(
      builder,
      _o.getPauseTracking());
  }
}


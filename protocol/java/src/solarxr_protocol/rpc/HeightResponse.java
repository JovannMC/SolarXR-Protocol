// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.rpc;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

/**
 * Returns the current max positional tracker height, an estimated full height (user height), and the current min positional tracker height
 */
@SuppressWarnings("unused")
public final class HeightResponse extends Table {
  public static void ValidateVersion() { Constants.FLATBUFFERS_22_10_26(); }
  public static HeightResponse getRootAsHeightResponse(ByteBuffer _bb) { return getRootAsHeightResponse(_bb, new HeightResponse()); }
  public static HeightResponse getRootAsHeightResponse(ByteBuffer _bb, HeightResponse obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public HeightResponse __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public float maxHeight() { int o = __offset(4); return o != 0 ? bb.getFloat(o + bb_pos) : 0.0f; }
  public float estimatedFullHeight() { int o = __offset(6); return o != 0 ? bb.getFloat(o + bb_pos) : 0.0f; }
  public float minHeight() { int o = __offset(8); return o != 0 ? bb.getFloat(o + bb_pos) : 0.0f; }

  public static int createHeightResponse(FlatBufferBuilder builder,
      float maxHeight,
      float estimatedFullHeight,
      float minHeight) {
    builder.startTable(3);
    HeightResponse.addMinHeight(builder, minHeight);
    HeightResponse.addEstimatedFullHeight(builder, estimatedFullHeight);
    HeightResponse.addMaxHeight(builder, maxHeight);
    return HeightResponse.endHeightResponse(builder);
  }

  public static void startHeightResponse(FlatBufferBuilder builder) { builder.startTable(3); }
  public static void addMaxHeight(FlatBufferBuilder builder, float maxHeight) { builder.addFloat(0, maxHeight, 0.0f); }
  public static void addEstimatedFullHeight(FlatBufferBuilder builder, float estimatedFullHeight) { builder.addFloat(1, estimatedFullHeight, 0.0f); }
  public static void addMinHeight(FlatBufferBuilder builder, float minHeight) { builder.addFloat(2, minHeight, 0.0f); }
  public static int endHeightResponse(FlatBufferBuilder builder) {
    int o = builder.endTable();
    return o;
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public HeightResponse get(int j) { return get(new HeightResponse(), j); }
    public HeightResponse get(HeightResponse obj, int j) {  return obj.__assign(__indirect(__element(j), bb), bb); }
  }
  public HeightResponseT unpack() {
    HeightResponseT _o = new HeightResponseT();
    unpackTo(_o);
    return _o;
  }
  public void unpackTo(HeightResponseT _o) {
    float _oMaxHeight = maxHeight();
    _o.setMaxHeight(_oMaxHeight);
    float _oEstimatedFullHeight = estimatedFullHeight();
    _o.setEstimatedFullHeight(_oEstimatedFullHeight);
    float _oMinHeight = minHeight();
    _o.setMinHeight(_oMinHeight);
  }
  public static int pack(FlatBufferBuilder builder, HeightResponseT _o) {
    if (_o == null) return 0;
    return createHeightResponse(
      builder,
      _o.getMaxHeight(),
      _o.getEstimatedFullHeight(),
      _o.getMinHeight());
  }
}


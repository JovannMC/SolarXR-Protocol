// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.rpc;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

@SuppressWarnings("unused")
public final class SkeletonResetAllRequest extends Table {
  public static void ValidateVersion() { Constants.FLATBUFFERS_2_0_0(); }
  public static SkeletonResetAllRequest getRootAsSkeletonResetAllRequest(ByteBuffer _bb) { return getRootAsSkeletonResetAllRequest(_bb, new SkeletonResetAllRequest()); }
  public static SkeletonResetAllRequest getRootAsSkeletonResetAllRequest(ByteBuffer _bb, SkeletonResetAllRequest obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public SkeletonResetAllRequest __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }


  public static void startSkeletonResetAllRequest(FlatBufferBuilder builder) { builder.startTable(0); }
  public static int endSkeletonResetAllRequest(FlatBufferBuilder builder) {
    int o = builder.endTable();
    return o;
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public SkeletonResetAllRequest get(int j) { return get(new SkeletonResetAllRequest(), j); }
    public SkeletonResetAllRequest get(SkeletonResetAllRequest obj, int j) {  return obj.__assign(__indirect(__element(j), bb), bb); }
  }
  public SkeletonResetAllRequestT unpack() {
    SkeletonResetAllRequestT _o = new SkeletonResetAllRequestT();
    unpackTo(_o);
    return _o;
  }
  public void unpackTo(SkeletonResetAllRequestT _o) {
  }
  public static int pack(FlatBufferBuilder builder, SkeletonResetAllRequestT _o) {
    if (_o == null) return 0;
    startSkeletonResetAllRequest(builder);
    return endSkeletonResetAllRequest(builder);
  }
}

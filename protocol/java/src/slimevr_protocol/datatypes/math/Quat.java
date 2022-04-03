// automatically generated by the FlatBuffers compiler, do not modify

package slimevr_protocol.datatypes.math;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

@SuppressWarnings("unused")
public final class Quat extends Struct {
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public Quat __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public float x() { return bb.getFloat(bb_pos + 0); }
  public float y() { return bb.getFloat(bb_pos + 4); }
  public float z() { return bb.getFloat(bb_pos + 8); }
  public float w() { return bb.getFloat(bb_pos + 12); }

  public static int createQuat(FlatBufferBuilder builder, float x, float y, float z, float w) {
    builder.prep(4, 16);
    builder.putFloat(w);
    builder.putFloat(z);
    builder.putFloat(y);
    builder.putFloat(x);
    return builder.offset();
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public Quat get(int j) { return get(new Quat(), j); }
    public Quat get(Quat obj, int j) {  return obj.__assign(__element(j), bb); }
  }
}

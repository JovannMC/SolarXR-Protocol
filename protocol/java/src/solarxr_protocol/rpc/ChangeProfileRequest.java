// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.rpc;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

@SuppressWarnings("unused")
public final class ChangeProfileRequest extends Table {
  public static void ValidateVersion() { Constants.FLATBUFFERS_22_10_26(); }
  public static ChangeProfileRequest getRootAsChangeProfileRequest(ByteBuffer _bb) { return getRootAsChangeProfileRequest(_bb, new ChangeProfileRequest()); }
  public static ChangeProfileRequest getRootAsChangeProfileRequest(ByteBuffer _bb, ChangeProfileRequest obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public ChangeProfileRequest __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public String profileName() { int o = __offset(4); return o != 0 ? __string(o + bb_pos) : null; }
  public ByteBuffer profileNameAsByteBuffer() { return __vector_as_bytebuffer(4, 1); }
  public ByteBuffer profileNameInByteBuffer(ByteBuffer _bb) { return __vector_in_bytebuffer(_bb, 4, 1); }
  public String type() { int o = __offset(6); return o != 0 ? __string(o + bb_pos) : null; }
  public ByteBuffer typeAsByteBuffer() { return __vector_as_bytebuffer(6, 1); }
  public ByteBuffer typeInByteBuffer(ByteBuffer _bb) { return __vector_in_bytebuffer(_bb, 6, 1); }

  public static int createChangeProfileRequest(FlatBufferBuilder builder,
      int profileNameOffset,
      int typeOffset) {
    builder.startTable(2);
    ChangeProfileRequest.addType(builder, typeOffset);
    ChangeProfileRequest.addProfileName(builder, profileNameOffset);
    return ChangeProfileRequest.endChangeProfileRequest(builder);
  }

  public static void startChangeProfileRequest(FlatBufferBuilder builder) { builder.startTable(2); }
  public static void addProfileName(FlatBufferBuilder builder, int profileNameOffset) { builder.addOffset(0, profileNameOffset, 0); }
  public static void addType(FlatBufferBuilder builder, int typeOffset) { builder.addOffset(1, typeOffset, 0); }
  public static int endChangeProfileRequest(FlatBufferBuilder builder) {
    int o = builder.endTable();
    return o;
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public ChangeProfileRequest get(int j) { return get(new ChangeProfileRequest(), j); }
    public ChangeProfileRequest get(ChangeProfileRequest obj, int j) {  return obj.__assign(__indirect(__element(j), bb), bb); }
  }
  public ChangeProfileRequestT unpack() {
    ChangeProfileRequestT _o = new ChangeProfileRequestT();
    unpackTo(_o);
    return _o;
  }
  public void unpackTo(ChangeProfileRequestT _o) {
    String _oProfileName = profileName();
    _o.setProfileName(_oProfileName);
    String _oType = type();
    _o.setType(_oType);
  }
  public static int pack(FlatBufferBuilder builder, ChangeProfileRequestT _o) {
    if (_o == null) return 0;
    int _profileName = _o.getProfileName() == null ? 0 : builder.createString(_o.getProfileName());
    int _type = _o.getType() == null ? 0 : builder.createString(_o.getType());
    return createChangeProfileRequest(
      builder,
      _profileName,
      _type);
  }
}


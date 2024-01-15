// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.rpc;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

@SuppressWarnings("unused")
public final class SaveImuCalibrationRequest extends Table {
  public static void ValidateVersion() { Constants.FLATBUFFERS_22_10_26(); }
  public static SaveImuCalibrationRequest getRootAsSaveImuCalibrationRequest(ByteBuffer _bb) { return getRootAsSaveImuCalibrationRequest(_bb, new SaveImuCalibrationRequest()); }
  public static SaveImuCalibrationRequest getRootAsSaveImuCalibrationRequest(ByteBuffer _bb, SaveImuCalibrationRequest obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public SaveImuCalibrationRequest __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }


  public static void startSaveImuCalibrationRequest(FlatBufferBuilder builder) { builder.startTable(0); }
  public static int endSaveImuCalibrationRequest(FlatBufferBuilder builder) {
    int o = builder.endTable();
    return o;
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public SaveImuCalibrationRequest get(int j) { return get(new SaveImuCalibrationRequest(), j); }
    public SaveImuCalibrationRequest get(SaveImuCalibrationRequest obj, int j) {  return obj.__assign(__indirect(__element(j), bb), bb); }
  }
  public SaveImuCalibrationRequestT unpack() {
    SaveImuCalibrationRequestT _o = new SaveImuCalibrationRequestT();
    unpackTo(_o);
    return _o;
  }
  public void unpackTo(SaveImuCalibrationRequestT _o) {
  }
  public static int pack(FlatBufferBuilder builder, SaveImuCalibrationRequestT _o) {
    if (_o == null) return 0;
    startSaveImuCalibrationRequest(builder);
    return endSaveImuCalibrationRequest(builder);
  }
}

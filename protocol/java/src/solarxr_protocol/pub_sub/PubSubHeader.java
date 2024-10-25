// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.pub_sub;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

@SuppressWarnings("unused")
public final class PubSubHeader extends Table {
  public static void ValidateVersion() { Constants.FLATBUFFERS_22_10_26(); }
  public static PubSubHeader getRootAsPubSubHeader(ByteBuffer _bb) { return getRootAsPubSubHeader(_bb, new PubSubHeader()); }
  public static PubSubHeader getRootAsPubSubHeader(ByteBuffer _bb, PubSubHeader obj) { _bb.order(ByteOrder.LITTLE_ENDIAN); return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb)); }
  public void __init(int _i, ByteBuffer _bb) { __reset(_i, _bb); }
  public PubSubHeader __assign(int _i, ByteBuffer _bb) { __init(_i, _bb); return this; }

  public byte uType() { int o = __offset(4); return o != 0 ? bb.get(o + bb_pos) : 0; }
  public Table u(Table obj) { int o = __offset(6); return o != 0 ? __union(obj, o + bb_pos) : null; }

  public static int createPubSubHeader(FlatBufferBuilder builder,
      byte uType,
      int uOffset) {
    builder.startTable(2);
    PubSubHeader.addU(builder, uOffset);
    PubSubHeader.addUType(builder, uType);
    return PubSubHeader.endPubSubHeader(builder);
  }

  public static void startPubSubHeader(FlatBufferBuilder builder) { builder.startTable(2); }
  public static void addUType(FlatBufferBuilder builder, byte uType) { builder.addByte(0, uType, 0); }
  public static void addU(FlatBufferBuilder builder, int uOffset) { builder.addOffset(1, uOffset, 0); }
  public static int endPubSubHeader(FlatBufferBuilder builder) {
    int o = builder.endTable();
    return o;
  }

  public static final class Vector extends BaseVector {
    public Vector __assign(int _vector, int _element_size, ByteBuffer _bb) { __reset(_vector, _element_size, _bb); return this; }

    public PubSubHeader get(int j) { return get(new PubSubHeader(), j); }
    public PubSubHeader get(PubSubHeader obj, int j) {  return obj.__assign(__indirect(__element(j), bb), bb); }
  }
  public PubSubHeaderT unpack() {
    PubSubHeaderT _o = new PubSubHeaderT();
    unpackTo(_o);
    return _o;
  }
  public void unpackTo(PubSubHeaderT _o) {
    solarxr_protocol.pub_sub.PubSubUnionUnion _oU = new solarxr_protocol.pub_sub.PubSubUnionUnion();
    byte _oUType = uType();
    _oU.setType(_oUType);
    Table _oUValue;
    switch (_oUType) {
      case solarxr_protocol.pub_sub.PubSubUnion.Message:
        _oUValue = u(new solarxr_protocol.pub_sub.Message());
        _oU.setValue(_oUValue != null ? ((solarxr_protocol.pub_sub.Message) _oUValue).unpack() : null);
        break;
      case solarxr_protocol.pub_sub.PubSubUnion.SubscriptionRequest:
        _oUValue = u(new solarxr_protocol.pub_sub.SubscriptionRequest());
        _oU.setValue(_oUValue != null ? ((solarxr_protocol.pub_sub.SubscriptionRequest) _oUValue).unpack() : null);
        break;
      case solarxr_protocol.pub_sub.PubSubUnion.TopicHandleRequest:
        _oUValue = u(new solarxr_protocol.pub_sub.TopicHandleRequest());
        _oU.setValue(_oUValue != null ? ((solarxr_protocol.pub_sub.TopicHandleRequest) _oUValue).unpack() : null);
        break;
      case solarxr_protocol.pub_sub.PubSubUnion.TopicMapping:
        _oUValue = u(new solarxr_protocol.pub_sub.TopicMapping());
        _oU.setValue(_oUValue != null ? ((solarxr_protocol.pub_sub.TopicMapping) _oUValue).unpack() : null);
        break;
      default: break;
    }
    _o.setU(_oU);
  }
  public static int pack(FlatBufferBuilder builder, PubSubHeaderT _o) {
    if (_o == null) return 0;
    byte _uType = _o.getU() == null ? solarxr_protocol.pub_sub.PubSubUnion.NONE : _o.getU().getType();
    int _u = _o.getU() == null ? 0 : solarxr_protocol.pub_sub.PubSubUnionUnion.pack(builder, _o.getU());
    return createPubSubHeader(
      builder,
      _uType,
      _u);
  }
}


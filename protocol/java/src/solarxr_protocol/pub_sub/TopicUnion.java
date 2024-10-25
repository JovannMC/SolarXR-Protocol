// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.pub_sub;

import com.google.flatbuffers.FlatBufferBuilder;

public class TopicUnion {
  private byte type;
  private Object value;

  public byte getType() { return type; }

  public void setType(byte type) { this.type = type; }

  public Object getValue() { return value; }

  public void setValue(Object value) { this.value = value; }

  public TopicUnion() {
    this.type = Topic.NONE;
    this.value = null;
  }

  public solarxr_protocol.pub_sub.TopicHandleT asTopicHandle() { return (solarxr_protocol.pub_sub.TopicHandleT) value; }
  public solarxr_protocol.pub_sub.TopicIdT asTopicId() { return (solarxr_protocol.pub_sub.TopicIdT) value; }

  public static int pack(FlatBufferBuilder builder, TopicUnion _o) {
    switch (_o.type) {
      case Topic.TopicHandle: return solarxr_protocol.pub_sub.TopicHandle.pack(builder, _o.asTopicHandle());
      case Topic.TopicId: return solarxr_protocol.pub_sub.TopicId.pack(builder, _o.asTopicId());
      default: return 0;
    }
  }
}


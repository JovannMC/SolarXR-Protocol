// automatically generated by the FlatBuffers compiler, do not modify

package slimevr_protocol;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

public class OutboundPacketT {
  private boolean acknowledgeMe;
  private slimevr_protocol.OutboundUnionUnion packet;

  public boolean getAcknowledgeMe() { return acknowledgeMe; }

  public void setAcknowledgeMe(boolean acknowledgeMe) { this.acknowledgeMe = acknowledgeMe; }

  public slimevr_protocol.OutboundUnionUnion getPacket() { return packet; }

  public void setPacket(slimevr_protocol.OutboundUnionUnion packet) { this.packet = packet; }


  public OutboundPacketT() {
    this.acknowledgeMe = false;
    this.packet = null;
  }
}


// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.device.pairing;

import java.nio.*;
import java.lang.*;
import java.util.*;
import com.google.flatbuffers.*;

public class DeviceSensorInfoT {
  private int id;
  private int type;
  private solarxr_protocol.device.pairing.ImuFeatureInfoT[] features;

  public int getId() { return id; }

  public void setId(int id) { this.id = id; }

  public int getType() { return type; }

  public void setType(int type) { this.type = type; }

  public solarxr_protocol.device.pairing.ImuFeatureInfoT[] getFeatures() { return features; }

  public void setFeatures(solarxr_protocol.device.pairing.ImuFeatureInfoT[] features) { this.features = features; }


  public DeviceSensorInfoT() {
    this.id = 0;
    this.type = 0;
    this.features = null;
  }
}


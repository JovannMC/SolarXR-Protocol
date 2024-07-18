// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.data_feed.tracker

import java.nio.*
import kotlin.math.sign
import com.google.flatbuffers.*

/**
 * Static description of a tracker
 */
@Suppress("unused")
class TrackerInfo : Table() {

    fun __init(_i: Int, _bb: ByteBuffer)  {
        __reset(_i, _bb)
    }
    fun __assign(_i: Int, _bb: ByteBuffer) : TrackerInfo {
        __init(_i, _bb)
        return this
    }
    val imuType : UShort
        get() {
            val o = __offset(4)
            return if(o != 0) bb.getShort(o + bb_pos).toUShort() else 0u
        }
    /**
     * The user-assigned role of the tracker.
     */
    val bodyPart : UByte
        get() {
            val o = __offset(6)
            return if(o != 0) bb.get(o + bb_pos).toUByte() else 0u
        }
    /**
     * average samples per second
     */
    val pollRate : solarxr_protocol.datatypes.HzF32? get() = pollRate(solarxr_protocol.datatypes.HzF32())
    fun pollRate(obj: solarxr_protocol.datatypes.HzF32) : solarxr_protocol.datatypes.HzF32? {
        val o = __offset(8)
        return if (o != 0) {
            obj.__assign(o + bb_pos, bb)
        } else {
            null
        }
    }
    /**
     * The orientation of the tracker when mounted on the body
     */
    val mountingOrientation : solarxr_protocol.datatypes.math.Quat? get() = mountingOrientation(solarxr_protocol.datatypes.math.Quat())
    fun mountingOrientation(obj: solarxr_protocol.datatypes.math.Quat) : solarxr_protocol.datatypes.math.Quat? {
        val o = __offset(10)
        return if (o != 0) {
            obj.__assign(o + bb_pos, bb)
        } else {
            null
        }
    }
    /**
     * Should the tracker's settings be editable by the user
     */
    val editable : Boolean
        get() {
            val o = __offset(12)
            return if(o != 0) 0.toByte() != bb.get(o + bb_pos) else false
        }
    /**
     * Indicates if the tracker is computed (solved position and rotation)
     */
    val isComputed : Boolean
        get() {
            val o = __offset(14)
            return if(o != 0) 0.toByte() != bb.get(o + bb_pos) else false
        }
    /**
     * Indicates if the tracker is using an IMU for its tracking data
     */
    val isImu : Boolean
        get() {
            val o = __offset(16)
            return if(o != 0) 0.toByte() != bb.get(o + bb_pos) else false
        }
    /**
     * A human-friendly name to display as the name of the tracker.
     */
    val displayName : String?
        get() {
            val o = __offset(18)
            return if (o != 0) __string(o + bb_pos) else null
        }
    val displayNameAsByteBuffer : ByteBuffer get() = __vector_as_bytebuffer(18, 1)
    fun displayNameInByteBuffer(_bb: ByteBuffer) : ByteBuffer = __vector_in_bytebuffer(_bb, 18, 1)
    /**
     * name to display as the name of the tracker set by the user
     */
    val customName : String?
        get() {
            val o = __offset(20)
            return if (o != 0) __string(o + bb_pos) else null
        }
    val customNameAsByteBuffer : ByteBuffer get() = __vector_as_bytebuffer(20, 1)
    fun customNameInByteBuffer(_bb: ByteBuffer) : ByteBuffer = __vector_in_bytebuffer(_bb, 20, 1)
    /**
     * Whether to allow yaw drift compensation for this tracker or not.
     */
    val allowDriftCompensation : Boolean
        get() {
            val o = __offset(22)
            return if(o != 0) 0.toByte() != bb.get(o + bb_pos) else false
        }
    /**
     * Mounting Reset orientation overrides the current `mounting_orientation` of
     * the tracker, this orientation is not saved and needs to be calculated
     * each time the server is ran
     */
    val mountingResetOrientation : solarxr_protocol.datatypes.math.Quat? get() = mountingResetOrientation(solarxr_protocol.datatypes.math.Quat())
    fun mountingResetOrientation(obj: solarxr_protocol.datatypes.math.Quat) : solarxr_protocol.datatypes.math.Quat? {
        val o = __offset(24)
        return if (o != 0) {
            obj.__assign(o + bb_pos, bb)
        } else {
            null
        }
    }
    /**
     * Indicates if the tracker is actually an HMD
     */
    val isHmd : Boolean
        get() {
            val o = __offset(26)
            return if(o != 0) 0.toByte() != bb.get(o + bb_pos) else false
        }
    /**
     * Indicates what type of data the tracker sends (note: it always ends up being rotation in the end)
     */
    val dataSupport : UByte
        get() {
            val o = __offset(28)
            return if(o != 0) bb.get(o + bb_pos).toUByte() else 0u
        }
    companion object {
        @JvmStatic
        fun validateVersion() = Constants.FLATBUFFERS_22_10_26()
        @JvmStatic
        fun getRootAsTrackerInfo(_bb: ByteBuffer): TrackerInfo = getRootAsTrackerInfo(_bb, TrackerInfo())
        @JvmStatic
        fun getRootAsTrackerInfo(_bb: ByteBuffer, obj: TrackerInfo): TrackerInfo {
            _bb.order(ByteOrder.LITTLE_ENDIAN)
            return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb))
        }
        @JvmStatic
        fun startTrackerInfo(builder: FlatBufferBuilder) = builder.startTable(13)
        @JvmStatic
        fun addImuType(builder: FlatBufferBuilder, imuType: UShort) = builder.addShort(0, imuType.toShort(), 0)
        @JvmStatic
        fun addBodyPart(builder: FlatBufferBuilder, bodyPart: UByte) = builder.addByte(1, bodyPart.toByte(), 0)
        @JvmStatic
        fun addPollRate(builder: FlatBufferBuilder, pollRate: Int) = builder.addStruct(2, pollRate, 0)
        @JvmStatic
        fun addMountingOrientation(builder: FlatBufferBuilder, mountingOrientation: Int) = builder.addStruct(3, mountingOrientation, 0)
        @JvmStatic
        fun addEditable(builder: FlatBufferBuilder, editable: Boolean) = builder.addBoolean(4, editable, false)
        @JvmStatic
        fun addIsComputed(builder: FlatBufferBuilder, isComputed: Boolean) = builder.addBoolean(5, isComputed, false)
        @JvmStatic
        fun addIsImu(builder: FlatBufferBuilder, isImu: Boolean) = builder.addBoolean(6, isImu, false)
        @JvmStatic
        fun addDisplayName(builder: FlatBufferBuilder, displayName: Int) = builder.addOffset(7, displayName, 0)
        @JvmStatic
        fun addCustomName(builder: FlatBufferBuilder, customName: Int) = builder.addOffset(8, customName, 0)
        @JvmStatic
        fun addAllowDriftCompensation(builder: FlatBufferBuilder, allowDriftCompensation: Boolean) = builder.addBoolean(9, allowDriftCompensation, false)
        @JvmStatic
        fun addMountingResetOrientation(builder: FlatBufferBuilder, mountingResetOrientation: Int) = builder.addStruct(10, mountingResetOrientation, 0)
        @JvmStatic
        fun addIsHmd(builder: FlatBufferBuilder, isHmd: Boolean) = builder.addBoolean(11, isHmd, false)
        @JvmStatic
        fun addDataSupport(builder: FlatBufferBuilder, dataSupport: UByte) = builder.addByte(12, dataSupport.toByte(), 0)
        @JvmStatic
        fun endTrackerInfo(builder: FlatBufferBuilder) : Int {
            val o = builder.endTable()
            return o
        }
    }
}

// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.rpc

import java.nio.*
import kotlin.math.sign
import com.google.flatbuffers.*

/**
 * Returns the current max positional tracker height, an estimated full height (user height), and the current min positional tracker height
 */
@Suppress("unused")
class HeightResponse : Table() {

    fun __init(_i: Int, _bb: ByteBuffer)  {
        __reset(_i, _bb)
    }
    fun __assign(_i: Int, _bb: ByteBuffer) : HeightResponse {
        __init(_i, _bb)
        return this
    }
    val maxHeight : Float
        get() {
            val o = __offset(4)
            return if(o != 0) bb.getFloat(o + bb_pos) else 0.0f
        }
    val estimatedFullHeight : Float
        get() {
            val o = __offset(6)
            return if(o != 0) bb.getFloat(o + bb_pos) else 0.0f
        }
    val minHeight : Float
        get() {
            val o = __offset(8)
            return if(o != 0) bb.getFloat(o + bb_pos) else 0.0f
        }
    companion object {
        @JvmStatic
        fun validateVersion() = Constants.FLATBUFFERS_22_10_26()
        @JvmStatic
        fun getRootAsHeightResponse(_bb: ByteBuffer): HeightResponse = getRootAsHeightResponse(_bb, HeightResponse())
        @JvmStatic
        fun getRootAsHeightResponse(_bb: ByteBuffer, obj: HeightResponse): HeightResponse {
            _bb.order(ByteOrder.LITTLE_ENDIAN)
            return (obj.__assign(_bb.getInt(_bb.position()) + _bb.position(), _bb))
        }
        @JvmStatic
        fun createHeightResponse(builder: FlatBufferBuilder, maxHeight: Float, estimatedFullHeight: Float, minHeight: Float) : Int {
            builder.startTable(3)
            addMinHeight(builder, minHeight)
            addEstimatedFullHeight(builder, estimatedFullHeight)
            addMaxHeight(builder, maxHeight)
            return endHeightResponse(builder)
        }
        @JvmStatic
        fun startHeightResponse(builder: FlatBufferBuilder) = builder.startTable(3)
        @JvmStatic
        fun addMaxHeight(builder: FlatBufferBuilder, maxHeight: Float) = builder.addFloat(0, maxHeight, 0.0)
        @JvmStatic
        fun addEstimatedFullHeight(builder: FlatBufferBuilder, estimatedFullHeight: Float) = builder.addFloat(1, estimatedFullHeight, 0.0)
        @JvmStatic
        fun addMinHeight(builder: FlatBufferBuilder, minHeight: Float) = builder.addFloat(2, minHeight, 0.0)
        @JvmStatic
        fun endHeightResponse(builder: FlatBufferBuilder) : Int {
            val o = builder.endTable()
            return o
        }
    }
}

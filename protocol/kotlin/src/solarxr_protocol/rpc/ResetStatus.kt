// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.rpc

@Suppress("unused")
class ResetStatus private constructor() {
    companion object {
        const val STARTED: UByte = 0u
        const val FINISHED: UByte = 1u
        val names : Array<String> = arrayOf("STARTED", "FINISHED")
        @JvmStatic
        fun name(e: Int) : String = names[e]
    }
}

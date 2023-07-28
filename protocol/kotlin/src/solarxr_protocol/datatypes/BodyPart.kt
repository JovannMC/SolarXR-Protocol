// automatically generated by the FlatBuffers compiler, do not modify

package solarxr_protocol.datatypes

/**
 * Different parts of the body. Roughly maps to each possible bone in the skeleton.
 * These are *NOT* the trackers.
 */
@Suppress("unused")
class BodyPart private constructor() {
    companion object {
        const val NONE: UByte = 0u
        const val HEAD: UByte = 1u
        const val NECK: UByte = 2u
        const val CHEST: UByte = 3u
        const val WAIST: UByte = 4u
        const val HIP: UByte = 5u
        const val LEFTUPPERLEG: UByte = 6u
        const val RIGHTUPPERLEG: UByte = 7u
        const val LEFTLOWERLEG: UByte = 8u
        const val RIGHTLOWERLEG: UByte = 9u
        const val LEFTFOOT: UByte = 10u
        const val RIGHTFOOT: UByte = 11u
        const val LEFTLOWERARM: UByte = 14u
        const val RIGHTLOWERARM: UByte = 15u
        const val LEFTUPPERARM: UByte = 16u
        const val RIGHTUPPERARM: UByte = 17u
        const val LEFTHAND: UByte = 18u
        const val RIGHTHAND: UByte = 19u
        const val LEFTSHOULDER: UByte = 20u
        const val RIGHTSHOULDER: UByte = 21u
        const val UPPERCHEST: UByte = 22u
        const val ACCESSORY: UByte = 23u
        val names : Array<String> = arrayOf("NONE", "HEAD", "NECK", "CHEST", "WAIST", "HIP", "LEFT_UPPER_LEG", "RIGHT_UPPER_LEG", "LEFT_LOWER_LEG", "RIGHT_LOWER_LEG", "LEFT_FOOT", "RIGHT_FOOT", "", "", "LEFT_LOWER_ARM", "RIGHT_LOWER_ARM", "LEFT_UPPER_ARM", "RIGHT_UPPER_ARM", "LEFT_HAND", "RIGHT_HAND", "LEFT_SHOULDER", "RIGHT_SHOULDER", "UPPER_CHEST", "ACCESSORY")
        @JvmStatic
        fun name(e: Int) : String = names[e]
    }
}

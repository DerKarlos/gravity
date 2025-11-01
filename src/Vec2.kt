import kotlin.math.sqrt

class Vec2 @JvmOverloads constructor(var x: Double = 0.0, var y: Double = 0.0) {
    // constructor(toCopy: Vec2) : this(toCopy.x, toCopy.y)

    /** A "close to zero" float epsilon value for use  */
    val epsilon: Double = 1.1920928955078125E-7

    /** Zero out this vector.  */
    fun setZero() {
        x = 0.0
        y = 0.0
    }

    /** Return the sum of this vector and another; does not alter either one.  */
    fun add(v: Vec2): Vec2 {
        return Vec2(x + v.x, y + v.y)
    }

    /** Return the difference of this vector and another; does not alter either one.  */
    fun sub(v: Vec2): Vec2 {
        return Vec2(x - v.x, y - v.y)
    }

    /** Return this vector multiplied by a scalar; does not alter this vector.  */
    fun mul(a: Double): Vec2 {
        return Vec2(x * a, y * a)
    }

    /** Return the length of this vector.  */
    fun length(): Double {
        return sqrt(x * x + y * y)
    }

    /** Normalize this vector and return the length before normalization. Alters this vector.  */
    fun normalize(): Double {
        val length = length()
        if (length < epsilon) {
            return 0.0
        }

        val invLength = 1.0 / length
        x *= invLength
        y *= invLength
        return length
    }


}
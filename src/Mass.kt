import java.awt.Color
import java.awt.Graphics
import kotlin.math.sqrt

class Mass(
    x: Double,
    y: Double,
    vx: Double,
    vy: Double,
    color: Color,
    mass: Double,
    diameter: Double,
    name: String,
) {

    companion object {

        //declare class!! variables
        var maxOrbit: Double = 0.0

        // The simulation only uses real measurement units (SI-Units)
        const val g: Double = 6.67384e-11 //        m^3/(kg*^2)  Earth gravity constant
        const val M_AE: Double = 149597870700.0 //  m per Astronomic Unit ~1e12

        // Values to adapt to visualisation todo: from sim.settings struct
        const val MAX_GRAVITY_DISTANCE = 1e38f // [AE]
        const val DRAW_FACT: Double = 200.0 // size x times, to make the mass visible
        const val DRAW_MIN: Int =   3 // minimum size [Pixel], to have the mass always visible
        const val DRAW_MAX: Int = 200 // maximum size [Pixel], to have large suns not overdraw it all
    }

    //declare instance variables
    private val diameter: Double = diameter * 1000 // convert km to m// km
    private val name: String
    private val mass: Double
    private val color: Color
    private var position: Vec2
    private var velocity: Vec2
    private var acceleration: Vec2


    // mass constructor assigns values to instance variables
    init {
        this.position = Vec2(x * M_AE, y * M_AE)
        this.velocity = Vec2(vx, vy)
        this.acceleration = Vec2()
        this.color = color
        this.mass = mass
        this.name = name
        // this.diameter = diameter * 1000 // convert km to m

        val orbit = position.length()
        if (maxOrbit < orbit) maxOrbit = orbit
        // Main.out("maxOrbit: " + maxOrbit);
    }

    fun draggedBy(other: Mass) { /* calculate gravitation-acceleration of other masse to this one */
        //this.dragged = function(oo) { with(this) {

        if ((other == this) ||  // Dont drag rag yourselves
            (other.mass == 0.0) // Other has mass (no ship)
        ) return

        val distanceVector: Vec2 = other.position.sub(this.position) // m (meter)
        val distance = distanceVector.length() // m

        // cheating: far away no dragging¸¸
        if (distance < MAX_GRAVITY_DISTANCE * M_AE ) {
            distanceVector.normalize() // x/y-part 0-1

            // wikipedia: Newtons gravity theory: Dragging decreases in square of distance
            // GRAVITATION m/s² is MASS kg / DISTANCE SQUARE m² * EARTH ACCELERATION G m^3/(kg*s²)
            val acceleration = other.mass / (distance * distance) * g * 1 /*2=???*/
            val accelerationVector = distanceVector.mul(acceleration)

            // accumulate other forces for this mass
            this.acceleration = this.acceleration.add(accelerationVector)
        }
    }

    fun move(dt: Double) {
        velocity = velocity.add(acceleration.mul(dt))
        position = position.add(velocity.mul(dt))
        acceleration.setZero()
    }

    fun draw(g: Graphics, panel: Panel) {
        // sqrt-sqrt / 2 is sheeting to reduce the huge differences in size
        var size = ((sqrt(sqrt((diameter / M_AE))) / 2.0 * (DRAW_FACT))).toInt()
        if (size < DRAW_MIN) size = DRAW_MIN // To have the mass always visible
        if (size > DRAW_MAX) size = DRAW_MAX // To have large suns not overdraw it all


        //paint the mass at x, y with a width and height of the mass size
        val screenPosition = panel.scale(position)
        g.color = color
        g.fillOval(screenPosition.x.toInt(), screenPosition.y.toInt(), size, size)
    }

}

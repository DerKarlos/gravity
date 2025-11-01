import java.awt.Color
import java.awt.Graphics
import kotlin.math.sqrt

class Mass(
    x: Double,
    y: Double,
    vx: Double,
    vy: Double,
    color: Color?,
    mass: Double,
    diameter: Double
) {
    //declare instance variables
    private val diameter: Double
    private val mass: Double
    private val color: Color?
    private var position: Vec2
    private var velocity: Vec2
    private var acceleration: Vec2

    // mass constructor assigns values to instance variables
    init {
        this.position = Vec2(x, y)
        this.velocity = Vec2(vx, vy)
        this.acceleration = Vec2()
        this.color = color
        this.mass = mass
        this.diameter = diameter // ???

        /* TODO: class org.jetbrains.kotlin.nj2k.types.JKJavaNullPrimitiveType */
        val orbit = position.length()
        if (maxOrbit < orbit) maxOrbit = orbit
        // Main.out("maxOrbit: " + maxOrbit);
    }

    fun draggedBy(other: Mass) { /* calculate gravitation-acceleration of other masse to this one */
        //this.dragged = function(oo) { with(this) {

        if ((other !== this) &&  // Sich selbst zieht man nicht an
            (other.mass > 0.0) // Anziehender hat Masse (kein Schiff)
        ) {
            // drag              						//                 Maßeinheit
            //var x = (oo.px-px) * mAE;	                        //        m aus AE
            //var y = (oo.py-py) * mAE;	                        //        m aus AE
            //var p = Pythagoras(x,y);   						// Vektorisiert   m

            val distance = other.position.sub(this.position).mul(M_AE)
            val length = distance.length()

            //	Schummel: über diesem Abstand keine Gravitation
            if (length < 1e99 /*???*/) {
                //    x = x / p;            						// x-Anteil       0-1
                //    y = y / p;            						// y              0-1
                distance.normalize()
                // wikipedia: Newtonsche_Gravitationstheorie: Die Anziehung nimmt im Quadrat der Entfernung ab
                // a = oo.mass / (p*p) * g * 2/*???*/; // GRAVITATION    kg/m² * m^3/(kg*s²) =Beschleunigung.
                val acceleration: Double =
                    (other.mass / (length * length)) * g * 2 /*???*/
                //    ax += ((a∗x) / mAE); // m/s² * 1-> AE/s²
                //    ay += ((a∗y) / mAE); // anderes Object anziehen (Anteilsmäßig für x und y)
                val x = distance.mul(acceleration).mul(1f / M_AE)
                this.acceleration = this.acceleration.add(x)
            }
        }
    }

    fun move(dt: Double) {
        velocity = velocity.add(acceleration.mul(dt))
        position = position.add(velocity.mul(dt))
        acceleration.setZero()
    }

    fun draw(g: Graphics, panel: Panel) {
        // sqrt is sheeting to reduce the huge differences in size
        var size = ((sqrt((diameter / KM_AE).toDouble()) * (D_FACT))).toInt()
        if (size < D_MIN) size = D_MIN // Damit die Masse garantiert immer sichtbar ist

        if (size > D_MAX) size = D_MAX // Damit die Sonnen nicht alles "überstrahlen"


        //paint the mass at x, y with a width and height of the mass size
        //System.out.println("position: " + position);
        val screenPosition = panel.scale(position)

        //set the brush color to the mass color
        g.color = color
        g.fillOval(screenPosition.x.toInt(), screenPosition.y.toInt(), size, size)
    }

    companion object {
        //declare class variables
        var maxOrbit: Double = 0.0

        // The simulation only uses real measurement units (SI-Units???)
        const val g: Double = 6.67384e-11 //         gravity constant    m^3/(kg*^2)
        const val KM_AE: Double = 149597870.700 //    km per Astronomic Unit   ~1e9
        const val M_AE:  Double = 149597870700.0 //      m per Astronomic Unit   ~1e12

        // Diese Werte sind Kompromisse zur Visualisierung. Bedienbar?
        // const val PG_MAX = 1e38f // Über diesem Abstand [AE] keine Gravitation
        const val D_FACT: Double = 200.0 // x-mal größer, sonst währen die Massen alle gleich klein/groß
        const val D_MIN: Int =   5 // Mindestgröße [Pixel], sonst währen die kleinen Planeten aus der Entfernung nicht sichtbar
        const val D_MAX: Int = 100 // Maximalgröße [Pixel], sonst währen die Sonnen größer als die Umlaufbahnen
    }
}

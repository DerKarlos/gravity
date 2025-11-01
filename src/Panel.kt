import java.awt.Color
import java.awt.Graphics
import javax.swing.JPanel

class Panel : JPanel() {
    // dynamic Class variables
    private val zView = 1.2

    // The simulation only uses real measurement units (SI-Units???)
    val dSun: Double = 1.3914e6 //          Sol diameter in km
    val dEar: Double = 12756.32 //        Earth diameter in km
    //val dJup: Double = 142984.0 //      Jupiter diameter in km
    val dLun: Double = 3476.0 //           Luna diameter in km

    val mSun: Double = 1.989e30 //          Sol mass in kg
    val mEar: Double = 5.974e24 //        Earth mass in kg
    //val mJup: Double = 1.899e27 //      Jupiter mass in kg
    val mLun: Double = 7.349e22 //         Luna mass in kg


    //                   Position.x,      .y, Speed.x,y,    color,  mass, diameter
    private var sun = Mass(0.0, 0.0, 0.0, 0.0, Color.YELLOW, mSun, dSun)
    private val mercury = Mass(0.0, .4, 0.0000003, .0, Color.RED, mLun, dLun)
    private val venus = Mass(0.0, .8, 0.00000015, .0, Color.GRAY, mEar, dEar)
    private val earth = Mass(0.0, 1.0, 0.0000001, .0, Color.BLUE, mEar, dEar)

    /**
     * Called once each frame to handle essential game operations
     */
    fun simulate(dt: Double) {
        // calc accelerations caused by masses
        earth.draggedBy(sun)
        venus.draggedBy(sun)
        mercury.draggedBy(sun)
        earth.draggedBy(venus)
        venus.draggedBy(earth)
        earth.draggedBy(mercury)
        mercury.draggedBy(earth)
        mercury.draggedBy(venus)
        venus.draggedBy(mercury)

        //move the masses one frame
        //sun.move(dt);
        venus.move(dt)
        mercury.move(dt)
        earth.move(dt)
    }

    /**
     * Updates and draws all the graphics on the screen
     */
    override fun paintComponent(g: Graphics) {
        //draw the background, set color to BLACK and fill in a rectangle
        g.color = Color.BLACK
        g.fillRect(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT)

        //draw the mass
        sun.draw(g, this)
        mercury.draw(g, this)
        venus.draw(g, this)
        earth.draw(g, this)
    }

    fun scale(position: Vec2): Vec2 {
        return position.mul(pixel / zView).add(WINDOW_CENTER)
    }

    companion object {
        // dynamic Class variables
        const val WINDOW_WIDTH: Int = 640
        const val WINDOW_HEIGHT: Int = 480 // ??? calculate frame
        val WINDOW_CENTER: Vec2 = Vec2(
            (WINDOW_WIDTH / 2).toDouble(),
            (WINDOW_HEIGHT / 2).toDouble()
        )

        // Die kleinere Ausdehnung zählt als normaler darstellbar Bildpunktebereich
        // The smallest extend of the window counts as visible screen range
        val pixel: Int = Math.min(WINDOW_WIDTH / 2, WINDOW_HEIGHT / 2)

        // static final Double g = 6.67384e-11f; //         gravity constant    m^3/(kg*^2)
        //val aKey: Double = 1e-7 //             truster power

        //val sTag: Double = 60.0 * 60 * 24 //     Seconds per day  ~10000    1e4
        //val sJahr: Double = sTag * 364.25 //   Seconds per year ~4000'000 4e6
    }
}

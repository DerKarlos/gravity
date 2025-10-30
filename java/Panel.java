import java.awt.*;
import javax.swing.*;

public class Panel extends JPanel {

    // dynamic Class variables

    static final int WINDOW_WIDTH = 640,
        WINDOW_HEIGHT = 480; // ??? calculate frame
    static final Vec2 WINDOW_CENTER = new Vec2(
        WINDOW_WIDTH / 2,
        WINDOW_HEIGHT / 2
    );
    // Die kleinere Ausdehnung zählt als normaler darstellbar Bildpunktebereich
    // The smalest extend of the window counts as visible screen range
    static final int pixel = Math.min(WINDOW_WIDTH / 2, WINDOW_HEIGHT / 2);

    // The simulation only uses real mesurement units (SI-Units???)

    static final Float dSun = 1.3914e6f; //          Sol diameter in km
    static final Float dEar = 12756.32f; //        Earth diameter in km
    static final Float dJup = 142984.0f; //      Jupiter diameter in km
    static final Float dLun = 3476.0f; //           Luna diameter in km

    static final Float mSun = 1.989e30f; //          Sol mass in kg
    static final Float mEar = 5.974e24f; //        Earth mass in kg
    static final Float mJup = 1.899e27f; //      Jupiter mass in kg
    static final Float mLun = 7.349e22f; //         Luna mass in kg

    // static final Float g = 6.67384e-11f; //         gravity constant    m^3/(kg*^2)

    static final Float aKey = 1e-7f; //             truster power

    static final Float sTag = 60 * 60 * 24f; //     Secounds per day  ~10000    1e4
    static final Float sJahr = sTag * 364.25f; //   Secounds per year ~4000'000 4e6

    // dynamic Class variables
    private Float zView = 1.2f;

    private Mass sun, mercury, venus, earth;

    public Panel() {
        // System.out.println("" + pixel + "" + zView);

        //      Position.x,   .y, Speed.x,y,    coulor,  mass, diameter
        sun = new Mass(0.f, 0.f, 0.f, 0.f, Color.YELLOW, mSun, dSun);
        mercury = new Mass(0f, .4f, 0.0000003f, 0f, Color.RED, mLun, dLun);
        venus = new Mass(0f, .8f, 0.00000015f, 0f, Color.GRAY, mEar, dEar);
        earth = new Mass(0f, 1f, 0.0000001f, 0f, Color.BLUE, mEar, dEar);
    }

    /**
     * Called once each frame to handle essential game operations
     */
    public void simulate(float dt) {
        // calc accelerations caused by masses
        earth.dragged_by(sun);
        venus.dragged_by(sun);
        mercury.dragged_by(sun);
        earth.dragged_by(venus);
        venus.dragged_by(earth);
        earth.dragged_by(mercury);
        mercury.dragged_by(earth);
        mercury.dragged_by(venus);
        venus.dragged_by(mercury);

        //move the masses one frame
        //sun.move(dt);
        venus.move(dt);
        mercury.move(dt);
        earth.move(dt);
    }

    /**
     * Updates and draws all the graphics on the screen
     */
    public void paintComponent(Graphics g) {
        //draw the background, set color to BLACK and fill in a rectangle
        g.setColor(Color.BLACK);
        g.fillRect(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT);

        //draw the mass
        sun.draw(g, this);
        mercury.draw(g, this);
        venus.draw(g, this);
        earth.draw(g, this);
    }

    public Vec2 scale(Vec2 position) {
        return position.mul(pixel / zView).add(WINDOW_CENTER);
    }
}

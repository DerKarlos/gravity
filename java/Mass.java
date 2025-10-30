import java.awt.Color;
import java.awt.Graphics;

public class Mass {

    //declare class variables
    static float maxOrbit;
    //declare instance variables
    private float diameter;
    private float mass;
    private Color color;
    private Vec2 position, velocity, acceleraton;

    // The simulation only uses real mesurement units (SI-Units???)
    static final Float g = 6.67384e-11f; //         gravity constant    m^3/(kg*^2)
    static final Float kmAE = 149597870.700f; //    km per Astronomic Unit   ~1e9
    static final Float mAE = 149597870700.f; //      m per Astronomic Unit   ~1e12

    // Diese Werte sind Kompromisse zur Visualisierung. Bedienbar?
    static final Float pgMax = 1e38f; // Über diesem Abstand [AE] keine Gravitaton
    static final Float dFact = 200f; // x mal größer, sonst währen die Massen alle gleich klein/groß
    static final int dMin = 5; // Mindestgröße [Pixsel], sonst währen die kleinen Planeten aus der Entfernung nicht sichtbar
    static final int dMax = 100; // Maximalgröße [Pixsel], sonst währen die Sonnen größer als die Umlaufbahnen

    // mass constructor assigns values to instance variables
    public Mass(
        float x,
        float y,
        float vx,
        float vy,
        Color color,
        float mass,
        float diameter
    ) {
        this.position = new Vec2(x, y);
        this.velocity = new Vec2(vx, vy);
        this.acceleraton = new Vec2();
        this.color = color;
        this.mass = mass;
        this.diameter = diameter; // ???

        var orbit = position.length();
        if (maxOrbit < orbit) maxOrbit = orbit;
        // Main.out("maxOrbit: " + maxOrbit);
    }

    public void dragged_by(Mass other) {
        //// Gravitation einer anderen Masse bei DIESER Ausüben
        //this.dragged = function(oo) { with(this) {

        if (
            (other != this) && // Sich selbst zieht man nicht an
            (other.mass > 0.) // Anziehender hat Masse (kein Schiff)
        ) {
            // drag              						//                 Maßeinheit
            //var x = (oo.px-px) * mAE;	                        //        m aus AE
            //var y = (oo.py-py) * mAE;	                        //        m aus AE
            //var p = Phytagoras(x,y);   						// Vektorisiert   m

            var distance = other.position.sub(this.position).mul(mAE);
            var length = distance.length();

            //	Schummel: über diesem Abstand keine Gravitaton
            if (length < 1e99 /*???*/) {
                //    x = x / p;            						// x-Anteil       0-1
                //    y = y / p;            						// y              0-1
                distance.normalize();
                // wikipedia: Newtonsche_Gravitationstheorie: Die Anziehung nimmt im Quadrat der Entfernung ab
                // a = oo.mass / (p*p) * g * 2/*???*/; // GRAVITATION    kg/m² * m^3/(kg*s²) =Beschleunigung.
                var acceleration =
                    (other.mass / (length * length)) * g * 2 /*???*/;
                //    ax += ((a*x) / mAE);   //                m/s²  *  1   -> AE/s²
                //    ay += ((a*y) / mAE);   //   anderes Object anziehen (Anteilsmäßig für x und y)
                var x = distance.mul(acceleration).mul(1.f / mAE);
                this.acceleraton = this.acceleraton.add(x);
            }
        }
    }

    public void move(float dt) {
        velocity = velocity.add(acceleraton.mul(dt));
        position = position.add(velocity.mul(dt));
        acceleraton.setZero();
    }

    public void draw(Graphics g, Panel panel) {
        // sqrt is sheeting to reduce the huge differences in size
        var size = (int) (Math.sqrt(diameter / kmAE) * (dFact));
        if (size < dMin) size = dMin; // Damit die Masse garantiert immer sichtbar ist
        if (size > dMax) size = dMax; // Damit die Sonnen nicht alles "überstrahlen""

        //paint the mass at x, y with a width and height of the mass size
        //System.out.println("position: " + position);
        var screenPosition = panel.scale(position);

        //set the brush color to the mass color
        g.setColor(color);
        g.fillOval((int) screenPosition.x, (int) screenPosition.y, size, size);
    }
}

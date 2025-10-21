import java.awt.Color;
import java.awt.Graphics;

public class Mass {

    //declare instance variables
    private int size;
    private float mass;
    private Color color;
    private Vec2 position, velocity, acceleraton;

    // mass constructor assigns values to instance variables
    public Mass(float x, float y, float vx, float vy, Color color, int mass) {
        this.position = new Vec2(x, y);
        this.velocity = new Vec2(vx, vy);
        this.acceleraton = new Vec2();
        this.color = color;
        this.mass = mass;
        this.size = mass; // ???
    }

    public void drag(Mass other, float dt) {
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
            var distance = other.position.sub(this.position);
            var length = distance.length();

            //	Schummel: über diesem Abstand keine Gravitaton
            if (length < 1e99 /*???*/) {
                //    x = x / p;            						// x-Anteil       0-1
                //    y = y / p;            						// y              0-1
                distance.normalize();
                // wikipedia: Newtonsche_Gravitationstheorie: Die Anziehung nimmt im Quadrat der Entfernung ab
                // a = oo.mass / (p*p) * g * 2/*???*/; // GRAVITATION    kg/m² * m^3/(kg*s²) =Beschleunigung.
                var acceleration = other.mass / (length * length); //??? * g * 2;
                //    ax += ((a*x) / mAE);   //                m/s²  *  1   -> AE/s²
                //    ay += ((a*y) / mAE);   //   anderes Object anziehen (Anteilsmäßig für x und y)
                this.acceleraton = this.acceleraton.add(
                    distance.mulLocal(acceleration)
                );
            }
        }
    }

    public void move(float dt) {
        velocity = velocity.add(acceleraton.mul(dt));
        position = position.add(velocity.mul(dt));
        acceleraton.setZero();
    }

    /**
     * Detect collision with screen borders and reverse direction
     * @param top - the y value of the top of the screen
     * @param bottom - the y value of the bottom of the screen
     */
    public void bounceOffEdges(int top, int bottom) {
        //if the y value is at the bottom of the screen
        if (position.y > bottom - size) {
            reverseY();
        }
        //if y value is at top of screen
        else if (position.y < top) {
            reverseY();
        }

        //if x value is at left or right side
        //hard-coded values, we will delete this section later
        if (position.x < 0) {
            reverseX();
        } else if (position.x > 640 - size) {
            reverseX();
        }
    }

    /**
     * Reverse's the ball's change in x value
     */
    public void reverseX() {
        velocity.x *= -1;
    }

    /**
     * Reverse's the ball's change in y value
     */
    public void reverseY() {
        velocity.y *= -1;
    }

    public void draw(Graphics g) {
        //set the brush color to the mass color
        g.setColor(color);

        //paint the mass at x, y with a width and height of the mass size
        System.out.println(size);
        g.fillOval((int) position.x, (int) position.y, size, size);
    }
}

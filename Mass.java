import java.awt.Color;
import java.awt.Graphics;

public class Mass {

    //declare instance variables
    private int size;
    private Color color;
    private Vec2 position, velocity;

    // mass constructor assigns values to instance variables
    public Mass(int x, int y, int cx, int cy, Color color, int size) {
        this.position = new Vec2(x, y);
        //        this.pos.x = x;
        //        this.pos.y = y;
        this.velocity = new Vec2(cx, cy);
        //this.cx = cx;
        //this.cy = cy;
        this.color = color;
        this.size = size;
    }

    public void move() {
        position = position.add(velocity);
        //position.x += cx;
        //position.y += cy;

        //    this.move = function(dt,pre) { with(this) {
        //		vx += ax*dt;  ax=0;  px += vx*dt;
        //	  vy += ay*dt;  ay=0;  py += vy*dt;
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

    public void paint(Graphics g) {
        //set the brush color to the mass color
        g.setColor(color);

        //paint the mass at x, y with a width and height of the mass size
        g.fillOval((int) position.x, (int) position.y, size, size);
    }
}

import java.awt.Color;
import java.awt.Graphics;

public class Mass {

    //declare instance variables
    private int x, y, cx, cy, size;
    private Color color;

    // private Vec2 pos;

    // mass constructor assigns values to instance variables
    public Mass(int x, int y, int cx, int cy, Color color, int size) {
        this.x = x;
        this.y = y;
        this.cx = cx;
        this.cy = cy;
        this.color = color;
        this.size = size;
    }

    public void move() {
        x += cx;
        y += cy;

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
        if (y > bottom - size) {
            reverseY();
        }
        //if y value is at top of screen
        else if (y < top) {
            reverseY();
        }

        //if x value is at left or right side
        //hard-coded values, we will delete this section later
        if (x < 0) {
            reverseX();
        } else if (x > 640 - size) {
            reverseX();
        }
    }

    /**
     * Reverse's the ball's change in x value
     */
    public void reverseX() {
        cx *= -1;
    }

    /**
     * Reverse's the ball's change in y value
     */
    public void reverseY() {
        cy *= -1;
    }

    public void paint(Graphics g) {
        //set the brush color to the mass color
        g.setColor(color);

        //paint the mass at x, y with a width and height of the mass size
        g.fillOval(x, y, size, size);
    }
}

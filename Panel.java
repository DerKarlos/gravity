import java.awt.*;
import javax.swing.*;

public class Panel extends JPanel {

    static final int WINDOW_WIDTH = 640,
        WINDOW_HEIGHT = 480;

    private Mass mass;

    public Panel() {
        mass = new Mass(300, 200, 3, 3, Color.YELLOW, 10);
    }

    /**
     * Called once each frame to handle essential game operations
     */
    public void simulate() {
        //move the mass one frame
        mass.move();
        //edge check/bounce
        mass.bounceOffEdges(0, WINDOW_HEIGHT);
    }

    /**
     * Updates and draws all the graphics on the screen
     */
    public void paintComponent(Graphics g) {
        //draw the background, set color to BLACK and fill in a rectangle
        g.setColor(Color.BLACK);
        g.fillRect(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT);

        //draw the mass
        mass.paint(g);
    }
}

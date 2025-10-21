import java.awt.*;
import javax.swing.*;

public class Panel extends JPanel {

    static final int WINDOW_WIDTH = 640,
        WINDOW_HEIGHT = 480;

    private Mass sol, earth;

    public Panel() {
        sol = new Mass(300.f, 200.f, 0.f, 0.f, Color.YELLOW, 10);
        earth = new Mass(500.f, 200.f, 0.f, 0.1f, Color.BLUE, 5);
    }

    /**
     * Called once each frame to handle essential game operations
     */
    public void simulate(float dt) {
        earth.drag(sol, dt);
        //move the mass one frame
        sol.move(dt);
        earth.move(dt);
        //edge check/bounce
        earth.bounceOffEdges(0, WINDOW_HEIGHT);
    }

    /**
     * Updates and draws all the graphics on the screen
     */
    public void paintComponent(Graphics g) {
        //draw the background, set color to BLACK and fill in a rectangle
        g.setColor(Color.BLACK);
        g.fillRect(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT);

        //draw the mass
        sol.draw(g);
        earth.draw(g);
    }
}

// https://kevinsguides.com/guides/code/java/javaprojs/simple-2d-pong/
// https://github.com/kevinsguides/java_simplepong/blob/main/Readme.md

import java.awt.event.ActionEvent;
import java.awt.event.ActionListener;
import javax.swing.*;

public class Main {

    //declare and initialize the frame
    static JFrame frame = new JFrame("Gravity Simulation");

    public static void main(String[] args) {
        // make it so program exits on close button click
        frame.setDefaultCloseOperation(WindowConstants.EXIT_ON_CLOSE);
        // the size of the game will be 480x640, the size of the JFrame needs to be slightly larger
        frame.setSize(640, 508);

        // System.out.println("h: " + r.height + "  w: " + r.width);

        // disable window resizing
        Panel sim = new Panel();
        // make the new GravitySim
        frame.setResizable(false);
        //add the game to the JFrame
        frame.add(sim);

        //show the window
        frame.setVisible(true);

        final int DT_MS = 15;
        Timer timer = new Timer(
            DT_MS,
            new ActionListener() {
                @Override
                public void actionPerformed(ActionEvent e) {
                    // simulation logic
                    sim.simulate(DT_MS * 1000); // 1000000 times faster orbit visualisation: about 10s per Earth orbit

                    // draw / repaint the screen
                    sim.repaint();
                }
            }
        );

        //start the timer after it's been created
        timer.start();
    }
}

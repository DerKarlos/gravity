// https://kevinsguides.com/guides/code/java/javaprojs/simple-2d-pong/
// https://github.com/kevinsguides/java_simplepong/blob/main/Readme.md

import java.awt.Rectangle;
import java.awt.event.ActionEvent;
import java.awt.event.ActionListener;
import javax.swing.*;

public class Main {

    static void out(String text) {
        System.out.println(text);
    }

    //declare and initialize the frame
    static JFrame frame = new JFrame("Pong");

    public static void main(String[] args) {
        // make it so program exits on close button click
        frame.setDefaultCloseOperation(WindowConstants.EXIT_ON_CLOSE);
        // the size of the game will be 480x640, the size of the JFrame needs to be slightly larger
        frame.setSize(640, 508);

        Rectangle r = frame.getBounds();
        var h = r.height;
        var w = r.width;
        out("h: " + h + "  w: " + w); // h: 508  w: 640

        // disable window resizing
        Panel sim = new Panel();
        // make the new GravitySim
        frame.setResizable(false);
        //add the game to the JFrame
        frame.add(sim);

        //show the window
        frame.setVisible(true);

        Timer timer = new Timer(
            33,
            new ActionListener() {
                @Override
                public void actionPerformed(ActionEvent e) {
                    //game logic
                    sim.simulate();

                    //repaint the screen
                    sim.repaint();
                }
            }
        );

        //start the timer after it's been created
        timer.start();
    }
}

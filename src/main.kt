import javax.swing.JFrame
import javax.swing.Timer
import javax.swing.WindowConstants

fun main() {

    //declare and initialize the frame
    val frame = JFrame("Gravity Simulation")

    // make it so program exits on close button click
    frame.setDefaultCloseOperation(WindowConstants.EXIT_ON_CLOSE)
    // the size of the game will be 640x480, the size of the JFrame needs to be slightly larger
    frame.setSize(640, 508)

    // disable window resizing
    frame.setResizable(false)

    // make the new GravitySim
    val sim = Panel()
    //add the game to the JFrame
    frame.add(sim)

    //show the window
    frame.isVisible = true

    val dtMs = 15
    val dtS = (dtMs * 1000).toDouble()
    val timer = Timer(
        dtMs
    )
    // What syntax is this ???
    {
        // simulation logic
        sim.simulate(dtS) // 1000000 times faster orbit visualisation: about 10s per Earth orbit

        // draw / repaint the screen
        sim.repaint()
    }

    //start the timer after it's been created
    timer.start()
}

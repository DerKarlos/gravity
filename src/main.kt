import javax.swing.JFrame
import javax.swing.Timer
import javax.swing.WindowConstants

fun main() {

    //declare and initialize the frame
    val frame = JFrame("Gravity Simulation")

    // make it so program exits on close button click
    frame.setDefaultCloseOperation(WindowConstants.EXIT_ON_CLOSE)
    // the size of the game will be 1500x480, the size of the JFrame needs to be slightly larger
    frame.setSize(1000, 708)

    // disable window resizing
    frame.setResizable(false)

    // make the new GravitySim
    val simulation = Panel()
    //add the game to the JFrame
    frame.add(simulation)

    //show the window
    frame.isVisible = true

    val dtMs = 15
    val secondsPerYear = 60.0 * 60.0 * 24 * 365.0
    val secondsPerOrbit =  secondsPerYear / 10
    val secondsPerFrame = dtMs / 1000.0 * secondsPerOrbit

    val timer = Timer(dtMs )
    // What syntax is this ???
    {
        // simulation logic
        simulation.move(secondsPerFrame) // 1000000 times faster orbit visualisation: about 10s per Earth orbit

        // draw / repaint the screen
        simulation.repaint()
    }

    //start the timer after it's been created
    timer.start()
}

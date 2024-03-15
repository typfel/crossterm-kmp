import crossterm.ClearType
import crossterm.Terminal
import crossterm.Event
import crossterm.KeyCode

fun main(args: Array<String>) {
    val terminal = Terminal()
    terminal.execute(
        listOf(
            crossterm.Command.EnableMouseCapture,
            crossterm.Command.EnterAlternateScreen
        )
    )
    terminal.enableRawMode()

    while(true) {
        val event = terminal.read()

        terminal.execute(
            listOf(
                crossterm.Command.Clear(ClearType.CURRENT_LINE),
                crossterm.Command.MoveTo(0u, 0u)
            )
        )

        print(event)

        if (event is Event.Key && event.event.code == KeyCode.Char("q")) {
            terminal.disableRawMode()
            terminal.execute(
                listOf(
                    crossterm.Command.DisableMouseCapture,
                    crossterm.Command.LeaveAlternateScreen
                )
            )
            return
        }
    }
}
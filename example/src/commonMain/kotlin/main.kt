import io.github.typfel.crossterm.ClearType
import io.github.typfel.crossterm.Terminal
import io.github.typfel.crossterm.Event
import io.github.typfel.crossterm.KeyCode
import io.github.typfel.crossterm.Command

fun main(args: Array<String>) {
    val terminal = Terminal()
    terminal.execute(
        listOf(
            Command.EnableMouseCapture,
            Command.EnterAlternateScreen
        )
    )
    terminal.enableRawMode()

    while(true) {
        val event = terminal.read()

        terminal.execute(
            listOf(
                Command.Clear(ClearType.CURRENT_LINE),
                Command.MoveTo(0u, 0u)
            )
        )

        print(event)

        if (event is Event.Key && event.event.code == KeyCode.Char("q")) {
            terminal.disableRawMode()
            terminal.execute(
                listOf(
                    Command.DisableMouseCapture,
                    Command.LeaveAlternateScreen
                )
            )
            return
        }
    }
}
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import com.jakewharton.mosaic.layout.height
import com.jakewharton.mosaic.layout.offset
import com.jakewharton.mosaic.layout.size
import com.jakewharton.mosaic.modifier.Modifier
import io.github.typfel.crossterm.Terminal
import io.github.typfel.crossterm.Event
import io.github.typfel.crossterm.KeyCode
import io.github.typfel.crossterm.Command
import com.jakewharton.mosaic.runMosaicBlocking
import com.jakewharton.mosaic.ui.Box
import com.jakewharton.mosaic.ui.Column
import com.jakewharton.mosaic.ui.Spacer
import com.jakewharton.mosaic.ui.Text
import com.jakewharton.mosaic.ui.unit.IntOffset
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext

private const val width = 20
private const val height = 10

private const val robotWidth = 3
private const val robotHeight = 1

fun main() = runMosaicBlocking {
    var x by mutableStateOf(0)
    var y by mutableStateOf(0)

    val terminal = Terminal()
    terminal.execute(listOf(Command.EnterAlternateScreen))
    terminal.enableRawMode()

    setContent {
        Column {
            Text("Use arrow keys to move the face. Press “q” to exit.")
            Text("Position: $x, $y   Robot: $robotWidth, $robotHeight   World: $width, $height")
            Spacer(Modifier.height(1))
            Box(modifier = Modifier.size(width, height).offset { IntOffset(x, y) }) {
                Text("^_^")
            }
        }
    }

    withContext(Dispatchers.Default) {
        while (true) {
            when (val event = terminal.read()) {
                is Event.Key ->
                    when (event.event.code) {
                        KeyCode.Char("q") -> break
                        KeyCode.Up -> y = (y - 1).coerceAtLeast(0)
                        KeyCode.Down -> y = (y + 1).coerceAtMost(height - robotHeight)
                        KeyCode.Right -> x = (x + 1).coerceAtMost(width - robotWidth)
                        KeyCode.Left -> x = (x - 1).coerceAtLeast(0)
                        else -> continue
                    }
                else -> continue
            }
        }
    }
}.also {
    val terminal = Terminal()
    terminal.disableRawMode()
    terminal.execute(listOf(Command.LeaveAlternateScreen))
}
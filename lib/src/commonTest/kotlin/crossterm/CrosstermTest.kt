package crossterm

import kotlin.test.Test
import kotlin.test.assertEquals

class CrosstermTest {

    @Test
    fun testFoo() {
        val terminal = Terminal()
        print(terminal.size())
//        terminal.execute(listOf(Command.Clear(ClearType.ALL)))
//        terminal.execute(Clear(ClearType.ALL))
        assertEquals(8u, add(5u, 3u))
    }
}
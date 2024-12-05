import java.io.File


fun main() {
    fun p01(input: List<List<Char>>) {
        val directions = mapOf('^' to Pair(0, -1), '>' to Pair(1, 0), 'v' to Pair(0, 1), '<' to Pair(-1, 0))
        val contraptionSizeX = input[0].size
        val contraptionSizeY = input.size
        val currentTile = Pair(0, 0) // x, y
        val currentDirections = mutableListOf<Pair<Char, Pair<Int, Int>>>() // ^ > v <
        var currentDirection = '>'
        val energized = false
        while (!energized) {
            when (input[currentTile.second][currentTile.first]) {
                '\\' -> {}
                '/' -> {}
                '-' -> {}
                '|' -> {}
                '.' -> {}
                else -> {
                    throw Exception("Invalid direction")
                }

            }
        }
    }

    val lines = File("src/main/resources/day15").readLines().map { it -> it.map { it } }
    println(p01(lines))
}
import java.io.File

fun main() {
    fun p01(input: List<Pair<String, Long>>) {
        val points = mutableListOf(Pair(0L, 0L))
        var curr = Pair(0L, 0L)
        var p = 0L
        var s = 0L

        for ((direction, length) in input) {
            when (direction) {
                "U" -> {
                    curr = Pair(curr.first, curr.second - length)
                }

                "R" -> {
                    curr = Pair(curr.first + length, curr.second)
                }

                "D" -> {
                    curr = Pair(curr.first, curr.second + length)
                }

                "L" -> {
                    curr = Pair(curr.first - length, curr.second)
                }

                else -> throw Exception("Invalid direction")
            }.also {
                points.add(curr)
                p += length
            }
        }

        points.windowed(2) {
            s += it[0].first * it[1].second - it[1].first * it[0].second
        }

        println("Cubic meters: ${(s + p) / 2 + 1}")
    }

    p01(File("src/main/resources/day18").readLines()
        .map { Pair(it.split(" ")[0], it.split(" ")[1].toLong()) })

    p01(
        File("src/main/resources/day18").readLines()
            .map {
                val curr = it.split(" ")[2].replace("#", "").replace(")", "").replace("(", "")
                Pair(
                    when (curr.last()) {
                        '0' -> "R"
                        '1' -> "D"
                        '2' -> "L"
                        '3' -> "U"
                        else -> throw Exception("Invalid direction")
                    },
                    curr.substring(0, 5).toLong(radix = 16)
                )
            }
    )
}
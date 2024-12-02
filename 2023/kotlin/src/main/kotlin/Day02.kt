import java.io.File

fun main() {
    val lines = File("src/main/resources/day02").readLines()

    fun part1(input: List<String>) {
        var sum = 0
        for (line in input) {
            var pass = true
            line.split(": ")[1].split(";").map { s -> s.trim() }.map { set -> set.split(',') }.forEach { set ->
                set.forEach { cube ->
                    val colors = cube.trim().split(" ")
                    when (colors[1]) {
                        "red" -> if (colors[0].toInt() > 12) pass = false
                        "green" -> if (colors[0].toInt() > 13) pass = false
                        "blue" -> if (colors[0].toInt() > 14) pass = false
                    }
                }
            }

            if (pass) {
                sum += line.split(": ")[0].split(" ")[1].toInt()
            }
        }

        println("sum: $sum")
    }


    fun part2(input: List<String>) {
        var powers = 0
        for (line in input) {
            val gameCubes = mutableListOf<Triple<Int, Int, Int>>()
            line.split(": ")[1].split(";").map { s -> s.trim() }.map { set -> set.split(',') }.forEach { set ->
                var red = 0
                var green = 0
                var blue = 0
                set.forEach { cube ->
                    val (value, color) = cube.trim().split(" ")
                    when (color) {
                        "red" -> red = value.toInt()
                        "green" -> green = value.toInt()
                        "blue" -> blue = value.toInt()
                    }
                }

                gameCubes.add(Triple(red, green, blue))
            }

            val (gameRedCube, gameGreenCube, gameBlueCube) = gameCubes.fold(
                Triple(
                    0,
                    0,
                    0
                )
            ) { (red, green, blue), (r, g, b) ->
                Triple(maxOf(red, r), maxOf(green, g), maxOf(blue, b))
            }

            powers += (gameRedCube * gameGreenCube * gameBlueCube)
        }

        println("powers: $powers")
    }

    part1(lines)
    part2(lines)
}
import java.io.File

fun main() {
    val lines = File("src/main/resources/day04").readLines()


    fun part01(input: List<String>) {
        var score = 0
        for (line in input) {
            val (winningNumbers, elfNumbers) = line.split(": ").last.split(" | ")
                .map { it.split(" ").filter { p -> p.isNotEmpty() } }

            score += (winningNumbers + elfNumbers).groupBy { it }
                .filter { it.value.size > 1 }.keys.filter { it.isNotEmpty() }
                .fold(0) { acc, _ ->
                    if (acc == 0) 1 else acc * 2
                }
        }

        println(score)
    }


    fun part02(input: List<String>) {
        var total = 0

        fun calcTotal(inp: List<String>) {
            total += inp.size
            if (inp.isNotEmpty()) {
                val newInp = mutableListOf<String>()

                for (line in inp) {
                    val (winningNumbers, elfNumbers) = line.split(": ").last.split(" | ")
                        .map { it.split(" ").filter { p -> p.isNotEmpty() } }

                    val numberMatches = (winningNumbers + elfNumbers).groupBy { it }
                        .filter { it.value.size > 1 }.keys.size

                    if (numberMatches > 0) {
                        for (i in input.indexOf(line) + 1..numberMatches + input.indexOf(line)) {
                            newInp.add(input[i])
                        }
                    }

                }

                calcTotal(newInp)
            }
        }

        calcTotal(input)
        println(total)
    }

    part01(lines)
    part02(lines)
}
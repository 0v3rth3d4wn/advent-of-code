import java.io.File

fun main() {
    fun p01(input: List<String>) {
        var sum = 0
        val tilted = mutableListOf<String>()

        val rotated = (List(input[0].length) { k ->
            List(input.size) { b ->
                input[b][k]
            }.reversed()
        })

        rotated.forEach { println(it) }

        val stripped = mutableListOf<String>()
        for (line in rotated) {
            val curr = StringBuilder()
            var count = 0
            var hasMetSquaredRock = false
            var hasMetO = false
            for (c in line) {
                if (c == '#') {
                    hasMetSquaredRock = true
                    curr.append('#'.toString().padStart(count + 1, '.'))
                    count = 0
                }

                if (c == '.' && hasMetSquaredRock) {
                    count++

                    if (hasMetO) {
//                        curr.append('.')
                        curr.append(".".repeat(count + 1))
                        hasMetO = false
                    }

                }

                if (c == 'O' && hasMetSquaredRock) {
                    count++
                }

                if (c == 'O') {
                    curr.append(c.toString().padStart(count + 1, '.'))
                    count = 0
                    hasMetSquaredRock = false
                    hasMetO = true
                }

            }
            if (hasMetSquaredRock) {
                curr.append(".".repeat(count + 1))
            }

//            if (hasMetSquaredRock) {
//                curr.append(".".repeat(count))
//            }


            println(curr.toString().padStart(input.size + 20, ' '))
//            println(curr.toString())
            for ((i, e) in curr.toString().padStart(input.size + 1, ' ').reversed().withIndex()) {
                if (e == 'O') {
                    sum += (input.size + 1 - i)
                }
//                println("${i + 1} -> $e")
            }


//            stripped.add(curr.toString().padStart(10, ' '))
        }
        println(sum)

//        println(stripped)


//        for (i in 0..input.size - 2) {
//            val newLine = StringBuilder()
//            for (j in 0..<input[i].length) {
//                if (listOf('O', '#').contains(input[i][j])) {
//                    newLine.append(input[i][j])
//                    continue
//                }
//
//                if (input[i][j] == '.' && input[i + 1][j] == 'O') {
//                    newLine.append('O')
//                } else {
//                    newLine.append(input[i][j])
//                }
//            }
//
//            tilted.add(newLine.toString())
//        }
//
//        input.forEach { println(it) }
//        println()
//        tilted.forEach { println(it) }
    }


    val lines = File("src/main/resources/demo").readLines()
    p01(lines)
}

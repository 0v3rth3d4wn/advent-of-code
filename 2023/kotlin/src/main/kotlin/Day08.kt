import java.io.File

fun main() {
    val lines = File("src/main/resources/day08").readLines()
    val nodes = mutableMapOf<String, Pair<String, String>>()

    lines.drop(2).forEach {
        val (k, v) = it.split(" = ")
        nodes[k] = Pair(
            v.split(", ")[0].replace("(", ""),
            v.split(", ")[1].replace(")", "")
        )
    }

    fun part01(input: List<String>): Int {
        val directions = input[0]
        var found = false
        var count = 0
        var curr: String? = "AAA"
        var directionIndex = 0

        while (!found) {
            if (directionIndex == directions.length) {
                directionIndex = 0
            }

            curr = if (directions[directionIndex] == 'L') {
                nodes[curr]?.first
            } else {
                nodes[curr]?.second
            }

            directionIndex++
            count++


            if (curr == "ZZZ") {
                found = true
            }
        }

        return count
    }

    fun part02(input: List<String>): Int {
        val directions = input[0]
        var found = false
        var count = 0
        var curr: List<String> = nodes.keys.filter { it.endsWith("A") }
        var directionIndex = 0

        while (!found) {
            if (directionIndex == directions.length) {
                directionIndex = 0
            }


            curr = if (directions[directionIndex] == 'L') {
                val newList = mutableListOf<String>()
                for (c in curr.asSequence()) {
                    val n = nodes[c]?.first
                    if (n != null) {
                        newList.add(n)
                    }
                }

                newList
            } else {
                val newList = mutableListOf<String>()
                for (c in curr.asSequence()) {
                    val n = nodes[c]?.second
                    if (n != null) {
                        newList.add(n)
                    }
                }

                newList
            }

            directionIndex++
            count++

            if (curr.all { it.endsWith("Z") }) {
                found = true
            }
        }

        return count

    }

//    println(part01(lines))
    println(part02(lines))

}

//19199
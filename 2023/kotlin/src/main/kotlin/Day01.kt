import java.io.File

fun main() {
    val lines = File("src/main/resources/day01").readLines()
    val words = mapOf(
        "1" to "1",
        "2" to "2",
        "3" to "3",
        "4" to "4",
        "5" to "5",
        "6" to "6",
        "7" to "7",
        "8" to "8",
        "9" to "9",
        "one" to "1",
        "two" to "2",
        "three" to "3",
        "four" to "4",
        "five" to "5",
        "six" to "6",
        "seven" to "7",
        "eight" to "8",
        "nine" to "9"
    )


    fun part01(input: List<String>, words: Map<String, String>) =
        input.fold(0) { acc, s ->
            acc + s.findAnyOf(words.values.distinct())?.second.plus(
                s.findLastAnyOf(words.values.distinct())?.second
            ).toInt()
        }

    fun part02(input: List<String>, words: Map<String, String>) =
        input.fold(0) { acc, s ->
            val p = Pair(
                words[s.findAnyOf(words.keys)?.second],
                words[s.findLastAnyOf(words.keys)?.second]
            )

            acc + p.first.plus(p.second).toInt()
        }

    println(part01(lines, words))
    println(part02(lines, words))
}
//
//
//    fun part0101(input: List<String>) =
//        input.map { it.filter { c -> c.toString().toIntOrNull() != null } }.fold(0) { acc, s ->
//            acc + s.first().plus(s.last().toString()).toInt()
//        }
//
//    fun part0202(input: List<String>): Int {
//        val wordsMap = mapOf(
//            "one" to "1",
//            "two" to "2",
//            "three" to "3",
//            "four" to "4",
//            "five" to "5",
//            "six" to "6",
//            "seven" to "7",
//            "eight" to "8",
//            "nine" to "9"
//        )
//
//        val formattedLines = input.map {
//            it.replace("(?>(one|two|three|four|five|six|seven|eight|nine))".toRegex()) { n ->
////                println(n.value)
//                wordsMap[n.value].orEmpty()
//            }
//
//        }
//
////        println(formattedLines)
//
//
//        return part0101(formattedLines)
//    }
//
//    println(part0101(lines))
//    println(part0202(lines))
//}


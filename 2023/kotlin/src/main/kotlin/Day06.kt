import java.io.File

fun main() {
    val lines = File("src/main/resources/day06").readLines()
    fun parse(line: String) = line.split(":")[1].split(" ").filter { it.isNotEmpty() }.map { it.toLong() }
    val racesP1: List<Pair<Long, Long>> = parse(lines[0]).zip(parse(lines[1])).map { Pair(it.first, it.second) }
    val racesP2 = listOf(racesP1.fold(Pair<Long, Long>(0, 0)) { (aFirst, aSecond), (cFirst, cSecond) ->
        Pair(
            (aFirst.toString() + cFirst.toString()).toLong(),
            (aSecond.toString() + cSecond.toString()).toLong()
        )
    })

    fun res(input: List<Pair<Long, Long>>) = println(input.fold(mutableListOf<Int>()) { acc, (t2, d2) ->
        acc.add((0..t2).fold(0) { a, b -> if (b * (t2 - b) > d2) a + 1 else a })
        acc
    }.fold(1) { acc, curr -> acc * curr })

    res(racesP1)
    res(racesP2)
}
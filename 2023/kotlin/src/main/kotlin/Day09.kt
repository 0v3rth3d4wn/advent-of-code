import java.io.File

fun generateSequences(history: List<Int>, sequences: MutableList<List<Int>>): MutableList<List<Int>> {
    if (history.all { it == 0 }) {
        return sequences
    }

    val difference = history.reversed().windowed(2).map { it.first - it.last }.reversed()
    sequences.add(difference)
    generateSequences(difference, sequences)

    return sequences
}

fun part01(input: List<List<Int>>): Int = input.sumOf { sequence ->
    var tmp = 0
    generateSequences(sequence, mutableListOf(sequence)).reversed().map { list ->
        val newList = list.map { it }.toMutableList()

        if (list.all { it == 0 }) {
            newList.add(tmp)
        } else if (list.distinct().size == 1) {
            newList.add(list.first)
        } else {
            newList.add(tmp + list.last)
        }

        tmp = newList.last

        newList
    }.reversed().map { it.last }.first
}

fun part02(input: List<List<Int>>): Int = input.sumOf { sequence ->
    var tmp = 0
    generateSequences(sequence, mutableListOf(sequence)).reversed().map { list ->
        val newList = list.map { it }.toMutableList()

        if (list.all { it == 0 }) {
            newList.add(0, tmp)
        } else {
            newList.add(0, list.first - tmp)
        }

        tmp = newList.first

        newList
    }.reversed().map { it.first }.first
}

fun main() {
    val lines = File("src/main/resources/day09").readLines().map { it -> it.split(" ").map { it.toInt() } }
    println(part01(lines))
    println(part02(lines))
}

//1798691765
//1104

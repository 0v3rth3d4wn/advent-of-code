import java.io.File

fun main() {
    val lines = File("src/main/resources/day03").readLines()
    val lineLength = lines[0].length
    val linesLength = lines.size
    val partNumbers = mutableListOf<Int>()

    for (line in lines) {
        val lineIndex = lines.indexOf(line)
        val numbers = "(\\d+)".toRegex().findAll(line).toList().map { Pair(it.range, it.value.toInt()) }

        for ((range, number) in numbers) {
            if (lineIndex == 0 && range.first == 0) {
                if (line[range.last + 1] != '.' || lines[lineIndex + 1].slice(range.first..range.last + 1)
                        .any { c -> c != '.' && !c.isDigit() }
                ) {
                    partNumbers.add(number)
                }
            } else if (lineIndex == 0 && range.last == lineLength - 1) {
                if (line[range.first - 1] != '.' || lines[lineIndex + 1].slice(range.first - 1..range.last)
                        .any { c -> c != '.' && !c.isDigit() }
                ) {
                    partNumbers.add(number)
                }
            } else if (lineIndex == linesLength - 1 && range.first == 0) {
                if (line[range.last + 1] != '.' || lines[lineIndex - 1].slice(range.first..range.last + 1)
                        .any { c -> c != '.' && !c.isDigit() }
                ) {
                    partNumbers.add(number)
                }
            } else if (lineIndex == linesLength - 1 && range.last == lineLength - 1) {
                if (line[range.first - 1] != '.' || lines[lineIndex - 1].slice(range.first - 1..range.last)
                        .any { c -> c != '.' && !c.isDigit() }
                ) {
                    partNumbers.add(number)
                }
            } else if (range.first == 0) {
                if (line[range.last + 1] != '.' ||
                    lines[lineIndex - 1].slice(range.first..range.last + 1).any { c -> c != '.' && !c.isDigit() } ||
                    lines[lineIndex + 1].slice(range.first..range.last + 1).any { c -> c != '.' && !c.isDigit() }
                ) {
                    partNumbers.add(number)
                }

            } else if (range.last == lineLength - 1) {
                if (line[range.first - 1] != '.' ||
                    lines[lineIndex - 1].slice(range.first - 1..range.last).any { c -> c != '.' && !c.isDigit() } ||
                    lines[lineIndex + 1].slice(range.first - 1..range.last).any { c -> c != '.' && !c.isDigit() }
                ) {
                    partNumbers.add(number)
                }
            } else if (lineIndex == 0) {
                if (lines[lineIndex + 1].slice(range.first - 1..range.last + 1).any { c -> c != '.' && !c.isDigit() } ||
                    line[range.first - 1] != '.' || line[range.last + 1] != '.') {

                    partNumbers.add(number)
                }
            } else if (lineIndex == lineLength - 1) {
                if (lines[lineIndex - 1].slice(range.first - 1..range.last + 1).any { c -> c != '.' && !c.isDigit() } ||
                    line[range.first - 1] != '.' || line[range.last + 1] != '.') {
                    partNumbers.add(number)
                }
            } else {
                if (lines[lineIndex - 1].slice(range.first - 1..range.last + 1).any { c -> c != '.' && !c.isDigit() } ||
                    lines[lineIndex + 1].slice(range.first - 1..range.last + 1).any { c -> c != '.' && !c.isDigit() } ||
                    line[range.first - 1] != '.' || line[range.last + 1] != '.') {
                    partNumbers.add(number)
                }
            }
        }
    }

    println(partNumbers.sum())

    fun part02(input: List<String>) {

        val starList = mutableListOf<Pair<Int, Int>>()
        val numberList = mutableListOf<Pair<Pair<IntRange, Int>, Int>>()
        val gearRatio = mutableListOf<List<Int>>()
        val lastX = lineLength - 1
        val lastY = linesLength - 1

        for (line in input) {
            val lineIndex = input.indexOf(line)
            val stars = "(\\*)".toRegex().findAll(line).toList().map { Pair(it.range, it.value) }
            val numbers = "(\\d+)".toRegex().findAll(line).toList().map { Pair(it.range, it.value.toInt()) }

            for (star in stars) {
                starList.add(Pair(star.first.first, lineIndex))
            }

            for ((range, number) in numbers) {
                numberList.add(Pair(Pair(range, number), lineIndex))
            }
        }

        for (star in starList) {
            if (star.first == 0 && star.second == 0) {
                val n = numberList.filter { (v, y) ->
                    (y == 0 && v.first.contains(star.first + 1)) || (y == 1 && v.first.contains(star.first))
                }.map { it.first.second }

                gearRatio.add(n)

            } else if (star.first == lastX && star.second == 0) {
                val n = numberList.filter { (v, y) ->
                    (y == 0 && v.first.contains(lastX - 1)) || (y == 1 && v.first.contains(lastX - 1))
                }.map { it.first.second }

                gearRatio.add(n)

            } else if (star.first == lastX && star.second == lastY) {
                val n = numberList.filter { (v, y) ->
                    (y == lastY && v.first.contains(lastX - 1)) || (y == lastY - 1 && v.first.contains(lastX - 1))
                }.map { it.first.second }

                gearRatio.add(n)

            } else if (star.first == 0 && star.second == lastY) {
                val n = numberList.filter { (v, y) ->
                    (y == lastY && v.first.contains(star.first + 1)) || (y == lastY - 1 && v.first.contains(lastX - 1))
                }.map { it.first.second }

                gearRatio.add(n)

            } else if (star.first == 0) {
                val n = numberList.filter { (v, y) ->
                    (y == star.second - 1 && v.first.contains(star.first)) ||
                            (y == star.second + 1 && v.first.contains(star.first)) ||
                            (y == star.second && v.first.contains(star.first + 1))
                }.map { it.first.second }

                gearRatio.add(n)

            } else if (star.first == lastX) {
                val n = numberList.filter { (v, y) ->
                    (y == star.second - 1 && v.first.contains(lastX - 1)) ||
                            (y == star.second + 1 && v.first.contains(lastX - 1)) ||
                            (y == star.second && v.first.contains(lastX - 1))

                }.map { it.first.second }

                gearRatio.add(n)

            } else if (star.second == 0) {
                val n = numberList.filter { (v, y) ->
                    (y == 0 && v.first.contains(star.first - 1)) ||
                            (y == 0 && v.first.contains(star.first + 1)) ||
                            (y == 1 && v.first.contains(star.first - 1)) ||
                            (y == 1 && v.first.contains(star.first + 1))
                }.map { it.first.second }

                gearRatio.add(n)

            } else if (star.second == lastY) {
                val n = numberList.filter { (v, y) ->
                    (y == lastY && v.first.contains(star.first - 1)) ||
                            (y == lastY && v.first.contains(star.first + 1)) ||
                            (y == lastY - 1 && v.first.contains(star.first - 1)) ||
                            (y == lastY - 1 && v.first.contains(star.first + 1))
                }.map { it.first.second }

                gearRatio.add(n)

            } else {
                val n = numberList.filter { (v, y) ->
                    (y == star.second - 1 && v.first.contains(star.first - 1)) ||
                            (y == star.second - 1 && v.first.contains(star.first + 1)) ||
                            (y == star.second - 1 && v.first.first == star.first) ||
                            (y == star.second + 1 && v.first.contains(star.first - 1)) ||
                            (y == star.second + 1 && v.first.contains(star.first + 1)) ||
                            (y == star.second + 1 && v.first.first == star.first) ||
                            (y == star.second && v.first.contains(star.first - 1)) ||
                            (y == star.second && v.first.contains(star.first + 1))
                }.map { it.first.second }

                gearRatio.add(n)
            }
        }

        println(gearRatio.filter { it.size == 2 }.map { it.first * it.last }.sum())

    }

    part02(lines)
}
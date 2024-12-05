import java.io.File

fun generate(record: String, start: Int, last: Int, list: MutableList<String>): MutableList<String> {
    if (start == last) {
        list.add(record)
        return list
    }

    generate(record + '0'.toString(), start + 1, last, list)
    generate(record + '1'.toString(), start + 1, last, list)

    return list
}

fun binaryString(n: Int): MutableList<String> {
    if (n <= 0) {
        return mutableListOf()
    }

    return generate("", 0, n, mutableListOf())
}

fun p01(input: List<String>) {
    var sum = 0
    for (row in input) {
        val conditions = row.split(" ").first
        val groups = row.split(" ").last.split(",").map { it.toInt() }
        val combinations = binaryString(conditions.count { it == '?' }).map { it.replace('0', '.').replace('1', '#') }
        val brokenIndexes = conditions.mapIndexedNotNull { index, c -> index.takeIf { c == '?' } }

        val regex = StringBuilder("(\\.*)?")
        for ((i, g) in groups.withIndex()) {
            regex.append("(")
            regex.append("#".repeat(g))
            regex.append(")")

            if (i == groups.size - 1) {
                regex.append("(\\.*)?")
            } else {
                regex.append("(\\.+)")
            }
        }

        for (i in combinations.indices) {
            val sb = StringBuilder(conditions)
            for (j in brokenIndexes) {
                sb.setCharAt(j, combinations[i][brokenIndexes.indexOf(j)]).toString()
            }

            if (sb.toString().matches(regex.toString().toRegex())) {
                sum += 1
            }
        }
    }

    println(sum)
}

fun main() {
    val lines = File("src/main/resources/demo").readLines()
    p01(lines)
}

fun p02(input: List<String>) {
    var sum = 0
    for (row in input) {
        val conditions = row.split(" ").first
        var newConditions = ""
        val newGroups = row.split(" ").last.plus(",").repeat(5).split(",").filter { it.isNotEmpty() }.map { it.toInt() }


        for (i in 0..4) {
            newConditions += conditions
            if (i != 4) newConditions += "?"
        }

        val unknownCount = newConditions.count { it == '?' }
//        val groups = row.split(" ").last.split(",").map { it.toInt() }

        val combinations = binaryString(unknownCount).map { it.replace('0', '.').replace('1', '#') }
        val brokenIndexes = conditions.mapIndexedNotNull { index, c -> index.takeIf { c == '?' } }


        val regex = StringBuilder("(\\.*)?")
        for ((i, g) in newGroups.withIndex()) {
            regex.append("(")
            regex.append("#".repeat(g))
            regex.append(")")

            if (i == newGroups.size - 1) {
                regex.append("(\\.*)?")
            } else {
                regex.append("(\\.+)")
            }
        }

        for (i in combinations.indices) {
            val sb = StringBuilder(newConditions)
            for (j in brokenIndexes) {
                sb.setCharAt(j, combinations[i][brokenIndexes.indexOf(j)]).toString()
            }

            if (sb.toString().matches(regex.toString().toRegex())) {
                sum += 1
            }
        }

    }

    println(sum)
}
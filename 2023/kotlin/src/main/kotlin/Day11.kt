import java.io.File
import kotlin.math.abs

fun main() {
    val lines = File("src/main/resources/day11").readLines().map { l -> l.map { it.toString() } }
    val rowsIndexes = lines.mapIndexed { index, row ->
        if (row.all { it == "." }) index else -1
    }.filter { it > 0 }
    val colsIndexes = List(lines.size) { colIndex ->
        if (List(lines.size) { rowIndex ->
                lines[rowIndex][colIndex]
            }.all { it == "." }) colIndex else -1
    }.filter { it > 0 }
    val passed = mutableListOf<Pair<Int, Int>>()
    val expansion = 999999 // p1 -> 1, p2 -> 999999
    var sum: Long = 0
    val galaxies = lines.foldIndexed(ArrayList<Pair<Int, Int>>()) { index, acc, str ->
        acc.apply {
            str.forEachIndexed { i, s ->
                if (s != ".") {
                    add(Pair(i, index))
                }
            }
        }
    }

    for (i in galaxies) {
        for (j in galaxies) {
            if (i != j && !passed.contains(j)) {
                sum += (abs(i.first - j.first) + colsIndexes.count { c -> (c > i.first && c < j.first) || (c > j.first && c < i.first) } * expansion) + (abs(
                    i.second - j.second
                ) + rowsIndexes.count { r -> (r > i.second && r < j.second) || (r > j.second && r < i.second) } * expansion)
            }
        }

        passed.add(i)
    }

    println(sum)
}

//10276166
//598693078798
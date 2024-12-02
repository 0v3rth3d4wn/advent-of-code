import java.io.File
import kotlin.math.abs

fun returnTop(start: Pair<Int, Int>, grid: List<String>): Pair<Int, Int>? {
    val (x, y) = start
    return if (y - 1 >= 0 && ((listOf("|", "7", "F").contains(grid[y - 1][x].toString()) && (listOf(
            "|",
            "J",
            "L",
        ).contains(grid[y][x].toString()))) || (grid[y][x] == 'S'))
    ) {
        return Pair(x, y - 1)
    } else null
}

fun returnRight(start: Pair<Int, Int>, grid: List<String>): Pair<Int, Int>? {
    val (x, y) = start
    return if (x + 1 <= grid[y].length - 1 && ((listOf(
            "-", "J", "7"
        ).contains(grid[y][x + 1].toString()) && (listOf(
            "-",
            "F",
            "L"
        ).contains(grid[y][x].toString()))) || (grid[y][x] == 'S'))
    ) {
        Pair(x + 1, y)
    } else null
}

fun returnBottom(start: Pair<Int, Int>, grid: List<String>): Pair<Int, Int>? {
    val (x, y) = start
    return if (y + 1 <= grid.size - 1 && ((listOf("|", "L", "J").contains(grid[y + 1][x].toString()) && listOf(
            "|",
            "7",
            "F",
        ).contains(grid[y][x].toString())) || (grid[y][x] == 'S'))
    ) {
        Pair(x, y + 1)
    } else null
}

fun returnLeft(start: Pair<Int, Int>, grid: List<String>): Pair<Int, Int>? {
    val (x, y) = start
    return if (x - 1 >= 0 && ((listOf("-", "F", "L").contains(grid[y][x - 1].toString()) && listOf(
            "-",
            "J",
            "7",
        ).contains(grid[y][x].toString())) || (grid[y][x] == 'S'))
    ) {
        Pair(x - 1, y)
    } else null
}

fun getAvailableDirections(
    current: Pair<Int, Int>,
    grid: List<String>,
    prev: Pair<Int, Int>,
    moves: MutableList<Pair<Int, Int>>,
    moveSizes: MutableList<Int>
): Int {
    for (d in listOfNotNull(
        returnTop(current, grid),
        returnRight(current, grid),
        returnBottom(current, grid),
        returnLeft(current, grid)
    ).filter { it != prev }) {
        moves.add(d)
        getAvailableDirections(d, grid, current, moves, moveSizes)
    }

    if (moves.isNotEmpty()) {
        moveSizes.add((moves.size + 1) / 2)
        moves.removeAll(moves)
    }

    return moveSizes.max()
}


fun getMoves(
    current: Pair<Int, Int>,
    grid: List<String>,
    prev: Pair<Int, Int>,
    moves: MutableList<Pair<Int, Int>>,
    allMoves: MutableList<MutableList<Pair<Int, Int>>>
): MutableList<MutableList<Pair<Int, Int>>> {
    for (d in listOfNotNull(
        returnTop(current, grid),
        returnRight(current, grid),
        returnBottom(current, grid),
        returnLeft(current, grid)
    ).filter { it != prev }) {
        moves.add(d)
        getMoves(d, grid, current, moves, allMoves)
    }

    if (moves.isNotEmpty()) {
        allMoves.add(moves.toMutableList())
        moves.removeAll(moves)
    }

    return allMoves
}

fun part01(input: List<String>): Int {
    val sY = input.indexOf(input.map { it }.find { it.contains('S') })
    val sX = input[sY].indexOf("S")

    return getAvailableDirections(Pair(sX, sY), input, Pair(sX, sY), mutableListOf(), mutableListOf())
}

fun part02(input: MutableList<String>) {
    val sY = input.indexOf(input.map { it }.find { it.contains('S') })
    val sX = input[sY].indexOf("S")
    val moves = getMoves(Pair(sX, sY), input, Pair(sX, sY), mutableListOf(), mutableListOf())
    val points = mutableListOf<Pair<Int, Int>>()
    val loop = moves.maxBy { it.size }
    loop.windowed(2) {
        points.add(it[0])
        points.add(it[1])
    }


    var p: Double = 0.0
    println(loop)
//    points.forEach {
//        it
//    }
//    (s - p) / 2
    var s = 0
    loop.windowed(2) {
        p += abs(it[0].first - it[1].first) + abs(it[0].second - it[1].second)
        s += it[0].first * it[1].second - it[1].first * it[0].second
    }

    println(s)
    println(p + 2)

    println(((s - p) / 2))

//    println(loop.map { it.second }.distinct())
//    var inside = 0

//    for (y in 0..<input.size) {
//        var walls = 0
//        var prev = ""
//        for (x in 0..<input[y].length) {
//            val ch = input[y][x].toString()
//
//            if (input[y][x] == '.') {
//                if (walls % 2 == 1) {
//                    inside += 1
//                    continue
//                } else {
//                    continue
//                }
//            }
//
//            if (ch == "-") {
//                continue
//            } else if (ch == "|") {
//                walls += 1
//                prev = ""
//            } else if (prev == "") {
//                prev = ch
//            } else {
//                if (prev == "L" && ch == "7") {
//                    walls += 1
//                } else if (prev == "F" && ch == "J") {
//                    walls += 1
//                }
//                prev = ""
//            }
//
//
////            if (loop.contains(Pair(x, y))) {
////                count++
////            }
////
////            if (input[y][x] == '.' && count % 2 == 1) {
////                val s = StringBuilder(input[y])
////                s.setCharAt(x, 'I')
////                input[y] = s.toString()
////            }
//        }
//    }

//    for (i in input) {
//        println(i)
//    }
//
}

fun main() {
    val lines = File("src/main/resources/day10").readLines()
//    println(part01(lines))
    part02(lines.toMutableList())
}
import java.io.File

fun main() {

    fun getReflections(pattern: List<String>): List<Int> {
        val equalLines = mutableListOf<Pair<Int, Int>>()
        val result = mutableListOf<Int>()

        for (y in 0..pattern.size - 2) {
            if (pattern[y] == pattern[y + 1]) {
                equalLines.add(Pair(y, y + 1))
            }
        }

        for (equals in equalLines) {
            val topRange = (0..<equals.first)
            val bottomRange = equals.second + 1..pattern.indices.last

            val topLines = pattern.slice(topRange).reversed()
            val bottomLines = pattern.slice(bottomRange)

            val compared = mutableListOf<Boolean>()

            for ((left, right) in topLines.zip(bottomLines)) {
                compared.add(left == right)
            }

            if (compared.all { it }) {
                result.add(topLines.size + 1)
            }
        }
//        println(result.filter { it > 0 })
        return result.filter { it > 0 }.map { it }

//        p1 result.filter{it > 0}.sum()
    }

//    fun getVertical(pattern: List<String>): Int {
//
//        val patternTransposed = List(pattern[0].length) { j ->
//            List(pattern.size) { i ->
//                pattern[i][j]
//            }.reversed().toString()
//        }
//
//        val equalLines = mutableListOf<Pair<Int, Int>>()
//        val result = mutableListOf<Int>()
//
//        for (y in 0..patternTransposed.size - 2) {
//            if (patternTransposed[y] == patternTransposed[y + 1]) {
//                equalLines.add(Pair(y, y + 1))
//            }
//        }
//
//        for (equals in equalLines) {
//            val topRange = (0..<equals.first)
//            val bottomRange = equals.second + 1..patternTransposed.indices.last
//
//            val topLines = patternTransposed.slice(topRange).reversed()
//            val bottomLines = patternTransposed.slice(bottomRange)
//
//            val compared = mutableListOf<Boolean>()
//
//            for ((left, right) in topLines.zip(bottomLines)) {
//                compared.add(left == right)
//            }
//
//            if (compared.all { it }) {
//                result.add(topLines.size + 1)
//            }
//        }
//
//        return result.filter { it > 0 }.sum()
//    }


    fun p01(patterns: List<List<String>>): Int =
        patterns.fold(0) { acc, curr ->
            acc + getReflections(curr).sum() * 100 + getReflections(List(curr[0].length) { j ->
                List(curr.size) { i ->
                    curr[i][j]
                }.reversed().toString()
            }).sum()
        }

    fun p02(patterns: List<List<String>>) {
        var sum = 0
        for (pattern in patterns) {
            val rows = getReflections(pattern)
            val cols = getReflections(List(pattern[0].length) { j ->
                List(pattern.size) { i ->
                    pattern[i][j]
                }.reversed().toString()
            })

            val patternSum = getReflections(pattern).sum() * 100 + getReflections(List(pattern[0].length) { j ->
                List(pattern.size) { i ->
                    pattern[i][j]
                }.reversed().toString()
            }).sum()

//            println("patternSum: $patternSum")

            val patternFormatted = pattern.map { it -> it.map { it }.toMutableList() }
            outer@ for (i in patternFormatted.indices) {
                for (j in 0..<patternFormatted[i].size) {
                    val changedPattern = pattern.map { it -> it.map { it }.toMutableList() }.toMutableList()
                    changedPattern[i][j] =
                        if (pattern.map { it -> it.map { it }.toMutableList() }[i][j] == '.') '#' else '.'
                    val newPattern = changedPattern.map { it.toString() }
                    val changedRows = getReflections(newPattern)
                    val changedCols = getReflections(List(newPattern[0].length) { k ->
                        List(newPattern.size) { b ->
                            newPattern[b][k]
                        }.reversed().toString()
                    })


                    if (changedRows != rows || changedCols != cols) {
//                        println("rows: $rows")
//                        println("changed rows: $changedRows")
//
//                        println("cols: $cols")
//                        println("changed cols: $changedCols")

                        val newRows = if (changedRows.size > 0) (changedRows - rows.toSet()).sum() else 0
                        val newCols = if (changedCols.size > 0) (changedCols - cols.toSet()).sum() else 0

                        val newPatternSum =
                            newRows * 100 + newCols

//                        println("newPatternSum: $newPatternSum")
                        //                    println("newPattern:")
                        //                    newPattern.forEach { println(it) }

                        if ((newPatternSum != patternSum) && newPatternSum > 0) {
                            sum += newPatternSum
                            break@outer
                        }
                    }

                }
            }
        }

        println("sum: $sum")
        //2700 -> low
//        15100 -> low
        //2606400 -> high

//        96800
    }


//    println(p01(File("src/main/resources/day13").readText().split("\n\n").map { it.split("\n") }))
    p02(File("src/main/resources/day13").readText().split("\n\n").map { it.split("\n") })
}


//p1 -> 34772
//p2 -> 35554
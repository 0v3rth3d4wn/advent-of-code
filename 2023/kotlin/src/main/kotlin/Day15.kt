import java.io.File

fun main() {
    fun hash(s: String) = s.fold(0) { curr, c -> (curr + c.code) * 17 % 256 }

    fun p01(input: List<String>) = input.fold(0) { seq, s -> seq + hash(s) }

    fun p02(input: List<String>): Int {
        val boxes: Map<Int, MutableList<Pair<String, Int>>> = (0..255).associateWith { mutableListOf() }
        for (seq in input) {
            val spl = if (seq.contains("=")) seq.split('=') else seq.split('-')
            val boxIndex = hash(spl.first())

            if (spl.last.isNotEmpty()) {
                if (boxes[boxIndex]?.find { it.first == spl.first } == null) {
                    boxes[boxIndex]?.add(Pair(spl.first, spl.last.toInt()))
                } else {
                    boxes[boxIndex]?.let { e ->
                        boxes[boxIndex]?.set(
                            e.indexOf(boxes[boxIndex]?.find { it.first == spl.first }),
                            Pair(spl.first, spl.last.toInt())
                        )
                    }

                }
            } else {
                boxes[boxIndex]?.removeIf { it.first == spl.first }
            }
        }

        return boxes.asSequence()
            .fold(0) { acc, entry -> acc + entry.value.foldIndexed(0) { i, curr, pair -> curr + (entry.key + 1) * (i + 1) * pair.second } }
    }

    val lines = File("src/main/resources/day15").readText().split(",")
    println(p01(lines))
    println(p02(lines))
}

//p1 -> 510273
//p2 -> 212449
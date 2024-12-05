import java.io.File

data class Part(val x: Int, val m: Int, val a: Int, val s: Int) {
    fun sum(): Int = x + m + a + s
}

fun main() {
    fun p01(input: List<String>) {
        fun getPartRating(cond: String, sign: String, part: Part) =
            when (cond.split(sign)[0]) {
                "x" -> part.x
                "m" -> part.m
                "a" -> part.a
                "s" -> part.s
                else -> throw Exception("Invalid category")
            }

        val workFlows = input[0].split("\n").fold(mutableMapOf<String, List<String>>()) { acc, line ->
            val parts = line.split("{")
            val key = parts[0].trim()
            val values = parts[1].replace("}", "").split(",").map { it.trim() }
            acc[key] = values
            acc
        }

        val p = input[1].split("\n").fold(mutableListOf<Part>()) { acc, line ->
            val ratings = line.replace("}", "").replace("{", "").split(",").map { it.split("=") }
            acc.apply {
                add(
                    Part(
                        ratings[0][1].toInt(),
                        ratings[1][1].toInt(),
                        ratings[2][1].toInt(),
                        ratings[3][1].toInt()
                    )
                )
            }
        }

        val acceptedParts = mutableListOf<Part>()

        p.forEach {
            var accepted = false
            var currFlow = workFlows["in"]!!.toMutableList()

            while (!accepted && currFlow.isNotEmpty()) {
                val condition = currFlow.removeFirst()

                when {
                    condition == "A" -> {
                        accepted = true
                        acceptedParts.add(it)
                    }

                    condition == "R" -> accepted = false

                    !condition.contains(":") -> currFlow =
                        if (workFlows[condition] != null) workFlows[condition]!!.toMutableList() else mutableListOf()

                    else -> {
                        val (cond, next) = condition.split(":")
                        if (cond.contains(">")) {
                            if (getPartRating(cond, ">", it) > cond.split(">")[1].toInt()) {
                                if (next == "A") {
                                    accepted = true
                                    acceptedParts.add(it)
                                }

                                if (next == "R") {
                                    accepted = false
                                }

                                currFlow =
                                    if (workFlows[next] != null) workFlows[next]!!.toMutableList() else mutableListOf()
                            }
                        } else {
                            if (getPartRating(cond, "<", it) < cond.split("<")[1].toInt()) {
                                if (next == "A") {
                                    accepted = true
                                    acceptedParts.add(it)
                                }

                                if (next == "R") {
                                    accepted = false
                                }

                                currFlow =
                                    if (workFlows[next] != null) workFlows[next]!!.toMutableList() else mutableListOf()
                            }
                        }
                    }
                }
            }
        }

        (1..4000).zip(1..4000).zip(1..4000).zip(1..4000).forEach {
            println(it)
        }

        println(acceptedParts.sumOf { it.sum() })
    }

    p01(File("src/main/resources/day19").readText().split("\n\n"))
}


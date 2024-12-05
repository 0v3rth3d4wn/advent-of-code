import java.io.File

fun main() {
    val lines = File("src/main/resources/day05").readLines()

    fun parse(start: String, end: String, input: List<String>, work: Sequence<UInt>): Sequence<UInt> {
        val list = mutableListOf<Pair<UIntRange, UIntRange>>()
        val map = mutableMapOf<UInt, UInt>()
        val endRange = if (end.isEmpty()) input.size - 1 else input.indexOf(end) - 1

        for (line in input.indexOf(start) + 1..<endRange) {
            val (destinationStart, sourceStart, rangeLength) = input[line].split(" ").map { it.toUInt() }
            list.add(
                Pair(
                    destinationStart..destinationStart + rangeLength - 1u,
                    sourceStart..sourceStart + rangeLength - 1u
                )
            )
        }

        for (thing in work) {
            map[thing] = thing
            for (listMap in list.reversed().asSequence()) {
                if (listMap.second.contains(thing)) {
                    map[thing] = listMap.first.elementAt(listMap.second.indexOf(thing))
                }
            }
        }

        return map.values.reversed().asSequence()
    }


    fun part01(input: List<String>) {
        val seeds = input[0].split(": ")[1].split(" ").map { it.toUInt() }.asSequence()
        val soils = parse("seed-to-soil map:", "soil-to-fertilizer map:", lines, seeds)
        val fertilizers = parse("soil-to-fertilizer map:", "fertilizer-to-water map:", lines, soils)
        val waters = parse("fertilizer-to-water map:", "water-to-light map:", lines, fertilizers)
        val lights = parse("water-to-light map:", "light-to-temperature map:", lines, waters)
        val temps = parse("light-to-temperature map:", "temperature-to-humidity map:", lines, lights)
        val humidity = parse("temperature-to-humidity map:", "humidity-to-location map:", lines, temps)
        val locations = parse("humidity-to-location map:", "", lines, humidity)

        println(locations.min())

    }

    fun part02(input: List<String>) {
        val seeds =
            input[0].split(": ")[1].split(" ").asSequence().map { it.toUInt() }.chunked(2)
                .map { (it.first..it.first + it.last - 1u).toList() }.flatten()

        val soils = parse("seed-to-soil map:", "soil-to-fertilizer map:", lines, seeds)
        val fertilizers = parse("soil-to-fertilizer map:", "fertilizer-to-water map:", lines, soils)
        val waters = parse("fertilizer-to-water map:", "water-to-light map:", lines, fertilizers)
        val lights = parse("water-to-light map:", "light-to-temperature map:", lines, waters)
        val temps = parse("light-to-temperature map:", "temperature-to-humidity map:", lines, lights)
        val humidity = parse("temperature-to-humidity map:", "humidity-to-location map:", lines, temps)
        val locations = parse("humidity-to-location map:", "", lines, humidity)

        println(locations.min())

    }

//    val timeInMillis = measureTimeMillis {
//        part01(lines)
//    }

//    println(timeInMillis)
    part02(lines)
}

//p1 177942185 -> time 41134
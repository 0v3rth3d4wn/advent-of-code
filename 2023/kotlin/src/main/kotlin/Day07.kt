import java.io.File

fun isOfAKind(hand: String, number: Int) = hand.groupBy { it }.any { it.value.size == number }
fun isFiveOfAKind(hand: String) = isOfAKind(hand, 5)
fun isFourOfAKind(hand: String) = isOfAKind(hand, 4)
fun isFullHouse(hand: String): Boolean {
    var found = false
    for (c in hand) {
        val byTwo = hand.toList().groupBy { it }.values.toList().filter { it.size == 2 }
        val byThree = hand.toList().groupBy { it }.values.toList().filter { it.size == 3 }

        if (byTwo.size == 1 && byThree.size == 1) {
            found = true
            break
        }
    }

    return found
}

fun isThreeOfAKind(hand: String) = isOfAKind(hand, 3)
fun isTwoPair(hand: String) = hand.groupBy { it }.values.count { it.size == 2 } == 2
fun isOnePair(hand: String) = isOfAKind(hand, 2)
fun isHighCard(hand: String): Boolean = hand.toSet().count() == 5

fun main() {
//    fun part01(input: List<String>) {
//
//        var sum = 0
//        val cardStrength =
//            mapOf(*("AKQJT98765432".toList() zip (14 downTo 2)).toTypedArray())
//
//        val hands = input.map { Pair(it.split(" ")[0], it.split(" ")[1]) }
//            .map { it ->
//                Triple(
//                    it.first.map { cardStrength[it] }, it.first,
//                    when {
//                        isFiveOfAKind(it.first) -> 7
//                        isFourOfAKind(it.first) -> 6
//                        isFullHouse(it.first) -> 5
//                        isThreeOfAKind(it.first) -> 4
//                        isTwoPair(it.first) -> 3
//                        isOnePair(it.first) -> 2
//                        else -> 1
//                    }
//                )
//
//            }
//            .sortedBy { it -> it.first.map { it.toString().padStart(2, '0') }.toString() }.sortedBy { it.third }
//
//        hands.forEachIndexed { index, triple ->
//            sum += (index + 1) * input.find { it.split(" ")[0] == triple.second }?.split(" ")!![1].toInt()
//        }
//
//        println(sum)
//    }

    fun part02(input: List<String>) {
        var sum = 0
        val cardStrength =
            mapOf(*("AKQT98765432J".toList() zip (13 downTo 1)).toTypedArray())
        val handMap = mutableMapOf<String, String>()

        input.forEach { it ->
            val hand = it.split(" ")[0]
            val newHand = hand.replace('J', hand.groupBy { it }.maxBy { it.value.size }.key)
            handMap[hand] = newHand
        }

        val hands = input.map { Pair(it.split(" ")[0], it.split(" ")[1]) }
            .map { it ->
                Triple(
                    it.first.map { cardStrength[it] }, it.first,
                    when {
                        isFiveOfAKind(handMap[it.first]!!) -> 7
                        isFourOfAKind(handMap[it.first]!!) -> 6
                        isFullHouse(handMap[it.first]!!) -> 5
                        isThreeOfAKind(handMap[it.first]!!) -> 4
                        isTwoPair(handMap[it.first]!!) -> 3
                        isOnePair(handMap[it.first]!!) -> 2
                        else -> 1
                    }
                )
            }
            .sortedBy { it.third }.sortedBy { it -> it.first.map { it.toString().padStart(2, '0') }.toString() }
            .sortedBy { it.third }

        hands.forEachIndexed { index, triple ->
            sum += (index + 1) * input.find { it.split(" ")[0] == triple.second }?.split(" ")!![1].toInt()
        }

        println(sum)
    }

    val lines = File("src/main/resources/day07").readLines()

//    part01(lines)
    part02(lines)
}

// p1 -> 250957639

//251463095 -> wrong

//251440512 -> too low
//251987333 -> too high

l1, l2 = File.readlines('input.txt').map { _1.split.map(&:to_i) }.transpose.map(&:sort)
r_l = l2.tally
p l1.zip(l2).map { (_1[0] - _1[1]).abs }.sum
p l2.filter_map { _1 * r_l[_1] unless r_l[_1].nil? }.sum

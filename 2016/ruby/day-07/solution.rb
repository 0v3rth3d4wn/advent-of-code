def tls?(input)
  (input.slice(0..input.length / 2 - 1) == input.slice(input.length / 2..).reverse) && (input[0] != input[1])
end

def prep(input, c)
  input.map { _1.chars.each_cons(c).to_a.map(&:join) }.flatten
end

ip = File.read('input').lines.map do |l|
  inner = l.scan(/\[\w+\]/).map { _1[1..-2] }
  prep(l.scan(/\w+/) - inner, 4).any? { tls?(_1) } && prep(inner, 4).all? { !tls?(_1) }
end

print ip.count(true)
print "\n"

cc = 0
File.read('input').lines.map do |l|
  inner = l.scan(/\[\w+\]/).map { _1[1..-2] }
  outer = l.scan(/\w+/) - inner
  # print outer
  print inner
  prep(outer, 3).filter { _1[0] == _1[2] && _1[0] != _1[1] }.each do |out|
    cc += 1 if prep(inner, 3).include?(out[1] + out[0] + out[1])
    # found = prep(inner, 3).include?(out[1] + out[0] + out[1])
    # next unless found

    # # ssls.push(out + found)
    # cc += 1
  end
end

print cc
# print ssls

# 247 too high
# 211 too low

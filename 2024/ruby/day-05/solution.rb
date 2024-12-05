rules, updates = File.read('input.txt').split("\n\n").map(&:split).then do
  [
    _1.map(&:strip).to_set,
    _2.map { |r| r.scan(/\d+/) }
  ]
end
p1, p2 = 0, 0

def ord?(updates, rules) updates.each_cons(2).map { |c| c.join('|') }.all? { rules.include?(_1) } end
def mid(updates) updates[updates.length / 2].to_i end

updates.each do |u|
  if ord?(u, rules)
    p1 += mid u
    next
  end

  p2 += mid(u.sort { |l, r| rules.include?("#{l}|#{r}") ? -1 : 1 })
end

puts p1
puts p2

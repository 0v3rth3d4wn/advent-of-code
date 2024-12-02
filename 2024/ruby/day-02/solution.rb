safe = 0
saved = 0
cmp = ->((a, b)) { a <=> b if (a - b).abs <= 3 && a != b }

File.read('input.txt').lines.map(&:strip).each do |report|
  if report.split.map(&:to_i).each_cons(2).map(&cmp).uniq.size <= 1
    safe += 1
  else
    report.split.length.times do |i|
      rr = report.split.map(&:to_i).reject.each_with_index { |_, idx| idx == i }.each_cons(2).map(&cmp)
      if rr.uniq.size == 1 && !rr.all?(nil)
        saved += 1
        break
      end
    end
  end
end

p safe # 598
p saved + safe # 634

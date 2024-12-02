def solution(file)
  s = 0
  f = File.read(file).strip
  f.each_line do |l|
    a, b, c = *l.split(' ').map(&:to_i)
    s += 1 if [a, b, c].permutation(3).all? { |(a, b, c)| a + b > c }
  end

  print s
end

def p2(file)
  # print File.read(file).strip.each_line.map { |e| e.strip.split(' ') }.transpose.flatten.map(&:to_i).each_slice(3).filter { _1.permutation(3).all? { |(a, b, c)| a + b > c } }.count
  print File.read(file).strip.each_line.map do |e|
    e.strip.split(' ')
  end.transpose.flatten.map(&:to_i).each_slice(3).filter do
    _1.permutation(3).all? do |(a, b, c)|
      a + b > c
    end
  end.count
end

# solution('input') # 1050
p2('input') # 1921

c = File.readlines('input.txt').map { |line| line.strip.chars }
f, f_tr, f_d_rev = c.then { |f| [f, f.transpose, f.map(&:reverse)] }
def inside?(inp, i, j, curr) j + curr <= inp.size - 1 && i + curr <= inp.size - 1 end
def diag(inp, i, j) [inp[i][j], inp[i + 1][j + 1], inp[i + 2][j + 2], inp[i + 3][j + 3]].join end
s1 = %w[XMAS SAMX]
s2 = %w[MAS SAM]
p1, p2 = 0, 0

f.size.times do |i|
  f[0].size.times do |j|
    p1 += 1 if s1.include? f[i][j..j + 3].join
    p1 += 1 if s1.include? f_tr[i][j..j + 3].join
    next unless [1, 2, 3].all? { inside?(f, i, j, _1) }

    p1 += 1 if s1.include? diag(f, i, j)
    p1 += 1 if s1.include? diag(f_d_rev, i, j)

    d1 = [f[i][j], f[i + 1][j + 1], f[i + 2][j + 2]].join
    d2 = [f[i + 2][j], f[i + 1][j + 1], f[i][j + 2]].join
    p2 += 1 if (s2.include? d1) && (s2.include? d2)
  end
end

puts p1

f.size.times do |i|
  f[0].size.times do |j|
    next unless [1, 2].all? { inside?(f, i, j, _1) }

    d1 = [f[i][j], f[i + 1][j + 1], f[i + 2][j + 2]].join
    d2 = [f[i + 2][j], f[i + 1][j + 1], f[i][j + 2]].join
    p2 += 1 if (s2.include? d1) && (s2.include? d2)
  end
end

puts p2

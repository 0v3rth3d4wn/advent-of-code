grid = Array.new(6).map { Array.new(50).map { '.' } }

File.readlines('input.txt').each do
  parsed = _1.gsub(/\b(by|column|row)\b/, ' ').scan(/\w+/)
  if parsed[0] == 'rect'

    x, y = parsed[1].split('x').map(&:to_i)
    grid.map!.each_with_index do |vrow, row|
      if row < y
        vrow.map.each_with_index do |vcol, col|
          col < x ? '#' : vcol
        end
      else
        vrow
      end
    end
  else
    dir, dir_index, dir_size = parsed[1..]
    if dir == 'y'
      grid[dir_index.to_i].rotate!(-dir_size.to_i)
    else
      grid.map { |r| r[dir_index.to_i] }.rotate(-dir_size.to_i).each_with_index { |el, i| grid[i][dir_index.to_i] = el }
    end
  end
end

puts grid.map(&:join).join.count('#') # 106
grid.each { |l| puts l.join } # CFLELOYFCS

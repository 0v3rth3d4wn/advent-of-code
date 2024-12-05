def distance(x1, x2, y1, y2)
  (x1 - x2).abs + (y1 - y2).abs
end

def p1(file)
  input = File.read(file).strip
  x1 = y1 = x2 = y2 = 0
  curr = 'N'
  moves = []
  p2 = nil

  input.split(',').map(&:strip).each do |d|
    mov = d[1..].to_i

    if d[0] == 'R'
      case curr
      when 'N'
        mov.times do
          x2 += 1
          moves.push([x2, y2])
          p2 = distance(x1, x2, y1, y2) if moves.count([x2, y2]) == 2 && p2.nil?
        end
        curr = 'E'
      when 'E'
        mov.times do
          y2 -= 1
          moves.push([x2, y2])
          p2 = distance(x1, x2, y1, y2) if moves.count([x2, y2]) == 2 && p2.nil?
        end
        curr = 'S'
      when 'S'
        mov.times do
          x2 -= 1
          moves.push([x2, y2])
          p2 = distance(x1, x2, y1, y2) if moves.count([x2, y2]) == 2 && p2.nil?
        end
        curr = 'W'
      else
        mov.times do
          y2 += 1
          moves.push([x2, y2])
          p2 = distance(x1, x2, y1, y2) if moves.count([x2, y2]) == 2 && p2.nil?
        end
        curr = 'N'
      end
    else
      case curr
      when 'N'
        mov.times do
          x2 -= 1
          moves.push([x2, y2])
          p2 = distance(x1, x2, y1, y2) if moves.count([x2, y2]) == 2 && p2.nil?
        end
        curr = 'W'
      when 'W'
        mov.times do
          y2 -= 1
          moves.push([x2, y2])
          p2 = distance(x1, x2, y1, y2) if moves.count([x2, y2]) == 2 && p2.nil?
        end
        curr = 'S'
      when 'S'
        mov.times do
          x2 += 1
          moves.push([x2, y2])
          p2 = distance(x1, x2, y1, y2) if moves.count([x2, y2]) == 2 && p2.nil?
        end
        curr = 'E'
      else
        mov.times do
          y2 += 1
          moves.push([x2, y2])
          p2 = distance(x1, x2, y1, y2) if moves.count([x2, y2]) == 2 && p2.nil?
        end
        curr = 'N'
      end
    end
  end

  p distance(x1, x2, y1, y2)
  p p2
end

# p1('demo1')
# p1('demo2')
# p1('demo3')
# p1('demo4')
p1('input')
# p1('input2')

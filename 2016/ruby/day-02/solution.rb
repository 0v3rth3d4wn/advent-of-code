KEYPAD = [[1, 2, 3], [4, 5, 6], [7, 8, 9]].freeze
UBER_KEYPAD = [[nil, nil, '1', nil, nil], [nil, '2', '3', '4', nil], [5, 6, 7, 8, 9], [nil, 'A', 'B', 'C', nil],
               [nil, nil, 'D', nil, nil]].freeze

def solution(input, start, keypad)
  curr = start
  code = ''
  f = File.read(input).strip
  f.each_line do |line|
    line.each_char do |c|
      case c
      when 'U'
        curr[1] -= 1 unless (curr[1] - 1).negative? || keypad[curr[1] - 1][curr[0]].nil?
      when 'R'
        curr[0] += 1 unless (curr[0] + 1 > keypad.length - 1) || keypad[curr[1]][curr[0] + 1].nil?
      when 'D'
        curr[1] += 1 unless (curr[1] + 1 > keypad.length - 1) || keypad[curr[1] + 1][curr[0]].nil?
      when 'L'
        curr[0] -= 1 unless (curr[0] - 1).negative? || keypad[curr[1]][curr[0] - 1].nil?
      end
    end
    code << keypad[curr[1]][curr[0]].to_s
  end

  puts code
end

solution('input', [1, 1], KEYPAD) # 12578
solution('input', [0, 2], UBER_KEYPAD) # 516DD

# p1('demo') # 1985
# p1('input') # 12578

# p2('demo', [0, 2], UBER_KEYPAD) # 5DB3
# p2('input', [0, 2], UBER_KEYPAD) # 516DD

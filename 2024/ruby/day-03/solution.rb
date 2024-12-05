p File.read('input.txt').scan(/mul\(\d+,\d+\)/).map { _1.scan(/\d+/).map(&:to_i).inject(:*) }.sum # 169021493

sum = 0
enabled = true
File.read('input.txt').scan(/mul\(\d+,\d+\)|don't\(\)|do\(\)/).each do |inst|
  enabled = false if inst == "don't()"
  enabled = true if inst == 'do()'
  sum += inst.scan(/\d+/).map(&:to_i).inject(:*) if !inst.scan(/\d+/).empty? && enabled
end

p sum # 111762583

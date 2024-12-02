h = { 'a' => 0, 'b' => 0, 'c' => 1, 'd' => 0 }
pp h
print "\n"
i = 0
instructions = File.readlines('input.txt').map(&:strip)
while i < instructions.length
  inst, reg1, reg2 = instructions[i].split(' ')
  if inst == 'cpy'
    h[reg2] = reg1.to_i.zero? ? h[reg1] : reg1.to_i
    i += 1
  elsif inst == 'inc'
    h[reg1] += 1
    i += 1
  elsif inst == 'dec'
    h[reg1] -= 1
    i += 1
  elsif inst == 'jnz'
    key = reg1.to_i.zero? ? h[reg1] : reg1.to_i
    addition = key != 0 ? reg2.to_i : 1
    i += addition
  end
  # pp h
end
print "\n"
print "\n"
print "\n"

pp h # {"a"=>318009, "b"=>196418, "c"=>0, "d"=>0} / {"a"=>9227663, "b"=>5702887, "c"=>0, "d"=>0}

# File.readlines('demo.txt').each do |line|
#   # inst, reg1, reg2 = line.scan(/\w+/)
#   # print inst, reg1, reg2
#   # print "\n"
#   # next
#   # if inst == 'cpy'

#   # end
# end

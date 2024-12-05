f = File.read('demo.txt').strip.lines.map(&:strip)
puts f.join('\n').scan(/XMAS/).count
print "\n"
puts f.map(&:reverse).join('\n').scan(/XMAS/).count
print "\n"
puts f.reverse.join('\n').scan(/XMAS/).count
print "\n"
puts f.map(&:chars).transpose.map(&:join).join("\n").scan(/XMAS/).count
print "\n"
puts f.map(&:chars).transpose.map(&:join).join("\n").reverse.scan(/XMAS/).count
print "\n"
puts f.reverse.map(&:chars).transpose.map(&:join).join("\n").scan(/XMAS/).count
print "\n"
puts f.reverse.map(&:chars).transpose.map(&:join).join("\n").reverse.scan(/XMAS/).count

# f = File.read('demo.txt').lines.map(&:strip).map(&:chars)
# width = f.first.length
# height = f.rotate.first.length
# print width
# print height
# print f
# f.each
# forward
# print f.scan(/XMAS/)
# print f.reverse.scan(/XMAS/)
# print f.lines.rotate.join

ff = (0..9).collect { |i| f[i][i] }
print ff

# f.each do |line|
#   line.each do |c|
#     print c
#   end
# end

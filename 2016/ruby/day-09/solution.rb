# pp 'ADVENT'
# pp 'A(1x5)BC'.split(/\(/)
# /(\w+)?(\(\w+\))(\w+)?/
# 'X(8x2)(3x3)ABCY'.split(/\(/).each_with_index.collect do |el, _i|
#   # if c[-1] == ')'
#   print el
#   marker = el.split(/\)/)
#   print marker
#   print "\n"
#   if marker.length > 1
#     rep = marker.first.split('x')
#     marker.last[0..rep.first.to_i] * rep.last.to_i
#   elsif marker.length == 1
#     marker.join unless marker[0].scan(/\d+/).join
#   end
# end
# pp '(6x1)(1x3)A'.split(/\(/)
# pp 'X(8x2)(3x3)ABCY'.split(/\(/)

# print cc.to_a

# sp = File.read('input').scan(/(\w+)?(\(\w+\))(\w+)?/).map(&:compact)
sp = 'X(8x2)(3x3)ABCY'.scan(/(\w+)?(\(\w+\))(\w+)?/).map(&:compact)

# sp.map { print _1.last[-1] }
cc = sp.each_with_index.map do |el, i|
  # print el
  if el.last[-1] == ')' && i < sp.length
    [el, sp[i + 1]].join
  else
    el.join
  end
end

print cc.to_a

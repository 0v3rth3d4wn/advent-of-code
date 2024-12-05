dr = [-1, +1, 0, 0] # NSEW
dc = [0, 0, +1, -1] # NSEW
curr = [1, 1]
goal = [31, 39]
visited = [curr]

rq = Queue.new
cq = Queue.new
rq.enq(curr[1])
cq.enq(curr[0])
nodes_left_in_layer = 1
nodes_in_next_layer = 0
move_count = 1
locs = 0

until rq.empty?
  r = rq.deq
  c = cq.deq
  break if curr == goal

  4.times do
    rr = (r + dr[_1]).to_i
    cc = (c + dc[_1]).to_i
    cell = (cc * cc + 3 * cc + 2 * cc * rr + rr + rr * rr + 1350).to_s(2).count('1').even? ? '.' : '#'
    next if rr == -1 || cc == -1 || cell == '#'

    curr = [cc, rr]

    if cell == '.' && !visited.include?(curr)
      rq.enq(rr)
      cq.enq(cc)
      visited.push(curr)
      nodes_in_next_layer += 1
    end

    break if curr == goal
  end
  nodes_left_in_layer -= 1

  next unless nodes_left_in_layer.zero?

  nodes_left_in_layer = nodes_in_next_layer
  nodes_in_next_layer = 0
  move_count += 1

  locs = visited.length if move_count == 51

end

print "move_count -> #{move_count} \n"
print "locs -> #{locs} \n"
# print "p2_count ->  #{p2_count}"
# print "visited -> #{visited} \n"

# while curr != goal
#   4.times do
#     rr = (visited[i][1] + dr[_1]).to_i
#     cc = (visited[i][0] + dc[_1]).to_i
#     cell = (cc * cc + 3 * cc + 2 * cc * rr + rr + rr * rr + 10).to_s(2).count('1').even? ? '.' : '#'
#     next if rr == -1 || cc == -1 || cell == '#'

#     # print rr # y
#     # print cc # x
#     # print ' -> '
#     # print (cc * cc + 3 * cc + 2 * cc * rr + rr + rr * rr + 10).to_s(2).count('1').even? ? '.' : '#'
#     # print "\n"
#     curr = [cc, rr]
#     # paths[visited[i]] = paths[visited[i]].nil? ? [curr] : paths[visited[i]] + [curr]
#     visited.push(curr) if cell == '.' && !visited.include?(curr) # TODO: maybe add walls

#     break if curr == goal
#   end

#   i += 1
# end

# print "visited -> #{visited} \n"
# print "walls -> #{walls} \n"
# print "\n"
# pp paths

# def is_valid_move(curr, target, vis)
#   return false if x - 1 < 0 || y - 1 < 0 || vis.include([x, y])

#   return true if (curr[x] == target[x] && curr[y] == target[y + 1] && curr[y] == target[y - 1]) ||
#                  (curr[y] == target[y] && curr[x] == target[x + 1] && curr[x] == target[x - 1])

#   false
# end

# paths = []
# visited.each do |node|

# end

# 20.times do |vis_i|
#   4.times do
#     rr = (visited[vis_i][1] + dr[_1]).to_i
#     cc = (visited[vis_i][0] + dc[_1]).to_i
#     next if rr == -1 || cc == -1

#     cell = (cc * cc + 3 * cc + 2 * cc * rr + rr + rr * rr + 10).to_s(2).count('1').even? ? '.' : '#'
#     print rr # y
#     print cc # x
#     print ' -> '
#     print (cc * cc + 3 * cc + 2 * cc * rr + rr + rr * rr + 10).to_s(2).count('1').even? ? '.' : '#'
#     print "\n"
#     curr = [cc, rr]
#     visited.push(curr) if cell == '.' && !visited.include?(curr) # TODO: maybe add walls
#     break if curr == goal
#   end
# end

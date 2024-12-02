factory = { bot: {}, output: {} }
instructions = []

File.readlines('input.txt').each do |line|
  line_parsed = line.scan(/\d+/).map(&:to_i)
  if line_parsed.length == 2
    if factory[:bot][line_parsed.last].nil?
      factory[:bot][line_parsed.last] = [line_parsed.first]
    else
      factory[:bot][line_parsed.last].push(line_parsed.first)
    end
  else
    instructions.push(line.scan(/\boutput\b|\bbot\b|\d+/).map { _1.scan(/\d+/).empty? ? _1.to_sym : _1.to_i }[1..])
  end
end

until instructions.empty?
  factory[:bot].to_a.filter { _1[1].length == 2 }.map(&:first).each do |robot|
    inst = instructions.find { _1[0] == robot }
    unless factory[:bot][inst[0]].empty?
      max = factory[:bot][inst[0]].max
      factory[inst[3]][inst[4]] ||= []
      factory[inst[3]][inst[4]].push(max)
      factory[:bot][inst[0]].delete(max)

      min = factory[:bot][inst[0]].min
      factory[inst[1]][inst[2]] ||= []
      factory[inst[1]][inst[2]].push(min)
      factory[:bot][inst[0]].delete(min)
    end

    instructions.delete_if { _1[0] == robot }

    print 'p1 -> ' << inst[4].to_s << "\n" if factory[inst[3]][inst[4]] == [61, 17]
    print 'p1 -> ' << inst[2].to_s << "\n" if factory[inst[1]][inst[2]] == [61, 17]
  end
end

print 'p2 -> ' << factory[:output].to_a.sort.map { _1[1] }[0..2].flatten.inject(&:*).to_s

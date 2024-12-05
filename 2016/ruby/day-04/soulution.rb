lines = File.read('input').strip.lines.map { _1.scan(/\w+/) }
letters = ('a'..'z').to_a

sectors = lines.map do |(*enc, sector, checksum)|
  ch = enc.join.chars.tally
          .sort { |a, b| a[1] == b[1] ? b[0] <=> a[0] : a[1] <=> b[1] }
          .reverse.take(5).map(&:first).join
  sector.to_i if ch == checksum
end

p sectors.compact.sum # 173787
p lines
  .map { |l| [l[0..-3].map { |w| w.chars.map { letters[(letters.index(_1) + l[-2].to_i) % 26] }.join }.join, l[-2]] }
  .to_h['northpoleobjectstorage']

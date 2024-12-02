p File.read('input').lines.map(&:strip).map(&:chars).transpose.map(&:tally)
      .map { _1.min_by { |_, v| -v }.first }.join

p File.read('input').lines.map(&:strip).map(&:chars).transpose.map(&:tally)
      .map { _1.min_by(&:last).first }.join

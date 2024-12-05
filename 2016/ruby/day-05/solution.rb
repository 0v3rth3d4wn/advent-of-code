require 'digest'
password = ''
loop.map.each_with_index do |_, i|
  break if password.length == 8

  md5 = Digest::MD5.hexdigest "ojvtpuvg#{i}"
  next unless md5.start_with?('00000')

  password << md5[5]
end

p password # 4543c154

pass = Array.new(8)

loop.map.each_with_index do |_, i|
  break if pass.all? { !_1.nil? }

  md5 = Digest::MD5.hexdigest "ojvtpuvg#{i}"
  next unless md5.start_with?('00000') && md5[5].to_i(16) <= 7 && pass[md5[5].to_i].nil?

  pass[md5[5].to_i] = md5[6]
end

p pass.join

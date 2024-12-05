require 'digest'
require 'bcrypt'

keys = []
input = 'ihaygndm'
loop.with_index do |_, i|
  match = Digest::MD5.hexdigest("#{input}#{i}").match(/([0-9]|[a-z])\1\1/)
  if match
    1000.times do |j|
      next unless Digest::MD5.hexdigest("#{input}#{i + j + 1}").match(/#{match.captures}{5}/)

      keys.push(i)
      break
    end
  end
  break if keys.length == 64
end

# print keys.last # 15035
# print BCrypt::Password.create(BCrypt::Password.create('abc0', cost: 44.89988864 / 2), cost: 44.89988864 / 2)
# OpenSSL::KDF.pbkdf2_hmac()
# 44.89988864

# https://atcoder.jp/contests/keyence2019/tasks/keyence2019_b

s = gets.chomp

keyence = 'keyence'
ans =
  case s
  when keyence
    'YES'
  when /\A#{keyence}.*\z/, /\A.*#{keyence}\z/
    'YES'
  when *(1...keyence.length).map { |i| /\A#{keyence.chars.insert(i, '.*').join}\z/ }
    'YES'
  else
    'NO'
  end
puts ans

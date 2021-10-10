# https://atcoder.jp/contests/agc013/tasks/agc013_a

n = gets.chomp.to_i
an = gets.chomp.split(" ").map(&:to_i)

def main(an)
  direction = nil
  count = 1
  an.each_cons(2) { |i, j|
    case direction
    when nil
      case
      when i == j
      when i < j
        direction = 'up'
      when i > j
        direction = 'down'
      end
    when 'up'
      case
      when i == j
      when i < j
      when i > j
        direction = nil
        count += 1
      end
    when 'down'
      case
      when i == j
      when i < j
        direction = nil
        count += 1
      when i > j
      end
    end
  }
  count
end

puts main(an)

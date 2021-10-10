# https://atcoder.jp/contests/abc218/tasks/abc218_c

n = gets.chomp.to_i

s = []
tmp = []
flag = true
n.times.each {
  l = gets.chomp.chars.map { |s| s == "." ? 0 : 1 }
  unless l.all?(&:zero?)
    if flag == true
      s << l
    else
      tmp << l
    end
  else
    flag = false
    tmp.each { |tt| s << tt }
    tmp = []
  end
}
tmp.each { |tt| s << tt }


t = []
tmp = []
flag = true
n.times.each {
  l = gets.chomp.chars.map { |s| s == "." ? 0 : 1 }
  unless l.all?(&:zero?)
    if flag == true
      t << l
    else
      tmp << l
    end
  else
    flag = false
    tmp.each { |tt| t << tt }
    tmp = []
  end
}
tmp.each { |tt| t << tt }

# pp s
# pp t

def compact(list)
  flag = true
  new_list = []
  tmp = []
  list.each { |l|
    # pp [l, tmp]
    unless l.all?(&:zero?)
      tmp.each {|tt| new_list << tt}
      tmp = []
      new_list << l
      flag = false
    else
      if flag == false
        tmp << l
      end
    end
  }
  new_list
end

s2 = compact(s.transpose)
t2 = compact(t.transpose)

# pp s2
# pp t2

if s2 == t2
  puts 'Yes'
  exit
end
trans = s2.transpose
if trans.map(&:reverse) == t2 || trans.reverse == t2 || s2.reverse.map(&:reverse) == t2
  puts 'Yes'
  exit
end

puts 'No'


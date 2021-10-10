# https://atcoder.jp/contests/abc213/tasks/abc213_d

N = gets.to_i
routes = {}
(N-1).times.each {
  a, b = gets.chomp.split(" ").map(&:to_i)
  # if b != 1
  #   routes[a] =
  #     if routes[a] == nil
  #       [b]
  #     else
  #       idx = routes[a].bsearch_index{|x| x > b } || -1
  #       routes[a].insert(idx, b)
  #     end
  # end
  # if a != 1
  #   routes[b] =
  #     if routes[b] == nil
  #       [a]
  #     else
  #       idx = routes[b].bsearch_index{|x| x > a } || -1
  #       routes[b].insert(idx, a)
  #     end
  # end
  # NOTE bsearch_index + insert よりも [] << element したあとに .uniq.sort のほうが早い
  routes[a] = (routes[a] || []) << b
  routes[b] = (routes[b] || []) << a
}
routes.transform_values!{|v| v.uniq.sort}

city = 1
arrived = {1 => true}
first_arrive = {}

ans = [1]
while true
  next_city = routes[city]&.shift
  if next_city == nil && city == 1
    break
  elsif next_city == nil
    city = first_arrive[city]
    ans << city
  elsif arrived[next_city] == nil
    arrived[next_city] = true
    first_arrive[next_city] = city
    city = next_city
    ans << city
  end
end

puts ans.join(" ")

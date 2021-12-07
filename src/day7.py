costs = []
cost = lambda x: x*(x+1)/2
for i in range(max(a)):
    cost_to_this = sum([cost(abs(i-j)) for j in a])
    costs.append(cost_to_this)
min(costs)

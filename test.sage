M = 128
t = 3
p = 0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f

print(M <= t * log(p, 2).n())

C = 2 # as specified in the paper
# Statistical Attacks
if (M <= (log(p, 2).n() - C) * (t+1)):
    rF = 6
else:
    rF = 10
print(rF)
import time
p=52435875175126190479447740508185965837690552500527637822603658699938581184513
seven=1
f=GF(p)
Seven=f(7)
s = f(1)
t0 = time.time()
#print(t0)
for _ in range(1000):
    seven = seven*7
    s = s*Seven
    inverse_mod(seven, p)

t1 = time.time()
print(t1-t0)

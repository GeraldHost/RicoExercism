o = set()
d = [5, 10, 20, 25]
def total(basket):
    for i in basket:
        o.add(i)
    n = len(o)
    p = d[n - 2] if n > 1 else 0
    return len(basket) * 800 * (1-(p/100))

factors = [(3, "Pling"), (5, "Plang"), (7, "Plong")]

def convert(number):
    result = [verb for factor, verb in factors if number % factor == 0]
    if not bool(len(result)): result.append(str(number))
    return ''.join(result)

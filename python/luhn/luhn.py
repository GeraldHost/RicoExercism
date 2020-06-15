# this only partially works.
import re
class Luhn:
    def __init__(self, card_num):
        self.card_num = card_num 

    def valid(self):
        a = re.sub("[^\d]", "", self.card_num)
        
        if len(a) <= 1:
            return False

        b = [int(c) for c in list(a)]
        c = [num * 2 if i % 2 else num for i, num in enumerate(b[::-1])]
        d = list(map(lambda x: x - 9 if x > 9 else x, c))

        return not bool(sum(c) % 10) 


def distance(strand_a, strand_b):
    if len(strand_a) != len(strand_b): raise ValueError("message")
    return len([char for i, char in enumerate(strand_a) if char != strand_b[i]])

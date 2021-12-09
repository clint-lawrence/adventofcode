
with open("08.txt") as data:
    print(sum(
        1
        for output in " ".join(line[61:-1] for line in data).split(" ")
        if len(output) in {2, 3, 4, 7}
    ))


def decode(segments):
    decode_map = {}
    def add(seg, num):
        decode_map[seg] = num
        decode_map[num] = seg

    segments_5 = []
    segments_6 = []

    for segment in segments:
        if len(segment) == 2:
            add(frozenset(segment), 1)
        elif len(segment) == 3:
            add(frozenset(segment), 7)
        elif len(segment) == 4:
            add(frozenset(segment), 4)
        elif len(segment) == 7:
            add(frozenset(segment), 8)
        elif len(segment) == 5:
            segments_5.append(frozenset(segment))
        elif len(segment) == 6:
            segments_6.append(frozenset(segment))
        else:
            raise Exception(f"invalid segment {segment}")
    
    # find segment 9, 6, 0
    for segment in segments_6:
        if segment == segment.union(decode_map[4]):
            add(segment, 9)
        elif segment == segment.union(decode_map[1]):
            add(segment, 0)
        else:
            add(segment, 6)
    
    # find segment 2, 3, 5
    for segment in segments_5:
        if segment == segment.union(decode_map[7]):
            add(segment, 3)
        elif segment.union(decode_map[1]) == decode_map[9]:
            add(segment, 5)
        else:
            add(segment, 2)
    return decode_map

with open("08.txt") as data:
    outputs = []
    for line in data:
        code, _, output = line.strip().partition(" | ")
        code = code.split(" ")
        output = output.split(" ")

        decoder = decode(code)
        outputs.append(
            1000 * decoder[frozenset(output[0])] +
            100 * decoder[frozenset(output[1])] + 
            10 * decoder[frozenset(output[2])] +
            1 * decoder[frozenset(output[3])]
        )
print(sum(outputs))

BASE_PATTERN = [1, 0, -1, 0]



def make_pattern(len_signal, digit):
	pattern = []
	for _ in range(digit):
		pattern.append(0)

	elem_in_pattern_idx = 0
	while len(pattern) < len_signal:
		pattern_elem = BASE_PATTERN[elem_in_pattern_idx % len(BASE_PATTERN)]
		for _ in range(digit + 1):
			pattern.append(pattern_elem)

		elem_in_pattern_idx += 1

	return pattern


def apply_pattern(pattern, input_signal):
	return str(abs(sum([p * int(s) for (p, s) in zip(pattern, list(input_signal))])) % 10)

def apply_phase(patterns, input_signal):
	return "".join([str(apply_pattern(p, input_signal)) for p in patterns])

def run_part1(input_signal, num_phases):
	patterns = [make_pattern(len(input_signal), p) for p in range(len(input_signal))]
	signal = input_signal
	for _ in range(num_phases):
		signal = apply_phase(patterns, signal)

	return signal[:8]

def run_part2(raw_input_signal, num_phases):
	# stolen from reddit -- 
	# relevant part is everything from the start index onward
	input_signal = raw_input_signal*10000
	relevant_pattern = input_signal[int(input_signal[:7]):]

	for i in range(num_phases):
		print(relevant_pattern)
		s = ""
		total = 0

		for index in range(len(relevant_pattern)):
			if index == 0:
				for char in list(relevant_pattern):
					total += int(char)
			elif index > 0:
				char = relevant_pattern[index - 1]
				total -= int(char)

			s += str(abs(total) % 10)

		relevant_pattern = s

	return relevant_pattern[:8]


assert(run_part1("12345678", 1) == "48226158")
assert(run_part1("12345678", 2) == "34040438")
assert(run_part1("12345678", 3) == "03415518")
assert(run_part1("12345678", 4) == "01029498")

assert(run_part1("80871224585914546619083218645595", 100) == "24176176")
assert(run_part1("19617804207202209144916044189917", 100) == "73745418")
assert(run_part1("69317163492948606335995924319873", 100) == "52432133")

INPUT_MESSAGE = "\
597150919766609778476861804721789882748688742489128919278817705064161286676791229587926244062310720\
132211266238814893179123097633851821336018404464691641520948019118465722353675853630919441535749347\
094085116885683625088770436435695196309508366992460462862624794078064940083280686072759316330949493\
442813981508001879713176845011131911848381188502871898308721288121882376806735137452696452192281836\
339867018714884672847164339536634984448297483644020223937279387813576640347397724578551664718028865\
652578588132916675256350018235846504208153161329438694998003749977771307558423191534638953644092262\
60937941771665247483191282218355610246363741092810592458"

assert(run_part2("03036732577212944063491565474664", 100) == "84462026")
assert(run_part2("02935109699940807407585447034323", 100) == "78725270")
assert(run_part2("03081770884921959731165446850517", 100) == "53553731")

assert(run_part2(INPUT_MESSAGE, 100) == "85600369")

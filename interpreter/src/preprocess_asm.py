from sys import argv, stderr

def lex(it):
	for i in it:
		i = i.split(';')[0].strip()
		if i != '': yield i

def to_le(x):
	h,l = x // 0x40, x % 0x40
	return f'{l:02x}{h:02x}'

def words_arg(str):
	if str[0] == '#':
		return 1
	elif str.split('@')[0] in ['a','bl','bh','cl','ch','x']:
		return 2
	else:
		return 3

def words_line(str):
	if str[0] == ':': return 0
	op, *args = str.split()
	if op in ['nop','ret','clc','sec']:
		return 1
	elif op in ['jmp','cal']:
		return 1+2
	elif op in ['bcc','bcs','bne','beq','bpl','bmi']:
		return 1+1
	else:
		return 2 + sum(words_arg(x) for x in args)

with open(argv[1]) as f:
	cursor = 0x80
	labels = {}
	for i in lex(f):
		print(f'{to_le(cursor):4} | {i}', file=stderr)
		if i[0] == ':':
			labels[i.split()[0][1:]] = to_le(cursor)
		cursor += words_line(i)

print(labels, file=stderr)

def replace(line, labels):
	for label in labels:
		old1 = f'jmp {label}\n'
		new1 = f'jmp {labels[label]}\n'
		old2 = f'cal {label}\n'
		new2 = f'cal {labels[label]}\n'
		line = line.replace(old1, new1).replace(old2, new2)
	return line


with open(argv[1]) as f:
	for line in f:
		print(replace(line, labels), end='')

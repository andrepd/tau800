'''Converts the input of day 1 into a TAU disk file'''

import sys

def words(x):
	d = x % 64
	x = x // 64
	c = x % 64
	x = x // 64
	b = x % 64
	x = x // 64
	a = x % 64
	return a,b,c,d

for i in sys.stdin:
	i = i.strip()
	if i:
		a,b,c,d = words(int(i))
		print(f'{a:02x} {b:02x} {c:02x} {d:02x}')
	else:
		print('00 00 00 00')
print('00 00 00 00')
print('00 00 00 00')

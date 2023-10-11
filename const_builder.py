map1 = [[0] * 256] * 2
map2 = [[0] * 256] * 2
map3 = [[0] * 256] * 2
for i in [9, 10, 32, 64, 123, 125]:
	map2[1][i] = 0
# (matched string, tt, td)
ops = [
	('+', 2, 0), ('++', 2, 6), ('+=', 5, 0),
	('-', 2, 1), ('--', 2, 7), ('-=', 5, 1),
	('*', 2, 3), ('**', 2, 5), ('*=', 5, 3), ('**=', 5, 5),
	('/', 2, 4), ('/=', 5, 4), ('%', 2, 2), ('%=', 5, 2),
	('!', 2, 8), ('!=', 3, 1), ('=', 5, 255), ('==', 3, 0),
	('<', 3, 2), ('>', 3, 3), ('<=', 3, 4), ('>=', 3, 5),
	(':', 2, 9), ('?', 2, 10), ('.', 2, 11),
	('(', 4, 0), (')', 4, 1), ('[', 4, 2), (']', 4, 3),
	('->', 2, 12), ('=>', 2, 13)
]
for (s, tt, td) in ops:
	row = 0
	s_bytes = str.encode(s)
	for b in s_bytes[:-1]:
		if map1[row][b] == 0:
			map1[row][b] = len(map1)
			row = len(map1)
			map1.append([0] * 256)
			map2.append([0] * 256)
			map3.append([0] * 256)
		else:
			row = map1[row][b]
	map2[row][s_bytes[-1]] = tt
	map3[row][s_bytes[-1]] = td

for (f, m) in [("map1", map1), ("map2", map2), ("map3", map3)]:
	with open(f"lang-inner/src/tables/table_init/{f}.in", "w") as file:
		file.truncate()
		file.write(f"vec!{m}")
		file.close()

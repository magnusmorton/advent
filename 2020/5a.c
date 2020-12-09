#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

int bsp(char *line, int range, int chars) {
	int min = 0;
	int max = range;
	
	
	for (int i = 0; i < chars; i++) {
		range /= 2;
		switch (line[i]) {
		case 'F':
		case 'L':
			max -= range;
			break;
		case 'B':
		case 'R':
			min += range;
			break;
		default:
			perror("woops");
			return -1;
		}
	}
	return min;
}

int main(void) {
	char *line = NULL;
	size_t len = 0;
	int max = 0;
	while (getline(&line, &len, stdin) != -1) {
		int row = bsp(line, 128, 7);
		int col = bsp(line+7,8,3);
		int seatid = row * 8 + col;
		max = (seatid > max) ? seatid : max;		
	}
	printf("%d\n", max);
	free(line);
}

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
	char *line = NULL;
	size_t len = 0;
	int x,y;
	x=y=0;
	long trees = 0;

	while (getline(&line, &len, stdin) != -1) {
		int strl = strlen(line) - 1;

		if (x >= strl)
			x = x % strl;
		if (line[x] == '#')
			trees++;
		x += 3;
	}

	printf("%ld\n", trees);
	free(line);
	
}

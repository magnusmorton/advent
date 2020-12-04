#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
	char *line = NULL;
	size_t len = 0;
	int xs[5] = {0};
	int xincs[5] = {1, 3, 5, 7, 1};
	int trees[5] = {0};
	int xy = 0;
	int counter = 0;
	while (getline(&line, &len, stdin) != -1) {
		int strl = strlen(line) - 1;

		for (int i=0; i < 5; i ++){
			if (i == 4 && counter % 2)
				continue;
			if (xs[i] >= strl)
				xs[i] = xs[i] % strl;
			if (line[xs[i]] == '#')
				trees[i]++;
			xs[i] += xincs[i];
		}
		counter++;
	}

	unsigned long tot = 1;
	for (int i = 0; i < 5; i++) {
		tot =  tot * trees[i];
	}
	printf("%ld\n", tot);
	free(line);
	
}

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int validate(int ans[], int count) {
	for (int i=0; i < 26; i++) {
		if (ans[i]) {
			count ++;
			ans[i] = 0;
		}
	}
	return count;
}

int main(void)
{
	char *line = NULL;
	size_t len = 0;
	unsigned long count = 0;
	int ans[26] = {0};
	while (getline(&line, &len, stdin) != -1) {

		if (line[0] == '\n') {
			count = validate(ans, count);
			continue;
		}
		char *curr = line;
		while (*curr != '\0' &&  *curr != '\n') {
			ans[*curr - 97] = 1;
			curr++;
		}
	}
	count = validate(ans, count);
	printf("count %ld\n", count);
	free(line);
}

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int validate(size_t ans[], size_t group_s) {
	size_t count = 0;
	for (int i=0; i < 26; i++) {
		if (ans[i] == group_s) {
			count ++;
		}
		ans[i] = 0;

	}
	return count;
}

int main(void)
{
	char *line = NULL;
	size_t len = 0;
	unsigned long count = 0;
	size_t  ans[26] = {0};
	size_t group_s = 0;
	while (getline(&line, &len, stdin) != -1) {

		if (line[0] == '\n') {
			count += validate(ans,  group_s);
			group_s = 0;
			continue;
		}

		char *curr = line;
		while (*curr != '\0' &&  *curr != '\n') {
			ans[*curr - 97]++;
			curr++;
		}
		group_s ++;
					
	}
	count += validate(ans, group_s);
	printf("count %ld\n", count);
	free(line);
}

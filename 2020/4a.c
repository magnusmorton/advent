#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char *fields[] = {"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"};
char cid[] = "cid";


int validate(int *found) {
	int check = 1;
	for (int i = 0; i < 7; i ++) {
		check *= found[i];
		found[i] = 0;
	}
	return check;
}

int main(void) {
	char *line = NULL;
	size_t len = 0;
	int valid = 0;

	int found[7];
	
	while (getline(&line, &len, stdin) != -1) {
		if (line[0] == '\n') {
			if (validate(found)) {
				valid++;
			}
			continue;
		}

		char *orig = line;
		char *sep;

		while ( (sep = strsep(&line, " ")) != NULL) {

			char *ssep = strsep(&sep, ":");

			for (int i=0; i < 7; i++) {
				if (strncmp(ssep, fields[i], 3) == 0) {
					found[i] = 1;
				}
			} 
		}
		line = orig;
	}
	if (validate(found))
		valid ++;
	free(line);
	printf("%d\n", valid);
	
}

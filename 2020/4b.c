#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

char *fields[] = {"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"};
char *eyes[] = {"amb", "blu", "brn", "gry", "grn", "hzl", "oth"};
char cid[] = "cid";


int validate(int *found) {
	int check = 1;
	for (int i = 0; i < 7; i ++) {
		check *= found[i];
		//printf("check %d\n", check);
		found[i] = 0;
	}
	return check;
}

int main(void) {
	char *line = NULL;
	size_t len = 0;
	int valid = 0;

	int found[7] = {0, 0, 0, 0, 0, 0, 0};
	
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


			if (strncmp(ssep, "byr", 3) == 0) {
				int year = atoi(sep);
				if (year >= 1920 && year <= 2002) {
					printf("foundbyr\n");
					found[0] = 1;
				}
			} else if (strncmp(ssep, "iyr", 3) == 0) {
				int year = atoi(sep);
				if (year >= 2010 && year <= 2020) {
					printf("foundiyr\n");
					found[1] = 1;
				}
			} else if (strncmp(ssep, "eyr", 3) == 0) {
			    int year = atoi(sep);
				if (year >= 2020 && year <= 2030) {
					printf("found_eyr\n");
					found[2] = 1;
				}
			} else if (strncmp(ssep, "hgt", 3) == 0) {
				char *endptr;
				long h = strtol(sep, &endptr, 10);
				int check = 0;
				if (strncmp(endptr, "cm", 2) == 0) {
					check = h >= 150 && h <= 193; 
				} else if (strncmp(endptr, "in", 2) == 0) {
					check = h >= 59 && h <= 76;
				}
				if (check) {
					printf("found_hgt\n");
					found[3] = 1;
				}
			} else if (strncmp(ssep, "hcl", 3) == 0) {
				int check = 1;

				if (sep[0] == '#') {
					char *curr = sep +1;
					int i = 0;
					while (*curr != '\0' && *curr != '\n') {
						if (!isxdigit(*curr))
							check = 0;
						i++;
						curr++;
					}
//					printf("hcl check %d", check);
					if (i != 6)
						check = 0;
   
				} else {
					check = 0;
				}
				found[4] = check;
			} else if (strncmp(ssep, "ecl", 3) == 0) {
				int check = 0;
				for (int i=0; i < 7; i++) {
					if (strncmp(sep, eyes[i], 3) == 0) {
						check = 1;
					}
						
				}
				found[5] = check;
			} else if (strncmp(ssep, "pid", 3) == 0) {
				int check = 1;
				char *curr = sep;
				int i = 0;
				while (*curr != '\0' && *curr !='\n') {
					if (!isdigit(*curr))
						check = 0;
					i ++;
					curr++;
				}
				if (i != 9)
					check = 0;
				
				found[6] = check;
			} 
		}
		line = orig;
	}
	if (validate(found))
		valid ++;
	free(line);
	printf("%d\n", valid);
	
}

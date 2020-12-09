#include <glib.h>
#include <stdio.h>

int main(void) {
	char *line = NULL;
	size_t len = 0;
	gchar **splits = NULL;
	long valid = 0;
	while (getline(&line, &len, stdin) != -1) {
		splits = g_strsplit(line, ":", 2);

		char *policy = splits[0];
		char *mins = strsep(&policy, "-");
		char *maxs = strsep(&policy, " ");
		char *pchar = policy;
		long min = atol(mins);
		long max = atol(maxs);
		char *pass = splits[1];
		g_strstrip(pass);

		int matched = 0;
		for (int i=0; i < strlen(pass); i++) {
			if (pchar[0] == pass[i])
				matched++;
		}
		if (matched >= min && matched <= max)
			valid++;

		g_strfreev(splits);
	}

	printf("%d\n", valid);
	free(line);
	
}

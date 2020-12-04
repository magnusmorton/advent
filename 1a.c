#include <glib.h>
#include <glib/gprintf.h>
#include <stdio.h>
#include <stdlib.h>

int main(void)
{
	char *line = NULL;
	size_t len = 0;
	GArray *garry;
	garry = g_array_new(FALSE, FALSE, sizeof(glong));
	while (getline(&line, &len, stdin) != -1) {
		glong l = atol(line);
		g_array_append_val(garry, l);
	}

	for (int i=0; i<garry->len; i++){
		glong curr = g_array_index(garry, glong, i);
		for (int j=i+1; j < garry->len; j++) {
			glong next = g_array_index(garry, glong, j);
			if (curr + next == 2020) {
				printf("%d\n", curr*next);
				return 0;
			}
		}
	}
	
}

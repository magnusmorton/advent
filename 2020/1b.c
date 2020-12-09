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
		glong one = g_array_index(garry, glong, i);
		for (int j=0; j < garry->len; j++) {
			glong two = g_array_index(garry, glong, j);
			if (two == one)
				continue;
			
			for (int k=0; k < garry->len; k++) {
				glong three = g_array_index(garry, glong, k);

				if (three == one || three == two)
					continue;
				
				if (one + two + three == 2020) {
					printf("%d\n", one*two*three);
					return 0;
				}
			}
		}
	}
	
}

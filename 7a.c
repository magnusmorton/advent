#include <stdio.h>
#include <stdlib.h>
#include <regex.h>
#include <string.h>
#include <gmodule.h>
unsigned long
hash(char *str)
{
	unsigned long hash = 5381;
	int c;
	
	while ((c = *str++))
		hash = ((hash << 5) + hash) + c; /* hash * 33 + c */
	
	return hash;
}



struct bag
{
	char *name;
	struct bag *contains;
};

typedef struct bag bag_t;


void
free_bag(bag_t *bag)
{
	free(bag->name);
	free(bag);
}

bag_t *make_bag(char* name) {
	bag_t *bag = (bag_t*) malloc(sizeof(bag_t));
	bag->name = name;
	return bag;
}

int main()
{
	char *line = NULL;
	size_t len = 0;

	GHashTable *ht = g_hash_table_new_full(g_str_hash, g_str_equal, g_free, g_free);

	while (getline(&line, &len, stdin) != -1) {
		char *decl;
		char *cont = strstr(line, " bags contain");
		decl = strndup(line, cont - line);
		cont = g_strstrip(cont + strlen(" bags contain"));
		cont[strlen(cont)-1] = '\0';

		if (strncmp(cont, "no other bags", 14) == 0)
			printf("foo\n");
		else {
			gchar** splits = g_strsplit(cont, ",", -1);
			gchar** p = splits;
			GArray *arr = g_array_new(TRUE, TRUE, sizeof(char*));
			while (*p) {
				char* split = *p;
				g_strstrip(split);
				char* sep = strsep(&split, " ");
				char* bag = strstr(split, " bag");
				char* colo = strndup(split, bag - split);
				printf("%s\n", colo);
				g_array_append_val(arr, colo);
				p++;
			}
			gsize data_len;
			g_hash_table_insert(ht, decl, g_array_steal(arr, &data_len));
			g_strfreev(splits);
		}
	}
	free(line);

	// search
	GHashTableIter iter;
	gpointer key, value;

	g_hash_table_iter_init (&iter, ht);
	while (g_hash_table_iter_next (&iter, &key, &value)) {
		// do something with key and value
		printf("%s\n", key);
	}
	g_hash_table_destroy(ht);
}

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <dirent.h>
#include <sys/stat.h>
#include <rpc/types.h>

#include "getdir.h"
#include "filter.h"

#define MAX_NAME_LENGTH 0x100

typedef struct
{
    char name[MAX_NAME_LENGTH];
    bool_t isDirectory;
    size_t size;
    time_t lastModified;
    bool_t filtered;
}
direntry_t;

// server state
char path[MAX_PATH_LENGTH];
DIR *dirptr;
direntry_t entry;

bool_t list(char *, bool_t, filter_t *);
bool_t openlist(char *);
direntry_t *nextlist(filter_t *);
bool_t closelist(void);
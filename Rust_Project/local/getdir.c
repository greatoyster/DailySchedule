#include "getdir.h"

char* getdir(void)
{
    // char* getdir (void) return the current directory.

    getcwd(currentDir,MAX_PATH_LENGTH);

    return currentDir;
}
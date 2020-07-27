#include "filecount.h"

int filecount(char *condition)
{
    int count = 0;
    struct dirent * entry;
    DIR * dirptr;

    dirptr = opendir(".");
    if(dirptr == NULL)
    {
        printf("open directory failed\n");
        return OPEN_DIRECTORY_FAILED;
    }

    while ((entry = readdir(dirptr)) != NULL)
    {
        char ptemp[MAX_PATH_LENGTH] = "";
        strcat(ptemp, "./");
        strcat(ptemp, entry->d_name);
        
        if(filter(ptemp, condition))
            count++;
    }

    closedir(dirptr);

    return count;
}

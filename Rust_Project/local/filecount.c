#include "filecount.h"

int filecount(filter_t *filter)
{
    /*Return the nunmber of the file. If it fails, it will return -1.  */
    int count = 0;
    struct dirent * entry;
    DIR * dirptr;

    dirptr = opendir(".");
    /*Open the current directory. If it fails, message will be sent to the console. */
    if(dirptr == NULL)
    {
        printf("open directory failed\n");
        return OPEN_DIRECTORY_FAILED;
    }

    while ((entry = readdir(dirptr)) != NULL)
            count++;

    closedir(dirptr);

    return count;
}

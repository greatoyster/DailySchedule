#include "ls.h"

bool_t openlist(char *directory)
{
    strcpy(path, directory);
    return ((dirptr = opendir(directory)) != NULL) ? TRUE : FALSE;
}

direntry_t *nextlist(char *condition)
{
    /*The iteration of current file, it will switch to next file. */
    if(dirptr != NULL)
    {
        struct dirent *dtemp;
        struct stat stemp;
        char ptemp[MAX_PATH_LENGTH];

        if((dtemp = readdir(dirptr)) != NULL)
        {
            strcpy(entry.name, dtemp->d_name);
            /*To detect if the type is directory. */
            entry.isDirectory = (dtemp->d_type == DT_DIR) ? TRUE : FALSE;

            strcpy(ptemp, path);
            strcat(ptemp, "/");
            strcat(ptemp, dtemp->d_name);
            /*read the status of the iterated file. */
            if(stat(ptemp, &stemp) == 0)
            {
                entry.size = stemp.st_size;
                entry.lastModified = stemp.st_mtime;
                entry.filtered = filter(ptemp, condition);
            }
            else
            {
                printf("failed to read stat of %s\n", dtemp->d_name);
                return NULL;
            }
        }
        else
            return NULL;
    }
    else
    {
        printf("failed to read next entry in list\n");
        printf("exiting...\n");
        exit(1);
    }

    return &entry;
}

bool_t closelist(void)
{
    return (closedir(dirptr) == 0) ? TRUE : FALSE;
}
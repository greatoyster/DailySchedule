#include "cd.h"


int changedir(char* str)
{
    /* int changedir(char* str)
    parameter char* str: the path of the directory you want to swtich to.
    If it fails, it will output message "cd fail"*/
    int ret=0;
    char* dir=str;
    
    ret=chdir(str);
    if (ret==-1)
    {
        printf("cd failed\n");
        return -1;
    }
    return 0;
}
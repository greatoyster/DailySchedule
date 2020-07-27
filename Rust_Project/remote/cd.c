#include "cd.h"
#include <rpc/types.h>

bool_t changedir(char* str)
{
    /* int changedir(char* str)
    parameter char* str: the path of the directory you want to swtich to.
    If it fails, it will output message "cd fail"*/

    return (chdir(str) == 0 ? TRUE : FALSE);
}
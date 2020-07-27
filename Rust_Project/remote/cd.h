#ifndef CD_H
#define CD_H
#include <stdlib.h>
#include <stdio.h>
#include <unistd.h>
int changedir(char* str);
/* int changedir(char* str)
    parameter char* str: the path of the directory you want to swtich to.
    If it fails, it will output message "cd fail"*/
#endif
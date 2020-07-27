#ifndef GETDIR_H
#define GETDIR_H
#include <stdlib.h>
#include <stdio.h>
#include <unistd.h>
#define MAX_PATH_LENGTH 0x100
char* getdir(void);
#endif

char currentDir[MAX_PATH_LENGTH];
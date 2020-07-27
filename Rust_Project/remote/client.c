#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#include "getdir.h"
#include "cd.h"
#include "filecount.h"
#include "ls.h"

#define MAX_INPUT_LENGTH 0x100

int main(int argc, char **argv)
{
    /*The main function of client. */
    if(argc == 1)
    {
        /*If arguements is missing, it will provide usage hint */
        printf("Usage: client [-f filename] <server name>\n");
        exit(1);
    }

    FILE *input = stdin;
    if(argc == 4 && strcmp(argv[1], "-f") == 0)
    {
        input = fopen(argv[2], "rb");
        if(input == NULL)
        {
            printf("invalid file: %s\n", argv[2]);
            exit(1);
        }
    }
    else if(argc >= 3)
    {
        printf("Usage: client [-f filename] <server name>\n");
        exit(1);
    }

    char buffer[MAX_INPUT_LENGTH];
    char *token[MAX_INPUT_LENGTH];
    printf("(client) ");
    while(fgets(buffer, MAX_INPUT_LENGTH, input))
    {
        char *copy = strdup(buffer);
        char *ptr = strtok(copy, " \n");
        while(ptr != NULL)
        {
            if(strncmp(ptr, "quit", 4) == 0)
                exit(0);
            else if(strncmp(ptr, "getdir", 6) == 0)
            {
                printf("getdir succeeded with: %s\n", getdir());
            }
            else if(strncmp(ptr, "cd", 2) == 0)
            {
                ptr = strtok(NULL, " \n");
                if(ptr != NULL)
                {
                    if(changedir(ptr) == 0)
                        printf("cd succeeded\n");
                }
                else
                {
                    printf("invalid path\n");
                    break;
                }
            }
            else if(strncmp(ptr, "filecount", 9) == 0)
            {
                printf("filecount succeeded with count of %d\n", filecount(NULL));
            }
            else if(strncmp(ptr, "ls", 2) == 0)
            {
                ptr = strtok(NULL, " \n");
                if(ptr != NULL)
                {
                    if(strncmp(ptr, "-l", 2) == 0)
                    {
                        ptr = strtok(NULL, " \n");
                        if(ptr == NULL)
                            list(".", TRUE, NULL);
                        else
                            list(ptr, TRUE, NULL);
                    }
                    else
                        list(ptr, FALSE, NULL);
                }
                else
                    list(".", FALSE, NULL);
            }
            else
            {
                printf("invalid command\n");
                printf("available command: getdir; cd; filecount; ls\n");
                printf("type 'quit' to quit\n");
                break;
            }
            
            ptr = strtok(NULL, " \n");
        }  

        printf("(client) ");
    }

    printf("\n");

    return 0;
}
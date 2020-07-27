#include "filter.h"

bool_t filter(char *path, char *condition)
{
    if(condition == NULL || condition[0] == '\0')
        return TRUE;

    struct stat s;
    if(stat(path, &s) == 0)
    {
        char *ptr = condition;
        if(strncmp(ptr, "size", 4) == 0)
        {
            ptr += 4;
            switch(*ptr)
            {
                case '<': return (s.st_size < atoi(ptr + 1));
                case '>': return (s.st_size > atoi(ptr + 1));
                case '=': return (s.st_size == atoi(ptr + 1));
                default: printf("invalid condition\n");
            }
        }
        else if(strncmp(ptr, "age", 3) == 0)
        {
            int age = (int)difftime(time(0), s.st_mtime);

            ptr += 3;
            switch(*ptr)
            {
                case '<': return (age < evalage(ptr + 1));
                case '>': return (age > evalage(ptr + 1));
                case '=': return (age == evalage(ptr + 1));
                default: printf("invalid condition\n");
            }
        }
        else if(strncmp(ptr, "type", 4) == 0)
        {
            ptr += 4;
            if(*ptr == '=')
            {
                ptr++;
                if(*ptr == 'd')
                    return ((s.st_mode & S_IFDIR) == S_IFDIR);
                else if(*ptr == 'r')
                    return ((s.st_mode & S_IFREG) == S_IFREG);
                else
                    printf("invalid condition\n");
            }
            else
                printf("invalid condition\n");
        }
        else
            printf("invalid condition\n");
    }

    return FALSE;
}

bool_t isfilter(char *ptr)
{
    return (strchr(ptr, '<') || strchr(ptr, '>') || strchr(ptr, '='));
}

int evalage(char *age)
{
    char *ptr = age;
    int magnitude = atoi(ptr);
    
    while(*ptr != '\0')
        ptr++;

    switch(*(ptr - 1))
    {
        case 's': return magnitude;
        case 'm': return magnitude * 60;
        case 'h': return magnitude * 60 * 60;
        case 'd': return magnitude * 60 * 60 * 24;
    }

    return -1;
}
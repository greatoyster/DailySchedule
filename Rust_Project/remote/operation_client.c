/*
 * This is sample code generated by rpcgen.
 * These are only templates and you can use them
 * as a guideline for developing your own functions.
 */

#include "filter.h"
#include "operation.h"
#include <string.h>
#include <time.h>
FILE *local_file;

void getdir_prog_1(char *host)
{
    CLIENT *clnt;
    char **result_1;
    char *getdir_1_arg;

#ifndef DEBUG
    clnt = clnt_create(host, GETDIR_PROG, VES, "udp");
    if (clnt == NULL)
    {
        clnt_pcreateerror(host);
        exit(1);
    }
#endif /* DEBUG */
    result_1 = getdir_1((void *)&getdir_1_arg, clnt);
    if (result_1 == (char **)NULL)
    {
        clnt_perror(clnt, "call failed");
    }
    printf("getdir succeeded with: %s\n", *result_1);
#ifndef DEBUG
    clnt_destroy(clnt);
#endif /* DEBUG */
}

void cd_prog_1(char *host, char *str)
{
    CLIENT *clnt;
    bool_t *result_1;
    char *changedir_1_arg;
    changedir_1_arg = str;

#ifndef DEBUG
    clnt = clnt_create(host, CD_PROG, VES, "udp");
    if (clnt == NULL)
    {
        clnt_pcreateerror(host);
        exit(1);
    }
#endif /* DEBUG */

    result_1 = changedir_1(&changedir_1_arg, clnt);
    if (result_1 == (bool_t *)NULL)
    {
        clnt_perror(clnt, "call failed");
    }

#ifndef DEBUG
    clnt_destroy(clnt);
#endif /* DEBUG */

    if (*result_1)
        printf("cd succeeded\n");
    else
        printf("invalid path\n");
}

void filecount_prog_1(char *host, char *arg)
{
    CLIENT *clnt;
    int *result_1;
    char *filecount_1_arg;
    filecount_1_arg = (arg == NULL ? "" : arg);

#ifndef DEBUG
    clnt = clnt_create(host, FILECOUNT_PROG, VES, "udp");
    if (clnt == NULL)
    {
        clnt_pcreateerror(host);
        exit(1);
    }
#endif /* DEBUG */

    result_1 = filecount_1(&filecount_1_arg, clnt);
    if (result_1 == (int *)NULL)
    {
        clnt_perror(clnt, "call failed");
    }
#ifndef DEBUG
    clnt_destroy(clnt);
#endif /* DEBUG */

    printf("filecount succeeded with count of %d\n", *result_1);
}

void ls_prog_1(char *host, char *directory, bool_t _l, char *arg)
{
    CLIENT *clnt;
    bool_t *result_1;
    char *openlist_1_arg;
    struct DIRENTRY_T *result_2;
    char *nextlist_1_arg;
    bool_t *result_3;
    char *closelist_1_arg;

    openlist_1_arg = directory;
    nextlist_1_arg = (arg == NULL ? "" : arg);

#ifndef DEBUG
    clnt = clnt_create(host, LS_PROG, VES, "udp");
    if (clnt == NULL)
    {
        clnt_pcreateerror(host);
        exit(1);
    }
#endif /* DEBUG */

    if (*openlist_1(&openlist_1_arg, clnt))
    {
        printf("ls succeeded\n");
        while (result_2 = nextlist_1(&nextlist_1_arg, clnt))
        {
            // a not so neat way to break a loop but returning a NULL from the server side doesn't seem to work
            if (result_2->name == NULL)
                break;
            time_t lm = (time_t)result_2->lastModified;
            if (result_2->filtered)
            {
                if (_l)
                    if (result_2->isDirectory)
                        printf("%-15s/ %d %-16zu %s", result_2->name, result_2->isDirectory, result_2->size, ctime(&lm));
                    else
                        printf("%-16s %d %-16zu %s", result_2->name, result_2->isDirectory, result_2->size, ctime(&lm));
                else if (result_2->isDirectory)
                    printf("%-15s/\n", result_2->name);
                else
                    printf("%-16s\n", result_2->name);
            }
        }
    }
    else
        printf("invalid path\n");

    result_3 = closelist_1((void *)&closelist_1_arg, clnt);
    if (result_3 == (bool_t *)NULL)
    {
        clnt_perror(clnt, "call failed");
    }

#ifndef DEBUG
    clnt_destroy(clnt);
#endif /* DEBUG */
}

void put_1(char *host, char *localfile, char *remotefile)
{
    /*this function put local file to remote.
    the remotefile name is optional and its default value is the local filename if there is no remote filenme input
    first detect if the file name is valid,
    then open local file to read,
    then read the file into blocks and pass the block to remote repeatly
    */
    CLIENT *clnt;
    bool_t *result_1;
    char *openfiletowrite_1_arg;
    bool_t *result_2;
    struct BLOCK_T put_nextwrite_1_arg;
    bool_t *result_3;
    char *put_closefile_1_arg;
    //detect whether there is optional arguement
    if (remotefile == NULL)
    {
        remotefile = localfile;
    }
    openfiletowrite_1_arg = remotefile;
    put_closefile_1_arg = remotefile;
    local_file = fopen(localfile, "r");
    if (local_file == NULL)
    {
        printf("can not open local file %s\n", localfile);
        return;
    }
    u_int count = 0;
    static char temp_str[MAX_BLOCK_SIZE + 1];
    char temp_ch;
    put_nextwrite_1_arg.data.data_val = temp_str;
    memset(temp_str, 0, sizeof(temp_str));
    //load the first block
    /* while (count<MAX_BLOCK_SIZE)
    {
        if ((temp_ch=fgetc(local_file)) ==EOF)
        {
            break;
        }
        temp_str[count]=temp_ch;
        count++;
    }
    put_nextwrite_1_arg.data.data_len=count;*/
    count = fread(temp_str, sizeof(char), MAX_BLOCK_SIZE, local_file);
    put_nextwrite_1_arg.data.data_len = count;
#ifndef DEBUG
    clnt = clnt_create(host, PUT, VES, "udp");
    if (clnt == NULL)
    {
        clnt_pcreateerror(host);
        exit(1);
    }
#endif /* DEBUG */

    result_1 = openfiletowrite_1(&openfiletowrite_1_arg, clnt);
    if (result_1 == (bool_t *)NULL)
    {
        clnt_perror(clnt, "call failed");
    }
    if (*result_1 == TRUE)
    {
        printf("open file %s to write successfully!\n", remotefile);
    }
    else
    {
        printf("can not open file %s to write !\n", remotefile);
        return;
    }
    result_2 = put_nextwrite_1(&put_nextwrite_1_arg, clnt);
    if (result_2 == (bool_t *)NULL)
    {
        clnt_perror(clnt, "call failed");
    }
    while (!(*result_2))
    {
        count = 0;
        memset(temp_str, 0, sizeof(temp_str));
        count = fread(temp_str, sizeof(char), MAX_BLOCK_SIZE, local_file);
        put_nextwrite_1_arg.data.data_len = count;
        result_2 = put_nextwrite_1(&put_nextwrite_1_arg, clnt);
    }
    result_3 = put_closefile_1((void *)&put_closefile_1_arg, clnt);
    if (result_3 == (bool_t *)NULL)
    {
        clnt_perror(clnt, "call failed");
    }
#ifndef DEBUG
    clnt_destroy(clnt);
#endif /* DEBUG */

    if (*result_3 == TRUE)
    {
        printf("close remote file %s successfully\n", remotefile);
    }
    else
    {
        printf("can not close remote file %s\n", remotefile);
    }
    int i;
    i = fclose(local_file);
    if (i != 0)
    {
        printf("warning: can not close local file\n");
    }
}

void get_1(char *host, char *remotefile, char *localfile)
{
    /*this function get file from remote
    the argurment localfile is optional,
    and its default value is remotefile name if there is no input 
    send the file name to remote 
    then to open the remote file,
    then get file from remote recurrently in blocks,
    after receiving the remote file, write them to the local file,
    at last, close local file stream. */
    CLIENT *clnt;
    bool_t *result_1;
    char *get_openfiletoread_1_arg;
    struct BLOCK_T *result_2;
    char *get_nextread_1_arg;
    bool_t *result_3;
    char *get_closefile_1_arg;
    get_openfiletoread_1_arg = remotefile;
    if (localfile == NULL)
    {
        localfile = remotefile;
    }

#ifndef DEBUG
    clnt = clnt_create(host, GET, VES, "udp");
    if (clnt == NULL)
    {
        clnt_pcreateerror(host);
        exit(1);
    }
#endif /* DEBUG */

    result_1 = get_openfiletoread_1(&get_openfiletoread_1_arg, clnt);
    if (result_1 == (bool_t *)NULL)
    {
        clnt_perror(clnt, "call failed");
    }
    if (*result_1 == TRUE)
    {
        printf("open remote file %s successfully\n", remotefile);
    }
    else
    {
        printf("can not open remote file %s\n", remotefile);
        return;
    }
    result_2 = get_nextread_1((void *)&get_nextread_1_arg, clnt);
    if (result_2 == (struct BLOCK_T *)NULL)
    {
        clnt_perror(clnt, "call failed");
    }

    local_file = fopen(localfile, "w");
    if (local_file == NULL)
    {
        printf("warning: can not open local file.\n");
        return;
    }

    fwrite(result_2->data.data_val, sizeof(char), result_2->data.data_len, local_file);
    while (result_2->data.data_len == 512)
    {
        result_2 = get_nextread_1((void *)&get_nextread_1_arg, clnt);
        fwrite(result_2->data.data_val, sizeof(char), result_2->data.data_len, local_file);
    }

    result_3 = get_closefile_1((void *)&get_closefile_1_arg, clnt);
    if (result_3 == (bool_t *)NULL)
    {
        clnt_perror(clnt, "call failed");
    }

    if (*result_3 == TRUE)
    {
        printf("close remote file %s successfully\n", remotefile);
    }
    else
    {
        printf("warning: can not close remote file %s\n", remotefile);
    }

#ifndef DEBUG
    clnt_destroy(clnt);
#endif /* DEBUG */

    int i = 0;
    i = fclose(local_file);
    if (i != 0)
    {
        printf("warning: close local file failed!\n");
    }
}

void randomread_1(char *host, char *remotefile, char *firstbyte, char *numbytes)
{
    CLIENT *clnt;
    bool_t *result_1;
    char *rand_openfiletoread_1_arg;
    struct BLOCK_T *result_2;
    struct RANDOMINFO_T rand_nextread_1_arg;
    bool_t *result_3;
    char *rand_closefile_1_arg;
    /*the fuction can get remote file in random access which is up to 512 bytes, 
    the arguements are remote file name, first byte and the lenth of the bytes,
    Note that the index of first byte is 0, 

    first pass related arguments to remote,
    after receiving block from romote,
    then print the block in the console.
    */
    char temp_str[MAX_BLOCK_SIZE + 1];
    memset(temp_str, 0, sizeof(temp_str));
    rand_openfiletoread_1_arg = remotefile;
    rand_nextread_1_arg.firstbyte = atoi(firstbyte);
    int len = atoi(numbytes);
    int fb = atoi(firstbyte);
    if (fb < 0)
    {
        printf("invalid arguement: firstbyte should be non-negative integer!\n");
        return;
    }
    if (len > 512)
    {
        printf("invalid arguement: the block is up to 512 bytes!\n");
        return;
    }
    rand_nextread_1_arg.bytenum = len;

#ifndef DEBUG
    clnt = clnt_create(host, RANDOMREAD, VES, "udp");
    if (clnt == NULL)
    {
        clnt_pcreateerror(host);
        exit(1);
    }
#endif /* DEBUG */

    result_1 = rand_openfiletoread_1(&rand_openfiletoread_1_arg, clnt);
    if (result_1 == (bool_t *)NULL)
    {
        clnt_perror(clnt, "call failed");
    }
    if (*result_1 == FALSE)
    {
        printf("can not open remote file\n");
        return;
    }

    result_2 = rand_nextread_1(&rand_nextread_1_arg, clnt);
    if (result_2 == (struct BLOCK_T *)NULL)
    {
        clnt_perror(clnt, "call failed");
    }
    printf("the bytes from remote:\n");
    for (int i = 0; i < result_2->data.data_len; ++i)
    {
        printf("%c", *(result_2->data.data_val++));
    }
    printf("\nrandomread succeeded transferring %d bytes\n", result_2->data.data_len);

    result_3 = rand_closefile_1((void *)&rand_closefile_1_arg, clnt);
    if (result_3 == (bool_t *)NULL)
    {
        clnt_perror(clnt, "call failed");
    }
    if (*result_3 == FALSE)
    {
        printf("can not close remote file\n");
    }
#ifndef DEBUG
    clnt_destroy(clnt);
#endif /* DEBUG */
}

int main(int argc, char **argv)
{
    char *host;
    FILE *input = stdin;
    if (argc == 2)
    {
        input = stdin;
        host = argv[1];
    }
    else if (argc == 4 && strcmp(argv[1], "-f") == 0)
    {
        input = fopen(argv[2], "r");
        if (input == NULL)
        {
            printf("invalid file: %s\n", argv[2]);
            exit(1);
        }

        host = argv[3];
    }
    else
    {
        printf("Usage: client [-f filename] <server name>\n");
        exit(1);
    }

    char buffer[MAX_INPUT_LENGTH];
    char *token[MAX_INPUT_LENGTH];
    printf("(client) ");
    while (fgets(buffer, MAX_INPUT_LENGTH, input))
    {
        char *copy = strdup(buffer);
        char *ptr = strtok(copy, " \r\n");
        while (ptr != NULL)
        {
            if (strncmp(ptr, "quit", 4) == 0)
                exit(0);
            else if (strncmp(ptr, "getdir", 6) == 0)
            {
                getdir_prog_1(host);
            }
            else if (strncmp(ptr, "cd", 2) == 0)
            {
                if (ptr = strtok(NULL, " \r\n"))
                {
                    cd_prog_1(host, ptr);
                }
                else
                {
                    printf("invalid path\n");
                    break;
                }
            }
            else if (strncmp(ptr, "filecount", 9) == 0)
            {
                ptr = strtok(NULL, " \r\n");
                filecount_prog_1(host, ptr);
            }
            else if (strncmp(ptr, "ls", 2) == 0)
            {
                if (ptr = strtok(NULL, " \r\n"))
                {
                    char *p;
                    if (strncmp(ptr, "-l", 2) == 0)
                    {
                        if (ptr = strtok(NULL, " \r\n"))
                        {
                            if (isfilter(ptr))
                                ls_prog_1(host, ".", TRUE, ptr);
                            else
                            {
                                p = strdup(ptr);
                                if (ptr = strtok(NULL, " \r\n"))
                                    ls_prog_1(host, p, TRUE, ptr);
                                else
                                    ls_prog_1(host, p, TRUE, NULL);

                                free(p);
                            }
                        }
                        else
                            ls_prog_1(host, ".", TRUE, NULL);
                    }
                    else if (isfilter(ptr))
                    {
                        ls_prog_1(host, ".", FALSE, ptr);
                    }
                    else
                    {
                        p = strdup(ptr);
                        if (ptr = strtok(NULL, " \n"))
                            ls_prog_1(host, p, FALSE, ptr);
                        else
                            ls_prog_1(host, p, FALSE, NULL);
                    }
                }
                else
                    ls_prog_1(host, ".", FALSE, NULL);
            }
            else if (strncmp(ptr, "put", 3) == 0)
            {
                if (ptr = strtok(NULL, " \r\n"))
                {
                    char *lfile = strdup(ptr);
                    ptr = strtok(NULL, " \r\n");

                    // ptr is NULL if second argument 'remotefile' is omitted
                    put_1(host, lfile, ptr);
                    free(lfile);
                }
                else
                    printf("usage: put <localfile> [remotefile]\n");
            }
            else if (strncmp(ptr, "get", 3) == 0)
            {
                if (ptr = strtok(NULL, " \r\n"))
                {
                    char *rfile = strdup(ptr);
                    ptr = strtok(NULL, " \r\n");

                    // ptr is NULL if second argument 'localfile' is omitted
                    get_1(host, rfile, ptr);
                    free(rfile);
                }
                else
                    printf("usage: get <remotefile> [localfile]\n");
            }
            else if (strncmp(ptr, "randomread", 10) == 0)
            {
                if (ptr = strtok(NULL, " \r\n"))
                {
                    char *rfile = strdup(ptr);

                    if (ptr = strtok(NULL, " \r\n"))
                    {
                        char *fbyte = strdup(ptr);

                        if (ptr = strtok(NULL, " \r\n"))
                            randomread_1(host, rfile, fbyte, ptr);
                        else
                            printf("usage: randomread <remotefile> <firstbyte> <numbytes>\n");

                        free(fbyte);
                    }
                    else
                        printf("usage: randomread <remotefile> <firstbyte> <numbytes>\n");

                    free(rfile);
                }
                else
                    printf("usage: randomread <remotefile> <firstbyte> <numbytes>\n");
            }
            else
            {
                printf("invalid command '%s'\n", ptr);
                printf("available command: getdir; cd; filecount; ls; put; get; randomread\n");
                printf("type 'quit' to quit\n");
                break;
            }

            ptr = strtok(NULL, " \r\n");
        }

        printf("(client) ");
    }

    printf("\n");
    return 0;
}

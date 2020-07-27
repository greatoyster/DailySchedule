# CS290K_Project
This is a group project for CS290K Advanced Distributed Systems at Shanghaitech University in Summer 2019.

## Group Member
- chibinz: 张驰斌
- greatoyster: 杨家琪

## Contribution
- chibinz:
    - filecount
    - ls
    - rpc related stuff, porting from local to remote(operation.x)
    - filtering

- greatoyster:
    - getdir
    - cd
    - put
    - get
    - random read

## Installation
```
cd remote
make

# before you start, check if rpcbind is working
sudo rpcinfo

# launch the server in one terminal 
sudo ./server

# launch the client in another terminal
sudo ./client localhost
```

## Documentation

### main (operation_client.c)
The main function first sets the input file stream if the `-f` flag is declared, otherwise
uses STDIN as the default input file stream.
Then it open a prompt for the user to type in commands. If the user types invalid command,
it will show a list of commands available. The commands are tokenized using `strtok()` and
parsed using string comparison, `strncmp`. The corresponding functions are called and relevant
arguments are passed to it.

### filecount
- Usage
filecount [condition]
- Function Declaration
`int filecount(char *condition);`
The function behaves exactly as project requirements demanded except for an
additional parameter, condition, which is used for filtering. An example of
condition would be "type=d", a null terminated C string.
- Dependencies
The function uses `opendir(), readdir(), closedir()` from the `<dirent.h>`
header file as well as the self implemented function filter() included in
`"filter.h"`.
- Implementation
The function attempts to initiate a pointer pointing to the current directory
`dirptr`, if unsuccessful returns OPEN_DIRECTORY_FAILED(-1).
If the `opendir()` succeeds, the function attempts to iterate through the list of
files pointed by `dirptr`, count is incremented iff `filter()` returned true.

### ls
- Usage
ls [-l] [path_to_file] [condition]
The order of the arguments is explicit, shuffling the order of the arguments
will cause the program to misinterpret the input.
- Function Declaration
```
bool_t list(char *, bool_t _l, char *);
bool_t openlist(char *directory);
direntry_t *nextlist(char *condition);
```
The function behaves exactly as project requirements demanded except for the
function `nextlist()` which has an additional parameter, condition, which is used 
for filtering. An example of condition would be "type=d", a null terminated C string.
- Custom Structure
```c
typedef struct
{
    char name[MAX_NAME_LENGTH];
    bool_t isDirectory;
    size_t size;
    time_t lastModified;
    bool_t filtered;
}
direntry_t;
```
The last attribute `filtered` is to be set for the client program to decide whether
or not the file is filtered and to be printed. The author attempted to return a
NULL pointer to the structure from the server program to the client program but it
caused undefined behaviors, that is, the pointer returned is not NULL and points to
somewhere in the memory. Thus, author added an additional attribute which works but
is somewhat clumsy.

- Dependencies
`<time.h>` for formatting `time_t` into human readable time strings using `ctime()`,
`<dirent.h>` for functions `opendir(), readdir(), closedir()`,
`<sys/stat.h>` for reading last modified time, file type, 
`<rpc/types.h>` for type `bool_t`,
`"getdir.h"` for constant MAX_PATH_LENGTH,
`"filter.h"` for function `filter()`.
- Implementation
`openlist()` first attempt to call `opendir()` and store the pointer in the global 
variable `dirptr`. Return TRUE on success else FALSE.
`nextlist()` iterate through the list of files and sets of attribute of `direntry_t`
of each file accordingly. finally, when `readdir()` reaches the end of the list of
files, it returns NULL. However, as mentioned before it is impossible to pass NULL
pointers between servers and clients directly. As an Alternative, the server sets
`name` attribute of the last file to NULL to notify the client to call `closelist`
`closelist()` wrapper around `closedir()`, return TRUE on success else FALSE.

### filter
- Function Declaration
`bool_t filter(char *path, char *condition);`
The function takes 2 parameters, `path` for the specific file you want to check its
validity against `condition`.
`bool_t isfilter(char *ptr);`
Check if a certain string is a filter condition.
`int evalage(char *);`
Convert custom age such "1d" to number of seconds.
- Dependencies
`<sys/stat.h>` for reading last modified time, file type and size,
`<rpc/types.h>` for type `bool_t`,
`<string.h>` to identify substrings, `strchr()`.
- Implementation
`filter()` uses stat to get the files' size, last modified time, and type,
`isfilter()` basically checks if conditions contain operands "<>=",
`evalage()` divides custom ages into 2 parts, the magnitude(or number) and the unit.
Afterwards, it multiplies them together to get `age` in seconds.

###  getdir

* Usage

  getdir

* Function Declaration

  `char* getdir(void);`

* Dependency

  `<rpc/types.h>` provides type `bool_t`.

  `<getdir.h>` for the `getdir()` function.

  `<unistd.h>` for the `getcwd()` function.

  `<stdio.h>` provides `printf()`

* Implementation

  To get the current directory, it uses function `getcwd()`, and it will print the result to console.

###  cd

- Usage

  cd pathname

  `pathname` is the path of the directory. 

- Function Declaration

  `bool_t changedir(char* str)`

- Dependency

  `<rpc/types.h>` provides type `bool_t`.

  `<unistd.h>` provides function `chdir()`.

  `<stdio.h>` provides `printf()`.

- Implementation

  To implement change directory, `chdir()` is used. The function will return True if it changes directory successfully. Otherwise, it will send message to the console to tell the user that it fails. 

###  put

- Usage

  put localfile [remotefile]

  `localfile` is the path of the local file.

  `remotefile` is the path of the remote file, and it is optional.

- Function Declaration

  `bool_t openfiletowrite(char* str)`

  `bool_t nextwrite(struct BLOCK_T)`

  `bool_t closefile(void)`

- Dependency

  `<stdio.h>` is used to operate file stream.

  `<rpc/types.h>` provides type `bool_t`.

  `<operation.h>` provides `struck BLOCK_T`

  `<string.h>` provides `memset()` to initialize the buffer.

- Custom structure

  - The definition in `XDR`

  ```C
  struct BLOCK_T
  {
  	opaque data<MAX_BLOCK_SIZE>;
  }; 
  ```

  * The definition in `<operation.h>`

  ```C
  struct BLOCK_T
  {
    struct{
  		u_int data_len;
          char *data_val;      
    }data;
  };
  ```

- Implementation

  client will first invoke `openfiletowrite()` to open the remote file to write using `fopen()`. If server can not open remote file, this function return False, and the client will abort invocation.

  If server opens remote file successfully, client will open local file using `fopen()`, which is ready to be written to the remote. Client uses `fread()` to load at most 512 bytes to the buffer, then pass buffer to server. 

  Server puts the buffer into file using `fwrite()`. If it receives a full buffer (512 bytes), `nextwrite()` will return FALSE to indicate that local file still has bytes left. Otherwise, `nextwrite()` will return TRUE.  

  If all the bytes in local file have been transferred, client will invoke `closefile()` to close remote file using `fclose()`.

  During the whole process, if any exception occurs, invocation will be aborted and related messages will be sent to the client.   

###  get

- Usage

  get remotefile [localfile]

  `remotefile` is the path of remote file

  `localfile` is the path of the local file, and it is optional

- Function Declaration

  `bool_t openfiletoread(char* str)`

  `struct BLOCK_T nextread(void)`

  `bool_t closefile(void)`

- Dependency

  `<stdio.h>` is used to operate file stream.

  `<rpc/types.h>` provides type `bool_t`.

  `<operation.h>` provides `struck BLOCK_T`.

  `<string.h>` provides `memset()` to initialize the buffer.

- Custom structure

  - The definition in `XDR`

  ```C
  struct BLOCK_T
  {
  	opaque data<MAX_BLOCK_SIZE>;
  }; 
  ```

  * The definition in `<operation.h>`

  ```C
  struct BLOCK_T
  {
    struct{
  		u_int data_len;
          char *data_val;      
    }data;
  };
  ```

- Implementation

  Client will first open remote file to read by invoking `openfiletoread()` using `fopen()`. And it will invoke `nextread()` later if the return value of  `openfiletoread()` is TRUE.

  Then server uses `fread()` to load buffer with at most 512 bytes. 

  The client will receive the buffer to server and put them into local file using `fwrite()`. Client will keep invoking `nextread()` until the return buffer has less than 512 bytes.

  After reading all the remote file, client will invoke `closefile()` to close remote file.

  During whole process, if any exception occurs, the invocation will be aborted, and related messages will be sent to the client.

###  randomread

- Usage

  randomread remotefile firstbyte numbytes

  `remotefile` is the path of the remote file

  `firstbyte` is the position of the first byte, start from 0.

  `numbytes` is the number of the bytes, and it is at most 512.

- Function Declaration

  `bool_t openfiletoread(char* str)`

  `struct BLOCK_T randomread(int firstbyte,int numbytes)`

  `bool_t closefile(void)`

- Dependency

  `<rpc/types.h>` provides type `bool_t`

  `<operation.h>` provides type `struct BLOCK_T` and `struct RANDOMINFO_T `

  `<stdio.h>` provides `fopen()`, `printf()` ,  `fclose()`

  `<string.h>` provides `memset()` to initialize the buffer.
  
- Custom structure

  ```C
  struct RANDOMINFO_T
  {
      int firstbyte;
      int bytenum;
  } ;
  ```

- Implementation

  The client will first invoke `openfiletoread()` to open local file after passing necessary arguments to the server and invoke `randomread()` later if the return value of `openfiletoread()` is TRUE.

  Then server reads file in random access according to arguments `firstbyte` and `numbytes`. The bytes it read will be loaded to the buffer and the buffer will be sent to the client.

  The client will print the content in the buffer with format string `%c` and related messages to the console. Meanwhile, the client will invoke `closefile()` to close remote file in the server.   

  During whole process, if any exception occurs, the invocation will be aborted, and related messages will be sent to the client.



## License

MIT


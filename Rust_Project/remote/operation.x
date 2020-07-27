const MAX_NAME_LENGTH = 0x100;
const MAX_INPUT_LENGTH = 0x100;
const MAX_BLOCK_SIZE = 0x200;

struct DIRENTRY_T
{
    string name<MAX_NAME_LENGTH>;
    bool isDirectory;
    int size;
    int lastModified;
    bool filtered;
};

struct RANDOMINFO_T
{
    int firstbyte;
    int bytenum;
} ;

struct BLOCK_T
{
    opaque data<MAX_BLOCK_SIZE>;
};

program GETDIR_PROG
{
    version VES
    {
        string GETDIR(void) = 1;
    } = 1;
} = 0x20000001;

program CD_PROG
{
    version VES
    {
        bool CHANGEDIR(string) = 1;
    } = 1;
} = 0x20000002;

program FILECOUNT_PROG
{
    version VES
    {
        int FILECOUNT(string) = 1;
    } = 1;
} = 0x20000003;

program LS_PROG
{
    version VES
    {
        bool OPENLIST(string) = 1;
        struct DIRENTRY_T NEXTLIST(string) = 2;
        bool CLOSELIST(void) = 3;
    } = 1;
} = 0x20000004;

program PUT
{ 
    version VES
    {
        bool openfiletowrite(string) = 1;
        bool put_nextwrite(struct BLOCK_T) = 2;
        bool put_closefile(void) = 3;
    } = 1;
} = 0x20000005;

program GET
{ 
    version VES
    {
        bool get_openfiletoread(string) = 1;
        struct BLOCK_T get_nextread(void) = 2;
        bool get_closefile(void) = 3;
    } = 1;
} = 0x20000006;

program RANDOMREAD
{ 
    version VES
    {
        bool rand_openfiletoread(string) = 1;
        struct BLOCK_T rand_nextread(struct RANDOMINFO_T) = 2;
        bool rand_closefile(void) = 3;
    } = 1;
} = 0x20000007;

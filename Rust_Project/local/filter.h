#ifndef FILTER
#define FILTER

#define EQUAL (0)
#define SMALLER (-1)

#define D 0
#define F 1

typedef struct
{
    int sizeFilterType;
    size_t size;
    int ageFilterType;
    char *age;
    int type;
}
filter_t;

#endif
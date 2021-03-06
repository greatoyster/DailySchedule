/*
 * Please do not edit this file.
 * It was generated using rpcgen.
 */

#ifndef _OPERATION_H_RPCGEN
#define _OPERATION_H_RPCGEN

#include <rpc/rpc.h>


#ifdef __cplusplus
extern "C" {
#endif

#define MAX_NAME_LENGTH 0x100
#define MAX_INPUT_LENGTH 0x100
#define MAX_BLOCK_SIZE 0x200

struct DIRENTRY_T {
	char *name;
	bool_t isDirectory;
	int size;
	int lastModified;
	bool_t filtered;
};
typedef struct DIRENTRY_T DIRENTRY_T;

struct RANDOMINFO_T {
	int firstbyte;
	int bytenum;
};
typedef struct RANDOMINFO_T RANDOMINFO_T;

struct BLOCK_T {
	struct {
		u_int data_len;
		char *data_val;
	} data;
};
typedef struct BLOCK_T BLOCK_T;

#define GETDIR_PROG 0x20000001
#define VES 1

#if defined(__STDC__) || defined(__cplusplus)
#define GETDIR 1
extern  char ** getdir_1(void *, CLIENT *);
extern  char ** getdir_1_svc(void *, struct svc_req *);
extern int getdir_prog_1_freeresult (SVCXPRT *, xdrproc_t, caddr_t);

#else /* K&R C */
#define GETDIR 1
extern  char ** getdir_1();
extern  char ** getdir_1_svc();
extern int getdir_prog_1_freeresult ();
#endif /* K&R C */

#define CD_PROG 0x20000002
#define VES 1

#if defined(__STDC__) || defined(__cplusplus)
#define CHANGEDIR 1
extern  bool_t * changedir_1(char **, CLIENT *);
extern  bool_t * changedir_1_svc(char **, struct svc_req *);
extern int cd_prog_1_freeresult (SVCXPRT *, xdrproc_t, caddr_t);

#else /* K&R C */
#define CHANGEDIR 1
extern  bool_t * changedir_1();
extern  bool_t * changedir_1_svc();
extern int cd_prog_1_freeresult ();
#endif /* K&R C */

#define FILECOUNT_PROG 0x20000003
#define VES 1

#if defined(__STDC__) || defined(__cplusplus)
#define FILECOUNT 1
extern  int * filecount_1(char **, CLIENT *);
extern  int * filecount_1_svc(char **, struct svc_req *);
extern int filecount_prog_1_freeresult (SVCXPRT *, xdrproc_t, caddr_t);

#else /* K&R C */
#define FILECOUNT 1
extern  int * filecount_1();
extern  int * filecount_1_svc();
extern int filecount_prog_1_freeresult ();
#endif /* K&R C */

#define LS_PROG 0x20000004
#define VES 1

#if defined(__STDC__) || defined(__cplusplus)
#define OPENLIST 1
extern  bool_t * openlist_1(char **, CLIENT *);
extern  bool_t * openlist_1_svc(char **, struct svc_req *);
#define NEXTLIST 2
extern  struct DIRENTRY_T * nextlist_1(char **, CLIENT *);
extern  struct DIRENTRY_T * nextlist_1_svc(char **, struct svc_req *);
#define CLOSELIST 3
extern  bool_t * closelist_1(void *, CLIENT *);
extern  bool_t * closelist_1_svc(void *, struct svc_req *);
extern int ls_prog_1_freeresult (SVCXPRT *, xdrproc_t, caddr_t);

#else /* K&R C */
#define OPENLIST 1
extern  bool_t * openlist_1();
extern  bool_t * openlist_1_svc();
#define NEXTLIST 2
extern  struct DIRENTRY_T * nextlist_1();
extern  struct DIRENTRY_T * nextlist_1_svc();
#define CLOSELIST 3
extern  bool_t * closelist_1();
extern  bool_t * closelist_1_svc();
extern int ls_prog_1_freeresult ();
#endif /* K&R C */

#define PUT 0x20000005
#define VES 1

#if defined(__STDC__) || defined(__cplusplus)
#define openfiletowrite 1
extern  bool_t * openfiletowrite_1(char **, CLIENT *);
extern  bool_t * openfiletowrite_1_svc(char **, struct svc_req *);
#define put_nextwrite 2
extern  bool_t * put_nextwrite_1(struct BLOCK_T *, CLIENT *);
extern  bool_t * put_nextwrite_1_svc(struct BLOCK_T *, struct svc_req *);
#define put_closefile 3
extern  bool_t * put_closefile_1(void *, CLIENT *);
extern  bool_t * put_closefile_1_svc(void *, struct svc_req *);
extern int put_1_freeresult (SVCXPRT *, xdrproc_t, caddr_t);

#else /* K&R C */
#define openfiletowrite 1
extern  bool_t * openfiletowrite_1();
extern  bool_t * openfiletowrite_1_svc();
#define put_nextwrite 2
extern  bool_t * put_nextwrite_1();
extern  bool_t * put_nextwrite_1_svc();
#define put_closefile 3
extern  bool_t * put_closefile_1();
extern  bool_t * put_closefile_1_svc();
extern int put_1_freeresult ();
#endif /* K&R C */

#define GET 0x20000006
#define VES 1

#if defined(__STDC__) || defined(__cplusplus)
#define get_openfiletoread 1
extern  bool_t * get_openfiletoread_1(char **, CLIENT *);
extern  bool_t * get_openfiletoread_1_svc(char **, struct svc_req *);
#define get_nextread 2
extern  struct BLOCK_T * get_nextread_1(void *, CLIENT *);
extern  struct BLOCK_T * get_nextread_1_svc(void *, struct svc_req *);
#define get_closefile 3
extern  bool_t * get_closefile_1(void *, CLIENT *);
extern  bool_t * get_closefile_1_svc(void *, struct svc_req *);
extern int get_1_freeresult (SVCXPRT *, xdrproc_t, caddr_t);

#else /* K&R C */
#define get_openfiletoread 1
extern  bool_t * get_openfiletoread_1();
extern  bool_t * get_openfiletoread_1_svc();
#define get_nextread 2
extern  struct BLOCK_T * get_nextread_1();
extern  struct BLOCK_T * get_nextread_1_svc();
#define get_closefile 3
extern  bool_t * get_closefile_1();
extern  bool_t * get_closefile_1_svc();
extern int get_1_freeresult ();
#endif /* K&R C */

#define RANDOMREAD 0x20000007
#define VES 1

#if defined(__STDC__) || defined(__cplusplus)
#define rand_openfiletoread 1
extern  bool_t * rand_openfiletoread_1(char **, CLIENT *);
extern  bool_t * rand_openfiletoread_1_svc(char **, struct svc_req *);
#define rand_nextread 2
extern  struct BLOCK_T * rand_nextread_1(struct RANDOMINFO_T *, CLIENT *);
extern  struct BLOCK_T * rand_nextread_1_svc(struct RANDOMINFO_T *, struct svc_req *);
#define rand_closefile 3
extern  bool_t * rand_closefile_1(void *, CLIENT *);
extern  bool_t * rand_closefile_1_svc(void *, struct svc_req *);
extern int randomread_1_freeresult (SVCXPRT *, xdrproc_t, caddr_t);

#else /* K&R C */
#define rand_openfiletoread 1
extern  bool_t * rand_openfiletoread_1();
extern  bool_t * rand_openfiletoread_1_svc();
#define rand_nextread 2
extern  struct BLOCK_T * rand_nextread_1();
extern  struct BLOCK_T * rand_nextread_1_svc();
#define rand_closefile 3
extern  bool_t * rand_closefile_1();
extern  bool_t * rand_closefile_1_svc();
extern int randomread_1_freeresult ();
#endif /* K&R C */

/* the xdr functions */

#if defined(__STDC__) || defined(__cplusplus)
extern  bool_t xdr_DIRENTRY_T (XDR *, DIRENTRY_T*);
extern  bool_t xdr_RANDOMINFO_T (XDR *, RANDOMINFO_T*);
extern  bool_t xdr_BLOCK_T (XDR *, BLOCK_T*);

#else /* K&R C */
extern bool_t xdr_DIRENTRY_T ();
extern bool_t xdr_RANDOMINFO_T ();
extern bool_t xdr_BLOCK_T ();

#endif /* K&R C */

#ifdef __cplusplus
}
#endif

#endif /* !_OPERATION_H_RPCGEN */

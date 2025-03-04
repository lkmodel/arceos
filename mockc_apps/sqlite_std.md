```bash
gcc -E sqlite3.c -o sqlite3.i
grep -Eo '\b[a-zA-Z_][a-zA-Z0-9_]*\s*\(' sqlite3.i | sed -E 's/\s*\(//' | sort -u > project_calls
ctags --fields=+iaS --extras=+q --c-kinds=+p -R  arceos/xtask/riscv64-linux-musl-cross/riscv64-linux-musl/include
cat tags | sort | awk '{print $1}' > std_functions.txt
grep -Fxf project_calls std_functions > used_std_functions
```

---

- [x] a64l
- [ ] abort
- [x] abs
- [ ] access
- [ ] acct
- [x] acos
- [ ] acosf
- [x] acosh
- [ ] acoshf
- [ ] acoshl
- [ ] acosl
- [ ] adjtime
- [ ] alarm
- [ ] aligned_alloc
- [ ] alloca
- [ ] asctime
- [ ] asctime_r
- [x] asin
- [ ] asinf
- [x] asinh
- [ ] asinhf
- [ ] asinhl
- [ ] asinl
- [ ] asprintf
- [x] atan
- [x] atan2
- [ ] atan2f
- [ ] atan2l
- [ ] atanf
- [x] atanh
- [ ] atanhf
- [ ] atanhl
- [ ] atanl
- [ ] atexit
- [ ] atof
- [ ] atoi
- [ ] atol
- [ ] atoll
- [ ] at_quick_exit
- [x] basename
- [ ] bcmp
- [ ] bcopy
- [ ] brk
- [ ] bsearch
- [ ] bzero
- [ ] calloc
- [x] cbrt
- [ ] cbrtf
- [ ] cbrtl
- [x] ceil
- [ ] ceilf
- [ ] ceill
- [ ] changes
- [x] chdir
- [ ] chmod
- [ ] chown
- [ ] chroot
- [ ] clearenv
- [ ] clearerr
- [ ] clearerr_unlocked
- [ ] clock
- [ ] clock_adjtime
- [ ] clock_getcpuclockid
- [ ] clock_getres
- [ ] clock_gettime
- [ ] clock_nanosleep
- [ ] clock_settime
- [ ] clone
- [ ] close
- [ ] confstr
- [ ] cookie_close_function_t
- [ ] cookie_read_function_t
- [ ] cookie_seek_function_t
- [ ] cookie_write_function_t
- [ ] copy_file_range
- [x] copysign
- [ ] copysignf
- [ ] copysignl
- [x] cos
- [ ] cosf
- [x] cosh
- [ ] coshf
- [ ] coshl
- [ ] cosl
- [ ] creat
- [ ] creat64
- [ ] crypt
- [ ] ctermid
- [ ] ctime
- [ ] ctime_r
- [ ] cuserid
- [ ] daemon
- [ ] difftime
- [ ] div
- [ ] dladdr
- [ ] dlclose
- [ ] dlerror
- [ ] dlinfo
- [ ] dlopen
- [ ] dlsym
- [ ] dprintf
- [ ] drand48
- [ ] drem
- [ ] dremf
- [x] dup
- [ ] dup2
- [x] dup3
- [ ] eaccess
- [ ] ecvt
- [ ] endusershell
- [ ] erand48
- [x] erf
- [x] erfc
- [ ] erfcf
- [ ] erfcl
- [ ] erff
- [ ] erfl
- [ ] euidaccess
- [ ] execl
- [ ] execle
- [ ] execlp
- [ ] execv
- [ ] execve
- [ ] execvp
- [ ] execvpe
- [ ] exit
- [x] exp
- [ ] exp10
- [ ] exp10f
- [ ] exp10l
- [x] exp2
- [ ] exp2f
- [ ] exp2l
- [ ] expf
- [ ] expl
- [ ] explicit_bzero
- [x] expm1
- [ ] expm1f
- [ ] expm1l
- [ ] fabs
- [ ] fabsf
- [ ] fabsl
- [ ] faccessat
- [ ] fadd
- [ ] fallocate
- [ ] fallocate64
- [ ] fchdir
- [ ] fchmod
- [ ] fchmodat
- [ ] fchown
- [ ] fchownat
- [ ] fclose
- [ ] fcntl
- [ ] fcvt
- [ ] fdatasync
- [ ] fdim
- [ ] fdimf
- [ ] fdiml
- [ ] fdopen
- [ ] feof
- [ ] feof_unlocked
- [ ] ferror
- [ ] ferror_unlocked
- [ ] fexecve
- [ ] fflush
- [ ] fflush_unlocked
- [ ] ffs
- [ ] ffsl
- [ ] ffsll
- [ ] fgetc
- [ ] fgetc_unlocked
- [ ] fgetpos
- [ ] fgetpos64
- [ ] fgets
- [ ] fgets_unlocked
- [ ] fileno
- [ ] fileno_unlocked
- [ ] finite
- [ ] finitef
- [ ] finitel
- [ ] flockfile
- [ ] floor
- [ ] floorf
- [ ] floorl
- [ ] fma
- [ ] fmaf
- [ ] fmal
- [ ] fmax
- [ ] fmaxf
- [ ] fmaxl
- [ ] fmemopen
- [ ] fmin
- [ ] fminf
- [ ] fminl
- [ ] fmod
- [ ] fmodf
- [ ] fmodl
- [ ] fopen
- [ ] fopen64
- [ ] fopencookie
- [ ] fork
- [ ] fpathconf
- [ ] fprintf
- [ ] fputc
- [ ] fputc_unlocked
- [ ] fputs
- [ ] fputs_unlocked
- [ ] fread
- [ ] fread_unlocked
- [ ] free
- [ ] freopen
- [ ] freopen64
- [ ] frexp
- [ ] frexpf
- [ ] frexpl
- [ ] fscanf
- [ ] fseek
- [ ] fseeko
- [ ] fseeko64
- [ ] fsetpos
- [ ] fsetpos64
- [ ] fstat
- [ ] fstat64
- [ ] fstatat
- [ ] fstatat64
- [ ] fsync
- [ ] ftell
- [ ] ftello
- [ ] ftello64
- [ ] ftruncate
- [ ] ftruncate64
- [ ] ftrylockfile
- [ ] funlockfile
- [ ] futimens
- [ ] futimes
- [ ] futimesat
- [ ] fwrite
- [ ] fwrite_unlocked
- [ ] gamma
- [ ] gcvt
- [ ] getc
- [ ] getchar
- [ ] getchar_unlocked
- [ ] getc_unlocked
- [ ] get_current_dir_name
- [ ] getcwd
- [ ] getdate
- [ ] getdelim
- [ ] getdomainname
- [ ] getdtablesize
- [ ] getegid
- [ ] getentropy
- [ ] getenv
- [ ] geteuid
- [ ] getgid
- [ ] getgroups
- [ ] gethostid
- [ ] gethostname
- [ ] getitimer
- [ ] getline
- [ ] getloadavg
- [ ] getlogin
- [ ] getlogin_r
- [ ] getopt
- [ ] getpagesize
- [ ] getpass
- [ ] getpgid
- [ ] getpgrp
- [ ] getpid
- [ ] getppid
- [ ] getresgid
- [ ] getresuid
- [ ] getsid
- [ ] getsubopt
- [ ] gettid
- [ ] gettimeofday
- [ ] getuid
- [ ] getusershell
- [ ] getw
- [ ] gmtime
- [ ] gmtime_r
- [ ] grantpt
- [ ] header
- [ ] hypot
- [ ] hypotf
- [ ] hypotl
- [ ] ilogb
- [ ] ilogbf
- [ ] ilogbl
- [ ] index
- [ ] initstate
- [ ] ioctl
- [ ] isalnum
- [ ] isalnum_l
- [ ] isalpha
- [ ] isalpha_l
- [ ] isascii
- [ ] isatty
- [ ] isblank
- [ ] isblank_l
- [ ] iscntrl
- [ ] iscntrl_l
- [ ] isctype
- [ ] isdigit
- [ ] isdigit_l
- [ ] isgraph
- [ ] isgraph_l
- [ ] isinf
- [ ] isinff
- [ ] isinfl
- [ ] islower
- [ ] islower_l
- [ ] isnan
- [ ] isnanf
- [ ] isnanl
- [ ] isprint
- [ ] isprint_l
- [ ] ispunct
- [ ] ispunct_l
- [ ] isspace
- [ ] isspace_l
- [ ] isupper
- [ ] isupper_l
- [ ] isxdigit
- [ ] isxdigit_l
- [ ] j0
- [ ] j0f
- [ ] j1
- [ ] j1f
- [ ] jn
- [ ] jnf
- [ ] jrand48
- [ ] k
- [ ] KEY
- [ ] l64a
- [ ] labs
- [ ] lchmod
- [ ] lchown
- [ ] lcong48
- [ ] ldexp
- [ ] ldexpf
- [ ] ldexpl
- [ ] ldiv
- [ ] length
- [ ] lgamma
- [ ] lgammaf
- [ ] lgammaf_r
- [ ] lgammal
- [ ] lgammal_r
- [ ] lgamma_r
- [ ] link
- [ ] linkat
- [ ] llabs
- [ ] lldiv
- [ ] llrint
- [ ] llrintf
- [ ] llrintl
- [ ] llround
- [ ] llroundf
- [ ] llroundl
- [ ] localtime
- [ ] localtime_r
- [ ] lockf
- [ ] lockf64
- [ ] log
- [ ] log10
- [ ] log10f
- [ ] log10l
- [ ] log1p
- [ ] log1pf
- [ ] log1pl
- [ ] log2
- [ ] log2f
- [ ] log2l
- [ ] logb
- [ ] logbf
- [ ] logbl
- [ ] logf
- [ ] logl
- [ ] lrand48
- [ ] lrint
- [ ] lrintf
- [ ] lrintl
- [ ] lround
- [ ] lroundf
- [ ] lroundl
- [ ] lseek
- [ ] lseek64
- [ ] lstat
- [ ] lstat64
- [ ] lutimes
- [ ] madvise
- [ ] malloc
- [ ] mblen
- [ ] mbstowcs
- [ ] mbtowc
- [ ] memccpy
- [ ] memchr
- [ ] memcmp
- [ ] memcpy
- [ ] memfd_create
- [ ] memmem
- [ ] memmove
- [ ] mempcpy
- [ ] memrchr
- [ ] memset
- [ ] mincore
- [ ] mkdir
- [ ] mkdirat
- [ ] mkdtemp
- [ ] mkfifo
- [ ] mkfifoat
- [ ] mknod
- [ ] mknodat
- [ ] mkostemp
- [ ] mkostemp64
- [ ] mkostemps
- [ ] mkostemps64
- [ ] mkstemp
- [ ] mkstemp64
- [ ] mkstemps
- [ ] mkstemps64
- [ ] mktemp
- [ ] mktime
- [ ] mlock
- [ ] mlock2
- [ ] mlockall
- [ ] mmap
- [ ] mmap64
- [ ] modf
- [ ] modff
- [ ] modfl
- [ ] mprotect
- [ ] mrand48
- [ ] mremap
- [ ] msync
- [ ] munlock
- [ ] munlockall
- [ ] munmap
- [ ] name_to_handle_at
- [ ] nan
- [ ] nanf
- [ ] nanl
- [ ] nanosleep
- [ ] nearbyint
- [ ] nearbyintf
- [ ] nearbyintl
- [ ] nextafter
- [ ] nextafterf
- [ ] nextafterl
- [ ] nexttoward
- [ ] nexttowardf
- [ ] nexttowardl
- [ ] nice
- [ ] nrand48
- [ ] open
- [ ] open64
- [ ] openat
- [ ] openat64
- [ ] open_by_handle_at
- [ ] open_memstream
- [ ] pathconf
- [ ] pause
- [ ] pclose
- [ ] perror
- [ ] pipe
- [ ] pipe2
- [ ] popen
- [ ] posix_fadvise
- [ ] posix_fadvise64
- [ ] posix_fallocate
- [ ] posix_fallocate64
- [ ] posix_madvise
- [ ] posix_memalign
- [ ] posix_openpt
- [ ] pow
- [ ] powf
- [ ] powl
- [ ] pread
- [ ] pread64
- [ ] printf
- [ ] pselect
- [ ] pthread_atfork
- [ ] pthread_attr_destroy
- [ ] pthread_attr_getdetachstate
- [ ] pthread_attr_getguardsize
- [ ] pthread_attr_getinheritsched
- [ ] pthread_attr_getschedparam
- [ ] pthread_attr_getschedpolicy
- [ ] pthread_attr_getscope
- [ ] pthread_attr_getstack
- [ ] pthread_attr_getstacksize
- [ ] pthread_attr_init
- [ ] pthread_attr_setdetachstate
- [ ] pthread_attr_setguardsize
- [ ] pthread_attr_setinheritsched
- [ ] pthread_attr_setschedparam
- [ ] pthread_attr_setschedpolicy
- [ ] pthread_attr_setscope
- [ ] pthread_attr_setstack
- [ ] pthread_attr_setstacksize
- [ ] pthread_barrierattr_destroy
- [ ] pthread_barrierattr_getpshared
- [ ] pthread_barrierattr_init
- [ ] pthread_barrierattr_setpshared
- [ ] pthread_barrier_destroy
- [ ] pthread_barrier_init
- [ ] pthread_barrier_wait
- [ ] pthread_cancel
- [ ] pthread_condattr_destroy
- [ ] pthread_condattr_getclock
- [ ] pthread_condattr_getpshared
- [ ] pthread_condattr_init
- [ ] pthread_condattr_setclock
- [ ] pthread_condattr_setpshared
- [ ] pthread_cond_broadcast
- [ ] pthread_cond_destroy
- [ ] pthread_cond_init
- [ ] pthread_cond_signal
- [ ] pthread_cond_timedwait
- [ ] pthread_cond_wait
- [ ] pthread_create
- [ ] pthread_detach
- [ ] pthread_equal
- [ ] pthread_exit
- [ ] pthread_getaffinity_np
- [ ] pthread_getattr_default_np
- [ ] pthread_getattr_np
- [ ] pthread_getconcurrency
- [ ] pthread_getcpuclockid
- [ ] pthread_getname_np
- [ ] pthread_getschedparam
- [ ] pthread_getspecific
- [ ] pthread_join
- [ ] pthread_key_create
- [ ] pthread_key_delete
- [ ] pthread_mutexattr_destroy
- [ ] pthread_mutexattr_getprioceiling
- [ ] pthread_mutexattr_getprotocol
- [ ] pthread_mutexattr_getpshared
- [ ] pthread_mutexattr_getrobust
- [ ] pthread_mutexattr_gettype
- [ ] pthread_mutexattr_init
- [ ] pthread_mutexattr_setprioceiling
- [ ] pthread_mutexattr_setprotocol
- [ ] pthread_mutexattr_setpshared
- [ ] pthread_mutexattr_setrobust
- [ ] pthread_mutexattr_settype
- [ ] pthread_mutex_consistent
- [ ] pthread_mutex_destroy
- [ ] pthread_mutex_getprioceiling
- [ ] pthread_mutex_init
- [ ] pthread_mutex_lock
- [ ] pthread_mutex_setprioceiling
- [ ] pthread_mutex_timedlock
- [ ] pthread_mutex_trylock
- [ ] pthread_mutex_unlock
- [ ] pthread_once
- [ ] pthread_rwlockattr_destroy
- [ ] pthread_rwlockattr_getpshared
- [ ] pthread_rwlockattr_init
- [ ] pthread_rwlockattr_setpshared
- [ ] pthread_rwlock_destroy
- [ ] pthread_rwlock_init
- [ ] pthread_rwlock_rdlock
- [ ] pthread_rwlock_timedrdlock
- [ ] pthread_rwlock_timedwrlock
- [ ] pthread_rwlock_tryrdlock
- [ ] pthread_rwlock_trywrlock
- [ ] pthread_rwlock_unlock
- [ ] pthread_rwlock_wrlock
- [ ] pthread_self
- [ ] pthread_setaffinity_np
- [ ] pthread_setattr_default_np
- [ ] pthread_setcancelstate
- [ ] pthread_setcanceltype
- [ ] pthread_setconcurrency
- [ ] pthread_setname_np
- [ ] pthread_setschedparam
- [ ] pthread_setschedprio
- [ ] pthread_setspecific
- [ ] pthread_spin_destroy
- [ ] pthread_spin_init
- [ ] pthread_spin_lock
- [ ] pthread_spin_trylock
- [ ] pthread_spin_unlock
- [ ] pthread_testcancel
- [ ] pthread_timedjoin_np
- [ ] pthread_tryjoin_np
- [ ] ptsname
- [ ] ptsname_r
- [ ] putc
- [ ] putchar
- [ ] putchar_unlocked
- [ ] putc_unlocked
- [ ] putenv
- [ ] puts
- [ ] putw
- [ ] pwrite
- [ ] pwrite64
- [ ] qsort
- [ ] qsort_r
- [ ] quick_exit
- [ ] quote
- [ ] raise
- [ ] rand
- [ ] random
- [ ] rand_r
- [ ] read
- [ ] readahead
- [ ] readlink
- [ ] readlinkat
- [ ] realloc
- [ ] reallocarray
- [ ] realpath
- [ ] rehash
- [ ] remainder
- [ ] remainderf
- [ ] remainderl
- [ ] remap_file_pages
- [ ] remove
- [ ] remquo
- [ ] remquof
- [ ] remquol
- [ ] rename
- [ ] renameat
- [ ] rewind
- [ ] rindex
- [ ] rint
- [ ] rintf
- [ ] rintl
- [ ] rmdir
- [ ] round
- [ ] roundf
- [ ] roundl
- [ ] s
- [ ] sbrk
- [ ] scalb
- [ ] scalbf
- [ ] scalbln
- [ ] scalblnf
- [ ] scalblnl
- [ ] scalbn
- [ ] scalbnf
- [ ] scalbnl
- [ ] scanf
- [ ] sched_getaffinity
- [ ] sched_getcpu
- [ ] sched_getparam
- [ ] sched_get_priority_max
- [ ] sched_get_priority_min
- [ ] sched_getscheduler
- [ ] sched_rr_get_interval
- [ ] sched_setaffinity
- [ ] sched_setparam
- [ ] sched_setscheduler
- [ ] sched_yield
- [ ] secure_getenv
- [ ] seed48
- [ ] select
- [ ] setbuf
- [ ] setbuffer
- [ ] setdomainname
- [ ] setegid
- [ ] setenv
- [ ] seteuid
- [ ] setgid
- [ ] sethostname
- [ ] setitimer
- [ ] setlinebuf
- [ ] setns
- [ ] setpgid
- [ ] setpgrp
- [ ] setregid
- [ ] setresgid
- [ ] setresuid
- [ ] setreuid
- [ ] setsid
- [ ] setstate
- [ ] settimeofday
- [ ] setuid
- [ ] setusershell
- [ ] setvbuf
- [ ] shm_open
- [ ] shm_unlink
- [ ] significand
- [ ] significandf
- [ ] sin
- [ ] sincos
- [ ] sincosf
- [ ] sincosl
- [ ] sinf
- [ ] sinh
- [ ] sinhf
- [ ] sinhl
- [ ] sinl
- [ ] sleep
- [ ] snprintf
- [ ] splice
- [ ] sprintf
- [ ] sqrt
- [ ] sqrtf
- [ ] sqrtl
- [ ] srand
- [ ] srand48
- [ ] srandom
- [ ] sscanf
- [ ] ssize_t
- [ ] stat
- [ ] stat64
- [ ] statx
- [ ] stpcpy
- [ ] stpncpy
- [ ] strcasecmp
- [ ] strcasecmp_l
- [ ] strcasestr
- [ ] strcat
- [ ] strchr
- [ ] strchrnul
- [ ] strcmp
- [ ] strcoll
- [ ] strcoll_l
- [ ] strcpy
- [ ] strcspn
- [ ] strdup
- [ ] strerror
- [ ] strerror_l
- [ ] strerror_r
- [ ] strftime
- [ ] strftime_l
- [ ] strlcat
- [ ] strlcpy
- [ ] strlen
- [ ] strncasecmp
- [ ] strncasecmp_l
- [ ] strncat
- [ ] strncmp
- [ ] strncpy
- [ ] strndup
- [ ] strnlen
- [ ] strpbrk
- [ ] strptime
- [ ] strrchr
- [ ] strsep
- [ ] strsignal
- [ ] strspn
- [ ] strstr
- [ ] strtod
- [ ] strtod_l
- [ ] strtof
- [ ] strtof_l
- [ ] strtok
- [ ] strtok_r
- [ ] strtol
- [ ] strtold
- [ ] strtold_l
- [ ] strtoll
- [ ] strtoul
- [ ] strtoull
- [ ] strverscmp
- [ ] strxfrm
- [ ] strxfrm_l
- [ ] substr
- [ ] swab
- [ ] symlink
- [ ] symlinkat
- [ ] sync
- [ ] sync_file_range
- [ ] syncfs
- [ ] syscall
- [ ] sysconf
- [ ] system
- [ ] T
- [ ] tan
- [ ] tanf
- [ ] tanh
- [ ] tanhf
- [ ] tanhl
- [ ] tanl
- [ ] tcgetpgrp
- [ ] tcsetpgrp
- [ ] tee
- [ ] tempnam
- [ ] tgamma
- [ ] tgammaf
- [ ] tgammal
- [ ] time
- [ ] timegm
- [ ] timer_create
- [ ] timer_delete
- [ ] timer_getoverrun
- [ ] timer_gettime
- [ ] timer_settime
- [ ] timespec_get
- [ ] tmpfile
- [ ] tmpfile64
- [ ] tmpnam
- [ ] toascii
- [ ] tolower
- [ ] tolower_l
- [ ] toupper
- [ ] toupper_l
- [ ] trunc
- [ ] truncate
- [ ] truncate64
- [ ] truncf
- [ ] truncl
- [ ] ttyname
- [ ] ttyname_r
- [ ] tzset
- [ ] u16
- [ ] ualarm
- [ ] uid_t
- [ ] umask
- [ ] ungetc
- [ ] unlink
- [ ] unlinkat
- [ ] unlockpt
- [ ] unsetenv
- [ ] unshare
- [ ] usleep
- [ ] utimensat
- [ ] utimes
- [ ] valloc
- [ ] vasprintf
- [ ] vdprintf
- [ ] vfork
- [ ] vfprintf
- [ ] vfscanf
- [ ] vhangup
- [ ] vmsplice
- [ ] vprintf
- [ ] vscanf
- [ ] vsnprintf
- [ ] vsprintf
- [ ] vsscanf
- [ ] wcstombs
- [ ] wctomb
- [ ] write
- [ ] x
- [ ] y0
- [ ] y0f
- [ ] y1
- [ ] y1f
- [ ] yn
- [ ] ynf

---

### **`<stdlib.h>`**

- 内存管理、进程控制、数值转换等

a64l
abort
abs
aligned_alloc
at_quick_exit
atexit
atof
atoi
atol
atoll
bsearch
calloc
div
ecvt
exit
free
gcvt
getenv
l64a
labs
llabs
lldiv
mblen
mbstowcs
mbtowc
qsort
qsort_r
quick_exit
rand
rand_r
realloc
realpath
strtod
strtof
strtol
strtold
strtoll
strtoul
strtoull
system
wcstombs
wctomb

---

### **`<stdio.h>`**

- 输入/输出操作

asprintf
clearerr
clearerr_unlocked
fclose
fdopen
feof
feof_unlocked
ferror
ferror_unlocked
fflush
fflush_unlocked
fgetc
fgetc_unlocked
fgets
fgets_unlocked
fileno
fileno_unlocked
fopen
fopen64
fopencookie
fprintf
fputc
fputc_unlocked
fputs
fputs_unlocked
fread
fread_unlocked
freopen
freopen64
fscanf
fseek
fseeko
fsetpos
ftell
ftello
fwrite
fwrite_unlocked
getc
getchar
getchar_unlocked
getc_unlocked
getdelim
getline
perror
printf
putc
putchar
putchar_unlocked
putc_unlocked
puts
remove
rename
rewind
scanf
snprintf
sprintf
sscanf
tmpfile
tmpnam
vfprintf
vfscanf
vprintf
vsnprintf
vsprintf

---

### **`<string.h>`**

- 字符串和内存操作

bcmp
bcopy
bzero
explicit_bzero
ffs
ffsl
ffsll
memccpy
memchr
memcmp
memcpy
memmem
memmove
mempcpy
memrchr
memset
stpcpy
stpncpy
strcasecmp
strcat
strchr
strchrnul
strcmp
strcoll
strcpy
strcspn
strdup
strerror
strerror_r
strlcat
strlcpy
strlen
strncasecmp
strncat
strncmp
strncpy
strndup
strnlen
strpbrk
strrchr
strsep
strsignal
strspn
strstr
strtok
strtok_r
strverscmp
strxfrm

---

### **`<math.h>`**

- 数学函数（含C99/C11扩展）

acos
acosf
acosh
acoshf
acoshl
acosl
asin
asinf
asinh
asinhf
asinhl
asinl
atan
atan2
atan2f
atan2l
atanf
atanh
atanhf
atanhl
atanl
cbrt
cbrtf
cbrtl
ceil
ceilf
ceill
copysign
copysignf
copysignl
cos
cosf
cosh
coshf
coshl
cosl
erf
erfc
erfcf
erfcl
erff
erfl
exp
exp10
exp10f
exp10l
exp2
exp2f
exp2l
expf
expl
expm1
expm1f
expm1l
fabs
fabsf
fabsl
fdim
fdimf
fdiml
floor
floorf
floorl
fma
fmaf
fmal
fmax
fmaxf
fmaxl
fmin
fminf
fminl
fmod
fmodf
fmodl
frexp
frexpf
frexpl
hypot
hypotf
hypotl
ilogb
ilogbf
ilogbl
j0
j0f
j1
j1f
jn
jnf
ldexp
ldexpf
ldexpl
lgamma
lgammaf
lgammal
llrint
llrintf
llrintl
llround
llroundf
llroundl
log
log10
log10f
log10l
log1p
log1pf
log1pl
log2
log2f
log2l
logb
logbf
logbl
logf
logl
lrint
lrintf
lrintl
lround
lroundf
lroundl
modf
modff
modfl
nan
nanf
nanl
nearbyint
nearbyintf
nearbyintl
nextafter
nextafterf
nextafterl
nexttoward
nexttowardf
nexttowardl
pow
powf
powl
remainder
remainderf
remainderl
remquo
remquof
remquol
rint
rintf
rintl
round
roundf
roundl
scalb
scalbf
scalbln
scalblnf
scalblnl
scalbn
scalbnf
scalbnl
sin
sincos
sincosf
sincosl
sinf
sinh
sinhf
sinhl
sinl
sqrt
sqrtf
sqrtl
tan
tanf
tanh
tanhf
tanhl
tanl
tgamma
tgammaf
tgammal
trunc
truncf
truncl
y0
y0f
y1
y1f
yn
ynf

---

### **`<time.h>`**

- 时间处理

asctime
clock
clock_getres
clock_gettime
ctime
difftime
gmtime
gmtime_r
localtime
localtime_r
mktime
nanosleep
strftime
time
timespec_get

---

### **`<ctype.h>`**

- 字符分类和转换

isalnum
isalpha
isascii
isblank
iscntrl
isdigit
isgraph
islower
isprint
ispunct
isspace
isupper
isxdigit
toascii
tolower
toupper

---

### **`<signal.h>`**

- 信号处理

raise

---

### **POSIX/系统相关头文件**

- **`<unistd.h>`**（POSIX系统调用）
-
    access
    alarm
    chdir
    chown
    close
    confstr
    daemon
    dup
    dup2
    dup3
    execl
    execle
    execlp
    execv
    execve
    execvp
    execvpe
    fork
    ftruncate
    getcwd
    getegid
    geteuid
    getgid
    getgroups
    getpid
    getppid
    getuid
    isatty
    link
    lseek
    pause
    pipe
    pread
    read
    rmdir
    sleep
    sysconf
    truncate
    unlink
    usleep
    write

- **`<pthread.h>`**（POSIX线程）

    pthread_*（所有以`pthread_`开头的函数）

- **`<sys/stat.h>`**（文件状态）

    chmod
    fchmod
    fstat
    lstat
    mkdir
    umask

- **`<fcntl.h>`**（文件控制）

    creat
    open
    openat

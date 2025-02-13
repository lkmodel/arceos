# `openat`

## `open()`,`openat()`, and `creat()` can fail with the following errors

- [x] `EACCES` The requested access to the file is not allowed, or search permission
   is denied for one of the directories in the path prefix of `pathname`, or the
   file did not exist yet and write access to the parent directory is not allowed.
   (See also path_resolution(7).)

- [ ] `EACCES` Where `O_CREAT` is specified, the `protected_fifos` or `protected_regular`
   `sysctl` is enabled, the file already exists and is a FIFO or regular file,
   the owner of the file is neither the current user nor the owner of the
   containing directory, and the containing directory is both world- or group-
   writable and sticky.
   For details, see the descriptions of `/proc/sys/fs/protected_fifos` and
   `/proc/sys/fs/protected_regular` in `proc(5)`.

- [ ] EBUSY  O_EXCL was specified in flags and pathname refers to a block device that is in use by  the  system
           (e.g., it is mounted).

- [ ] EDQUOT Where O_CREAT is specified, the file does not exist, and the user's quota of disk blocks or inodes
           on the filesystem has been exhausted.

- [x] `EEXIST` `pathname` already exists and `O_CREAT` and `O_EXCL` were used.

- [x] `EFAULT` `pathname` points outside your accessible address space.

- [ ] EFBIG  See EOVERFLOW.

- [ ] EINTR  While blocked waiting to complete an open of a slow device (e.g., a FIFO; see fifo(7)),  the  call
           was interrupted by a signal handler; see signal(7).

- [ ] `EINVAL` The `filesystem` does not support the `O_DIRECT` flag.
   See NOTES for more information.

- [x] `EINVAL` Invalid value in flags.

- [x] `EINVAL` `O_TMPFILE` was specified in flags, but neither `O_WRONLY` nor
   `O_RDWR` was specified.

- [ ] EINVAL O_CREAT  was specified in flags and the final component ("basename") of the new file's pathname is
           invalid (e.g., it contains characters not permitted by the underlying filesystem).

- [ ] EINVAL The final component ("basename")  of  pathname  is  invalid  (e.g.,  it  contains  characters  not
           permitted by the underlying filesystem).

- [x] `EISDIR` `pathname` refers to a directory and the access requested involved
   writing (that is, `O_WRONLY` or `O_RDWR` is set).

- [x] `EISDIR` `pathname` refers to an existing directory, `O_TMPFILE` and one of
   `O_WRONLY` or `O_RDWR` were specified in flags, but this kernel version
   does not provide the `O_TMPFILE` functionality.

- [ ] ELOOP  Too many symbolic links were encountered in resolving pathname.

- [ ] ELOOP  pathname was a symbolic link, and flags specified O_NOFOLLOW but not O_PATH.

- [ ] EMFILE The per-process limit on the number of open file descriptors has been reached (see the description
           of RLIMIT_NOFILE in getrlimit(2)).

- [x] ENAMETOOLONG
           pathname was too long.

- [ ] ENFILE The system-wide limit on the total number of open files has been reached.

- [ ] ENODEV pathname refers to a device special file and no corresponding device exists.   (This  is  a  Linux
           kernel bug; in this situation ENXIO must be returned.)

- [x] `ENOENT` `O_CREAT` is not set and the named file does not exist.

- [ ] ENOENT A directory component in pathname does not exist or is a dangling symbolic link.

- [ ] ENOENT pathname refers to a nonexistent directory, O_TMPFILE and one of O_WRONLY or O_RDWR were specified
           in flags, but this kernel version does not provide the O_TMPFILE functionality.

- [ ] ENOMEM The named file is a FIFO, but memory for the FIFO buffer can't be allocated because  the  per-user
           hard  limit  on memory allocation for pipes has been reached and the caller is not privileged; see
           pipe(7).

- [ ] ENOMEM Insufficient kernel memory was available.

- [ ] ENOSPC pathname was to be created but the device containing pathname has no room for the new file.

- [ ] ENOTDIR
           A component used as a directory in pathname is not, in  fact,  a  directory,  or  O_DIRECTORY  was
           specified and pathname was not a directory.

- [ ] ENXIO  O_NONBLOCK  |  O_WRONLY  is  set,  the  named file is a FIFO, and no process has the FIFO open for
           reading.

- [ ] ENXIO  The file is a device special file and no corresponding device exists.

- [ ] ENXIO  The file is a UNIX domain socket.

- [ ] EOPNOTSUPP
           The filesystem containing pathname does not support O_TMPFILE.

- [ ] EOVERFLOW
           pathname refers to a regular file that is too large to be opened.  The usual scenario here is that
           an  application  compiled on a 32-bit platform without -D_FILE_OFFSET_BITS=64 tried to open a file
           whose size exceeds (1<<31)-1 bytes; see also O_LARGEFILE above.  This is the  error  specified  by
           POSIX.1; in kernels before 2.6.24, Linux gave the error EFBIG for this case.

- [ ] EPERM The O_NOATIME flag was specified, but the effective user ID of the caller did not match the owner
           of the file and the caller was not privileged.

- [ ] EPERM The operation was prevented by a file seal; see fcntl(2).

- [ ] EROFS  pathname refers to a file on a read-only filesystem and write access was requested.

- [ ] ETXTBSY
           pathname refers to an executable image which is currently being  executed  and  write  access  was
           requested.

- [ ] ETXTBSY
           pathname  refers  to  a  file  that  is  currently in use as a swap file, and the O_TRUNC flag was
           specified.

- [ ] ETXTBSY
           pathname refers to a file that is currently being read by the kernel  (e.g.,  for  module/firmware
           loading), and write access was requested.

- [ ] EWOULDBLOCK
           The O_NONBLOCK flag was specified, and an incompatible lease was held on the file (see fcntl(2)).

    The following additional errors can occur for openat():

- [ ] EBADF  dirfd is not a valid file descriptor.

- [ ] ENOTDIR
           pathname  is  a  relative pathname and dirfd is a file descriptor referring to a file other than a
           directory.

# unlinkat

- [ ] `EACCES` Write access to the directory containing `pathname` is not allowed
   for the process's effective `UID`, or one of the directories in `pathname`
   did not allow search permission. (See also path_resolution(7).)

       EBUSY  The  file  pathname  cannot be unlinked because it is being used by the system or another process;
              for example, it is a mount point or the NFS client software created it to represent an active  but
              otherwise nameless inode ("NFS silly renamed").

- [x] `EFAULT` `pathname` points outside your accessible address space.

       EIO    An I/O error occurred.

       EISDIR pathname refers to a directory.  (This is the non-POSIX value returned by Linux since 2.1.132.)

       ELOOP  Too many symbolic links were encountered in translating pathname.

- [x] `ENAMETOOLONG` `pathname` was too long.

- [x] `ENOENT` A component in `pathname` does not exist or is a dangling symbolic
   link, or `pathname` is empty.

       ENOMEM Insufficient kernel memory was available.

       ENOTDIR
              A component used as a directory in pathname is not, in fact, a directory.

       EPERM  The  system  does  not  allow  unlinking  of  directories,  or  unlinking  of directories requires
              privileges that the calling process doesn't have.  (This is the POSIX prescribed error return;  as
              noted above, Linux returns EISDIR for this case.)

       EPERM (Linux only)
              The filesystem does not allow unlinking of files.

       EPERM or EACCES
              The directory containing pathname has the sticky bit (S_ISVTX) set and the process's effective UID
              is neither the UID of the file to be deleted nor that of the  directory  containing  it,  and  the
              process is not privileged (Linux: does not have the CAP_FOWNER capability).

       EPERM  The file to be unlinked is marked immutable or append-only.  (See ioctl_iflags(2).)

       EROFS  pathname refers to a file on a read-only filesystem.

       The  same  errors  that  occur  for  unlink()  and rmdir(2) can also occur for unlinkat().  The following
       additional errors can occur for unlinkat():

       EBADF  dirfd is not a valid file descriptor.

       EINVAL An invalid flag value was specified in flags.

       EISDIR pathname refers to a directory, and AT_REMOVEDIR was not specified in flags.

       ENOTDIR
              pathname is relative and dirfd is a file descriptor referring to a file other than a directory.

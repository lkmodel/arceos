# 1.37_stable 
[源代码](https://git.busybox.net/busybox/tree/?h=1_37_stable)
## 目录结构

| **目录**            | **内容说明**                   | **典型命令/功能示例**                           |
| ----------------- | -------------------------- | --------------------------------------- |
| **archival**      | 归档与压缩工具（支持多种格式的解压/压缩）      | `tar`, `gzip`, `cpio`, `ar`             |
| **coreutils**     | 基础 Unix 核心命令（文件操作、系统基础操作）  | `cat`, `ls`, `cp`, `rm`, `chmod`        |
| **console-tools** | 控制台配置和键盘映射管理               | `setfont`, `loadkeys`, `dumpkmap`       |
| **debianutils**   | Debian 系统中常见的辅助工具          | `run-parts`, `start-stop-daemon`        |
| **e2fsprogs**     | ext2/ext3/ext4 文件系统工具      | `chattr`, `lsattr`, `e2fsck`            |
| **editors**       | 文本编辑工具                     | `vi`, `sed`, `awk`, `ed`                |
| **findutils**     | 文件查找与处理工具                  | `find`, `grep`, `xargs`                 |
| **init**          | 系统初始化与运行级管理                | `init`, `reboot`, `halt`                |
| **loginutils**    | 用户登录与身份验证工具                | `login`, `passwd`, `su`, `getty`        |
| **modutils**      | 内核模块管理（加载、卸载、查看）           | `insmod`, `rmmod`, `lsmod`              |
| **networking**    | 网络配置与通信工具（协议栈、服务端/客户端）     | `ifconfig`, `ping`, `telnet`, `ftp`     |
| **procps**        | 进程与系统状态监控工具                | `ps`, `top`, `kill`, `free`             |
| **shell**         | Shell 解析器及内置命令             | `ash`, `echo`, `test`, `[`              |
| **util-linux**    | 来自 `util-linux` 的系统工具      | `mount`, `umount`, `swapon`             |
| **libbb**         | Busybox 公共基础库（内存管理、字符串处理等） | `xmalloc`, `parse_number`, `verror_msg` |
| **applets**       | Applets 调度框架和入口逻辑          |                                         |
| **miscutils**     | 杂项工具（不直接归类到其他目录的实用程序）      | `hwclock`, `mdev`, `blkid`              |
| **sysklogd**      | 系统日志                       |                                         |
## 针对Risc-V
```sh
git checkout 1_37_stable
```
关闭SHA硬件加速，可以直接打patch
```diff
diff --git a/libbb/hash_md5_sha.c b/libbb/hash_md5_sha.c
index 57a801459..75a61c32c 100644
--- a/libbb/hash_md5_sha.c
+++ b/libbb/hash_md5_sha.c
@@ -1313,7 +1313,9 @@ unsigned FAST_FUNC sha1_end(sha1_ctx_t *ctx, void *resbuf)
 	hash_size = 8;
 	if (ctx->process_block == sha1_process_block64
 #if ENABLE_SHA1_HWACCEL
+# if defined(__GNUC__) && (defined(__i386__) || defined(__x86_64__))
 	 || ctx->process_block == sha1_process_block64_shaNI
+# endif
 #endif
 	) {
 		hash_size = 5;
```
## 编译整个busybox
```sh
make defconfig
make CROSS_COMPILE=riscv64-linux-musl- ARCH=riscv
```
### 结果
```sh
file busybox
busybox: ELF 64-bit LSB pie executable, UCB RISC-V, RVC, double-float ABI, version 1 (SYSV), dynamically linked, interpreter /lib/ld-musl-riscv64.so.1, stripped
```
## 编译单个应用
修改`make_single_applets.sh`
```sh
sed -i 's/make / ARCH=riscv CROSS_COMPILE=${cross} make /g' make_single_applets.sh
```
### 编译所有应用为单个应用
```sh
make defconfig #必要，用于生成include/applets.h
cross=riscv64-linux-musl- ./make_single_applets.sh
```
### 编译指定的应用为单个应用
```sh
make distclean #避免奇怪的问题发生
make defconfig #必要，用于生成include/applets.h
cross=riscv64-linux-musl- ./make_single_applets.sh ASH LS WHOAMI
```
### 结果
```sh
file busybox_LS
busybox_LS: ELF 64-bit LSB pie executable, UCB RISC-V, RVC, double-float ABI, version 1 (SYSV), dynamically linked, interpreter /lib/ld-musl-riscv64.so.1, stripped
```
## 应用列表
```sh
grep ^IF_ include/applets.h \
| grep -v '^IF_FEATURE_' \
| sed 's/IF_\([A-Z0-9._-]*\)(.*/\1/' \
| grep -v '^BUSYBOX$' \
| sort | uniq
```

1. ACPID
2. ADDGROUP
3. ADD_SHELL
4. ADDUSER
5. ADJTIMEX
6. AR
7. ARP
8. ARPING
9. ASCII
10. ASH
11. AWK
12. BASE32
13. BASE64
14. BASENAME
15. BASH_IS_ASH
16. BASH_IS_HUSH
17. BB_ARCH
18. BBCONFIG
19. BB_SYSCTL
20. BC
21. BEEP
22. BLKDISCARD
23. BLKID
24. BLOCKDEV
25. BOOTCHARTD
26. BRCTL
27. BUNZIP2
28. BZCAT
29. BZIP2
30. CAL
31. CAT
32. CHAT
33. CHATTR
34. CHCON
35. CHGRP
36. CHMOD
37. CHOWN
38. CHPASSWD
39. CHPST
40. CHROOT
41. CHRT
42. CHVT
43. CKSUM
44. CLEAR
45. CMP
46. COMM
47. CONSPY
48. CP
49. CPIO
50. CRC32
51. CROND
52. CRONTAB
53. CRYPTPW
54. CTTYHACK
55. CUT
56. DATE
57. DC
58. DD
59. DEALLOCVT
60. DELGROUP
61. DELUSER
62. DEPMOD
63. DEVFSD
64. DEVMEM
65. DF
66. DHCPRELAY
67. DIFF
68. DIRNAME
69. DMESG
70. DNSD
71. DNSDOMAINNAME
72. DOS2UNIX
73. DPKG
74. DPKG_DEB
75. DU
76. DUMPKMAP
77. DUMPLEASES
78. ECHO
79. ED
80. EGREP
81. EJECT
82. ENV
83. ENVDIR
84. ENVUIDGID
85. ETHER_WAKE
86. EXPAND
87. EXPR
88. FACTOR
89. FAKEIDENTD
90. FALLOCATE
91. FALSE
92. FATATTR
93. FBSET
94. FBSPLASH
95. FDFLUSH
96. FDFORMAT
97. FDISK
98. FGCONSOLE
99. FGREP
100. FIND
101. FINDFS
102. FLASHCP
103. FLASH_ERASEALL
104. FLASH_LOCK
105. FLASH_UNLOCK
106. FLOCK
107. FOLD
108. FREE
109. FREERAMDISK
110. FSCK
111. FSCK_MINIX
112. FSFREEZE
113. FSTRIM
114. FSYNC
115. FTPD
116. FTPGET
117. FTPPUT
118. FUSER
119. GETENFORCE
120. GETFATTR
121. GETOPT
122. GETSEBOOL
123. GETTY
124. GREP
125. GROUPS
126. GUNZIP
127. GZIP
128. HALT
129. HD
130. HDPARM
131. HEAD
132. HEXDUMP
133. HEXEDIT
134. HOSTID
135. HOSTNAME
136. HTTPD
137. HUSH
138. HWCLOCK
139. I2CDETECT
140. I2CDUMP
141. I2CGET
142. I2CSET
143. I2CTRANSFER
144. ID
145. IFCONFIG
146. IFDOWN
147. IFENSLAVE
148. IFPLUGD
149. IFUP
150. INETD
151. INIT
152. INOTIFYD
153. INSMOD
154. INSTALL
155. IONICE
156. IOSTAT
157. IP
158. IPADDR
159. IPCALC
160. IPCRM
161. IPCS
162. IPLINK
163. IPNEIGH
164. IPROUTE
165. IPRULE
166. IPTUNNEL
167. KBD_MODE
168. KILL
169. KILLALL
170. KILLALL5
171. KLOGD
172. LAST
173. LESS
174. LINK
175. LINUX32
176. LINUX64
177. LINUXRC
178. LN
179. LOADFONT
180. LOADKMAP
181. LOAD_POLICY
182. LOGGER
183. LOGIN
184. LOGNAME
185. LOGREAD
186. LOSETUP
187. LPD
188. LPQ
189. LPR
190. LS
191. LSATTR
192. LSMOD
193. LSOF
194. LSPCI
195. LSSCSI
196. LSUSB
197. LZCAT
198. LZMA
199. LZOP
200. LZOPCAT
201. MAKEDEVS
202. MAKEMIME
203. MAN
204. MATCHPATHCON
205. MD5SUM
206. MDEV
207. MESG
208. MICROCOM
209. MIM
210. MINIPS
211. MKDIR
212. MKDOSFS
213. MKE2FS
214. MKFIFO
215. MKFS_EXT2
216. MKFS_MINIX
217. MKFS_REISER
218. MKFS_VFAT
219. MKNOD
220. MKPASSWD
221. MKSWAP
222. MKTEMP
223. MODINFO
224. MODPROBE
225. MORE
226. MOUNT
227. MOUNTPOINT
228. MPSTAT
229. MT
230. MV
231. NAMEIF
232. NANDDUMP
233. NANDWRITE
234. NBDCLIENT
235. NC
236. NETCAT
237. NETSTAT
238. NICE
239. NL
240. NMETER
241. NOHUP
242. NOLOGIN
243. NPROC
244. NSENTER
245. NSLOOKUP
246. NTPD
247. NUKE
248. OD
249. OPENVT
250. PARTPROBE
251. PASSWD
252. PASTE
253. PATCH
254. PGREP
255. PIDOF
256. PING
257. PING6
258. PIPE_PROGRESS
259. PIVOT_ROOT
260. PKILL
261. PMAP
262. POPMAILDIR
263. POWEROFF
264. POWERTOP
265. PRINTENV
266. PRINTF
267. PS
268. PSCAN
269. PSTREE
270. PWD
271. PWDX
272. RAIDAUTORUN
273. RDATE
274. RDEV
275. READAHEAD
276. READLINK
277. READPROFILE
278. REALPATH
279. REBOOT
280. REFORMIME
281. REMOVE_SHELL
282. RENICE
283. RESET
284. RESIZE
285. RESTORECON
286. RESUME
287. REV
288. RFKILL
289. RM
290. RMDIR
291. RMMOD
292. ROUTE
293. RPM
294. RPM2CPIO
295. RTCWAKE
296. RUNCON
297. RUN_INIT
298. RUNLEVEL
299. RUN_PARTS
300. RUNSV
301. RUNSVDIR
302. RX
303. SCRIPT
304. SCRIPTREPLAY
305. SED
306. SEEDRNG
307. SELINUXENABLED
308. SENDMAIL
309. SEQ
310. SESTATUS
311. SETARCH
312. SETCONSOLE
313. SETENFORCE
314. SETFATTR
315. SETFILES
316. SETFONT
317. SETKEYCODES
318. SETLOGCONS
319. SETPRIV
320. SETSEBOOL
321. SETSERIAL
322. SETSID
323. SETUIDGID
324. SHA1SUM
325. SHA256SUM
326. SHA3SUM
327. SHA512SUM
328. SH_IS_ASH
329. SH_IS_HUSH
330. SHOWKEY
331. SHRED
332. SHUF
333. SLATTACH
334. SLEEP
335. SMEMCAP
336. SOFTLIMIT
337. SORT
338. SPLIT
339. SSL_CLIENT
340. START_STOP_DAEMON
341. STAT
342. STRINGS
343. STTY
344. SU
345. SULOGIN
346. SUM
347. SV
348. SVC
349. SVLOGD
350. SVOK
351. SWAPOFF
352. SWAPON
353. SWITCH_ROOT
354. SYNC
355. SYSLOGD
356. TAC
357. TAIL
358. TAR
359. TASKSET
360. TC
361. TCPSVD
362. TEE
363. TELNET
364. TELNETD
365. TEST
366. TEST1
367. TEST2
368. TFTP
369. TFTPD
370. TIME
371. TIMEOUT
372. TOP
373. TOUCH
374. TR
375. TRACEROUTE
376. TRACEROUTE6
377. TREE
378. TRUE
379. TRUNCATE
380. TS
381. TSORT
382. TTY
383. TTYSIZE
384. TUNCTL
385. TUNE2FS
386. UBIATTACH
387. UBIDETACH
388. UBIMKVOL
389. UBIRENAME
390. UBIRMVOL
391. UBIRSVOL
392. UBIUPDATEVOL
393. UDHCPC
394. UDHCPC6
395. UDHCPD
396. UDPSVD
397. UEVENT
398. UMOUNT
399. UNAME
400. UNCOMPRESS
401. UNEXPAND
402. UNIQ
403. UNIT_TEST
404. UNIX2DOS
405. UNLINK
406. UNLZMA
407. UNLZOP
408. UNSHARE
409. UNXZ
410. UNZIP
411. UPTIME
412. USERS
413. USLEEP
414. UUDECODE
415. UUENCODE
416. VCONFIG
417. VI
418. VLOCK
419. VOLNAME
420. W
421. WALL
422. WATCH
423. WATCHDOG
424. WC
425. WGET
426. WHICH
427. WHO
428. WHOAMI
429. WHOIS
430. XARGS
431. XXD
432. XZ
433. XZCAT
434. YES
435. ZCAT
436. ZCIP
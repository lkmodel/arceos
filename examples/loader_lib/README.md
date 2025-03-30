#

## 保留错误

目前无法解决的的一个问题在于，当在脚本中写入

```
busybox_LS -la
busybox_TOUCH test.txt
busybox_LS
busybox_CAT test.txt
```

之后直接执行就会报错，而且是page fault，问题定位于在第一个应用中，在两次pthread_self中发生了错误，导致了之后在kernel中跑飞了。

# config.mk

# 选择要编译的功能
# ENABLE_MATH := 1
# ENABLE_COMPLEX := 0
# ENABLE_STRING := 1

# 选择 malloc 库版本
# 当启动Safe Malloc编译选项时，malloc将会全面替换成ArceOS自己实现的Malloc库
# 在这个库中：
# - malloc, calloc在创建空间失败的时候，将会打印报错并退出，而不是返回MULL
#   以保证在运行没有对malloc返回值进行良好检查的代码时，不会陷入到不安全的运行情况中。
# - TODO
USE_SAFE_MALLOC := 1

# 编译选项
OPTIMIZE := 0

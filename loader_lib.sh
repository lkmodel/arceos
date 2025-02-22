echo "Loader my lib version"
#!/bin/bash

# 设置默认参数
log="warn"
qemu_log="y"
type="default"

# 定义枚举类型
declare -A valid_types
valid_types=(
	["default"]="Default type"
	["type1"]="Type 1 description"
	["type2"]="Type 2 description"
	["type3"]="Type 3 description"
)

# 解析命令行参数
options=$(getopt -o l:q:t --long log:,qemu_log:,type:, -- "$@")
eval set -- "$options"

# 提取选项和参数
while true; do
	case $1 in
	-l | --log)
		shift
		log=$1
		shift
		;;
	-q | --qemu_log)
		shift
		qemu_log=$1
		shift
		;;
	-t | --type)
		shift
		type=$1
		shift
		;;
	--)
		shift
		break
		;;
	*)
		echo "Invalid option: $1" exit 1
		;;
	esac
done

# 核查 type 是否有效
if [[ ! -v "valid_types[$type]" ]]; then
	echo "Invalid type: $type. Valid types are: ${!valid_types[*]}"
	exit 1
fi

make run ARCH=riscv64 A=examples/loader_lib LOG="$log" QEMU_LOG="$qemu_log"

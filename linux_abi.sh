#!/bin/bash

# Default settings
ARCH="riscv64"
LOG="warn"
QEMU_LOG="n"
TTYPE="dynamic"
C_APP="hello_app"

# Parse arguments
for arg in "$@"; do
    case $arg in
        ARCH=*)
            ARCH="${arg#*=}"
            ;;
        LOG=*)
            LOG="${arg#*=}"
            ;;
        QEMU_LOG=*)
            QEMU_LOG="${arg#*=}"
            ;;
        TTYPE=*)
            TTYPE="${arg#*=}"
            ;;
        C_APP=*)
            C_APP="${arg#*=}"
            ;;
    esac
done

echo "Architecture: $ARCH"
echo "Log level: $LOG"
echo "QEMU log: $QEMU_LOG"
echo "Test type: $TTYPE"
echo "C app: $C_APP"

CURRENT=$PWD

get_sudo() {
    # Check if sudo permissions are already available
    if sudo -n true 2>/dev/null; then
        echo "Sudo permissions already available"
        return 0
    fi

    # Attempt up to 3 times
    local max_attempts=3
    local attempt=1

    while [ $attempt -le $max_attempts ]; do
        echo "Requesting sudo permissions (attempt $attempt/$max_attempts)"
        if sudo -v; then
            echo "Sudo permissions granted"
            # Keep sudo permissions active
            sudo -v
            return 0
        fi
        echo "Permission denied, please retry"
        ((attempt++))
    done

    echo "Failed to obtain sudo permissions after maximum attempts"
    return 1
}

check_installation() {
    if which riscv64-linux-musl-gcc > /dev/null 2>&1 || [ -d "/opt/musl_riscv64" ]; then
        echo "riscv64-linux-musl-gcc is already installed or directory exists"
        return 0
    else
        echo "Neither compiler nor directory exists"
        return 1
    fi
}

install_musl_riscv64() {
    if check_installation; then
        return 0
    fi

    get_sudo || return 1
    
    # Clone the repository
    cd ~
    rm -rf musl-cross-make
    git clone https://github.com/richfelker/musl-cross-make.git || {
        echo "Failed to clone the repository"
        return 1
    }

    # Navigate to the directory and configure
    cd musl-cross-make || return 1
    cp ./config.mak.dist ./config.mak || return 1
    
    # Add configuration details
    sed -i '15i\TARGET = riscv64-linux-musl' config.mak
    sed -i '22i\OUTPUT = /opt/musl_riscv64' config.mak

    # Compile and install
    make -j4 || {
        echo "Build failed"
        return 1
    }
    
    sudo make install -j4 || {
        echo "Installation failed"
        return 1
    }

    # Add to PATH
    if [ -f ~/.bashrc ]; then
        if ! grep -q "/opt/musl_riscv64/bin" ~/.bashrc; then
            echo 'export PATH=$PATH:/opt/musl_riscv64/bin' >> ~/.bashrc
            echo "Please run 'source ~/.bashrc' to make the changes take effect"
        fi
    fi

    if [ -f ~/.zshrc ]; then
        if ! grep -q "/opt/musl_riscv64/bin" ~/.zshrc; then
            echo 'export PATH=$PATH:/opt/musl_riscv64/bin' >> ~/.zshrc
            echo "Please run 'source ~/.zshrc' to make the changes take effect"
        fi
    fi

    # 直接修改当前会话的 PATH
    export PATH=$PATH:/opt/musl_riscv64/bin

    echo "Musl RISC-V64 toolchain installation complete"
    cd ~
    rm -rf musl-cross-make
    return 0
}

check_branch() {
    local branch_name=$1
    if [ "$(git rev-parse --abbrev-ref HEAD)" = "$branch_name" ]; then
        echo "Currently on branch $branch_name"
        return 0
    else
        echo "Not on branch $branch_name"
        git switch $branch_name || {
            echo "Failed to switch branch"
            return 1
        }
        echo "Switched to branch $branch_name"
        return 0
    fi
}

# Invoke functions
install_musl_riscv64

cd $CURRENT
check_branch "mocklibc"

static_test() {
    cd $CURRENT/payload/$C_APP
    echo "Starting static compilation test"
    # 运行测试脚本并捕获返回值
    bash ./run_test.sh TTYPE=static
    test_result=$?
    
    if [ $test_result -ne 0 ]; then
        echo "Static compilation test failed"
        return $test_result  # 返回错误状态
    fi

    cd $CURRENT
    echo "Starting execution"
    make defconfig ARCH=riscv64
    make A=examples/loader ARCH=$ARCH LOG=$LOG QEMU_LOG=$QEMU_LOG run
}

dynamic_test() {
    cd $CURRENT/payload/$C_APP
    echo "Starting dynamic compilation test"
    # 运行测试脚本并捕获返回值
    bash ./run_test.sh TTYPE=dynamic
    test_result=$?
    
    if [ $test_result -ne 0 ]; then
        echo "Dynamic compilation test failed"
        return $test_result  # 返回错误状态
    fi

    cd $CURRENT
    echo "Starting execution"
    make defconfig ARCH=riscv64
    make A=examples/loader ARCH=$ARCH LOG=$LOG QEMU_LOG=$QEMU_LOG run
}

echo "Mocklibc build"

export PATH=$PATH:/opt/musl_riscv64/bin

rustup target add riscv64gc-unknown-linux-musl

if [ "$C_APP" = "hello_app" ]; then
    cargo build --target riscv64gc-unknown-linux-musl --release -p mocklibc
    rm -rf $CURRENT/payload/hello_app/libmocklibc.*
    mv ./target/riscv64gc-unknown-linux-musl/release/libmocklibc.* ./payload/hello_app
fi

echo "Starting tests"

if [ "$TTYPE" = "dynamic" ]; then
    dynamic_test
elif [ "$TTYPE" = "static" ]; then
    static_test
elif [ "$TTYPE" = "all" ]; then
    dynamic_test
    echo "----------------------------------------"
    echo " "
    echo "----------------------------------------"
    static_test
else
    echo "Test failed"
fi

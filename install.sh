#!/bin/bash
set -e

# Teammate 安装脚本

INSTALL_DIR="${HOME}/.local/bin"
BINARY_NAME="teammate"
RELEASE_URL="https://github.com/yourusername/teammate/releases"

# 颜色输出
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() { echo -e "${GREEN}[INFO]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# 检测系统
detect_os() {
    case "$(uname -s)" in
        Linux*)     echo "linux";;
        Darwin*)    echo "macos";;
        CYGWIN*|MINGW*|MSYS*) echo "windows";;
        *)          echo "unknown";;
    esac
}

# 检测架构
detect_arch() {
    case "$(uname -m)" in
        x86_64)     echo "x86_64";;
        aarch64)    echo "aarch64";;
        arm64)      echo "aarch64";;
        *)          echo "x86_64";;
    esac
}

# 从源码编译安装
install_from_source() {
    log_info "从源码编译安装..."

    if ! command -v cargo &> /dev/null; then
        log_error "未找到 Rust/Cargo，请先安装: https://rustup.rs/"
        exit 1
    fi

    # 获取脚本所在目录（支持软链接）
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

    cd "$SCRIPT_DIR"

    log_info "编译中..."
    cargo build --release

    # 创建安装目录
    mkdir -p "$INSTALL_DIR"

    # 复制二进制文件
    cp "target/release/${BINARY_NAME}" "${INSTALL_DIR}/${BINARY_NAME}"
    chmod +x "${INSTALL_DIR}/${BINARY_NAME}"

    log_info "已安装到: ${INSTALL_DIR}/${BINARY_NAME}"
}

# 下载预编译二进制
install_from_release() {
    local os=$(detect_os)
    local arch=$(detect_arch)
    local version="${1:-latest}"

    log_info "下载预编译二进制 (${os}-${arch})..."

    if [ "$version" = "latest" ]; then
        version=$(curl -sL "${RELEASE_URL}/latest" | grep -o 'v[0-9.]*' | head -1)
    fi

    local filename="${BINARY_NAME}-${os}-${arch}"
    local url="${RELEASE_URL}/download/${version}/${filename}.tar.gz"

    log_info "从 ${url} 下载..."

    # 创建临时目录
    local tmp_dir=$(mktemp -d)
    cd "$tmp_dir"

    # 下载并解压
    curl -sL "$url" -o "${filename}.tar.gz"
    tar xzf "${filename}.tar.gz"

    # 创建安装目录
    mkdir -p "$INSTALL_DIR"

    # 复制二进制文件
    cp "${filename}/${BINARY_NAME}" "${INSTALL_DIR}/${BINARY_NAME}"
    chmod +x "${INSTALL_DIR}/${BINARY_NAME}"

    # 清理
    cd -
    rm -rf "$tmp_dir"

    log_info "已安装到: ${INSTALL_DIR}/${BINARY_NAME}"
}

# 添加到 PATH（可选）
configure_path() {
    if [[ ":$PATH:" != *":${INSTALL_DIR}:"* ]]; then
        log_warn "需要将 ${INSTALL_DIR} 添加到 PATH"
        echo ""
        echo "在 ~/.bashrc 或 ~/.zshrc 中添加:"
        echo "  export PATH=\"${INSTALL_DIR}:\$PATH\""
        echo ""
    fi
}

# 主程序
main() {
    local install_type="source"

    # 解析参数
    while [[ $# -gt 0 ]]; do
        case $1 in
            --release)
                install_type="release"
                shift
                ;;
            --version)
                version="$2"
                shift 2
                ;;
            --help|-h)
                echo "Teammate 安装脚本"
                echo ""
                echo "用法: $0 [选项]"
                echo ""
                echo "选项:"
                echo "  --release     下载预编译版本（需要配置 RELEASE_URL）"
                echo "  --version     指定版本号"
                echo "  --help        显示帮助"
                exit 0
                ;;
            *)
                log_error "未知参数: $1"
                exit 1
                ;;
        esac
    done

    echo "======================================"
    echo "     Teammate 安装程序"
    echo "======================================"
    echo ""

    case $install_type in
        source)
            install_from_source
            ;;
        release)
            install_from_release "$version"
            ;;
    esac

    echo ""
    configure_path
    echo ""
    log_info "安装完成！运行 'teammate --help' 开始使用。"
}

main "$@"

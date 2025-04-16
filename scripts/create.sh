#!/bin/bash

# 定义参数
cmd=$1
src_dir="./src/example"
dest_dir="./src/$2"

# 判断 cmd 是否为 'mod'
if [ "$cmd" = "mod" ]; then
    # 检查源目录是否存在
    if [ -d "$src_dir" ]; then
        # 使用 cp 命令递归复制
        cp -r "$src_dir" "$dest_dir"

        # 检查是否复制成功
        if [ $? -eq 0 ]; then
            echo "目录 $src_dir 已成功复制到 $dest_dir"
        else
            echo "复制失败，请检查错误信息"
            exit 1
        fi
    else
        echo "源目录 $src_dir 不存在"
        exit 1
    fi
else
    echo "无效的命令参数: cmd=$cmd"
    echo "请使用 cmd='mod' 来执行复制操作"
    exit 1
fi
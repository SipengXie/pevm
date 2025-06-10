# Convert to JSON

这个工具用于将pevm项目中的二进制数据文件转换为JSON格式，方便查看和分析。

## 功能

- 将 `block_hashes.bincode` 转换为 `block_hashes.json`
- 将 `bytecodes.bincode.gz` 转换为 `bytecodes.json`
- 支持单独转换其中一种数据类型
- 自动创建输出目录

## 使用方法

### 基本用法

```bash
# 从pevm根目录运行
cargo run -p convert_to_json
```

这将读取 `data/` 目录下的数据文件，并在 `json_output/` 目录下生成JSON文件。

### 自定义路径

```bash
# 指定自定义的数据目录和输出目录
cargo run -p convert_to_json -- --data-dir /path/to/data --output-dir /path/to/output
```

### 只转换特定类型

```bash
# 只转换区块哈希
cargo run -p convert_to_json -- --block-hashes-only

# 只转换字节码
cargo run -p convert_to_json -- --bytecodes-only
```

## 输出格式

### block_hashes.json
```json
{
  "1000000": "0x1234567890abcdef...",
  "1000001": "0xfedcba0987654321...",
  ...
}
```

### bytecodes.json
```json
{
  "0x1234567890abcdef...": {
    "Legacy": {
      "bytecode": "0x608060405234801561001057600080fd5b50...",
      "original_len": 247,
      "jump_table": [false, false, true, ...]
    }
  },
  ...
}
```

## 数据结构说明

- **block_hashes**: 区块号到区块哈希的映射
- **bytecodes**: 代码哈希到EVM字节码的映射，包含不同类型的字节码（Legacy、EIP7702、EOF）

## 依赖

该工具依赖于pevm crate中定义的数据结构，确保在运行前pevm项目已正确编译。 
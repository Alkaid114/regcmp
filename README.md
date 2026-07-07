# regcmp

> [!TIP]
> 面向**形式语言与自动机**课程的形式正则表达式及自动机等价性判定工具。并非工程上的正则表达式（PCRE/PCRE2）

## 在线使用 WASM 版

浏览器直接用：[https://alkaid114.github.io/regcmp/](https://alkaid114.github.io/regcmp/)

支持正则表达式和自动机两种输入模式

## 预编译二进制

前往 [GitHub Releases](https://github.com/Alkaid114/regcmp/releases) 获取

## 构建

```bash
git clone <repo-url>
cd regcmp
cargo build --release
```

## 用法

```bash
regcmp <input1> <input2> [-v|--verbose]
```

输入可以是正则表达式或自动机文件路径，程序自动检测。

### 正则表达式 示例

```bash
# 基本比较
regcmp 'a*|b*' 'b*|a*'

# 详细输出（查看中间过程）
regcmp -v 'a(b|c)' 'ab|ac'

# + 和 | 都表示并运算，^* 和 * 都表示 Kleene 星
regcmp 'a+b' 'a|b'
regcmp 'a^*b^*' 'a*b*'
```

### 正则表达式 符号

| 符号 | 含义 | 说明 |
|---|---|---|
| `#` | 空语言 | ∅，不接受任何串 |
| `$` | 空串 | ε，仅接受空串 |
| `a` `b` `c` ... | 字母表字符 | 从输入自动推断 |
| `\|` `+` | 并运算 | `R\|S` 或 `R+S` |
| `*` `^*` | Kleene 星 | `R*` 或 `R^*` |
| `()` | 分组 | 改变运算优先级 |

优先级：`*` > 隐式连接 > `|` / `+`

### 自动机文件 示例

```bash
# 自动机 vs 正则
regcmp dfa.txt 'a*|b*'

# 自动机 vs 自动机
regcmp nfa1.txt dfa2.txt
```

### 自动机文件 格式

纯文本，每行一个语义单元：

```plaintext
# 注释以 # 开头
start: q0           # 起始态（可选，缺省取第一个出现的状态）
accept: q0 q2       # 接受态（可选，缺省无接受态）
q0 $ q1             # ε 转移（$ 表示 ε）
q1 a q2             # 字符转移：源 符号 目标
```

规则：

- 状态名支持字母、数字、`_`、`-`、`.`
- 符号为单个字符，`$` 表示 ε
- 每行 3 个字段：源 符号 目标，空格分隔

## 算法

```plaintext
正则/自动机1              正则/自动机2
     │                       │
     ↓                       ↓
    NFA                     NFA
     │                       │
     ↓                       ↓
子集构造 DFA             子集构造 DFA
     │                       │
     ↓                       ↓
Hopcroft 最小化        Hopcroft 最小化
     │                       │
     └──────────┬────────────┘
                ↓
            同构比较
                │
                ↓
           等价 / 不等价
```

1. **解析** — 正则表达式经递归下降解析为 AST，再经 Thompson 构造转为 NFA；自动机文件直接解析为 NFA
2. **子集构造法** — 将 NFA 转换为等价的 DFA
3. **Hopcroft 最小化** — 将 DFA 最小化为唯一规范形式
4. **同构比较** — 构造两个 DFA 的积自动机，检查是否存在差异

## 许可证

MIT

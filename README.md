# regcmp

> [!TIP]
> 面向**形式语言与自动机**课程的形式正则表达式等价性判定工具。
>
> 这不是工程上的正则表达式（PCRE/PCRE2），而是《形式语言与自动机》中定义的原始正则表达式。

## 在线使用WASM版

浏览器直接用：[https://alkaid114.github.io/regcmp/](https://alkaid114.github.io/regcmp/)

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
regcmp <regex1> <regex2> [-v|--verbose]
```

### 示例

```bash
# 基本比较
regcmp 'a*|b*' 'b*|a*'          # 等价

# 详细输出（查看中间过程）
regcmp -v 'a(b|c)' 'ab|ac'      # 等价

# + 和 | 都表示并运算
regcmp 'a+b' 'a|b'              # 等价

# ^* 和 * 都表示 Kleene 星
regcmp 'a^*b^*' 'a*b*'          # 等价
```

### 支持的符号

| 符号 | 含义 | 说明 |
|---|---|---|
| `#` | 空语言 | ∅，不接受任何串 |
| `$` | 空串 | ε，仅接受空串 |
| `a` `b` `c` ... | 字母表字符 | 从输入自动推断 |
| `\|` `+` | 并运算 | `R\|S` 或 `R+S` |
| `*` `^*` | Kleene 星 | `R*` 或 `R^*` |
| `()` | 分组 | 改变运算优先级 |

### 优先级

```
* (Kleene 星)  >  隐式连接  >  | / + (并运算)
```

## 算法

```
正则表达式1             正则表达式2
    │                       │
    ↓                       ↓
   AST                     AST
    │                       │
    ↓                       ↓
Thompson NFA          Thompson NFA
    │                       │
    ↓                       ↓
子集构造 DFA           子集构造 DFA
    │                       │
    ↓                       ↓
Hopcroft 最小化        Hopcroft 最小化
    │                       │
    └──────────┬────────────┘
               ↓
           同构比较
               │
               ↓
         等价/不等价
```

1. **解析** — 递归下降解析器将输入字符串解析为 AST
2. **Thompson 构造法** — 将 AST 转换为 ε-NFA
3. **子集构造法** — 将 NFA 转换为等价的 DFA
4. **Hopcroft 最小化** — 将 DFA 最小化为唯一规范形式
5. **同构比较** — 构造两个 DFA 的积自动机，检查是否存在差异

## 许可证

MIT

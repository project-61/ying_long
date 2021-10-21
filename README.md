# YingLong

应龙 (YìngLóng) 数字电路逻辑描述语言

The 应龙 (YìngLóng) Digital Circuits Logic description language

## Numbering

+ The startup time(Plan): 2th Five Month Plan(2021.5~1021.10)
+ Project: Pr61

## Design

### 问题

为了解决 Chisel 和 HLS 两个HDL语言的宿主语言小众，学习成本高，工具链sbt和stack的安装难，环境部署痛苦的问题。
YingLong计划诞生了。

### 设计目标

+ 依赖问题简单：利用Rust强大的包管理器和构建功能。
+ 利用语言特性构造 Internal DSL 和调试/测试框架。
+ 导出为Pyd使Python调用，待考虑从Python单独裁切子集作为DSL。

### 设计

使用形式验证作为电路设计验证手段之一，此外还有测试框架和仿真调试。

#### module

```rs
module! {
    name: ...,
    in: {
        "a": Uint<32>,
        "b": Uint<32>,
    },
    out: a | b
}
```

#### 组合逻辑电路

##### 基本操作

bit and/or/xor, logic and/or/xor，Uint add/sub。

Mux等其它拓展功能用库提供。

为了实现构造后ast类型检查，以及实现External DSL(Python Subset)的支持，
使用`Pin`作为基本单元（AST）组合操作逻辑，
`Bits<N>`等类型作为`Pin`的特例，用于编译期Type Check。

##### 生成组合逻辑电路

```rs
pub fn bitnot(i: Pin) -> Pin {
    let a = constant!(true);
    // a.xor(i)
    a ^ i
}
```

##### 组合生成函数

```rs
pub fn not_n<N>(input: Bits<N>) -> Bits<N> {
    let mut ret = [null(); N];
    for i in 0..N {
        ret[i] = !input[i];
    }
    bits!(ret)
}
// ...

let a: Bits<32> = ...;
let b = not_n(a);

```

```rs
#[inline]
pub fn or_n<N>(input: Bits<N>) -> Bits<N> {
    let mut ret = null();
    if N == 0 {
        return bits(ret);
    }
    ret = input[0];
    for i in 1..N {
        ret = input[i] | ret;
    }
    ret
}
// ...

let a: Bits<32> = ...;
let b: Bit = or_n(a);

```

#### 时序逻辑电路

提供内置的`Reg`和`Clock`类型

// 待定

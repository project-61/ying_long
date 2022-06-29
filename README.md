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

+ 完全解决所有可能存在的依赖问题
+ 借用 Python 的动态性构造 Internal DSL 前端，同时降低学习成本
+ 使用 Rust 处理语义层，在其上做所有分析和转换

### 设计

使用形式验证作为电路设计验证手段之一，此外还有测试框架和仿真调试。

### DataType

+ Type
  + Integer
    + Sint(length)
    + Uint(length)
  + Clock
  + Vector(Type, length)
  + Bundle(Map[str, Type])

### Circuit Node Type

+ Wire(Type)
+ Reg(Type, Clock, Reset)
  + Reset(Option[Tuple[Expr, Expr]])
+ Mem(Type, length, Clock, )
+ Port
  + Input(Type)
  + Output(Type)

+ Module(name: Id, port/wire/mem, statements: list[Stmt])
  + PortDefs(list[Port])
  + WireDefs(list[Wire])
  + RegDefs(list[Reg])
  + MemDefs(list[Mem])
  + Stmt
    + Bind(name: Id, value: Expr)
    + When(cond: Expr, then: Stmt, else_: Option[Else])
      + Else(Expr)

+ Expr
  + Id(str)
  + Literal(Union[int, str, float])
  + Mux(cond: Expr, then: Expr, else: Expr)
  + Operator(Op, list[Expr])

+ Op
  + Add
  + Sub
  + Mul
  + Div
  + Mod
  + Lt
  + Leq
  + Gt
  + Geq
  + Eq
  + Neq
  + Pad
  + AsUInt
  + AsSInt
  + AsClock
  + Shl
  + Shr
  + Cvt
  + Neg
  + Not
  + And
  + Or
  + Xor
  + AndReduce
  + OrReduce
  + XorReduce
  + Cat
  + Bits

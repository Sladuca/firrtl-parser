mod expr;
mod string;
mod types;

// String that represents a name or identifier given to an object according to FIRRTL spec
// [a-zA-Z_][\w_]+
pub type IDStr = String;

#[derive(Debug, Clone, PartialEq)]
pub struct Circuit {
    id: IDStr,
    modules: Vec<Module>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    id: IDStr,
    ports: Vec<Port>,
    stmt: Stmt,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Port {
    direction: Direction,
    id: IDStr,
    ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    I,
    O,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    UInt {
        width: Option<usize>,
    },
    SInt {
        width: Option<usize>,
    },
    Fixed {
        width: Option<usize>,
        point: Option<usize>,
    },
    Clock,
    Analog {
        width: Option<usize>,
    },
    Bundle {
        fields: Vec<Field>,
    },
    Vector {
        ty: Box<Type>,
        len: usize,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Field {
    Flipped(FieldInner),
    Default(FieldInner),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldInner {
    id: IDStr,
    ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Wire(Wire),
    Reg(Reg),
    Mem(Mem),
    Instance(Instance),
    Node(Node),
    Connect(Connect),
    PartialConnect(Connect),
    Invalidate(Invalidate),
    Attatch(Attatch),
    Conditional(Conditional),
    Stop(Stop),
    Printf(Printf),
    Empty(Empty),
    Group(Vec<Stmt>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Wire {
    id: IDStr,
    ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Reg {
    id: IDStr,
    ty: Type,
    clk: IDStr,
    init: RegInit,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RegInit {
    signal: Expr,
    val: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mem {
    id: IDStr,
    opts: MemOpts,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemOpts {
    data_type: Type,
    depth: usize,
    read_latency: usize,
    write_latency: usize,
    read_under_write: ReadUnderWrite,
    r_ports: Vec<IDStr>,
    w_ports: Vec<IDStr>,
    rw_ports: Vec<IDStr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReadUnderWrite {
    Old,
    New,
    Undefined,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Instance {
    id: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    id: String,
    rhs: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Connect {
    lhs: Expr,
    rhs: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Invalidate {
    lhs: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attatch {
    exprs: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Conditional {
    cond: Expr,
    if_true: Box<Stmt>,
    if_false: Box<Stmt>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Stop {
    clk: Expr,
    halt: Expr,
    exit_code: i32,
    id: Option<IDStr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Printf {
    clk: Expr,
    print_signal: Expr,
    fmt: String,
    params: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Empty {}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Ref(IDStr),
    Field(Box<Expr>, IDStr),
    Access(Box<Expr>, usize),
    DynAccess(Box<Expr>, Box<Expr>),
    Mux(Box<Expr>, Box<Expr>, Box<Expr>),
    CondValid(Box<Expr>, Box<Expr>),
    PrimOp(PrimOpExpr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    UInt(LitVal, Option<usize>),
    SInt(LitVal, Option<usize>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum LitVal {
    Hex(String),
    Oct(String),
    Bin(String),
    Dec(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrimOpExpr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    Leq(Box<Expr>, Box<Expr>),
    Gt(Box<Expr>, Box<Expr>),
    Geq(Box<Expr>, Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    Neq(Box<Expr>, Box<Expr>),
    Pad(Box<Expr>, usize),
    AsUInt(Box<Expr>),
    AsSInt(Box<Expr>),
    AsFixed(Box<Expr>),
    AsClock(Box<Expr>),
    Shl(Box<Expr>, usize),
    Shr(Box<Expr>, usize),
    DynShl(Box<Expr>, Box<Expr>),
    DynShr(Box<Expr>, Box<Expr>),
    ArithCvtSigned(Box<Expr>),
    Neg(Box<Expr>),
    Not(Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Xor(Box<Expr>, Box<Expr>),
    Andr(Box<Expr>),
    Orr(Box<Expr>),
    Xorr(Box<Expr>),
    Concat(Box<Expr>, Box<Expr>),
    Bits(Box<Expr>, usize, usize),
    Head(Box<Expr>, usize),
    Tail(Box<Expr>, usize),
    IncP(Box<Expr>, usize),
    DecP(Box<Expr>, usize),
    SetP(Box<Expr>, usize),
}

#[derive(Debug, Copy, PartialEq, Clone)]
pub enum PrimOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Lt,
    Leq,
    Gt,
    Geq,
    Eq,
    Neq,
    Pad,
    AsUInt,
    AsSInt,
    AsClock,
    AsFixed,
    Shl,
    Shr,
    DynShl,
    DynShr,
    ArithCvtSigned,
    Neg,
    Not,
    And,
    Or,
    Xor,
    Andr,
    Orr,
    Xorr,
    Concat,
    Bits,
    Head,
    Tail,
    IncP,
    DecP,
    SetP,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

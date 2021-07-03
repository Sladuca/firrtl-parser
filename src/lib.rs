mod string;
mod types;
mod expr;

// String that represents a name or identifier given to an object according to FIRRTL spec
// [a-zA-Z_][\w_]+
pub type IDStr = String;

/// FIRRTL info item, without the surrounding `@[]`.
pub type Info = String;

#[derive(Debug, Clone)]
pub struct Circuit {
    id: IDStr,
    infos: Vec<Info>,
    modules: Vec<Module>,
}

#[derive(Debug, Clone)]
pub struct Module {
    id: IDStr,
    infos: Vec<Info>,
    ports: Vec<Port>,
    stmt: Stmt,
}

#[derive(Debug, Clone)]
pub struct Port {
    direction: Direction,
    id: IDStr,
    ty: Type,
    infos: Vec<Info>,
}

#[derive(Debug, Clone)]
pub enum Direction {
    I,
    O,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Field {
    Flipped(FieldInner),
    Default(FieldInner),
}

#[derive(Debug, Clone)]
pub struct FieldInner {
    id: IDStr,
    ty: Type,
    infos: Vec<Info>,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Wire {
    id: IDStr,
    ty: Type,
    infos: Vec<Info>,
}

#[derive(Debug, Clone)]
pub struct Reg {
    id: IDStr,
    ty: Type,
    clk: IDStr,
    init: RegInit,
    infos: Vec<Info>,
}

#[derive(Debug, Clone)]
pub struct RegInit {
    signal: Expr,
    val: Expr,
}

#[derive(Debug, Clone)]
pub struct Mem {
    id: IDStr,
    infos: Vec<Info>,
    opts: MemOpts,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum ReadUnderWrite {
    Old,
    New,
    Undefined,
}

#[derive(Debug, Clone)]
pub struct Instance {
    id: String,
    infos: Vec<Info>,
}

#[derive(Debug, Clone)]
pub struct Node {
    id: String,
    rhs: Expr,
    infos: Vec<Info>,
}

#[derive(Debug, Clone)]
pub struct Connect {
    lhs: Expr,
    rhs: Expr,
    infos: Vec<Info>,
}

#[derive(Debug, Clone)]
pub struct Invalidate {
    lhs: Expr,
    infos: Vec<Info>,
}

#[derive(Debug, Clone)]
pub struct Attatch {
    exprs: Expr,
    infos: Vec<Info>,
}

#[derive(Debug, Clone)]
pub struct Conditional {
    cond: Expr,
    infos: Vec<Info>,
    if_true: Box<Stmt>,
    if_false: Box<Stmt>,
}

#[derive(Debug, Clone)]
pub struct Stop {
    clk: Expr,
    halt: Expr,
    exit_code: i32,
    id: Option<IDStr>,
    infos: Vec<Info>,
}

#[derive(Debug, Clone)]
pub struct Printf {
    clk: Expr,
    print_signal: Expr,
    fmt: String,
    params: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub struct Empty {
    infos: Vec<Info>,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Literal {
    UInt(LitVal, Option<usize>),
    SInt(LitVal, Option<usize>),
}

#[derive(Debug, Clone)]
pub enum LitVal {
    Hex(String),
    Oct(String),
    Bin(String),
    Dec(String),
}

#[derive(Debug, Clone)]
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
    SetP(Box<Expr>, usize)
}

#[derive(Debug, Copy, Clone)]
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
    SetP
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

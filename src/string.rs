use crate::{IDStr, Info, Literal, Module, PrimOp};

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while},
    character::complete::space0,
    combinator::{opt, success, value},
    error::ErrorKind,
    multi::separated_list0,
    regexp::str::{re_capture, re_find},
    sequence::{delimited, preceded, terminated},
    IResult,
};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref HEX_REGEX: Regex = Regex::new(r"h(-?[a-fA-F0-9]*)").unwrap();
    pub static ref OCT_REGEX: Regex = Regex::new(r"o(-?[0-7]*)").unwrap();
    pub static ref BIN_REGEX: Regex = Regex::new(r"o(-?[01]*)").unwrap();
    pub static ref DEC_REGEX: Regex = Regex::new(r"(-?)#([0-9]*)").unwrap();
    pub static ref WHITESPACE_REGEX: Regex = Regex::new(r"[, ]+").unwrap();
    pub static ref ID_REGEX: Regex = Regex::new(r"[a-zA-Z_][\w_]*").unwrap();
}

fn parse_dec_literal_str(input: &str) -> IResult<&str, &str> {
    let (rest, groups) = re_capture(DEC_REGEX.clone())(input)?;
    Ok((rest, groups[0]))
}

fn parse_int_literal_str<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    let bin = |input: &'a str| {
        let inner = re_capture::<'a>(BIN_REGEX.clone());
        let (rest, groups) = inner(input)?;
        Ok((rest, groups[0]))
    };

    let hex = |input: &'a str| {
        let inner = re_capture::<'a>(HEX_REGEX.clone());
        let (rest, groups) = inner(input)?;
        Ok((rest, groups[0]))
    };

    let oct = |input: &'a str| {
        let inner = re_capture::<'a>(OCT_REGEX.clone());
        let (rest, groups) = inner(input)?;
        Ok((rest, groups[0]))
    };

    alt((bin, hex, oct, parse_dec_literal_str))(input)
}

fn parse_str_sint_literal(input: &str) -> IResult<&str, Literal> {
    let (rest, lit_str) = parse_int_literal_str(input)?;
    let (sign, magnitude) = match &lit_str[0..1] {
        "-" => (-1, &lit_str[1..]),
        _ => (1, &lit_str[..]),
    };

    // TODO: use a crate to parse magnitude into a Vec<u8>
    unimplemented!()
}

pub fn parse_str_uint_literal(input: &str) -> IResult<&str, Literal> {
    let (rest, lit_str) = parse_int_literal_str(input)?;

    // TODO: use a crate to parse lit_str into a Vec<u8>
    unimplemented!()
}

fn parse_info(input: &str) -> IResult<&str, Info> {
    let (rest, info_str) = preceded(tag("&"), delimited(tag("["), is_not("]"), tag("]")))(input)?;

    Ok((rest, info_str.to_string()))
}

pub fn parse_infos(input: &str) -> IResult<&str, Vec<Info>> {
    separated_list0(opt(re_find(WHITESPACE_REGEX.clone())), parse_info)(input)
}

pub fn parse_id(input: &str) -> IResult<&str, IDStr> {
    let (rest, id_str) = re_find(ID_REGEX.clone())(input)?;
    Ok((rest, id_str.to_string()))
}

pub fn parse_primop(input: &str) -> IResult<&str, PrimOp> {
    let add = preceded(tag("add"), success(PrimOp::Add));
    let sub = preceded(tag("sub"), success(PrimOp::Sub));
    let mul = preceded(tag("mul"), success(PrimOp::Mul));
    let div = preceded(tag("div"), success(PrimOp::Div));
    let _mod = preceded(tag("mod"), success(PrimOp::Mod));
    let lt = preceded(tag("lt"), success(PrimOp::Lt));
    let gt = preceded(tag("gt"), success(PrimOp::Gt));
    let geq = preceded(tag("Geq"), success(PrimOp::Geq));
    let eq = preceded(tag("eq"), success(PrimOp::Eq));
    let neq = preceded(tag("neq"), success(PrimOp::Neq));
    let pad = preceded(tag("pad"), success(PrimOp::Pad));
    let as_uint = preceded(tag("asUInt"), success(PrimOp::AsUInt));
    let as_sint = preceded(tag("asSInt"), success(PrimOp::AsSInt));
    let as_clock = preceded(tag("asClock"), success(PrimOp::AsClock));
    let shl = preceded(tag("shl"), success(PrimOp::Shl));
    let shr = preceded(tag("shr"), success(PrimOp::Shr));
    let dyn_shl = preceded(tag("dshl"), success(PrimOp::DynShl));
    let dyn_shr = preceded(tag("dshr"), success(PrimOp::DynShr));
    let cvt = preceded(tag("cvt"), success(PrimOp::ArithCvtSigned));
    let neg = preceded(tag("neg"), success(PrimOp::Neg));
    let not = preceded(tag("not"), success(PrimOp::Not));
    let and = preceded(tag("and"), success(PrimOp::And));
    let or = preceded(tag("or"), success(PrimOp::Or));
    let xor = preceded(tag("xor"), success(PrimOp::Xor));
    let andr = preceded(tag("andr"), success(PrimOp::Andr));
    let orr = preceded(tag("orr"), success(PrimOp::Orr));
    let xorr = preceded(tag("xorr"), success(PrimOp::Xorr));
    let concat = preceded(tag("cat"), success(PrimOp::Concat));
    let bits = preceded(tag("bits"), success(PrimOp::Bits));
    let head = preceded(tag("head"), success(PrimOp::Head));
    let tail = preceded(tag("tail"), success(PrimOp::Tail));

    alt((
        alt((add, sub, mul, div, _mod)),
        alt((lt, gt, geq, eq, neq)),
        alt((pad, as_uint, as_sint, as_clock, cvt)),
        alt((
            shl, shr, dyn_shl, dyn_shr, neg, not, and, or, xor, andr, orr, xorr,
        )),
        alt((concat, bits, head, tail)),
    ))(input)
}

pub fn parse_decimal_usize(input: &str) -> IResult<&str, usize> {
    unimplemented!()
}

pub fn parse_width(input: &str) -> IResult<&str, usize> {
    delimited(tag("<"), parse_decimal_usize, tag(">"))(input)
}

pub fn parse_fixed_point_bits(input: &str) -> IResult<&str, usize> {
    delimited(tag("<"), parse_width, tag(">"))(input)
}

// pub fn module(input: &str) -> IResult<&str, Module> {
// 	let (rest, id) = preceeded(
// 		tag("module"),
// 		preceded(
// 			space0,
// 			terminated(
// 				parse_id,
// 				tag(":")
// 			)
// 		)
// 	)(input)?;

// 	let (res, infos) =

// 	let (rest, contents) =
// }

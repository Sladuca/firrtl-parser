use crate::{IDStr, Info, LitVal, Module, PrimOp};

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while},
    character::complete::digit1,
    combinator::{opt, success, value},
    error::{ErrorKind, make_error},
    multi::separated_list0,
    regexp::str::{re_capture, re_find},
    sequence::{delimited, preceded, terminated, pair},
    IResult,
};

use lazy_static::lazy_static;
use regex::Regex;

fn minus_only_first(mut f: impl FnMut(&(usize, char)) -> bool) -> impl FnMut(&(usize ,char)) -> bool {
    return move |tup| {
        if tup.0 == 0 {
            tup.1 == '-' || f(tup)
        } else {
            f(tup)
        }
    }
}

fn parse_hex(input: &str) -> IResult<&str, &str> {
    match input.chars().enumerate().take_while(minus_only_first(|(i, c)| c.is_ascii_hexdigit())).last() {
        Some((i, _)) =>  Ok((&input[i+1..], &input[0..i+1])),
        None => Err(nom::Err::Error(make_error(input, ErrorKind::HexDigit)))
    }
}

fn is_oct_digit(c: char) -> bool {
    c == '0' || c == '1' || c == '2' || c == '3' || c == '4' || c == '5' || c == '6' || c == '7'
}

fn parse_oct(input: &str) -> IResult<&str, &str> {
    match input.chars().enumerate().take_while(minus_only_first(|(i, c)| is_oct_digit(*c))).last() {
        Some((i, _)) => Ok((&input[i+1..], &input[0..i+1])),
        None => Err(nom::Err::Error(make_error(input, ErrorKind::OctDigit)))
    }
}

fn parse_bin(input: &str) -> IResult<&str, &str> {
    match input.chars().enumerate().take_while(minus_only_first(|(i, c)| *c == '1' || *c == '0')).last() {
        Some((i, _)) => Ok((&input[i+1..], &input[0..i+1])),
        None => Err(nom::Err::Error(make_error(input, ErrorKind::Digit)))
    }
}

fn parse_dec(input: &str) -> IResult<&str, &str> {
    match input.chars().enumerate().take_while(minus_only_first(|(i, c)| c.is_ascii_digit())).last() {
        Some((i, _)) => Ok((&input[i+1..], &input[0..i+1])),
        None => Err(nom::Err::Error(make_error(input, ErrorKind::Digit)))
    }
}

pub fn parse_litval<'a>(input: &'a str) -> IResult<&'a str, LitVal> {
    let dec = |input: &'a str| {
        let (rest, s) = parse_dec(input)?;
        Ok((rest, LitVal::Dec(s.to_string())))
    };

    let bin = |input: &'a str| {
        let (rest, s) = parse_bin(input)?;
        Ok((rest, LitVal::Bin(s.to_string())))
    };

    let hex = |input: &'a str| {
        let (rest, s) = parse_hex(input)?;
        Ok((rest, LitVal::Hex(s.to_string())))
    };

    let oct = |input: &'a str| {
        let (rest, s) = parse_oct(input)?;
        Ok((rest, LitVal::Oct(s.to_string())))
    };

    alt((bin, hex, oct, dec))(input)
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

pub fn parse_primop_name(input: &str) -> IResult<&str, PrimOp> {
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
    let as_fixed = preceded(tag("asFixed"), success(PrimOp::AsFixed));
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
    let incp = preceded(tag("incp"), success(PrimOp::IncP));
    let decp = preceded(tag("decp"), success(PrimOp::DecP));
    let setp = preceded(tag("setp"), success(PrimOp::SetP));

    alt((
        alt((add, sub, mul, div, _mod)),
        alt((lt, gt, geq, eq, neq)),
        alt((pad, as_uint, as_sint, as_fixed, as_clock, cvt)),
        alt((
            shl, shr, dyn_shl, dyn_shr, neg, not, and, or, xor, andr, orr, xorr,
        )),
        alt((concat, bits, head, tail)),
        alt((incp, decp, setp)),
    ))(input)
}

pub fn parse_decimal_usize(input: &str) -> IResult<&str, usize> {
    let (rest, dec_str) = digit1(input)?;

    // guranteed not to fail bc dec_str is always an ascii digit 0-9
    Ok((rest, dec_str.parse().unwrap()))
}

pub fn parse_width(input: &str) -> IResult<&str, usize> {
    delimited(tag("<"), parse_decimal_usize, tag(">"))(input)
}

pub fn parse_fixed_point_bits(input: &str) -> IResult<&str, usize> {
    delimited(tag("<"), parse_width, tag(">"))(input)
}

#[cfg(test)]
mod test {
    use super::{
        parse_litval
    };
    use crate::{
        LitVal
    };

    #[test]
    pub fn test_parse_litval_valid() {
        let hex_0 = "h0";
        let hex_1 = "h80A51";
        let hex_2 = "h-d4Cf";

        let (_, res) = parse_litval(hex_0).unwrap();
        assert_eq!(LitVal::Hex("0".into()), res);

        let (_, res) = parse_litval(hex_1).unwrap();
        assert_eq!(LitVal::Hex("80A51".into()), res);

        let (_, res) = parse_litval(hex_2).unwrap();
        assert_eq!(LitVal::Hex("-d4Cf".into()), res);


        let oct_0 = "o7";
        let oct_1 = "o-01137";
        let oct_2 = "o-1234567";

        let (_, res) = parse_litval(oct_0).unwrap();
        assert_eq!(LitVal::Oct("7".into()), res);

        let (_, res) = parse_litval(oct_1).unwrap();
        assert_eq!(LitVal::Oct("-01137".into()), res);

        let (_, res) = parse_litval(oct_2).unwrap();
        assert_eq!(LitVal::Oct("-1234567".into()), res);

        let bin_0 = "b01101010101010011100100";
        let bin_1 = "b-1000100110101";

        let (_, res) = parse_litval(bin_0).unwrap();
        assert_eq!(LitVal::Bin("01101010101010011100100".into()), res);

        let (_, res) = parse_litval(bin_1).unwrap();
        assert_eq!(LitVal::Bin("-1000100110101".into()), res);

        let dec_0 = "1234";
        let dec_1 = "-1";
        let dec_2 = "-45";

        let (_, res) = parse_litval(dec_0).unwrap();
        assert_eq!(LitVal::Dec("1234".into()), res);

        let (_, res) = parse_litval(dec_1).unwrap();
        assert_eq!(LitVal::Dec("-1".into()), res);

        let (_, res) = parse_litval(dec_2).unwrap();
        assert_eq!(LitVal::Dec("-45".into()), res);
    }

    #[test]
    fn parse_litval_invalid() {
        let mut reses = Vec::new();
        
        reses.push(parse_litval("deadbeef"));
        reses.push(parse_litval("hpeef"));
        reses.push(parse_litval("hhead"));
        reses.push(parse_litval("o8"));
        reses.push(parse_litval("b2"));
        reses.push(parse_litval("-hdeadbeef"));
        reses.push(parse_litval("-b01"));

        for (i, res) in reses.iter().enumerate() {
            if let Ok((rest, res)) = res {
                panic!("Expected test {} to fail, got ({}, {:#?}) instead", i, rest, res);
            }
        }
    }


}
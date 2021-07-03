use crate::{
	Expr,
	Literal,
	LitVal,
	PrimOp,
	string::{
		parse_litval,
		parse_width,
		parse_id,
		parse_primop_name,
		parse_decimal_usize	
	}
};

use nom::{
	IResult,
	sequence::{preceded, delimited, pair, tuple, terminated},
	bytes::complete::tag,
	combinator::opt
};


pub fn parse_uint_literal(input: &str) -> IResult<&str, Expr> {
	let (rest, width) = preceded(tag("UInt"), opt(parse_width))(input)?;
	let (rest, val) = delimited(tag("("), parse_litval, tag(")"))(rest)?;
	Ok((rest, Expr::Literal(Literal::UInt(val, width))))
}

pub fn parse_sint_literal(input: &str) -> IResult<&str, Expr> {
	let (rest, width) = preceded(tag("SInt"), opt(parse_width))(input)?;
	let (rest, val) = delimited(tag("("), parse_litval, tag(")"))(rest)?;
	Ok((rest, Expr::Literal(Literal::SInt(val, width))))
}

pub fn parse_ref(input: &str) -> IResult<&str, Expr> {
	let (rest, id) = parse_id(input)?;
	Ok((rest, Expr::Ref(id)))
}

pub fn parse_sub_field(input: &str) -> IResult<&str, Expr> {
	let (rest, (expr, field)) = pair(parse_expr, parse_id)(input)?;
	Ok((rest, Expr::Field(Box::new(expr), field)))
}

pub fn parse_access(input: &str) -> IResult<&str, Expr> {
	let (rest, (expr, idx)) = pair(
		parse_expr,
		delimited(tag("["), parse_decimal_usize, tag("]"))
	)(input)?;

	Ok((rest, Expr::Access(Box::new(expr), idx)))
}

pub fn parse_dyn_access(input: &str) -> IResult<&str, Expr> {
	let (rest, (lhs, rhs)) = pair(
		parse_expr,
		delimited(tag("["), parse_expr, tag("]"))
	)(input)?;

	Ok((rest, Expr::DynAccess(Box::new(lhs), Box::new(rhs))))
}

pub fn parse_mux(input: &str) -> IResult<&str, Expr> {
	let (rest, (a, b, sel)) = preceded(
		tag("mux"), 
		delimited(
			tag("("),
			tuple((parse_expr, parse_expr, parse_expr)),
			tag(")")
		)
	)(input)?;

	Ok((rest, Expr::Mux(Box::new(a), Box::new(b), Box::new(sel))))
}

pub fn parse_cond_valid(input: &str) -> IResult<&str, Expr> {
	let (rest, (expr, cond)) = preceded(
		tag("validif"),
		pair(parse_expr, parse_expr)
	)(input)?;

	Ok((rest, Expr::CondValid(Box::new(expr), Box::new(cond))))
}

fn paren_pair<'a, T, U>(
	left: impl FnMut(&'a str) -> IResult<&'a str, T>,
	right: impl FnMut(&'a str) -> IResult<&'a str, U>
) -> impl FnMut(&'a str) -> IResult<&'a str, (T, U)> {
	delimited(tag("("), pair(terminated(left, tag(", ")), right), tag(")"))
}

pub fn parse_primop(input: &str) -> IResult<&str, Expr> {
	let (rest, op) = parse_primop_name(input)?;
	unimplemented!()
}

pub fn parse_expr(input: &str) -> IResult<&str, Expr> {
	unimplemented!()
}
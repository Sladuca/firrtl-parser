use crate::{
    string::{
        parse_decimal_usize, parse_fixed_point_bits, parse_id, parse_infos, parse_width,
        WHITESPACE_REGEX,
    },
    Field, FieldInner, Type,
};

use nom::{
    bytes::complete::tag,
    combinator::{opt, success},
    multi::separated_list1,
    regexp::str::re_find,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

pub fn parse_uint_type(input: &str) -> IResult<&str, Type> {
    let (rest, width) = preceded(tag("UInt"), opt(parse_width))(input)?;
    Ok((rest, Type::UInt { width }))
}

pub fn parse_sint_type(input: &str) -> IResult<&str, Type> {
    let (rest, width) = preceded(tag("SInt"), opt(parse_width))(input)?;
    Ok((rest, Type::SInt { width }))
}

pub fn parse_fixed_point_type(input: &str) -> IResult<&str, Type> {
    let (rest, (width, point)) = preceded(
        tag("Fixed"),
        tuple((opt(parse_width), opt(parse_fixed_point_bits))),
    )(input)?;

    Ok((rest, Type::Fixed { width, point }))
}

pub fn parse_clock_type(input: &str) -> IResult<&str, Type> {
    preceded(tag("Clock"), success(Type::Clock))(input)
}

pub fn parse_analog_type(input: &str) -> IResult<&str, Type> {
    let (rest, width) = preceded(tag("Analog"), opt(parse_width))(input)?;
    Ok((rest, Type::Analog { width }))
}

pub fn parse_field(input: &str) -> IResult<&str, Field> {
    let (rest, (flip, id, ty, infos)) = tuple((
        opt(tag("flip")),
        terminated(parse_id, tag(": ")),
        parse_type,
        parse_infos,
    ))(input)?;

    let res = match flip {
        Some(_) => Field::Flipped(FieldInner { id, ty, infos }),
        None => Field::Default(FieldInner { id, ty, infos }),
    };

    Ok((rest, res))
}

pub fn parse_bundle(input: &str) -> IResult<&str, Type> {
    let inner = separated_list1(re_find(WHITESPACE_REGEX.clone()), parse_field);
    let (rest, fields) = delimited(tag("{"), inner, tag("}"))(input)?;

    Ok((rest, Type::Bundle { fields }))
}

pub fn parse_vector(input: &str) -> IResult<&str, Type> {
    let (rest, (ty, len)) = tuple((
        parse_type,
        delimited(tag("["), parse_decimal_usize, tag("]")),
    ))(input)?;

    Ok((
        rest,
        Type::Vector {
            ty: Box::new(ty),
            len,
        },
    ))
}

pub fn parse_type(input: &str) -> IResult<&str, Type> {
    unimplemented!()
}

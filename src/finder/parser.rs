use nom::multi::separated_list0;
use nom::{
    bytes::complete::{tag, take_till},
    character::complete::digit1,
    combinator::{map_res, recognize},
    IResult,
};

fn parse_line(input: &str, start: String, ch: char) -> IResult<&str, &str> {
    let (input, _) = tag(start.as_str())(input)?;
    let (input, output) = take_till(|c| c == ch)(input)?;

    Ok((input, output))
}

fn parse_list_series(input: &str) -> IResult<&str, Vec<u8>> {
    separated_list0(tag(", "), map_res(recognize(digit1), str::parse::<u8>))(input)
}

pub fn parse_location(
    input: &str,
) -> IResult<&str, (String, String, String, String, String, Vec<u8>)> {
    let ch: char = '\r';
    let (input, name) = parse_line(input, "Nombre del receptor: ".into(), ch)?;
    let (input, address) = parse_line(input, "\r\nDirección: ".into(), ch)?;
    let (input, city) = parse_line(input, "\r\nPoblación: ".into(), ch)?;
    let (input, province) = parse_line(input, "\r\nProvincia: ".into(), ch)?;
    let (input, phone) = parse_line(input, "\r\nTelefono: ".into(), ch)?;
    let (input, _) = tag("\r\nSeries: ")(input)?;
    let (input, series) = parse_list_series(input)?;
    let (input, _) = tag("\r\n")(input)?;

    Ok((
        input,
        (
            name.to_string(),
            address.to_string(),
            city.to_string(),
            province.to_string(),
            phone.to_string(),
            series,
        ),
    ))

    // Ok((
    //     input,
    //     LotteryLocation::new(
    //         name.to_string(),
    //         address.to_string(),
    //         city.to_string(),
    //         province.to_string(),
    //         phone.to_string(),
    //         series,
    //     ),
    // ))
}

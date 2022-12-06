use std::fmt;

use nom::multi::separated_list0;
use nom::{
    bytes::complete::{tag, take_till},
    character::complete::digit1,
    combinator::{map_res, recognize},
    IResult,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LotteryLocation {
    name: String,
    address: String,
    city: String,
    province: String,
    phone: String,
    series: Vec<u8>,
}

impl LotteryLocation {
    fn new(
        name: String,
        address: String,
        city: String,
        province: String,
        phone: String,
        series: Vec<u8>,
    ) -> Self {
        LotteryLocation {
            name,
            address,
            city,
            province,
            phone,
            series,
        }
    }

    pub fn parse(input: &str) -> IResult<&str, LotteryLocation> {
        let (input, _) = tag("Nombre del receptor: ")(input)?;
        let (input, name) = take_till(|c| c == '\r')(input)?;
        let (input, _) = tag("\r\nDirección: ")(input)?;
        let (input, address) = take_till(|c| c == '\r')(input)?;
        let (input, _) = tag("\r\nPoblación: ")(input)?;
        let (input, city) = take_till(|c| c == '\r')(input)?;
        let (input, _) = tag("\r\nProvincia: ")(input)?;
        let (input, province) = take_till(|c| c == '\r')(input)?;
        let (input, _) = tag("\r\nTelefono: ")(input)?;
        let (input, phone) = take_till(|c| c == '\r')(input)?;
        let (input, _) = tag("\r\nSeries: ")(input)?;
        let (input, series) = Self::parse_list_series(input)?;
        let (input, _) = tag("\r\n")(input)?;

        Ok((
            input,
            LotteryLocation::new(
                name.to_string(),
                address.to_string(),
                city.to_string(),
                province.to_string(),
                phone.to_string(),
                series,
            ),
        ))
    }

    fn parse_list_series(input: &str) -> IResult<&str, Vec<u8>> {
        separated_list0(tag(", "), map_res(recognize(digit1), str::parse::<u8>))(input)
    }
}

impl fmt::Display for LotteryLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let series = self
            .series
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        let string = format!(
            "Name: {}\nAddress: {}\nCity: {}\nProvince: {}\nPhone: {}\nSeries: {}",
            if !self.name.is_empty() {
                &self.name
            } else {
                "[Not set]"
            },
            self.address,
            self.city,
            self.province,
            self.phone,
            series
        );

        write!(f, "{}", string)
    }
}

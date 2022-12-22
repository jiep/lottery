use std::fmt;

use nom::IResult;
use serde::{Deserialize, Serialize};

use super::parser::parse_location;

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
        let (input, (name, address, city, province, phone, series)) = parse_location(input)?;

        Ok((
            input,
            LotteryLocation::new(name, address, city, province, phone, series),
        ))
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

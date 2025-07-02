use std::fmt;

use nom::Parser;
use nom::{bytes::complete::tag, multi::separated_list1, IResult};
use serde::{Deserialize, Serialize};

use crate::common::{consts::FIND_URL, url::make_find_url};

use super::location::LotteryLocation;

#[derive(Serialize, Deserialize, Debug)]
pub struct Lottery {
    pub number: u32,
    pub locations: Vec<LotteryLocation>,
}

impl Lottery {
    pub fn new(number: u32) -> Self {
        Lottery {
            number,
            locations: vec![],
        }
    }

    fn parse(input: &str) -> IResult<&str, Vec<LotteryLocation>> {
        let (input, _) = tag("\r\n")(input)?;
        separated_list1(tag("\r\n"), LotteryLocation::parse).parse(input)
    }

    fn make_request(url: &str, number: u32) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{url}{number:0>5}");
        let res = reqwest::blocking::get(url)?;
        let input = res.text().unwrap();

        Ok(input)
    }

    pub fn load_from_draw_id(
        draw_id: u32,
        number: u32,
    ) -> Result<Lottery, Box<dyn std::error::Error>> {
        let url = make_find_url(FIND_URL, draw_id);
        let input = Self::make_request(&url, number);

        if let Ok((rest, locations)) = Self::parse(input?.as_str()) {
            assert!(rest.is_empty());
            return Ok(Lottery { number, locations });
        }

        Ok(Lottery {
            number,
            locations: vec![],
        })
    }

    pub fn parse_to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}

impl fmt::Display for Lottery {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted_string = self
            .locations
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n\n");
        write!(
            f,
            "Locations for {:0>5}\n\n{formatted_string}", self.number
        )
    }
}

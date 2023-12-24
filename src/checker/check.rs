use std::collections::HashMap;

use super::prize::JSONResponse;

use crate::common::consts::CHECK_URL;
use crate::common::url::make_check_url;

fn make_request(draw_id: u32) -> Result<String, Box<dyn std::error::Error>> {
    let url = make_check_url(CHECK_URL, draw_id);
    let res = reqwest::blocking::get(url)?;
    let input = res.text().unwrap();

    Ok(input)
}

fn get_prizes(draw_id: u32) -> Result<HashMap<String, u32>, Box<dyn std::error::Error>> {
    let data = make_request(draw_id)?;
    let json: JSONResponse = serde_json::from_str(&data)?;
    let prizes: HashMap<String, u32> = json
        .compruebe
        .iter()
        .map(|x| (x.decimo.clone(), x.prize))
        .collect();

    Ok(prizes)
}

pub fn check(
    draw_id: u32,
    to_check: Vec<String>,
) -> Result<Vec<(String, u32)>, Box<dyn std::error::Error>> {
    let mut out: Vec<(String, u32)> = vec![];
    let prizes = get_prizes(draw_id)?;
    for x in to_check.iter() {
        let number = format!("{:0>6}", x);
        let prize = prizes.get(&number).unwrap_or(&0_u32);
        out.push((x.to_string(), *prize));
    }

    Ok(out)
}

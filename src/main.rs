use std::env;
use std::fs;
use regex::Regex;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    for i in 1..args.len() {
        let tmp_html = fs::read_to_string(&args[i])?;
        let (mut tmp_date, tmp_title)  = data(tmp_html)?;
        tmp_date.push_str("- "); tmp_date.push_str(&tmp_title);  tmp_date.push_str(".html");
        rename(&args[i], &tmp_date)?;
    }
    Ok(())
}

fn data(html: String) -> Result<(String, String), std::io::Error>{
    // let mat_date = reg_date.captures(&html).unwrap();
    // let ret_date = mat_date.get(1).unwrap().as_str();
    let mut ret_date: String = "1970 01 01 00 00 00 ".to_string();
    let reg_date = Regex::new(r"(?:https:\/\/web.archive.org\/web\/)(([0-9]{4})([0-9]{2})([0-9]{2})([0-9]{2})([0-9]{2})([0-9]{2}))(?:\/)").unwrap().captures(&html);
    if !reg_date.is_none() {
        ret_date = "".to_string();
        let reg_dates = reg_date.unwrap();
        for i in 2..reg_dates.len() {
            ret_date.push_str(reg_dates.get(i).map_or("", |m| m.as_str())); ret_date.push_str(" ");
        }
    }
    ret_date.replace_range(4..5, "-"); ret_date.replace_range(7..8, "-"); ret_date.replace_range(13..14, "_"); ret_date.replace_range(16..17, "_");
    let reg_title: Regex = Regex::new(r"(?:<title>)(.*)(?:<\/title>)").unwrap();
    let mat_title: regex::Captures<'_> = reg_title.captures(&html).unwrap();
    let ret_title: String = mat_title.get(1).unwrap().as_str().to_owned();
    Ok((String::from(ret_date), ret_title ))
}

fn rename(route: &str, replacer: &str) -> Result<bool, std::io::Error>{
    let last: &str = route.split('/').last().unwrap();
    fs::rename(route, route.replace(last, replacer))?; 
    Ok(true)
}
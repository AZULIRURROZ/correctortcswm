use std::env;
use std::fs;
use regex::Regex;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

fn main() -> std::io::Result<()> {
    let mut tmp_args: Vec<String> = env::args().collect();
    let mut tmp_result = "%Y-%m-%dT%H:%M:%SZ - %7title7".to_string();
    if tmp_args.len()>1{
        if tmp_args.get(1).unwrap().contains("%"){
            tmp_result = tmp_args.get(1).unwrap().to_string();
            tmp_args.remove(1);
        }
    }
    for i in 1..tmp_args.len() {
        let tmp_html = fs::read_to_string(&tmp_args[i]).expect(&tmp_args[i]);
        let mut tmp_title  = data(tmp_html, &tmp_result).expect(&tmp_args[i]);
        tmp_title.push_str(".html");
        rename(&tmp_args[i], &tmp_title).expect(&tmp_args[i]);
    }
    println!("Ok! {} - the sum of renamed files. ", tmp_args.len()-1);
    Ok(())
}

fn data(html: String, result: &str) -> Result<String, std::io::Error>{
    let reg_date_url = Regex::new(r"(?:(?:https|http):\/\/web.archive.org\/web\/)(([0-9]{4})([0-9]{2})([0-9]{2})([0-9]{2})([0-9]{2})([0-9]{2}))(?:\/)").unwrap().captures(&html);
    let reg_date_txt = Regex::new(r"(?:(?:saved date: )(\w{3} \w{3} \d{1,2} \d{4} \d{2}:\d{2}:\d{2} GMT(?:\+|\-)\d{4}))").unwrap().captures(&html);
    let mut tmp_datetime: DateTime<Utc> = chrono::offset::Utc::now();
    if !reg_date_url.is_none() {
        let tmp_match_date = reg_date_url.unwrap().get(1).unwrap().as_str().to_string();
        let tmp_naivedatetime = NaiveDateTime::parse_from_str(&tmp_match_date, "%Y%m%d%H%M%S").unwrap();
        tmp_datetime = Utc.from_utc_datetime(&tmp_naivedatetime);
     } else if !reg_date_txt.is_none() {
        let tmp_match_date = reg_date_txt.unwrap().get(1).unwrap().as_str().to_string();
        tmp_datetime = DateTime::parse_from_str(&tmp_match_date, "%a %b %e %Y %H:%M:%S GMT%z").unwrap().with_timezone(&Utc);
    } 
    let mut tmp_subtitle: String = "".to_string();
    let reg_title= Regex::new(r"(?:<title>)(.*)(?:<\/title>)").unwrap().captures(&html);
    if !reg_title.is_none() {
        let tmp_match_title: regex::Captures<'_> = reg_title.unwrap();
        let reg_untitle = Regex::new(r"[^a-zA-Z0-9 _-]").unwrap();
        tmp_subtitle = reg_untitle.replace_all(tmp_match_title.get(1).unwrap().as_str(), "_").to_string();
    }
    tmp_subtitle = result.replace("%7title7", &tmp_subtitle);
    tmp_subtitle = tmp_datetime.format(&tmp_subtitle).to_string();
    Ok(tmp_subtitle)
}

fn rename(fun_route: &str, fun_replace: &str) -> Result<bool, std::io::Error>{
    let tmp_last: &str = fun_route.split('/').last().unwrap();
    fs::rename(fun_route, fun_route.replace(tmp_last, fun_replace))?; 
    Ok(true)
}

use super::utils;
extern crate regex;
use self::regex::Regex;
use std::error;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::str;

pub fn read(
    fname: String,
    aux_fname: String,
    alias_fname: String,
    attrs: Vec<&str>,
    url: String,
) -> Result<(), Box<error::Error>> {
    let xml_tag_regex =
        |tag: String| -> Regex { Regex::new(&format!(r"<{}>(.+?)</{}>", tag, tag)).unwrap() };
    let regexes: Vec<Regex> = attrs
        .iter()
        .into_iter()
        .map(|x| xml_tag_regex(x.to_string()))
        .collect();
    let cdata_re = Regex::new(r"<!\[CDATA\[(.+?)\]\](.+?)?>")?;
    let aux_f = OpenOptions::new().read(true).open(&aux_fname)?;
    let mut aux_reader = BufReader::new(aux_f);
    let f = OpenOptions::new().read(true).open(fname)?;
    let mut reader = BufReader::new(f);
    let n_items = utils::count_items(&aux_fname)?;

    let mut feed_url = url.clone();

    if url.len() > 0 && !utils::is_url(&feed_url) {
        feed_url = utils::alias_to_url(&url, &alias_fname)?;
    }

    for offset in 0..n_items {
        let header = utils::read_aux_cell((n_items - offset - 1) as usize, &mut aux_reader)?;
        if url.len() > 0 && !utils::hashes_equal(utils::hash(&feed_url), header.feed_hash) {
            continue;
        }
        utils::read_item(header, &mut reader, &regexes, &cdata_re)?;
    }
    Ok(())
}

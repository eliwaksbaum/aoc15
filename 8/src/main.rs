use std::fs;
use regex::Regex;

fn main()
{
    let input = fs::read_to_string("input.txt").unwrap();

    //1
    let hex_match = Regex::new(r"\\x[0-9a-f]{2}").unwrap();
    let mut count = 0;
    for l in input.lines()
    {
        let l = &l[1..l.len()-1];
        let slash = l.matches(r"\\").collect::<Vec<_>>().len();
        let quote = l.matches(r#"\""#).collect::<Vec<_>>().len();
        let hex = hex_match.find_iter(l).collect::<Vec<_>>().len() * 3;
        count += slash + quote + hex + 2;
    }
    println!("{}", count);

    //2
    count = 0;
    for l in input.lines()
    {
        let slash = l.matches(r"\").collect::<Vec<_>>().len();
        let quote = l.matches(r#"""#).collect::<Vec<_>>().len();
        count += slash + quote + 2;
    }
    println!("{}", count);
}

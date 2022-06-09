use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

fn main()
{
    let input = fs::read_to_string("input.txt").unwrap();
    let splits: Vec<Vec<&str>> = input.lines().map(|s| s.split(' ').collect::<Vec<&str>>()).collect();

    let mut distances = HashMap::new();
    let mut towns = HashSet::new();

    for line in splits
    {
        let a = line[0];
        let b = line[2];
        let distance = line[4];

        distances.insert((a,b), distance);
        distances.insert((b,a), distance);
        towns.insert(a);
        towns.insert(b);
    }

    for i in 
}

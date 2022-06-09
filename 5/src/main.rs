use std::fs;

fn main()
{
    let input = fs::read_to_string("input.txt").unwrap();
    let strings: Vec<&str> = input.lines().collect();

    //1
    let vowels = ["a", "e", "i", "o", "u"];    
    let doubles: Vec<String> = ('a'..='z').into_iter().map(|c| {
        let mut s =String::from(c);
        s.push(c);
        s
    }).collect();

    let mut count = 0;
    for s in &strings
    {
        let s = *s;
        //Part 1
        if s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy")
        {
            continue;
        }

        let mut vowel_count = 0;
        for v in vowels
        {
            vowel_count += s.matches(v).collect::<Vec<_>>().len();
        }
        if vowel_count < 3
        {
            continue;
        }

        for d in &doubles
        {
            if s.contains(d)
            {
                count += 1;
                break;
            }
        }
    }
    println!("{}", count);
    
    //2
    count = 0;
    let mut pairs = vec![];
    let mut sandwiches = vec![];
    for c1 in 'a'..='z'
    {
        for c2 in 'a'..='z'
        {
            let mut pair = String::from(c1);
            pair.push(c2);
            pairs.push(pair);

            let mut sandwich = String::from(c1);
            sandwich.push(c2);
            sandwich.push(c1);
            sandwiches.push(sandwich);
        }
    }

    for s in &strings
    {
        let s = *s;

        let mut sandwich_found = false;
        for sw in &sandwiches
        {
            if s.contains(sw)
            {
                sandwich_found = true;
                break;
            }
        }
        if !sandwich_found
        {
            continue;
        }

        for p in &pairs
        {
            if s.match_indices(p).collect::<Vec<_>>().len() >= 2
            {
                count += 1;
                break;
            }
        }
    }
    println!("{}", count);
}

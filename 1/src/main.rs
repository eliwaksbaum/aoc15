use std::fs;

fn main()
{
    let input = fs::read_to_string("input.txt").unwrap();

    let mut floor: i32 = 0;
    let mut pos = None;
    for (i, char) in input.char_indices()
    {
        match char
        {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => ()
        }
        if floor == -1 && pos == None
        {
            pos = Some(i+1);
        }
    }
    
    println!("{}", floor);
    println!("{}", pos.unwrap());
}
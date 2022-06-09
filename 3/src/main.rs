use std::fs;
use std::collections::HashSet;

fn main()
{
    let directions = fs::read_to_string("input.txt").unwrap();

    println!("{}", santa(&directions));
    println!("{}", robo_santa(&directions));
}

fn santa(directions: &String) -> usize
{
    let mut cur_house = (0, 0);
    
    let mut houses = HashSet::new();
    houses.insert((0, 0));

    for arrow in directions.chars()
    {
        cur_house = next_house(&cur_house, arrow);
        houses.insert(cur_house);
    }
    houses.len()
}

fn robo_santa(directions: &String) -> usize
{
    let mut santa_house = (0, 0);
    let mut robo_house = (0, 0);

    let mut houses = HashSet::new();
    houses.insert((0, 0));

    let mut toggle = 0;

    for arrow in directions.chars()
    {
        let cur_house = if toggle%2 == 0 {&mut santa_house} else {&mut robo_house};
        *cur_house = next_house(cur_house, arrow);
        houses.insert(*cur_house);

        toggle += 1;
    }
    houses.len()
}

fn next_house(cur_house: &(i32, i32), arrow: char) -> (i32, i32)
{
    let (x, y) = cur_house;
    match arrow
    {
        '^' => (*x, *y+1),
        'v' => (*x, *y-1),
        '>' => (*x+1, *y),
        '<' => (*x-1, *y),
        _ => panic!()
    }
}
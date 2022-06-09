use std::fs;
use regex::Regex;

fn main()
{
    let re = Regex::new(r"\d{1,3},\d{1,3}").unwrap();
    let input = fs::read_to_string("input.txt").unwrap();

    let instructions: Vec<Instruction> = input.lines().map(|l| {
        let coordinates: Vec<(usize, usize)> = re.find_iter(l).map(|mat| tuple_from_commapair(&l[mat.start()..mat.end()])).collect();
        let flavor;
        if &l[0..6] == "toggle"
        {
            flavor = IType::Toggle;
        }
        else if &l[5..8] == "off"
        {
            flavor = IType::Off;
        }
        else
        {
            flavor = IType::On;
        }
        Instruction { flavor: flavor, top_left: coordinates[0], bottom_right: coordinates[1] }
    }).collect();

    //1
    let mut lights = [[false;1000];1000];
    for inst in &instructions
    {
        for x in inst.top_left.0..=inst.bottom_right.0
        {
            for y in inst.top_left.1..=inst.bottom_right.1
            {
                lights[x][y] = match inst.flavor {
                    IType::Toggle =>  !lights[x][y],
                    IType::Off => false,
                    IType::On => true
                }
            }
        }
    }
    
    let mut count = 0;
    for i in 0..1000
    {
        for j in 0..1000
        {
            if lights[i][j]
            {
                count += 1;
            }
        }
    }
    println!("{}", count);

    //2
    let mut lights = [[0;1000];1000];
    for inst in &instructions
    {
        for x in inst.top_left.0..=inst.bottom_right.0
        {
            for y in inst.top_left.1..=inst.bottom_right.1
            {
                lights[x][y] = match inst.flavor {
                    IType::Toggle =>  lights[x][y] + 2,
                    IType::Off => 0.max(lights[x][y] - 1),
                    IType::On => lights[x][y] + 1
                }
            }
        }
    }
    
    let mut strength = 0;
    for i in 0..1000
    {
        for j in 0..1000
        {
            strength += lights[i][j];
        }
    }
    println!("{}", strength);

}

struct Instruction
{
    flavor: IType,
    top_left: (usize, usize),
    bottom_right: (usize, usize)
}

enum IType
{
    On,
    Off,
    Toggle
}

fn tuple_from_commapair(s: &str) -> (usize, usize)
{
    let splits: Vec<usize> = s.split(",").map(|sp| str::parse(sp).unwrap()).collect();
    (splits[0], splits[1])
}

// struct Region
// {
//     x: (u16, u16),
//     y: (u16, u16)
// }

// impl Region
// {
//     fn overlaps(&self, &other: Region)
//     {
//         let overlaps_x = 
//     }
// }
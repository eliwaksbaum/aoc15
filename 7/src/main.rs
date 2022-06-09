use std::fs;
use std::collections::HashMap;

fn main()
{
    let input = fs::read_to_string("input.txt").unwrap();

    let mut wire_map = HashMap::new();
    wire_map.insert("ONE", Wire::Ready(1));
    let mut wires = vec![];

    for l in input.lines()
    {
        let left_right: Vec<&str> = l.split(" -> ").collect();
        let operands: Vec<&str> = left_right[0].split(" ").collect();
        
        let wire_name = left_right[1];
        let wire_value = match operands.len() {
            1 => match str::parse(operands[0]) {
                Result::Ok(signal) => Wire::Ready(signal),
                Result::Err(_) => Wire::Waiting(Gate::Copy(operands[0]))
            },
            2 => Wire::Waiting(Gate::Not(operands[1])),
            3 => match operands[1] {
                "AND" => if operands[0] == "1" {
                    Wire::Waiting(Gate::And("ONE", operands[2]))
                } else {
                    Wire::Waiting(Gate::And(operands[0], operands[2]))
                },
                "OR" => Wire::Waiting(Gate::Or(operands[0], operands[2])),
                "RSHIFT" => Wire::Waiting(Gate::Rshift(operands[0], str::parse(operands[2]).unwrap())),
                "LSHIFT" => Wire::Waiting(Gate::Lshift(operands[0], str::parse(operands[2]).unwrap())),
                _ => panic!()
            },
            _ => panic!()
        };

        wire_map.insert(wire_name, wire_value);
        wires.push(wire_name);
    }

    let mut a_value;
    let mut cur_map = step_through(&wire_map);
    loop
    {
        match cur_map["a"]
        {
            Wire::Ready(value) => {a_value = value; break;},
            Wire::Waiting(_) => {cur_map = step_through(&cur_map);}
        }
    }
    println!("{}", a_value);

    wire_map.insert("b", Wire::Ready(a_value));
    cur_map = wire_map;
    loop
    {
        match cur_map["a"]
        {
            Wire::Ready(value) => {a_value = value; break;},
            Wire::Waiting(_) => {cur_map = step_through(&cur_map);}
        }
    }
    println!("{}", a_value);

}

enum Wire<'a>
{
    Ready (u32),
    Waiting (Gate<'a>)
}

enum Gate<'a>
{
    And (&'a str, &'a str),
    Or (&'a str, &'a str),
    Not (&'a str),
    Rshift (&'a str, u8),
    Lshift (&'a str, u8),
    Copy (&'a str)
}

fn step_through<'a> (table: &HashMap<&'a str, Wire<'a>>) -> HashMap<&'a str, Wire<'a>>
{
    let mut next_map = HashMap::new();
    for (name, value) in table
    {
        let update = match value {
            Wire::Ready(s) => Wire::Ready(*s),
            Wire::Waiting(gate) => match gate {
                Gate::And(s, t) => {
                    match (table.get(s).unwrap(), table.get(t).unwrap()) {
                        (Wire::Ready(a), Wire::Ready(b)) => Wire::Ready(*a & *b),
                        _ => Wire::Waiting(Gate::And(*s, *t))
                    }
                },
                Gate::Or(s, t) => {
                    match (table.get(s).unwrap(), table.get(t).unwrap()) {
                        (Wire::Ready(a), Wire::Ready(b)) => Wire::Ready(*a | *b),
                        _ => Wire::Waiting(Gate::Or(*s, *t))
                    }
                },
                Gate::Not(s) => {
                    match table.get(s).unwrap() {
                        Wire::Ready(a) => Wire::Ready(!*a),
                        _ => Wire::Waiting(Gate::Not(*s))
                    }
                },
                Gate::Rshift(s, p) => {
                    match table.get(s).unwrap() {
                        Wire::Ready(a) => Wire::Ready(*a >> p),
                        _ => Wire::Waiting(Gate::Rshift(*s, *p))
                    }
                },
                Gate::Lshift(s, p) => {
                    match table.get(s).unwrap() {
                        Wire::Ready(a) => Wire::Ready(*a << p),
                        _ => Wire::Waiting(Gate::Lshift(*s, *p))
                    }
                },
                Gate::Copy(s) => {
                    match table.get(s).unwrap() {
                        Wire::Ready(a) => Wire::Ready(*a),
                        _ => Wire::Waiting(Gate::Copy(*s))
                    }
                }
            }
        };
        next_map.insert(*name, update);
    }
    next_map
}

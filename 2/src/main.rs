use std::fs;

fn main()
{
    let file = fs::read_to_string("input.txt").unwrap();
    let dimensions: Vec<Vec<usize>> = file.lines().map(
        |s| s.split("x").map(
            |d| d.parse().unwrap()
        ).collect()
    ).collect();

    let mut paper = 0;
    let mut ribbon = 0;
    for v in &dimensions
    {
        let (l, w, h) = (v[0], v[1], v[2]);
        let faces = (l*w, l*h, w*h);
        
        paper += 2*(faces.0 + faces.1 + faces.2) + faces.0.min(faces.1).min(faces.2);
        ribbon += 2*(l + w + h) - 2*l.max(w).max(h) + l*w*h;
    }

    println!("{}", paper);
    println!("{}", ribbon);   
}
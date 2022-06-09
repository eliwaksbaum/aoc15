fn main()
{
    let key = vec![b'i', b'w', b'r', b'u', b'p', b'v', b'q', b'b'];

    //1
    let mut num = vec![48u8];
    loop
    {
        let data: Vec<u8> = key.iter().cloned().chain(num.iter().cloned()).collect();
        let digest = md5::compute(data);
        if digest.0[0..2] == [0; 2] && digest.0[2] < 16
        {
            break;
        }
        num = increment_byte_list(num);
    }
    let answer: Vec<char> = num.into_iter().map(|c| c as char).collect();
    println!("{:?}", answer);

    //2 (takes almost a minute :( )
    num = vec![48u8];
    loop
    {
        let data: Vec<u8> = key.iter().cloned().chain(num.iter().cloned()).collect();
        let digest = md5::compute(data);
        if digest.0[0..3] == [0; 3]
        {
            break;
        }
        num = increment_byte_list(num);
    }
    let answer: Vec<char> = num.into_iter().map(|c| c as char).collect();
    println!("{:?}", answer);
}

fn increment_byte_list(mut cur_bytes: Vec<u8>) -> Vec<u8>
{
    let n = cur_bytes.len();
    if cur_bytes[n-1] < 57
    {
        cur_bytes[n-1] += 1;
        cur_bytes
    }
    else if n == 1
    {
        vec![49, 48]
    }
    else
    {
        cur_bytes.truncate(n-1);
        let mut next = increment_byte_list(cur_bytes);
        next.push(48);
        next
    }
}
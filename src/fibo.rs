pub fn fibonacci(number: usize) -> Option<usize> {
    println!("usize MAX: {}", usize::MAX);
    match number {
        0 => Some(0),
        1 => Some(1),
        _ => {
            let mut table: Vec<usize> = vec![0; number + 1];
            table[1] = 1;
            for i in 2..=number {
                //validate if table[i - 1] + table[i - 2] is not greater than usize::MAX
                if table[i - 1] > usize::MAX - table[i - 2] {
                    return None;
                }

                table[i] = table[i - 1].wrapping_add(table[i - 2]);
            }
            Some(table[number])
        }
    }
}

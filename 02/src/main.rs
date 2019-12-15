fn find_pair(input: &Vec<i32>) -> (i32, i32) {
    for p1 in 0..=99 {
        for p2 in 0..=99 {
            if eval_program(input, p1, p2) == 19690720 {
                return (p1, p2)
            }
        }
    }

    (-1, -1)
}

fn eval_program(original_input: &Vec<i32>, p1: i32, p2: i32) -> i32 {
    let mut input = original_input.clone();
    
    input[1] = p1;
    input[2] = p2;

    let mut i = 0;
    while i < input.len() {
        if input[i] == 99 {
            break;
        }

        let op = input[i];
        let i1 = input[i+1] as usize;
        let i2 = input[i+2] as usize;
        let i3 = input[i+3] as usize;

        if op == 1 {
            input[i3] = input[i1] + input[i2];
        } else if op == 2 {
            input[i3] = input[i1] * input[i2];
        }
        
        i += 4;
    };

    input[0]
}

fn main() {
    let original_input: Vec<i32> = vec![1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,13,1,19,1,6,19,23,2,23,6,27,1,5,27,31,1,10,31,35,2,6,35,39,1,39,13,43,1,43,9,47,2,47,10,51,1,5,51,55,1,55,10,59,2,59,6,63,2,6,63,67,1,5,67,71,2,9,71,75,1,75,6,79,1,6,79,83,2,83,9,87,2,87,13,91,1,10,91,95,1,95,13,99,2,13,99,103,1,103,10,107,2,107,10,111,1,111,9,115,1,115,2,119,1,9,119,0,99,2,0,14,0];

    println!("res: {}", eval_program(&original_input, 12, 2));

    let (noun, verb) = find_pair(&original_input);
    println!("res2: {}", 100*noun + verb);
}

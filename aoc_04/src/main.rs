fn cumple(st: String) -> bool {
    let mut seen_adjacents = false;

    let mut iter = st.chars().peekable();
    for i in 0..st.len() - 1 {
        let a = iter.next().unwrap().to_string().parse::<i8>().unwrap();
        let b = iter.peek().unwrap().to_string().parse::<i8>().unwrap();

        if !seen_adjacents && a == b {
            seen_adjacents = true;
        }

        if b < a {
            return false;
        }
    }

    seen_adjacents
}

fn main() {
    let lower = 231832;
    let higher = 767346;
    let mut many = 0;

    for n in lower..=higher {
        let st = n.to_string();
        if cumple(st) {
            many += 1
        }
    }

    println!("Cumplen: {}", many);
}

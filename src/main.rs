// Reference: http://web.mit.edu/neboat/Public/6.042/randomwalks.pdf

fn main() {
    let mut n = 100; // Initial amount
    const T: u32 = 200; // Desired amount
    let mut steps = 0; // Counter

    loop {
        // Break when you reach an absorbing state
        if n == 0 || n == T {
            break;
        }

        // 0.5 for an unbiased random walk, any other p value for a biased random walk
        let s = step(0.5);
        n = n.wrapping_add(s as u32);

        for _ in 0..n {
            print!("/");
        }

        println!("");
        steps += 1;
    }

    if n == 0 {
        println!("Reached 0 after {} steps.", steps);
    } else {
        println!("Reached {} after {} steps.", T, steps);
    }
}

fn step(p: f32) -> i8 {
    let r = rand::random::<f32>();

    if r < p {
        1
    } else {
        -1
    }
}

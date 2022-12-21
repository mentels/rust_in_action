fn main() {
    for (x, y) in (0..).zip(0..) {
        if x+y > 100 {
            println!("x: {} y: {}", x, y);
            break
        }
    }

    // TODO: nested loop with label
    'outer: for x in 0.. {
        for y in 0.. {
            for z in 0.. {
                if x + y + z > 1000 {
                    println!("x: {} y: {} z: {}", x, y, z);
                    break 'outer;
                }
            }
        }
    }
}
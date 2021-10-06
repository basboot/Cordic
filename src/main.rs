/// Cordic: approximate sin and cos through coordinate rotation digital computer
/// https://www.allaboutcircuits.com/technical-articles/an-introduction-to-the-cordic-algorithm/

fn power_of_2(n: i32) -> f64 {
    assert!(n <= 0, "No positive powers allowed");
    let mut result = 1_f64;

    if n == 0 {
        1_f64
    } else {
        for i in 0..-n {
            result *= 0.5;
        }
        result
    }
}

fn main() {
    // number of iterations
    let n_tan = 10;
    // precomputed values theta_i, for which tan(theta_i) = 2^{-i} in radians, i = 0..n_tan-1
    let precomputed_tan = [0.785398163397448, 0.463647609000806, 0.244978663126864,
                                    0.124354994546761, 0.062418809995957, 0.031239833430268,
                                    0.015623728620477, 0.007812341060101, 0.003906230131967,
                                    0.001953122516479];
    // precomputed scaling factor scaling = cos(theta_0) * ... * cos(theta_{n_tan-1})
    let precomputed_scaling = 0.607253321089875;

    let desired_angle = 1.0;

    // initial values
    let mut x = 1_f64;
    let mut y = 0_f64;
    let mut z = desired_angle;  // error
    let mut sigma: f64;

    // iterate
    for i in 0..n_tan {
        // calculate sign for negative feedback
        sigma = if z > 0_f64 { 1_f64 } else { -1_f64 };

        // take iteration steps
        let x_prev = x;
        let y_prev = y;
        x = x - sigma * power_of_2(-i) * y_prev;
        y = y + sigma * power_of_2(-i) * x_prev;
        z = z - sigma * precomputed_tan[i as usize];

        println!("i={} s={} x={} y={} z={}",
            i, sigma, x, y, z);
    }

    let cos_theta = precomputed_scaling * x;
    let sin_theta = precomputed_scaling * y;

    println!("Desired angle {}, approximation error {}", desired_angle, z);
    println!("sin({}) = {}", desired_angle, sin_theta);
    println!("cos({}) = {}", desired_angle, cos_theta);
}

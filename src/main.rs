/// Cordic: approximate sin and cos through coordinate rotation digital computer
/// https://www.allaboutcircuits.com/technical-articles/an-introduction-to-the-cordic-algorithm/

fn power_of_2(n: i32) -> f64 {
    assert!(n <= 0, "No positive powers allowed");
    let mut result = 1_f64;

    if n == 0 {
        1_f64
    } else {
        for _i in 0..-n {
            result *= 0.5;
        }
        result
    }
}

fn cordic_float(theta: f64) {
    // number of iterations
    let n_tan = 10;
    // precomputed values theta_i, for which tan(theta_i) = 2^{-i} in radians, i = 0..n_tan-1
    let precomputed_tan = [0.785398163397448, 0.463647609000806, 0.244978663126864,
        0.124354994546761, 0.062418809995957, 0.031239833430268,
        0.015623728620477, 0.007812341060101, 0.003906230131967,
        0.001953122516479];
    // precomputed scaling factor scaling = cos(theta_0) * ... * cos(theta_{n_tan-1})
    let precomputed_scaling = 0.607253321089875;

    let desired_angle = theta;

    // initial values
    let mut x = 1_f64;
    let mut y = 0_f64;
    let mut z = desired_angle;  // error

    // iterate
    for i in 0..n_tan {
        // take iteration steps
        let delta_x = power_of_2(-i) * y;
        let delta_y = power_of_2(-i) * x;
        x += if z > 0_f64 { -delta_x } else { delta_x };
        y += if z > 0_f64 { delta_y } else { -delta_y };
        z += if z > 0_f64 { -precomputed_tan[i as usize] } else { precomputed_tan[i as usize] };

        println!("i={} x={} y={} z={}",
                 i, x, y, z);
    }

    let cos_theta = precomputed_scaling * x;
    let sin_theta = precomputed_scaling * y;

    println!("Desired angle {}, approximation error {}", desired_angle, z);
    println!("sin({}) = {}", desired_angle, sin_theta);
    println!("cos({}) = {}", desired_angle, cos_theta);
}

fn add_values(a: f64, delta_a: f64, sign_a: bool, sign_delta_a: bool) -> (f64, bool) {
    // signs not equal, subtract and possible swap values
    if sign_a^sign_delta_a {
        // if second value is larger, swap sign and values
        let first_value = if delta_a > a { delta_a } else { a };
        let second_value = if delta_a > a { a } else { delta_a };
        let result_sign = if delta_a > a { !sign_a } else { sign_a };
        (first_value - second_value, result_sign)
    } else {
        // signs equal, no problem, just add and keep sign
        (a + delta_a, sign_a)
    }
}

fn cordic_int(theta: f64) {
    // number of iterations
    let n_tan = 10;
    // precomputed values theta_i, for which tan(theta_i) = 2^{-i} in radians, i = 0..n_tan-1
    let precomputed_tan = [0.785398163397448, 0.463647609000806, 0.244978663126864,
        0.124354994546761, 0.062418809995957, 0.031239833430268,
        0.015623728620477, 0.007812341060101, 0.003906230131967,
        0.001953122516479];
    // precomputed scaling factor scaling = cos(theta_0) * ... * cos(theta_{n_tan-1})
    let precomputed_scaling = 0.607253321089875;

    let desired_angle = theta;

    // initial values
    let mut x = (1_f64, true);
    let mut y = (0_f64, true);
    let mut z = desired_angle;  // error

    // iterate
    for i in 0..n_tan {
        // take iteration steps
        assert!(x.0 >= 0.0, "Something went wrong x is negative");
        assert!(y.0 >= 0.0, "Something went wrong y is negative");

        let delta_x = (power_of_2(-i) * y.0, y.1);
        let delta_y = (power_of_2(-i) * x.0, x.1);
        x = add_values(x.0, delta_x.0, if z > 0_f64 { !delta_x.1 } else { delta_x.1 }, delta_x.1);
        y = add_values(y.0, delta_y.0, if z > 0_f64 { delta_y.1 } else { !delta_y.1 }, delta_y.1);
        z += if z > 0_f64 { -precomputed_tan[i as usize] } else { precomputed_tan[i as usize] };

        // println!("i={} x={} y={} z={}",
        //          i, x.0, y.0, z);
    }

    let cos_theta = precomputed_scaling * x.0;
    let sin_theta = precomputed_scaling * y.0;

    println!("Desired angle {}, approximation error {}", desired_angle, z);
    println!("sin({}) = {}", desired_angle, sin_theta);
    println!("cos({}) = {}", desired_angle, cos_theta);
}


fn main() {
    cordic_float(1.0);
    cordic_int(1.0);

    // let test: (f64, bool) = add_values(0.0, 0.0, true, false);
    //
    //
    // print!("Test {} {}", test.1, test.0);
}

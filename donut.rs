use std::{thread::sleep, time::Duration};

fn main() {

    // Constante.
    const SCREEN_WIDTH: usize = 80;
    const SCREEN_HEIGHT: usize = 22;

    const R1: f64 = 1.0;                                           // R1 is the radius of the donut.
    const R2: f64 = 2.0;                                           // R2 is the point where the donut is centred.
    const K2: f64 = 5.0;                                           // K2 is the distance between the user and the donut.
    const K1: f64 = (SCREEN_WIDTH as f64) * K2 * 1.5 / (8.0 * (R1 + R2));    // K1 is z' distance between the user and the screen.

    let mut a: f64 = 1.0;
    let mut b: f64 = 1.0;                                              // A and B are, respectively, the rotation around the x-axis and the z-axis.
    
    let theta_space: f64 = 0.07;                                     // Theta are the angle spacing to build the cercle. 
    let phi_space: f64 = 0.02;
    let pi2: f64 = 6.28;

    let ascii = ['.', ',' ,'-', '~', ':', ';', '=', '!', '*', '#', '$', '@'];
    
    // Main loop.
    loop {
        let mut output = [' '; SCREEN_WIDTH * SCREEN_HEIGHT];      // Output.
        let mut zbuffer = [0.0; SCREEN_WIDTH * SCREEN_HEIGHT];       // Z buffer for depth.
        
        // Clear screen.
        println!("\x1b[2J");
        
        a += 0.07;
        b += 0.03;

        // // Compute sinus and cosinus.
        let cos_a: f64 = a.sin();
        let sin_a: f64 = a.cos();
        let cos_b: f64 = b.cos();
        let sin_b: f64 = b.sin();

        // Rotate around the cross sectionnal circle of a torus.
        for theta in 0..(pi2 / theta_space) as usize {
            let st: f64 = ((theta as f64) * theta_space).sin();
            let ct: f64 = ((theta as f64) * theta_space).cos();

            for phi in 0..(pi2 / phi_space) as usize {
                let sp: f64 = ((phi as f64) * phi_space).sin();
                let cp: f64 = ((phi as f64) * phi_space).cos();
                
                // Precompute x, y = (R2,0,0) + (R1 cos theta, R1 sin theta, 0).
                let ox = R2 + R1 * ct;
                let oy = R1 * st;

                let x = ox * (cos_b * cp + sin_a * sin_b * sp) - oy * cos_a * sin_b;              // Final 3D x.
                let y = ox * (sin_b * cp - sin_a * cos_b * sp) + oy * cos_a * cos_b;              // Final 3D x.
                let ooz = 1.0 / (K2 + cos_a * ox * sp + sin_a * oy);                              // Final 3D 1/z.

                let xp: usize = (((SCREEN_WIDTH / 2) as f64) + K1 * x * ooz) as usize;            // x'.
                let yp: usize = (((SCREEN_HEIGHT / 2) as f64) - K1 / 2.0 * y * ooz) as usize;     // y'.
                let o = xp + SCREEN_WIDTH * yp;

                // Luminance.
                let l = cp * ct * sin_b - cos_a * ct * sp - sin_a * st + cos_b * (cos_a * st - ct * sin_a * sp);

                if l > 0.0 && o < SCREEN_WIDTH * SCREEN_HEIGHT && ooz > zbuffer[o] {
                    zbuffer[o] = ooz;
                    output[o] = ascii[if l > 0.0 { (l * 8.0) as usize } else { 0 }];
                }
            }   
        }

        
        for i in 0..output.len() {
            print!("{}", output[i]);
            if i % 80 == 0 { println!();}

        }
        
        // Sleep.
        sleep(Duration::from_millis(50));
    }
}
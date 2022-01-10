#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_variables)]

use rand::prelude::*;
use rand::seq::SliceRandom;
use rand_pcg::Pcg64;

/// Default Permutation.  The Noise Algorithm uses this permutation/lookup table to decide vector gradients.
static PERMUTATION: [u16; 512] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
    203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
    74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
    220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76,
    132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
    186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
    59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
    70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
    178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
    241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204,
    176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141,
    128, 195, 78, 66, 215, 61, 156, 180, 151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194,
    233, 7, 225, 140, 36, 103, 30, 69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234,
    75, 0, 26, 197, 62, 94, 252, 219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174,
    20, 125, 136, 171, 168, 68, 175, 74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83,
    111, 229, 122, 60, 211, 133, 230, 220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25,
    63, 161, 1, 216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188,
    159, 86, 164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147,
    118, 126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170,
    213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253,
    19, 98, 108, 110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193,
    238, 210, 144, 12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31,
    181, 199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93,
    222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180,
];

/// The Maximum Value Possible for Raw generation.
const MAX: f32 = 0.855349;
/// The Minimum Value Possible for Raw generation.
const MIN: f32 = -0.855349;

/// The Noise Configuration
/// 
/// The Centerpiece of Simple-Simplex. Once you've created a Noise Configuration, you can start generating values. 
/// Each variable of the NoiseConfig will affect the outputted value. Call the NoiseConfig::new() constructor to set it up. 
pub struct NoiseConfig {
    pub octaves: i32,
    pub x_frequency: f32,
    pub y_frequency: f32,
    pub amplitude: f32,
    pub lacunarity: f32,
    pub gain: f32,
    pub range: (f32, f32),
    pub seed: u64,
    pub permutation: [u16; 512],
}

impl NoiseConfig {
    /// Creates and new Noise Configuration and Defines FBM Values.
    /// # Examples:
    ///```rust,ignore
    /// 
    /// use simple_simplex::NoiseConfig;
    /// 
    ///     let config = NoiseConfig::new(
    ///         3, // octaves
    ///         0.01, // x_freq
    ///         0.01, // y_freq
    ///         0.05, // amplitude
    ///         2.5, // lacunarity
    ///         0.5, // gain
    ///         (256.0, 255), // outputted range (max, min)
    ///         67893402, // Seed
    ///     );
    /// ```
    ///
    /// For more information on what these values mean, please read the GitHub Documentation.
    pub fn new(
        octaves: i32,
        x_frequency: f32,
        y_frequency: f32,
        amplitude: f32,
        lacunarity: f32,
        gain: f32,
        range: (f32, f32),
        seed: u64,
    ) -> Self {
        // Define Permutation.
        let permutation: [u16; 512] = set_seed(seed);
        // Create a new noise config, prepare to return it.
        let result = NoiseConfig {
            octaves,
            x_frequency,
            y_frequency,
            amplitude,
            lacunarity,
            gain,
            range,
            seed,
            permutation,
        };
        return result;
    }

    /// Generates a range of values generated with Simplex Noise, and enhanced with Fractal Brownian Motion.
    /// Returns a simplex noise value WITH Fractal Brownian Motion (FBM) applied.  This value is also converted into a range.
    pub fn generate_range(&self, x: f32, y: f32) -> f32 {
        let (value, max_value, min_value) = self.fbm(x, y);
        return convert_range(max_value, min_value, self.range.0, self.range.1, value);
    }

    /// Generates Simplex Noise Values, and then applies Fractal Brownian Motion.
    /// Returns a Simplex Noise value WITH FBM applied. This is NOT converted into a range.
    pub fn generate_rangeless(&self, x: f32, y: f32) -> f32 {
        return self.fbm(x, y).0;
    }

    /// This method is private, and is not intended for use by an end-user.
    /// This function is used for applying Fractal Brownian Motion to Simplex Noise Values.
    fn fbm(&self, x: f32, y: f32) -> (f32, f32, f32) {
        // Values defined according to Config, but can also be changed.
        let mut x_frequency: f32 = self.x_frequency;
        let mut y_frequency: f32 = self.y_frequency;
        let mut amplitude: f32 = self.amplitude;

        // The Value each octave will be added to.
        let mut value: f32 = 0.0;

        // The Max value possible given the amplitude and gain.
        let mut max_value: f32 = 0.0;

        // the min value possible given the amplitude and gain.
        let mut min_value: f32 = 0.0;

        for i in 0..self.octaves {
            // Adds Amplitude and generates a noise value.
            value += amplitude * generate(x * x_frequency, y * y_frequency, &self.permutation);

            // Finds the max value given the gain and amplitude.
            max_value += MAX * amplitude;

            // Find the min value
            min_value += MIN * amplitude;

            // Modifys values at a specified rate.
            x_frequency *= self.lacunarity;
            y_frequency *= self.lacunarity;
            amplitude *= self.gain;
        }

        // Returns a tuple.
        // value = final product.
        // max_value, maximum value of the NoiseConfig through this FBM. 
        // min_value, the minimum value of the NoiseConfig through this FBM. 
        return (value, max_value, min_value);
    }

    /// Generates raw simplex values. 
    /// DOES NOT apply FBM.
    /// Converts return value to the range specified in the NoiseConfig. 
    pub fn generate_raw_range(&self, x: f32, y: f32) -> f32 {
        return convert_range(
            MAX,
            MIN,
            self.range.0,
            self.range.1,
            generate(x, y, &self.permutation),
        );
    }

    /// generates raw values, with no FBM or range conversion.
    /// returns a simplex noise value WITHOUT Fractal Brownian Motion and IS NOT converted into the specified range.
    pub fn generate_raw(&self, x: f32, y: f32) -> f32 {
        return generate(x, y, &self.permutation);
    }

    /// Outputs data on a noise configuration.
    /// This feature is incomplete.  Please suggest more analysis data on Github.
    pub fn analyze(&self, amount: i32) {
        let mut max: f32 = 0.0;
        let mut min: f32 = 0.0;
        let mut value: f32 = 0.0;

        for x in 0..amount {
            for y in 0..amount {
                value = self.generate_rangeless(x as f32, y as f32);
                if value > max {
                    max = value;
                } else if min > value {
                    min = value;
                }
            }
        }

        println!("Max Value: {}, Min Value: {}", max, min);
    }

    /// Output generates noise values to terminal.  Width should not be wider than your terminal can handle.
    ///
    /// Note: The `range` of the noise config you output must correlate with theh length of the `vector` of characters used.
    ///
    /// # Examples:
    /// ```rust,ignore
    ///
    /// use simple_simplex::NoiseConfig;
    ///
    /// fn main() {
    ///     let vector: Vec<char> = vec![' ',' ', ' ', '.', '-', '=', 'z','X', '#'];
    ///     let config: NoiseConfig = NoiseConfig::new(
    ///         3, // Octaves
    ///         0.01, // X-Frequency
    ///         0.01, // Y-Frequency
    ///         0.05, // Amplitude
    ///         2.5, // Lacunarity
    ///         0.5, // Gain
    ///         (0.0, (vector.len() - 1) as f32), // range, must be 0, vector.len() -1 when using output.
    ///         97838586 // seed
    ///     );
    ///
    ///     config.output(150, &vector);
    /// }
    /// ```
    pub fn output(&self, size: i32, vector: &Vec<char>) {
        // Set up variables
        let mut values: Vec<f32> = vec![];
        let mut max: f32 = 0.0;
        let mut min: f32 = 0.0;
        let mut v: f32 = 0.0;

        // Generate Values and find max/min
        for x in 0..size {
            for y in 0..size {
                v = self.generate_rangeless(x as f32, y as f32);
                values.push(v);
                if v > max {
                    max = v;
                } else if min > v {
                    min = v;
                }
            }
        }

        // Find the difference value, or the space in between
        let dif: f32 = (max + (min * -1.0)) / vector.len() as f32;

        // initialize noisemap
        let mut graph: Vec<char> = vec![];

        for v in 0..values.len() {
            for c in 0..vector.len() {
                if (values[v] > (min + (dif * c as f32)))
                    && ((min + (dif * (c as f32 + 1.0))) > values[v])
                {
                    graph.push(vector[c]);
                    break;
                }
            }
        }

        // Output the values.
        for h in 0..size - 1 {
            for w in 0..size - 1 {
                print!("{}", graph[(h + (w * size)) as usize])
            }
            println!("");
        }
    }

    /// Outputs a noisemap onto Terminal in 1d form.
    /// The `length` is the width of the outputted map. For starters,
    /// try 100.
    ///
    /// The `Height` is the height of the map. For starters, try `30`.
    /// # Examples
    /// ```rust,ignore
    /// // Create a noise config called `config`
    ///
    /// config.output_1d(30, 100);
    /// ```
    pub fn output_1d(&self, length: i32, height: i32) {
        // Set up variables
        let mut values: Vec<f32> = vec![];
        let mut max: f32 = 0.0;
        let mut min: f32 = 0.0;
        let mut v: f32 = 0.0;

        // Generate values and find max/min
        for i in 0..length {
            v = self.generate_rangeless(i as f32, 0.0);
            values.push(v);
            if v > max {
                max = v;
            } else if min > v {
                min = v;
            }
        }

        // Find the difference, or the space in between.
        let dif: f32 = (max + (min * -1.0)) / height as f32;

        // create vector to store characters for graph.
        let mut graph: Vec<char> = vec![];

        // Find out where the '*' should go.
        for v in 0..values.len() {
            for i in 0..height {
                if (values[v] > (min + (dif * i as f32)))
                    && ((min + (dif * (i as f32 + 1.0))) > values[v])
                {
                    graph.push('*');
                } else {
                    graph.push(' ');
                }
            }
        }

        // Output graph
        for c in 0..height {
            print!("|");
            for r in 0..length {
                print!("{}", graph[(c + (r * height)) as usize])
            }
            println!("");
        }

        // tidy things up
        print!("+");
        for i in 1..length {
            print!("-");
        }
        // output analysis data.
        println!(
            "\n Max Value: {}, Min Value: {}, Difference: {}",
            max, min, dif
        );

        //println!("{:?}", graph);
    }
}

/// This function is private and is not intended to be used by an end-user.
/// Scrambles a Noise Configuration's Permutation when it is created.
/// uses `rand` with `Pcg64` and SliceRandom's shuffle function to shuffle.
fn set_seed(seed: u64) -> [u16; 512] {
    if seed == 0 {
        return PERMUTATION;
    } else {
        let mut permutation = PERMUTATION;
        let mut rng = Pcg64::seed_from_u64(seed);
        permutation.shuffle(&mut rng);
        return permutation;
    }
}

/// This function is private and is not intended to be used by an end-user.
/// This function converts a value from one range to another.
fn convert_range(old_max: f32, old_min: f32, new_max: f32, new_min: f32, value: f32) -> f32 {
    let scale: f32 = (new_min - new_max) / (old_min - old_max);
    return new_max + (value - old_max) * scale;
}

// Constant for `generate`
const F2: f32 = 0.366025403;
// Constant for `generate`
const G2: f32 = 0.211324865;

/// Generate raw simplex noise values
/// # Examples
/// ```rust,ignore
/// use simple_simplex;
///
/// let value: f32 = generate(10, 10);
/// ```
///
/// use `generate` to create your own FBM implementations, or just to generate
/// raw values.
///
/// The maximum value this can generate is 0.855349, and the minimum is -0.855349.
///
/// To increase or decrease the `scale`, multiply `x` and `y` by a value less than 0.
pub fn generate(x: f32, y: f32, permutation: &[u16; 512]) -> f32 {
    let mut n0: f32 = 0.0;
    let mut n1: f32 = 0.0;
    let mut n2: f32 = 0.0;

    let s = (x + y) * F2;
    let xs = x + s;
    let ys = y + s;
    let i = fast_floor(xs);
    let j = fast_floor(ys);

    let t: f32 = ((i + j) as f32) * G2;
    let x_0 = i as f32 - t;
    let y_0 = j as f32 - t;
    let x_0 = x - x_0;
    let y_0 = y - y_0;

    let mut i1: i32 = 0;
    let mut j1: i32 = 0;
    if x_0 > y_0 {
        i1 = 1;
        j1 = 0;
    } else {
        i1 = 0;
        j1 = 1;
    }

    let x1 = x_0 - i1 as f32 + G2;
    let y1 = y_0 - j1 as f32 + G2;
    let x2 = x_0 - 1.0 + 2.0 * G2;
    let y2 = y_0 - 1.0 + 2.0 * G2;

    let ii = modulo(i, 256);
    let jj = modulo(j, 256);

    let mut t0 = 0.5 - x_0 * x_0 - y_0 * y_0;
    if t0 < 0.0 {
        n0 = 0.0;
    } else {
        t0 *= t0;
        let temp = permutation[jj as usize];
        n0 = t0 * t0 * gradient(permutation[(ii + temp as i32) as usize], x_0, y_0);
    }

    let mut t1 = 0.5 - x1 * x1 - y1 * y1;
    if t1 < 0.0 {
        n1 = 0.0;
    } else {
        t1 *= t1;
        let temp = permutation[(jj + j1) as usize];
        n1 = t1 * t1 * gradient(permutation[(ii + i1 + temp as i32) as usize], x1, y1);
    }

    let mut t2 = 0.5 - x2 * x2 - y2 * y2;
    if t2 < 0.0 {
        n2 = 0.0;
    } else {
        t2 *= t2;
        let temp = permutation[(jj + 1) as usize];
        n2 = t2 * t2 * gradient(permutation[(ii + 1 + temp as i32) as usize], x2, y2);
    }

    return 40.0 * (n0 + n1 + n2);
}

/// This function is private and is not intended to be used by an end-user
/// Function for Simplex Noise Algorithm.
/// Quickly finds the floor of a number faster than std can.
fn fast_floor(x: f32) -> i32 {
    if x > 0.0 {
        return x as i32;
    } else {
        return (x as i32) - 1;
    }
}

/// This function is private and is not intended to be used by an end-user.
/// Function for simplex noise algorithm.
/// Calculates Modulo
fn modulo(x: i32, m: i32) -> i32 {
    let a = x % m;
    if 0 > a {
        return a + m;
    } else {
        return a;
    }
}

/// This function is private and is not intended to be used by an end-user.
/// Function for simplex noise algorithm.
/// Calculates gradients.
fn gradient(hash: u16, x: f32, y: f32) -> f32 {
    let h = hash & 7;

    let mut u: f32 = if 4 > h { x } else { y };
    let v: f32 = if 4 > h { y } else { x };

    if h & 1 != 0 {
        u *= -1.0;
    }

    return u + (if h & 2 != 0 { -2.0 * v } else { 2.0 * v });
}

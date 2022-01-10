# Information about the Noise Configuration

The Noise Configuration is how you interface with the Simplex Noise Algorithm in simple-simplex. Once you've created a Noise Configuration, you can run all of its methods, such as `generate_raw()` or `generate_range()`, as well as output methods like `output()` and `output_1d()`. The Noise Configuration contains data about the desired range (if any), the seed, and Fractal Brownian Motion variables. 

# Creating a Noise Configuration

A Noise Configuration generally looks like this:

```rs

    use simple_simplex::NoiseConfig;

    let config: NoiseConfig = NoiseConfig::new(
        5, // Octaves
        0.015, // X-Frequency
        0.015, // Y-Frequency
        0.05, // Amplitude
        2.5, // Lacunarity
        0.5, // Gain
        (0.0, (vector.len() - 1) as f32), // range
        97838586 // seed
    );
```

A Noise Configuration has 8 fields to fill out. 6 of these relate to FBM, octaves, x-freq, y-freq, amplitude, lacunarity, and gain. 

Once a Noise Configuration has been created, you can use its methods. 

# What each Field Means

1. Octaves

    Refers to the amount of bleh im done with this for now, will add more docs later. 
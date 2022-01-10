# Simplex Noise Generator and Fractal Brownian Motion algorithm. 

Import with `use simple_simplex::NoiseConfig;`

Simple-Simplex is a library for generating values procedurally.  This is called `noise`.

Simple-Simplex also applies what is called `Fractal Brownian Motion`, or `FBM` to generated values. 

When many of these `noise` values are generated and put into an array/list/vector, they are called `noisemaps`. Noisemaps can be used for many things,
such as generating images, terrain in games like No Mans Sky / Minecraft, particle effects, digital art, or even procedurally generated dungeons. 

Simple-Simplex supports..

- generating noise values, applying fractal brownian motion, and converting the output into a range. 

- generating noise values without fractal brownian motion or without a range. 

- outputting Simplex Noise / FBM values directly to terminal in ascii with the `output()` and `output_1d()` methods. 

Below: Example of the `output()` method, sent in the VS Code terminal. 
![An example of output().](/images/sadtkj.png "Output Example")

Below: Example of the `output_1d()` method, sent in the VS Code Terminal. 
![An example of output_1d().](/images/thingafication.png "Output_1d Example")

`output()` and `output_1d()` allow users to understand what their final noisemap will look like for a given noise. 

# Quick-Start Guide

If you do not already understand the terms `FBM`, `Fractal Brownian Motion`, `noise`, `Simplex Noise`, or `noisemap`, I suggest you read
TERMSNDEF.md in /docs. 

This section will help you get started using simple-simplex

### 1: Create a Noise Configuration. 

simple-simplex uses what is called a `Noise Configuration`. See the example below: 

```rs

    use simple_simplex::NoiseConfig;

    let config: NoiseConfig = NoiseConfig::new(
        3, // Octaves
        0.01, // X-Frequency
        0.01, // Y-Frequency
        0.05, // Amplitude
        2.5, // Lacunarity
        0.5, // Gain
        (0.0, 255.0), // range
        4201337 // seed
    );
```

In the above image, we are creating a new NoiseConfig.  a NoiseConfig contains data about how FBM will behave. 
For a more in depth explanation of how these variable behave, see NOISECONFIG.md in /docs. 

### 2: Generate values. 

Once a Noise Configuration has been created, you're ready to start generating values!

There are a few methods available to you right off the bat:

1. `config.generate_raw()`:

    Generates a *raw* simplex noise value. By raw we mean that FBM is not applied to the result. This is not converted into a range. 

2. `config.generate_raw_range()`:

    Generates a *raw* simplex noise value. This value does not have FBM applied. The value is converted to the range specified in the Noise Configuration. 

3. `config.generate_range()`:

    Generates an FBM-applied simplex noise value.  This means FBM is applied. This value is converted into the specified range. 

4. `config.generate_rangeless()`:

    Generates an FBM-applied simplex noise value. This means FBM is applied. This value is not converted into the specified range. 

5. `config.analyze()`:

    Generates a large amount of values and prints data about the configuration's output to terminal.  Currently feature-incomplete. 
    Prints:
    - Maximum Value data. 
    - Minimum Value data. 

6. `config.output()`:

    Generates and outputs a noisemap to terminal.  Note the `size` variable refers to both the width and the height of the outputted map. Be sure not to make the map
    too wide, or it will wrap your terminal window. 

    `output()` also needs a vector of ascii characters to output. Looks like this: 
    ```rs
    let vector: Vec<char> = vec![' ', '.', '-', '=', 'z','X', '#'];
    ```

    These characters are what the method will use to output. They must be in ascending or descending order.  

7. `config.output_1d()` 

    Generates and outputs a 1d noisemap to terminal.  Don't make the `height` variable too high. A good number is `30`. Be sure not to make the `length` variable too high, or else it will wrap the terminal window. 

Now you can start generating values! 

If you have any suggestions / bug reports / feature requests / etc, please submit them as an issue. 

    





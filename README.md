# mandelbrot

To generate the `.ppm` image run the following shell command

```sh
cargo run --release > image.ppm
```

Warning: this might take a while. You can lower `image_width` (the variable at the top of `main()`) to decrease the resolution and make it run faster.

The program currently generates a 10000 x 8960 image. (might be outdated if i forget to change this!)

use boids::run;

fn main() {
    pollster::block_on(run());
}

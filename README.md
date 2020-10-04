# tache

### Example

```rust
struct Arm(Motor);

impl Arm {
  fn new() -> Self {
    Self(Motor::new(0))
  }
  
  fn run(&mut self, output: f64) {
    self.0.set_output(output);
  }
}

fn main() {
  Robot::new()
    .with(Arm::new())
    .on_start(|exec| {
      exec(|c| {
        let arm = req!(c, Arm);
        arm.move(1.0);
      });
    })
    .run();
}
```

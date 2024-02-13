
struct Temperature
{
    t1: f32,
    t2: f32,
    th: f32,
    ta: f32,
    vd: f32,
    kh: f32
}

impl Temperature {
    fn get_temp_for_time_step(self) -> f64
    {
    //     Add formula here.

        return 0.0
    }
}

fn main() {
    let timer = timer::Timer::new();
    let t: Temperature = Temperature{
        t1: 0.0,
        t2: 0.0,
        th: 0.0,
        ta: 0.0,
        vd: 0.0,
        kh: 0.0,
    };
    t.get_temp_for_time_step();
  //   timer.schedule_repeating(chrono::Duration::milliseconds(5), move || {
  //   // *count.lock().unwrap() += 1;
  //       println!("Hello, world!");
  // });
}

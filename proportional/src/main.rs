
struct Temperature
{
    t1: f32,
    t2: f32,
    th: f32,
    ta: f32,
    vd: f32
}

fn main() {
    let timer = timer::Timer::new();
    timer.schedule_repeating(chrono::Duration::milliseconds(5), move || {
    // *count.lock().unwrap() += 1;
        println!("Hello, world!");
  });
}

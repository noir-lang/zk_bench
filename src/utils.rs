use std::time::Instant;

pub(crate) fn get_time(time: Instant) -> String {
    let duration = time.elapsed();
    if duration.as_secs() > 1 {
        format!("{} s", duration.as_secs_f32())
    } else {
        format!("{} ms", duration.as_millis())
    }
}

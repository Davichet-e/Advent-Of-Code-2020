mod day_1;
mod day_2;
mod day_3;
#[allow(dead_code)]
fn main() -> std::io::Result<()> {
    day_1::day_1()?;
    day_2::day_2()?;
    day_3::day_3()?;
    Ok(())
}

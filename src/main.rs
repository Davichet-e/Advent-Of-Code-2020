mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;

fn main() -> std::io::Result<()> {
    day_1::day_1()?;
    day_2::day_2()?;
    day_3::day_3()?;
    day_4::day_4()?;
    day_5::day_5()?;
    day_6::day_6()?;
    day_7::day_7()?;
    Ok(())
}

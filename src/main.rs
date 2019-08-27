mod moonmoji;
mod clockmoji;

fn main() {
    println!("{}  {}", moonmoji::get_emoji(&None), clockmoji::get_emoji(&None));
}

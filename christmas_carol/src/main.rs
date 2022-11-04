static DAYS: [&str; 12] = ["first", "second", "third", "forth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

static DAILY_GIFTS: [&str; 12] = [
    "A song and a Christmas tree.",
    "Two candy canes,",
    "Three boughs of holly,",
    "Four colored lights,",
    "A shining star,",
    "Little silver bells,",
    "Candles a-glowing,",
    "Gold and silver tinsel,",
    "A guardian angel,",
    "Some mistletoe,",
    "Gifts for one and all,",
    "All their good wishes,"];

fn main() {
    println!("A Song And A Christmas Tree   \tby Andy Williams");
    for i in 0..(DAYS.len()) {
        println!("{}",format!("\nOn the {} day of Christmas\nMy good friends brought to me", DAYS[i]));
        for j in (0..=i).rev() {
            println!("{}",DAILY_GIFTS[j]);
        }
    }
}

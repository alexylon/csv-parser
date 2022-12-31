pub fn create_big_csv() -> String {
    let str = "Price;\"Fo\"\"od\"\n";
    let mut big_csv = "".to_string();

    for _ in 0..100_000 {
        big_csv = format!("{}{}", big_csv, str);
    }

    big_csv

}
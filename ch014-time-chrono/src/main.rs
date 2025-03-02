use std::time::{Duration, Instant};
extern crate chrono;

fn main() {
    let dur1 = Duration::from_secs(15);
    println!("{}", dur1.as_millis());
    let dur2 = Duration::from_millis(14500);
    //let dur3 = dur1.sub(dur2);
    let dur3 = dur1.checked_sub(dur2);
    println!("{}", dur3.unwrap_or_default().as_millis());

    let now = Instant::now();
    std::thread::sleep(Duration::from_millis(200));
    println!("Elapsed time {}", now.elapsed().as_nanos());

    let utc_now = chrono::Utc::now();
    println!("{}", utc_now);
    println!("{}", utc_now.format("%Z %Y-%m-%d %H:%M:%S"));

    let local_now = chrono::Local::now();
    println!("{}", local_now);
    println!("{}", local_now.format("%Z %Y-%m-%d %H:%M:%S"));
    // ISO周编号系统是一种标准，用于表示年份中的周数。在这种系统中，一年的第一周（即第1周）是包含该年第一个星期四的那一周
    // 这意味着该年的第一周至少有4天落在新一年中。同时，在这种系统中，周从星期一开始，到星期日结束。
    let date1 = chrono::NaiveDate::from_isoywd_opt(2025, 1, chrono::Weekday::Mon).unwrap();
    println!("{}", date1.format("Day of year is %j"));
    date1
        .iter_days()
        .take(4)
        .for_each(|d| {
        println!("{}", d.format("%j"))
    });
    // 从给定的年份和一年中的天数（即一年中的第几天，通常称为“year and ordinal day”或简称“yo”）创建一个NaiveDate实例
    let date2 = chrono::NaiveDate::from_yo_opt(2025, 365);
    println!("{}", date2.unwrap().format("%A"));

    let birthday = chrono::NaiveDate::parse_from_str("2024|||09|||07", "%Y|||%m|||%d");
    println!("{:#?}", birthday.ok().unwrap());
}

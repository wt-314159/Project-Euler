fn main() {
    let sundays = count_sundays();
    println!("There were {} Sundays in 20th century", sundays);
}

fn count_sundays() -> usize {
    let mut year = 1901;
    let mut month = 0; // index months starting from 0
    let mut day = 2; // January 1st 1901 was a Tuesday, represent this as 2, so Sunday is 1
    let mut count = 0;
    while year < 2001 {
        let days = get_number_of_days_in_month(month, year);
        day += days;
        day = day % 7;
        if day == 0 {
            count += 1;
        }
        month += 1;
        if month == 12 {
            year += 1;
            month = 0;
        }
    }
    count
}

fn get_number_of_days_in_month(month: usize, year: usize) -> usize {
    // Jan = 0, Feb = 1, March = 2, April = 3, May = 4, Jun = 5, Jul = 6, Aug = 7, Sep = 8, Oct = 9, Nov = 10, Dec = 11
    match month {
        8 | 3 | 5 | 10 => 30,
        0 | 2 | 4 | 6 | 7 | 9 | 11 => 31,
        1 => {
            if year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400)) {
                29
            } else {
                28
            }
        }
        x => panic!("Month {x} is out of range"),
    }
}

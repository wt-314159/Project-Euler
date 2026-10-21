use common::Integer;

fn main() {
    let nums = [1, 10, 100, 1_000, 10_000, 100_000, 1_000_000];
    let prod: u32 = nums.map(|n| get_nth_digit(n).unwrap()).iter().product();
    println!("Product is {prod}");
}

fn get_nth_digit(n: u32) -> Option<u32> {
    let mut t = 0;
    let mut d = 1;
    let mut idx = 0;
    while t < n {
        let extra_idx = 10u32.pow(d) - idx;
        let extra = d * extra_idx;
        // println!("d * 10^{d} - {t} = {extra}  -- extra_idx: {extra_idx}");
        if t + extra > n {
            // println!(
            //     "stopping, t: {t}, n - t: {}, idx: {idx}, extra_idx: {extra_idx}",
            //     n - t
            // );
            let remain = (n - t) % d;
            idx += (n - t) / d;
            // println!("remaining: {remain}, final_idx: {idx}");
            let digits = idx.digits_vec();
            return digits.get(remain as usize).copied();
        } else if t + extra == n {
            // println!("t: {t}, extra: {extra}, extra_idx: {extra_idx}");
            idx += extra_idx;
            // println!("stopping exactly, extra_idx: {extra_idx}");
            let digits = idx.digits_vec();
            return digits.get(0).copied();
        }
        // println!("t: {t}, extra: {extra}, d: {d}, extra_idx: {extra_idx}");
        t += extra;
        idx += extra_idx;
        // println!("t: {t}");
        d += 1;
    }
    None
}

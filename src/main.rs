use rand::Rng;
use std::env;


fn generate_card(bin: &str, length: usize) -> String {
    let mut rng = rand::rng();
    
    let mut bin2 = String::new();
    for ch in bin.chars() {
        if ch.to_lowercase().to_string() == "x" {
            let digit = rng.random_range(0..10);
            bin2.push_str(&digit.to_string());
        } else {
            bin2.push(ch);
        }
        
        if bin2.len() >= length - 1 {
            break;
        }
    }
    
    while bin2.len() < length - 1 {
        let digit = rng.random_range(0..10);
        bin2.push_str(&digit.to_string());
    }
    let mut card1_l: Vec<u32> = bin2.chars()
        .map(|c| c.to_digit(10).unwrap_or(0))
        .collect();
    let mut card2_l: Vec<u32> = card1_l.clone();
    
    for i in (0..card2_l.len()).rev().step_by(2) {
        card2_l[i] *= 2;
        if card2_l[i] > 9 {
            card2_l[i] -= 9;
        }
    }
    
    let sum: u32 = card2_l.iter().sum();
    
    let mod_val = sum % 10;
    let check_sum = if mod_val != 0 { 10 - mod_val } else { 0 };
    
    card1_l.push(check_sum);
    
    card1_l.iter().map(|d| d.to_string()).collect()
}

fn generate_month() -> String {
    let month_list = [
        "01", "02", "03", "04", "05", "06", 
        "07", "08", "09", "10", "11", "12"
    ];
    let mut rng = rand::rng();
    let index = rng.random_range(0..12);
    month_list[index].to_string()
}

fn generate_year() -> String {
    let year_list = [
        "2026", "2027", "2028", "2029", "2030", "2031", 
        "2032", "2033", "2034", "2035", "2036", "2037", 
        "2038", "2039", "2040"
    ];
    let mut rng = rand::rng();
    let index = rng.random_range(0..year_list.len());
    year_list[index].to_string()
}

fn generate_ccv(bin: &str) -> String {
    let ccv_length = if bin.starts_with("34") || bin.starts_with("37") {
        4
    } else {
        3
    };
    
    let mut rng = rand::rng();
    (0..ccv_length)
        .map(|_| rng.random_range(0..10).to_string())
        .collect()
}

fn generate_multiple_bins(bins: &[String], ccv: &str, month: &str, year: &str, count: usize) -> String {
    let mut results = Vec::new();
    
    for bin in bins {
        if bin.is_empty() {
            continue;
        }
        
        let ccv = if ccv.is_empty() { "Random" } else { ccv };
        let count = if count == 0 { 20 } else { count };
        
        let card_length = if bin.starts_with("34") || bin.starts_with("37") {
            15
        } else if bin.starts_with("30") || bin.starts_with("36") || bin.starts_with("38") || bin.starts_with("39") {
            14
        } else {
            16
        };
        
        for _ in 0..count {
            let generated_card = generate_card(bin, card_length);
            let generated_month = if month == "Random" { generate_month() } else { month.to_string() };
            let generated_year = if year == "Random" { generate_year() } else { year.to_string() };
            let generated_ccv = if ccv == "Random" { generate_ccv(bin) } else { ccv.to_string() };
            results.push(format!("{}|{}|{}|{}", generated_card, generated_month, generated_year, generated_ccv));
        }
    }
    
    results.join("\n")
}

fn print_usage() {
    let program = env::args().next().and_then(|s| std::path::Path::new(&s).file_name().map(|n| n.to_string_lossy().into_owned())).unwrap_or_else(|| "program".to_string());
    println!("Usage: {} <BIN1,BIN2,BIN3...> [OPTIONS]", program);
    println!("Options:");
    println!("  -c, --ccv CCV       specify CCV (default: Random)");
    println!("  -m, --month MONTH   specify month (default: Random)");
    println!("  -y, --year YEAR     specify year (default: Random)");
    println!("  -n, --count COUNT   number of cards to generate per BIN (default: 20)");
    println!();

}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        return;
    }
    
    let bins_str = &args[1];
    
    let mut ccv = "Random".to_string();
    let mut month = "Random".to_string();
    let mut year = "Random".to_string();
    let mut count = 20;
    
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "-c" | "--ccv" => {
                if i + 1 < args.len() {
                    ccv = args[i + 1].clone();
                    i += 2;
                } else {
                    println!("Error: --ccv requires a value");
                    return;
                }
            }
            "-m" | "--month" => {
                if i + 1 < args.len() {
                    month = args[i + 1].clone();
                    i += 2;
                } else {
                    println!("Error: --month requires a value");
                    return;
                }
            }
            "-y" | "--year" => {
                if i + 1 < args.len() {
                    year = args[i + 1].clone();
                    i += 2;
                } else {
                    println!("Error: --year requires a value");
                    return;
                }
            }
            "-n" | "--count" => {
                if i + 1 < args.len() {
                    match args[i + 1].parse::<usize>() {
                        Ok(num) => count = num,
                        Err(_) => {
                            println!("Error: --count must be a number");
                            return;
                        }
                    }
                    i += 2;
                } else {
                    println!("Error: --count requires a value");
                    return;
                }
            }
            "-h" | "--help" => {
                print_usage();
                return;
            }
            _ => {
                println!("Unknown option: {}", args[i]);
                print_usage();
                return;
            }
        }
    }
    
    let bins: Vec<String> = bins_str.split(',').map(|s| s.trim().to_string()).collect();
    
    let result = generate_multiple_bins(&bins, &ccv, &month, &year, count);
    
    if !result.is_empty() {
        println!("{}", result);
    } else {
        println!("please provide at least one valid BIN :(");
    }
}

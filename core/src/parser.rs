// Data parser utility; currently used to prevent automatic rerolls per reroll check phase

pub fn parse_reroll_input(input: &str) -> Result<Vec<usize>, String> {
    let lower_input = input.trim().to_lowercase();

    if lower_input == "x" {
        return Ok(Vec::new()); 
    }
    if lower_input == "0" {
        return Ok(vec![0, 1, 2, 3, 4, 5]);
    }

    let mut indices = Vec::new();
    for segment in lower_input.split(|c: char| c == ' ' || c == ',').filter(|s| !s.is_empty()) {
        match segment.parse::<usize>() {
            Ok(die_num) if die_num >= 1 && die_num <= 6 => {
                indices.push(die_num - 1); // Convert 1-6 to 0-5
            }
            _ => return Err(format!("'{}' is not a valid die index (1-6)", segment)),
        }
    }
    Ok(indices)
}
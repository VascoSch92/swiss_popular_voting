pub fn string_to_u32(candidate: Option<&String>) -> Option<u32> {
    match candidate.is_none() {
        true=> None,
        false=> {
            let candidate_clean = candidate.unwrap().replace("'", "").parse::<u32>();
            match candidate_clean {
                Ok(value) => Some(value),
                Err(_) => None,
            }
        }
    }

}

pub fn string_to_f32(candidate: String) -> Option<f32> {
    let cleaned_str = candidate.replace("'", "").parse::<f32>();
    match cleaned_str {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}

pub fn ratio_to_f32(candidate: Option<&String>) -> Option<f32> {
    match candidate.is_none() {
        true => None,
        false => {
            let cleaned_str = candidate.unwrap().trim_end_matches('%');
            // Parse the cleaned string into a f32
            match cleaned_str.parse::<f32>() {
                Ok(value) => {
                    let decimal = value / 100.0; // Convert percentage to decimal
                    Some(decimal)
                }
                Err(_) => None,
            }
        }
    }
}

pub fn integer_and_fraction_to_f32(candidate: Option<&String>) -> Option<f32> {
    if candidate.is_none(){
        return None;
    }
    // Split the input into integer and fraction parts
    let parts: Vec<&str> = candidate.unwrap().split_whitespace().collect();

    match parts.len() {
        0 => None,
        1 => {
            let value = parts[0];
            if value.contains('/') {
                return string_to_f32(value.to_string());
            }
            return string_to_f32(value.to_string());
        }
        2 => {
            // Parse the integer part
            let integer_part: f32 = parts[0].parse().ok()?;

            // Parse the fraction part (e.g., "1/2")
            let fraction_parts: Vec<&str> = parts[1].split('/').collect();

            // Return None if the fraction part is not in the form "numerator/denominator"
            if fraction_parts.len() != 2 {
                return None;
            }

            // Parse numerator and denominator
            let numerator: f32 = fraction_parts[0].parse().ok()?;
            let denominator: f32 = fraction_parts[1].parse().ok()?;

            // Calculate the fractional value
            let fraction = numerator / denominator;

            // Return the combined value as f32
            Some(integer_part + fraction)
        }
        _ => None,
    }
}

pub fn convert_date_to_us_format(date: &str) -> Option<String> {
    // Split the date by `.`
    let parts: Vec<&str> = date.split('.').collect();

    // Ensure there are exactly 3 parts: day, month, year
    if parts.len() == 3 {
        let day = parts[0];
        let month = parts[1];
        let year = parts[2];

        // Return the formatted date as "YYYY-MM-DD"
        return Some(format!("{}-{}-{}", year, month, day));
    }

    None // Return None if the format is invalid
}

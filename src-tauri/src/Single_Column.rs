use chrono::{DateTime, Duration, Local, NaiveDate, TimeZone, Utc};
use image::io::Reader as ImageReader;
use regex::Regex;
use std::io::{BufReader, Read};
use std::process::Command;
use std::str;

// struct candidate {
//     experience: Vec<experience>,
// }

// struct experience {
//     name: String,
//     years: String,
// }


// fn update_candid(data:String)-> candidate{

//     let mut nabeel = candidate {
//         experience: Vec::new(),
//     };

//     // regexer(&data,&mut nabeel);

//     return nabeel;

// }


pub async fn caller(ocr_info: String)-> (Vec<String>,Vec<String>) {
    let text=classifier_single_column(ocr_info.to_string()).await;
    
    let corrected = replace_parentheses(&text).await;
    let  final_cleaned=count_characters_between_matches(&corrected).await;
    // println!("{:?}", final_cleaned.clone());
    let y = regexer(&final_cleaned).await;
    
    // let y= regexer(&text).await;
    return (y.0,y.1);
}

// fn regexer(mut text: &str, user: &mut candidate) {

    pub async fn replace_parentheses(input: &str) -> String {
    let y = input.replace("()", "0");
    let z=y.replace("—","-");
    let a= z.replace("•"," ");
    a
}
    
    pub async fn count_characters_between_matches(input_text: &str) -> String {
    
        let mut remove_start: Vec<usize> = Vec::new(); // Empty vector for starting indices
        let mut remove_end: Vec<usize> = Vec::new();
        
        let pattern = Regex::new(r#"(?i)(Jan(?:uary)?|Feb(?:ruary)?|Mar(?:ch)?|Apr(?:il)?|May|Jun(?:e)?|Jul(?:y)?|Aug(?:ust)?|Sept(?:ember)?|Sep(?:tember)?|Oct(?:ober)?|Nov(?:ember)?|Dec(?:ember)?|\d{1,2})\s*[/\s]*\s*(\d{4})\s*(?:—|-|–|—|to)\s*(?i)(Jan(?:uary)?|Feb(?:ruary)?|Mar(?:ch)?|Apr(?:il)?|May|Jun(?:e)?|Jul(?:y)?|Aug(?:ust)?|Sept(?:ember)?|Sep(?:tember)?|Oct(?:ober)?|Nov(?:ember)?|Dec(?:ember)?|\d{1,2})\s*[/\s]*\s*(\d{4}|Present)?\s*"#).unwrap();
    
        for captures in pattern.captures_iter(input_text) {
            let x= captures.get(1).unwrap().start();
            let y = captures.get(4).unwrap().end();
            let z = input_text.get(y..);
                let bool_check=caller_one_after_another(z.unwrap()).await;
                if(bool_check == true){
                    remove_start.push(x);
                    remove_end.push(y);
                }
            
        }
        let final_cleaned=cut_multiple(remove_start.clone(), remove_end.clone(), input_text.to_string()).await;
        // println!("{:?},{:?}",remove_start,remove_end);
        // println!("{:?}",final_cleaned.clone());
       
        return  final_cleaned; 
    
    }
    
    pub async fn cut_multiple(remove_start: Vec<usize>, remove_end: Vec<usize>, mut input: String) -> String {
        for i in (0..remove_start.len()).rev() {
            input.drain(remove_start[i]..remove_end[i]);
        }
        input
    }
    
    
    
    
    pub async fn caller_one_after_another(input_text: &str)-> bool{
        let pattern = Regex::new(r#"(?i)(Jan(?:uary)?|Feb(?:ruary)?|Mar(?:ch)?|Apr(?:il)?|May|Jun(?:e)?|Jul(?:y)?|Aug(?:ust)?|Sept(?:ember)?|Sep(?:tember)?|Oct(?:ober)?|Nov(?:ember)?|Dec(?:ember)?|\d{1,2})\s*[/\s]*\s*(\d{4})\s*(?:—|-|–|—|to)\s*(?i)(Jan(?:uary)?|Feb(?:ruary)?|Mar(?:ch)?|Apr(?:il)?|May|Jun(?:e)?|Jul(?:y)?|Aug(?:ust)?|Sept(?:ember)?|Sep(?:tember)?|Oct(?:ober)?|Nov(?:ember)?|Dec(?:ember)?|\d{1,2})\s*[/\s]*\s*(\d{4}|Present)?\s*"#).unwrap();
        if let Some(captures) = pattern.captures(input_text) {
            let first_match_start = captures.get(0).unwrap().start();
            if(first_match_start < 10){
                return true;
            }else{
                return false;
            }
        } else{
            return false;
        }   
    }
    
    pub async fn regexer(mut text: &str) -> (Vec<String>,Vec<String>) {
        let mut title: Vec<String> = Vec::new();
        let mut exp_num:Vec<String>=Vec::new();
        let time_zone: chrono::FixedOffset = chrono::FixedOffset::east(5 * 3600);
    
        // Get the current time in GMT+5
        let now_gmt5: DateTime<chrono::FixedOffset> = Utc::now().with_timezone(&time_zone);
        let current_month = now_gmt5.format("%m").to_string();
        let current_year = now_gmt5.format("%Y").to_string();
    
        let pattern = Regex::new(r#"(?i)(Jan(?:uary)?|Feb(?:ruary)?|Mar(?:ch)?|Apr(?:il)?|May|Jun(?:e)?|Jul(?:y)?|Aug(?:ust)?|Sept(?:ember)?|Sep(?:tember)?|Oct(?:ober)?|Nov(?:ember)?|Dec(?:ember)?|\d{1,2})\s*[/\s]*\s*(\d{4})\s*(?:-|–|to)\s*(?i)(Jan(?:uary)?|Feb(?:ruary)?|Mar(?:ch)?|Apr(?:il)?|May|Jun(?:e)?|Jul(?:y)?|Aug(?:ust)?|Sept(?:ember)?|Sep(?:tember)?|Oct(?:ober)?|Nov(?:ember)?|Dec(?:ember)?|\d{1,2})?\s*[/\s]*\s*(\d{4}|Present)?\s*"#).unwrap();
    
        for captures in pattern.captures_iter(text) {
            let start = captures.get(1).unwrap().start();
            // let end = captures.get(4).unwrap().end();
            let starters = get_words_before_index(text, start).await;
            // let enders = get_words_after_index(text, end);
            // let combined = starters.clone() + enders.as_str();
            title.push(starters);
            // println!("{}", starters);
            // let mut temp =experience{
            //     name:starters,
            //     years:String::new(),
            // };
    
            let mut month1 = captures.get(1).unwrap().as_str().to_lowercase();
            let mut year1 = captures.get(2).unwrap().as_str();
            let mut month2 = captures.get(3).map_or("", |m| m.as_str()).to_lowercase();
            let mut year2 = captures.get(4).map_or("", |y| y.as_str());
            //    month1
    
            
            if month1 == "january" || month1 == "jan" || month1 == "01" {
                month1 = "1".to_string();
            }
            if month1 == "february" || month1 == "feb" || month1 == "02" {
                month1 = "2".to_string();
            }
            if month1 == "march" || month1 == "mar" || month1 == "03" {
                month1 = "3".to_string();
            }
            if month1 == "april" || month1 == "apr" || month1 == "04" {
                month1 = "4".to_string();
            }
            if month1 == "may" || month1 == "05" {
                month1 = "5".to_string();
            }
            if month1 == "june" || month1 == "jun" || month1 == "06" {
                month1 = "6".to_string();
            }
            if month1 == "july" || month1 == "jul" || month1 == "07" {
                month1 = "7".to_string();
            }
            if month1 == "august" || month1 == "aug" || month1 == "08" {
                month1 = "8".to_string();
            }
            if month1 == "september" || month1 == "sept" || month1 == "sep" || month1 == "09" {
                month1 = "9".to_string();
            }
            if month1 == "october" || month1 == "oct" {
                month1 = "10".to_string();
            }
            if month1 == "november" || month1 == "nov" {
                month1 = "11".to_string();
            }
            if month1 == "december" || month1 == "dec" {
                month1 = "12".to_string();
            }
    
            // month2
            if month2 == "january" || month2 == "jan" || month2 == "01" {
                month2 = "1".to_string();
            }
            if month2 == "february" || month2 == "feb" || month2 == "02" {
                month2 = "2".to_string();
            }
            if month2 == "march" || month2 == "mar" || month2 == "03" {
                month2 = "3".to_string();
            }
            if month2 == "april" || month2 == "apr" || month2 == "04" {
                month2 = "4".to_string();
            }
            if month2 == "may" || month2 == "05" {
                month2 = "5".to_string();
            }
            if month2 == "june" || month2 == "jun" || month2 == "06" {
                month2 = "6".to_string();
            }
            if month2 == "july" || month2 == "jul" || month2 == "07" {
                month2 = "7".to_string();
            }
            if month2 == "august" || month2 == "aug" || month2 == "08" {
                month2 = "8".to_string();
            }
            if month2 == "september" || month2 == "sept" || month2 == "sep" || month2 == "09" {
                month2 = "9".to_string();
            }
            if month2 == "october" || month2 == "oct" {
                month2 = "10".to_string();
            }
            if month2 == "november" || month2 == "nov" {
                month2 = "11".to_string();
            }
            if month2 == "december" || month2 == "dec" {
                month2 = "12".to_string();
            }
    
            let start_date = format!("{} {}", month1, year1);
            let end_date = if !month2.is_empty() && !year2.is_empty() {
                format!("{} {}", month2, year2)
            } else {
                "Present".to_string()
            };
            if (end_date == "Present") {
                let dur = time_diff_years(
                    month1.to_string(),
                    year1.to_string(),
                    current_month.clone(),
                    current_year.clone(),
                ).await;
                let dur_rounded = format!("{:.3}", dur);
                // println!("{:?}", &dur_rounded);
                exp_num.push(dur_rounded);
                // temp.years=dur_rounded
            } else {
                // println!("{:?},{:?},{:?},{:?}",month1.clone(),year1.clone(),month2.clone(),year2.clone());
                let dur = time_diff_years(
                    month1.to_string(),
                    year1.to_string(),
                    month2.to_string(),
                    year2.to_string(),
                ).await;
                let dur_rounded = format!("{:.3}", dur);
                // println!("{:?}", &dur_rounded);
                exp_num.push(dur_rounded);
    
                // temp.years=dur_rounded
            }
    
            // user.experience.push(temp)
        }
    
        return(title,exp_num);
    }
    
    pub async fn time_diff_years(mut mo1: String, mut y1: String, mo2: String, y2: String) -> f64 {
        let mut month1 = mo1.parse::<i32>().unwrap();
        let month2 = mo2.parse::<i32>().unwrap();
        let mut year1 = y1.parse::<i32>().unwrap();
        let year2 = y2.parse::<i32>().unwrap();
    
        let mut yearss: f64 = 0.0;
        let mut monthss: f64 = 0.0;
        let mut add_: bool = true;
        if (month1 > month2) {
            add_ = false;
        }
    
        while year1 != year2 {
            year1 = year1 + 1;
            yearss = yearss + 1.0;
        }
    
        while month1 != month2 {
            if (month1 > month2) {
                month1 = month1 - 1;
                monthss = monthss + 1.0;
            }
            // month1<month2
            else {
                month1 = month1 + 1;
                monthss = monthss + 1.0;
            }
        }
    
        if (add_ == true) {
            yearss = yearss + (monthss / 12.0);
        } else {
            yearss = yearss - (monthss / 12.0);
        }
        return yearss;
    }
    
    pub async fn get_words_before_index(s: &str, index: usize) -> String {
        let exceptions = ["&", "in", "at", "on", "of" , "member"];
        let mut words = String::new();
        let mut i = index - 1;
        let mut should_continue = true;
        while should_continue && i > 0 {
            let mut word = String::new();
            while i > 0 && s.chars().nth(i - 1).unwrap().is_ascii_alphabetic() {
                i -= 1;
                word.insert(0, s.chars().nth(i).unwrap());
            }
            if !word.is_empty() {
                if word.chars().next().unwrap().is_ascii_uppercase()
                    || exceptions.contains(&word.to_lowercase().as_str())
                {
                    if !words.is_empty() {
                        words.insert(0, ' ');
                    }
                    words.insert_str(0, &word);
                } else {
                    should_continue = false;
                }
            } else {
                i -= 1;
            }
        }
        let final_words = remove_starting_in(&words.clone()).await;
        let final_withoutmonths = remove_months(final_words.clone()).await;
        return final_withoutmonths;
    }
    
    pub async fn remove_months(input_string: String) -> String {
        let months = [
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
            "Jan",
            "Feb",
            "Mar",
            "Apr",
            "Jun",
            "Jul",
            "Aug",
            "Sept",
            "Sep",
            "Oct",
            "Nov",
            "Dec",
        ];
        let mut output_string = String::new();
        for word in input_string.split_whitespace() {
            if !months.contains(&word) {
                output_string.push_str(word);
                output_string.push(' ');
            }
        }
        output_string.trim().to_string()
    }
    
    pub async fn remove_starting_in(s: &str) -> String {
        let mut words = s.split_whitespace();
        let mut result = String::new();
        if let Some(word) = words.next() {
            if (word.to_lowercase() == "in"
                || word.to_lowercase() == "to"
                || word.to_lowercase() == "at"
                || word.to_lowercase() == "of"
                || word.to_lowercase() == "&"
                || word.to_lowercase() == "on"
                || word.to_lowercase() == "member")
            {
                for w in words {
                    result.push_str(w);
                    result.push(' ');
                }
                result.pop(); // remove last space
                return result;
            }
            result.push_str(word);
            result.push(' ');
        }
        for w in words {
            result.push_str(w);
            result.push(' ');
        }
        result.pop(); // remove last space
        result
    }

fn get_words_after_index(s: &str, index: usize) -> String {
    let exceptions = ["&", "in", "at", "on", "of"];
    let mut words = String::new();
    let mut i = index + 1;
    let mut should_continue = true;
    while should_continue && i < s.len() {
        let mut word = String::new();
        while i < s.len() && s.chars().nth(i).unwrap().is_ascii_alphabetic() {
            word.push(s.chars().nth(i).unwrap());
            i += 1;
        }
        if !word.is_empty() {
            if word.chars().next().unwrap().is_ascii_uppercase()
                || exceptions.contains(&word.to_lowercase().as_str())
            {
                if !words.is_empty() {
                    words.push(' ');
                }
                words.push_str(&word);
            } else {
                should_continue = false;
            }
        } else {
            i += 1;
        }
    }
    let final_words = remove_ending_in(&words);
    let e_words = process_string(&final_words);
    return e_words;
}

fn process_string(s: &str) -> String {
    let mut words = s.split_whitespace().collect::<Vec<&str>>();
    if words.len() == 1 {
        return String::new();
    }
    words.pop();
    words.join(" ")
}

fn remove_ending_in(s: &str) -> String {
    let mut words = s.split_whitespace().rev();
    let mut result = String::new();
    if let Some(word) = words.next() {
        if (word.to_lowercase() == "in"
            || word.to_lowercase() == "to"
            || word.to_lowercase() == "at"
            || word.to_lowercase() == "of"
            || word.to_lowercase() == "&"
            || word.to_lowercase() == "on")
        {
            for w in words.rev() {
                result.insert_str(0, w);
                result.insert(0, ' ');
            }
            result.push_str(&word);
            return result;
        }
        result.push_str(word);
        result.push(' ');
    }
    for w in words.rev() {
        result.insert_str(0, w);
        result.insert(0, ' ');
    }
    result.pop(); // remove last space
    result
}

   pub async fn classifier_single_column(ocr_info: String) -> String {
    let mut fin_edu: String = "".to_string();
    let mut fin_exp: String = "".to_string();
    let mut fin_ski: String = "".to_string();
    let mut fin_pro: String = "".to_string();
    let mut fin_elec: String = "".to_string();
    let mut fin_cer: String = "".to_string();
    let mut fin_cirr: String = "".to_string();
    let mut fin_int: String = "".to_string();
    let mut fin_ach: String = "".to_string();
    let mut fin_summ: String = "".to_string();
    let mut fin_lang: String = "".to_string();
    let mut fin_per: String = "".to_string();
    let mut fin_lead: String = "".to_string();

    // 43== QUALIFICATION

    //edu - academic quali
    // skills -  technical profile
    let keywords = [
        "Education",
        "Experience",
        "Skills",
        "Projects",
        "Technical Profile",
        "Academic Qualification",
        "Academic Elective",
        "Cerification",
        "Meta Cirricular",
        "Meta-Cirricular",
        "Extra Curricular",
        "Extra-Curricular",
        "EDUCATION",
        "EXPERIENCE",
        "SKILLS",
        "PROJECTS",
        "TECHNICAL PROFILE",
        "ACADEMIC QUALIFICATION",
        "ACADEMIC ELECTIVE",
        "CERTIFICATION",
        "META CIRRICULAR",
        "META-CIRRICULAR",
        "EXTRA CURRICULAR",
        "EXTRA-CIRRICULAR",
        "Certificates",
        "CERTIFICATES",
        "Interests",
        "INTERESTS",
        "Hobbies",
        "HOBBIES",
        "Technical Tool",
        "TECHNICAL TOOL",
        "Achievements",
        "ACHIEVEMENTS",
        "Accomplishments",
        "ACCOMPLISHMENTS",
        "Summary",
        "SUMMARY",
        "Objective",
        "OBJECTIVE",
        "Educational Detail",
        "EDUCATIONAL DETAIL",
        "Qualification",
        "QUALIFICATION",
        "Languages",
        "LANGUAGES",
        "Personal",
        "PERSONAL",
        "Leadership",
        "LEADERSHIP",
    ];

    let mut int_vec = Vec::new();
    for keyword in &keywords {
        let pos = find_word(ocr_info.as_str(), keyword);
        int_vec.push(pos);
    }

    let edu_vec = [0, 12, 5, 17, 40, 41, 42, 43];
    let meta_vec = [8, 9, 10, 11, 20, 21, 22, 23];
    let exp_vec = [1, 13];
    let elec_vec = [6, 18];
    let skill_vec = [2, 14, 4, 16, 30, 31];
    let proj_vec = [3, 15];
    let cer_vec = [7, 19, 24, 25];
    let interest_vec = [26, 27, 28, 29];
    let ach_vec = [32, 33, 34, 35];
    let summ_vec = [36, 37, 38, 39];
    let lang_vec = [44, 45];
    let per_vec = [46, 47];
    let lead_vec = [48, 49];

    let keywords_2 = [
        "Education",
        "Experience",
        "Skills",
        "Projects",
        "Academic Elective",
        "Cerification",
        "Meta Cirricular",
        "Interests",
        "Achievements",
        "Summary",
        "Languages",
        "Personal",
        "Leadership",
    ];
    for keyword in &keywords_2 {
        if (keyword.to_string() == "Education") {
            for val in edu_vec {
                let a = int_vec.clone();
                if (a[val] != None) {
                    let x = find_next_smallest(int_vec.clone(), int_vec[val]);
                    if (x != None) {
                        fin_edu = slice_in_between(&ocr_info.as_str(), int_vec[val], x);
                    } else {
                        fin_edu = slice_in_between(
                            &ocr_info.as_str(),
                            int_vec[val],
                            Some(ocr_info.len()),
                        );
                    }
                }
            }
        }

        if (keyword.to_string() == "Meta Cirricular") {
            for val in meta_vec {
                let a = int_vec.clone();
                if (a[val] != None) {
                    let x = find_next_smallest(int_vec.clone(), int_vec[val]);
                    if (x != None) {
                        fin_cirr = slice_in_between(&ocr_info.as_str(), int_vec[val], x);
                    } else {
                        fin_cirr = slice_in_between(
                            &ocr_info.as_str(),
                            int_vec[val],
                            Some(ocr_info.len()),
                        );
                    }
                }
            }
        }

        if (keyword.to_string() == "Experience") {
            for val in exp_vec {
                let a = int_vec.clone();
                if (a[val] != None) {
                    let x = find_next_smallest(int_vec.clone(), int_vec[val]);
                    if (x != None) {
                        fin_exp = slice_in_between(&ocr_info.as_str(), int_vec[val], x);
                    } else {
                        fin_exp = slice_in_between(
                            &ocr_info.as_str(),
                            int_vec[val],
                            Some(ocr_info.len()),
                        );
                    }
                }
            }
        }

        if (keyword.to_string() == "Academic Elective") {
            for val in elec_vec {
                let a = int_vec.clone();
                if (a[val] != None) {
                    let x = find_next_smallest(int_vec.clone(), int_vec[val]);
                    if (x != None) {
                        fin_elec = slice_in_between(&ocr_info.as_str(), int_vec[val], x);
                    } else {
                        fin_elec = slice_in_between(
                            &ocr_info.as_str(),
                            int_vec[val],
                            Some(ocr_info.len()),
                        );
                    }
                }
            }
        }

        if (keyword.to_string() == "Skills") {
            for val in skill_vec {
                let a = int_vec.clone();
                if (a[val] != None) {
                    let x = find_next_smallest(int_vec.clone(), int_vec[val]);
                    if (x != None) {
                        fin_ski = slice_in_between(&ocr_info.as_str(), int_vec[val], x);
                    } else {
                        fin_ski = slice_in_between(
                            &ocr_info.as_str(),
                            int_vec[val],
                            Some(ocr_info.len()),
                        );
                    }
                }
            }
        }

        if (keyword.to_string() == "Projects") {
            for val in proj_vec {
                let a = int_vec.clone();
                if (a[val] != None) {
                    let x = find_next_smallest(int_vec.clone(), int_vec[val]);
                    if (x != None) {
                        fin_pro = slice_in_between(&ocr_info.as_str(), int_vec[val], x);
                    } else {
                        fin_pro = slice_in_between(
                            &ocr_info.as_str(),
                            int_vec[val],
                            Some(ocr_info.len()),
                        );
                    }
                }
            }
        }

        if (keyword.to_string() == "Certification") {
            for val in cer_vec {
                let a = int_vec.clone();
                if (a[val] != None) {
                    let x = find_next_smallest(int_vec.clone(), int_vec[val]);
                    if (x != None) {
                        fin_cer = slice_in_between(&ocr_info.as_str(), int_vec[val], x);
                    } else {
                        fin_cer = slice_in_between(
                            &ocr_info.as_str(),
                            int_vec[val],
                            Some(ocr_info.len()),
                        );
                    }
                }
            }
        }

        if (keyword.to_string() == "Interests") {
            for val in interest_vec {
                let a = int_vec.clone();
                if (a[val] != None) {
                    let x = find_next_smallest(int_vec.clone(), int_vec[val]);
                    if (x != None) {
                        fin_int = slice_in_between(&ocr_info.as_str(), int_vec[val], x);
                    } else {
                        fin_int = slice_in_between(
                            &ocr_info.as_str(),
                            int_vec[val],
                            Some(ocr_info.len()),
                        );
                    }
                }
            }
        }

        if (keyword.to_string() == "Achievements") {
            for val in ach_vec {
                let a = int_vec.clone();
                if (a[val] != None) {
                    let x = find_next_smallest(int_vec.clone(), int_vec[val]);
                    if (x != None) {
                        fin_ach = slice_in_between(&ocr_info.as_str(), int_vec[val], x);
                    } else {
                        fin_ach = slice_in_between(
                            &ocr_info.as_str(),
                            int_vec[val],
                            Some(ocr_info.len()),
                        );
                    }
                }
            }
        }

        if (keyword.to_string() == "Summary") {
            for val in summ_vec {
                let a = int_vec.clone();
                if (a[val] != None) {
                    let x = find_next_smallest(int_vec.clone(), int_vec[val]);
                    if (x != None) {
                        fin_summ = slice_in_between(&ocr_info.as_str(), int_vec[val], x);
                    } else {
                        fin_summ = slice_in_between(
                            &ocr_info.as_str(),
                            int_vec[val],
                            Some(ocr_info.len()),
                        );
                    }
                }
            }
        }

        if (keyword.to_string() == "Languages") {
            for val in lang_vec {
                let a = int_vec.clone();
                if (a[val] != None) {
                    let x = find_next_smallest(int_vec.clone(), int_vec[val]);
                    if (x != None) {
                        fin_lang = slice_in_between(&ocr_info.as_str(), int_vec[val], x);
                    } else {
                        fin_lang = slice_in_between(
                            &ocr_info.as_str(),
                            int_vec[val],
                            Some(ocr_info.len()),
                        );
                    }
                }
            }
        }

        if (keyword.to_string() == "Personal") {
            for val in per_vec {
                let a = int_vec.clone();
                if (a[val] != None) {
                    let x = find_next_smallest(int_vec.clone(), int_vec[val]);
                    if (x != None) {
                        fin_per = slice_in_between(&ocr_info.as_str(), int_vec[val], x);
                    } else {
                        fin_per = slice_in_between(
                            &ocr_info.as_str(),
                            int_vec[val],
                            Some(ocr_info.len()),
                        );
                    }
                }
            }
        }

        if (keyword.to_string() == "Leadership") {
            for val in lead_vec {
                let a = int_vec.clone();
                if (a[val] != None) {
                    let x = find_next_smallest(int_vec.clone(), int_vec[val]);
                    if (x != None) {
                        fin_lead = slice_in_between(&ocr_info.as_str(), int_vec[val], x);
                    } else {
                        fin_lead = slice_in_between(
                            &ocr_info.as_str(),
                            int_vec[val],
                            Some(ocr_info.len()),
                        );
                    }
                }
            }
        }
    }
    return fin_exp;
}

fn slice_in_between(s: &str, start: Option<usize>, end: Option<usize>) -> String {
    if let (Some(start), Some(end)) = (start, end) {
        return s[start..end].to_owned();
    }
    String::new()
}

fn find_next_smallest(vec: Vec<Option<usize>>, val: Option<usize>) -> Option<usize> {
    vec.into_iter()
        .flatten()
        .filter(|&x| x > val.unwrap_or(usize::MAX))
        .min()
}

fn find_word(s: &str, word: &str) -> Option<usize> {
    let found = s.find(word);
    match found {
        Some(pos) => Some(pos + word.len()),
        None => None,
    }
}

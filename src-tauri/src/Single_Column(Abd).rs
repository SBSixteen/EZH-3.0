use regex::Regex;
use chrono::{DateTime, Utc, TimeZone, Duration, Local, NaiveDate};
use image::io::Reader as ImageReader;
use rusty_tesseract::{Args, Image};
use std::io::{BufReader, Read};
use std::str;
use std::process::Command;

// struct candidate {
//     experience: Vec<experience>
// }

// struct experience{
//     name: String,
//     years: String,
// }


// fn main(){
// // let mut nabeel = candidate{
// //     experience: Vec::new(),

// // };

    

//     let ocr_info = "";

//     let text=classifier_single_column(ocr_info.to_string());
//     println!("{}",text);
//     let y= regexer(&text);
//     // let y= regexer(&text,nabeel);


// }




// fn regexer(mut text:&str, &mut user:candidate) {

fn regexer(mut text:&str) {
    
    let time_zone = chrono::FixedOffset::east(5 * 3600);

    // Get the current time in GMT+5
    let now_gmt5: DateTime<chrono::FixedOffset> = Utc::now().with_timezone(&time_zone);
    let current_month = now_gmt5.format("%m").to_string();
    let current_year=now_gmt5.format("%Y").to_string();
   
    let pattern = Regex::new(r#"(Jan(?:uary)?|Feb(?:ruary)?|Mar(?:ch)?|Apr(?:il)?|May|Jun(?:e)?|Jul(?:y)?|Aug(?:ust)?|Sept(?:ember)?|Sep(?:tember)?|Oct(?:ober)?|Nov(?:ember)?|Dec(?:ember)?|\d{1,2})\s*[/\s]*\s*(\d{4})\s*(?:-|–|to)\s*(Jan(?:uary)?|Feb(?:ruary)?|Mar(?:ch)?|Apr(?:il)?|May|Jun(?:e)?|Jul(?:y)?|Aug(?:ust)?|Sept(?:ember)?|Sep(?:tember)?|Oct(?:ober)?|Nov(?:ember)?|Dec(?:ember)?|\d{1,2})?\s*[/\s]*\s*(\d{4}|Present)?\s*"#).unwrap();


    for captures in pattern.captures_iter(text) {
        let start = captures.get(1).unwrap().start();
        let end = captures.get(4).unwrap().end();
        let starters=get_words_before_index(text,start);
        let enders=get_words_after_index(text, end);
        let combined = starters.clone() + enders.as_str();
        println!("{}", starters);
        // let mut temp =experience{
        //     name:starters,
        //     years:String::new(),
        // };

        let mut month1 = captures.get(1).unwrap().as_str();
        let mut year1 = captures.get(2).unwrap().as_str();
        let mut month2 = captures.get(3).map_or("", |m| m.as_str());
        let mut year2 = captures.get(4).map_or("", |y| y.as_str());
//    month1

        if month1 == "January" || month1 == "Jan" || month1 == "01" {
            month1="1";
        }
        if month1 == "February" || month1 == "Feb" || month1 == "02" {
            month1="2";
        }
        if month1 == "March" || month1 == "Mar" || month1 == "03" {
            month1="3";
        }
        if month1 == "April" || month1 == "Apr" || month1 == "04" {
            month1="4";
        }
        if month1 == "May" || month1 == "05" {
            month1="5";
        }
        if month1 == "June" || month1 == "Jun" || month1 == "06" {
            month1="6";
        }
        if month1 == "July" || month1 == "Jul" || month1 == "07" {
            month1="7";
        }
        if month1 == "August" || month1 == "Aug" || month1 == "08" {
            month1="8";
        }
        if month1 == "September" || month1 == "Sept" || month1 == "Sep" || month1 == "09"{
            month1="9";
        }
        if month1 == "October"  || month1 == "Oct"{
            month1="10";
        }
        if month1 == "November" || month1 == "Nov"{
            month1="11";
        }
        if month1 == "December" || month1 == "Dec"{
            month1="12";
        }

// month2
        if month2 == "January" || month2 == "Jan" || month2 == "01" {
            month2="1";
        }
        if month2 == "February" || month2 == "Feb" || month2 == "02" {
            month2="2";
        }
        if month2 == "March" || month2 == "Mar" || month2 == "03" {
            month2="3";
        }
        if month2 == "April" || month2 == "Apr" || month2 == "04" {
            month2="4";
        }
        if month2 == "May" || month2 == "05" {
            month2="5";
        }
        if month2 == "June" || month2 == "Jun" || month2 == "06" {
            month2="6";
        }
        if month2 == "July" || month2 == "Jul" || month2 == "07" {
            month2="7";
        }
        if month2 == "August" || month2 == "Aug" || month2 == "08" {
            month2="8";
        }
        if month2 == "September" || month2 == "Sept" || month2 == "Sep" || month2 == "09"{
            month2="9";
        }
        if month2 == "October" || month2 == "Oct"{
            month2="10";
        }
        if month2 == "November"|| month2 == "Nov"{
            month2="11";
        }
        if month2 == "December"|| month2 == "Dec"{
            month2="12";
        }



        let start_date = format!("{} {}", month1, year1);
        let end_date = if !month2.is_empty() && !year2.is_empty() {
            format!("{} {}", month2, year2)
        } else {
            "Present".to_string()
        };
    if(end_date=="Present"){
        let dur= time_diff_years(month1.to_string(),year1.to_string(),current_month.clone(),current_year.clone());
        let dur_rounded=format!("{:.3}",dur);
        println!("{:?}",&dur_rounded);
        // temp.years=dur_rounded

    }else{
        let dur= time_diff_years(month1.to_string(),year1.to_string(),month2.to_string(),year2.to_string());
        let dur_rounded=format!("{:.3}",dur);
        println!("{:?}",&dur_rounded);
        // temp.years=dur_rounded
    }

        // user.experience.push(temp)
    }

}


fn time_diff_years(mut mo1: String, mut y1: String, mo2: String, y2:String) -> f64 {
    let mut month1 = mo1.parse::<i32>().unwrap();
    let month2 = mo2.parse::<i32>().unwrap();
    let mut year1 = y1.parse::<i32>().unwrap();
    let year2 = y2.parse::<i32>().unwrap();


   let mut yearss:f64=0.0;
   let mut monthss:f64=0.0;
   let mut add_:bool=true;
   if(month1>month2){
    add_=false;
   }

   while year1!=year2{
    year1=year1+1;
    yearss=yearss+1.0;
   }

   while month1!=month2{
    if(month1>month2){
    month1=month1-1;
    monthss=monthss+1.0;
    }
// month1<month2
   else{
    month1=month1+1;
    monthss=monthss+1.0;
   }
}

if(add_==true){
    yearss=yearss+(monthss/12.0);
}
else{
    yearss=yearss-(monthss/12.0);
}
   return yearss;
}



fn get_words_before_index(s: &str, index: usize) -> String {
    let exceptions = ["&", "in", "at", "on", "of"];
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
    let final_words=remove_starting_in(&words.clone());
    let final_withoutmonths=remove_months(final_words.clone());
    return final_withoutmonths;
}

fn remove_months(input_string: String) -> String {
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December", 
                  "Jan", "Feb", "Mar", "Apr", "Jun", "Jul", "Aug", "Sept","Sep", "Oct", "Nov", "Dec"];
    let mut output_string = String::new();
    for word in input_string.split_whitespace() {
        if !months.contains(&word) {
            output_string.push_str(word);
            output_string.push(' ');
        }
    }
    output_string.trim().to_string()
}


fn remove_starting_in(s: &str) -> String {
    let mut words = s.split_whitespace();
    let mut result = String::new();
    if let Some(word) = words.next() {
        if (word.to_lowercase() == "in" || word.to_lowercase() == "to"||word.to_lowercase() == "at"|| word.to_lowercase() == "of"|| word.to_lowercase() == "&" || word.to_lowercase() == "on") {
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
    let e_words=process_string(&final_words);
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
        if (word.to_lowercase() == "in" || word.to_lowercase() == "to"||word.to_lowercase() == "at"|| word.to_lowercase() == "of"|| word.to_lowercase() == "&" || word.to_lowercase() == "on") {
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



fn classifier_single_column(ocr_info:String)->String{

    let mut fin_edu: String="".to_string();
    let mut fin_exp: String="".to_string();
    let mut fin_ski: String= "".to_string();
    let mut fin_pro: String="".to_string();
    let mut fin_elec: String="".to_string();
    let mut fin_cer: String="".to_string();
    let mut fin_cirr: String="".to_string();

// 43== QUALIFICATION


//edu - academic quali 
// skills -  technical profile
    let keywords = ["Education",  "Experience", "Skills","Projects", "Technical Profile" ,"Academic Qualification" , "Academic Elective", "Cerification"  ,  
                                "Meta Cirricular","Meta-Cirricular",  "Extra Curricular", "Extra-Curricular",
                                "EDUCATION" , "EXPERIENCE", "SKILLS","PROJECTS", "TECHNICAL PROFILE", "ACADEMIC QUALIFICATION" , "ACADEMIC ELECTIVE", "CERTIFICATION" ,  
                                "META CIRRICULAR","META-CIRRICULAR",  "EXTRA CURRICULAR", "EXTRA-CIRRICULAR",
                                "Certificates","CERTIFICATES","Interests","INTERESTS","Hobbies","HOBBIES","Technical Tool","TECHNICAL TOOL",
                                "Achievements","ACHIEVEMENTS","Accomplishments","ACCOMPLISHMENTS",
                                "Summary","SUMMARY","Objective","OBJECTIVE","Educational Detail","EDUCATIONAL DETAIL",
                                "Qualification","QUALIFICATION","Languages","LANGUAGES","Personal",
                                "PERSONAL","Leadership","LEADERSHIP"];
    let mut int_vec = Vec::new();
    for keyword in &keywords {
        let pos = find_word(ocr_info.as_str() , keyword);
        int_vec.push(pos);
    }

    let keywords_2 = ["Education",  "Experience", "Skills","Projects","Academic Elective", "Cerification","Meta Cirricular","Interests","Achievements","Summary","Languages","Personal","Leadership"];
    for keyword in &keywords_2 {
        if(keyword.to_string() == "Education"){
            let a = int_vec.clone();
            if(a[0]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[0]);
                if(x != None){
                    fin_edu=slice_in_between(&ocr_info.as_str(),int_vec[0],x);
                }
                else{
                    fin_edu=slice_in_between(&ocr_info.as_str(),int_vec[0],Some(ocr_info.len())); 
                }
            }
            else if(a[12]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[12]);
                if(x != None){
                    fin_edu=slice_in_between(&ocr_info.as_str(),int_vec[12],x);
                }
                else{
                    fin_edu=slice_in_between(&ocr_info.as_str(),int_vec[12],Some(ocr_info.len())); 
                }
            }
            // academic qualification
            else if(a[5]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[5]);
                if(x != None){
                    fin_edu=slice_in_between(&ocr_info.as_str(),int_vec[5],x);
                }
                else{
                    fin_edu=slice_in_between(&ocr_info.as_str(),int_vec[5],Some(ocr_info.len())); 
                }
            }
            else if(a[17]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[17]);
                if(x != None){
                    fin_edu=slice_in_between(&ocr_info.as_str(),int_vec[17],x);
                }
                else{
                    fin_edu=slice_in_between(&ocr_info.as_str(),int_vec[17],Some(ocr_info.len())); 
                }
            }
            else if(a[40]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[40]);
                if(x != None){
                    fin_edu=slice_in_between(&ocr_info.as_str(),int_vec[40],x);
                }
                else{
                    fin_edu=slice_in_between(&ocr_info.as_str(),int_vec[40],Some(ocr_info.len())); 
                }
            }
            else if(a[41]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[41]);
                if(x != None){
                    fin_edu=slice_in_between(&ocr_info.as_str(),int_vec[41],x);
                }
                else{
                    fin_edu=slice_in_between(&ocr_info.as_str(),int_vec[41],Some(ocr_info.len())); 
                }
            }
            else if(a[42]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[42]);
                if(x != None){
                    fin_edu=slice_in_between(&ocr_info.as_str(),int_vec[42],x);
                }
                else{
                    fin_edu=slice_in_between(&ocr_info.as_str(),int_vec[42],Some(ocr_info.len())); 
                }
            }
            else if(a[43]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[43]);
                if(x != None){
                    fin_edu=slice_in_between(&ocr_info.as_str(),int_vec[43],x);
                }
                else{
                    fin_edu=slice_in_between(&ocr_info.as_str(),int_vec[43],Some(ocr_info.len())); 
                }
            }
        }

        if(keyword.to_string() == "Meta Cirricular"){
            let a = int_vec.clone();
            if(a[8]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[8]);
                if(x != None){
                    fin_cirr=slice_in_between(&ocr_info.as_str(),int_vec[8],x);
                }
                else{
                    fin_cirr=slice_in_between(&ocr_info.as_str(),int_vec[8],Some(ocr_info.len())); 
                }
            }
            else if(a[9]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[9]);
                if(x != None){
                    fin_cirr=slice_in_between(&ocr_info.as_str(),int_vec[9],x);
                }
                else{
                    fin_cirr=slice_in_between(&ocr_info.as_str(),int_vec[9],Some(ocr_info.len())); 
                }
            }
            else if(a[10]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[10]);
                if(x != None){
                    fin_cirr=slice_in_between(&ocr_info.as_str(),int_vec[10],x);
                }
                else{
                    fin_cirr=slice_in_between(&ocr_info.as_str(),int_vec[10],Some(ocr_info.len())); 
                }
            }
            else if(a[11]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[11]);
                if(x != None){
                    fin_cirr=slice_in_between(&ocr_info.as_str(),int_vec[11],x);
                }
                else{
                    fin_cirr=slice_in_between(&ocr_info.as_str(),int_vec[11],Some(ocr_info.len())); 
                }
            }
            else if(a[20]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[20]);
                if(x != None){
                    fin_cirr=slice_in_between(&ocr_info.as_str(),int_vec[20],x);
                }
                else{
                    fin_cirr=slice_in_between(&ocr_info.as_str(),int_vec[20],Some(ocr_info.len())); 
                }
            }
            else if(a[21]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[21]);
                if(x != None){
                    fin_cirr=slice_in_between(&ocr_info.as_str(),int_vec[21],x);
                }
                else{
                    fin_cirr=slice_in_between(&ocr_info.as_str(),int_vec[21],Some(ocr_info.len())); 
                }
            }
            else if(a[22]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[22]);
                if(x != None){
                    fin_cirr=slice_in_between(&ocr_info.as_str(),int_vec[22],x);
                }
                else{
                    fin_cirr=slice_in_between(&ocr_info.as_str(),int_vec[22],Some(ocr_info.len())); 
                }
            }
            else if(a[23]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[23]);
                if(x != None){
                    fin_cirr=slice_in_between(&ocr_info.as_str(),int_vec[23],x);
                }
                else{
                    fin_cirr=slice_in_between(&ocr_info.as_str(),int_vec[23],Some(ocr_info.len())); 
                }
            }

        }

        if(keyword.to_string() == "Experience"){
            let a = int_vec.clone();
            if(a[1]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[1]);
                if(x != None){
                    fin_exp=slice_in_between(&ocr_info.as_str(),int_vec[1],x);
                }
                else{
                    fin_exp=slice_in_between(&ocr_info.as_str(),int_vec[1],Some(ocr_info.len())); 
                }
            }
            else if(a[13]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[13]);
                if(x != None){
                    fin_exp=slice_in_between(&ocr_info.as_str(),int_vec[13],x);
                }
                else{
                    fin_exp=slice_in_between(&ocr_info.as_str(),int_vec[13],Some(ocr_info.len())); 
                }
            }
        }

        if(keyword.to_string() == "Academic Elective"){
            let a = int_vec.clone();
            if(a[6]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[6]);
                if(x != None){
                    fin_elec=slice_in_between(&ocr_info.as_str(),int_vec[6],x);
                }
                else{
                    fin_elec=slice_in_between(&ocr_info.as_str(),int_vec[6],Some(ocr_info.len())); 
                }
            }
            else if(a[18]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[18]);
                if(x != None){
                    fin_elec=slice_in_between(&ocr_info.as_str(),int_vec[18],x);
                }
                else{
                    fin_elec=slice_in_between(&ocr_info.as_str(),int_vec[18],Some(ocr_info.len())); 
                }
            }
        }

        if(keyword.to_string() == "Skills"){
            let a = int_vec.clone();
            if(a[2]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[2]);
                if(x != None){
                    fin_ski=slice_in_between(&ocr_info.as_str(),int_vec[2],x);
                }
                else{
                    fin_ski=slice_in_between(&ocr_info.as_str(),int_vec[2],Some(ocr_info.len())); 
                }
            }
            else if(a[14]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[14]);
                if(x != None){
                    fin_ski=slice_in_between(&ocr_info.as_str(),int_vec[14],x);
                }
                else{
                    fin_ski=slice_in_between(&ocr_info.as_str(),int_vec[14],Some(ocr_info.len())); 
                }
            }
            else if(a[4]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[4]);
                if(x != None){
                    fin_ski=slice_in_between(&ocr_info.as_str(),int_vec[4],x);
                }
                else{
                    fin_ski=slice_in_between(&ocr_info.as_str(),int_vec[4],Some(ocr_info.len())); 
                }
            }
            else if(a[16]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[16]);
                if(x != None){
                    fin_ski=slice_in_between(&ocr_info.as_str(),int_vec[16],x);
                }
                else{
                    fin_ski=slice_in_between(&ocr_info.as_str(),int_vec[16],Some(ocr_info.len())); 
                }
            }
            else if(a[30]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[30]);
                if(x != None){
                    fin_ski=slice_in_between(&ocr_info.as_str(),int_vec[30],x);
                }
                else{
                    fin_ski=slice_in_between(&ocr_info.as_str(),int_vec[30],Some(ocr_info.len())); 
                }
            }
            else if(a[31]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[31]);
                if(x != None){
                    fin_ski=slice_in_between(&ocr_info.as_str(),int_vec[31],x);
                }
                else{
                    fin_ski=slice_in_between(&ocr_info.as_str(),int_vec[31],Some(ocr_info.len())); 
                }
            }

        }


        if(keyword.to_string() == "Projects"){
            let a = int_vec.clone();
            if(a[3]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[3]);
                if(x != None){
                    fin_pro=slice_in_between(&ocr_info.as_str(),int_vec[3],x);
                }
                else{
                    fin_pro=slice_in_between(&ocr_info.as_str(),int_vec[3],Some(ocr_info.len())); 
                }
            }
            else if(a[15]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[15]);
                if(x != None){
                    fin_pro=slice_in_between(&ocr_info.as_str(),int_vec[15],x);
                }
                else{
                    fin_pro=slice_in_between(&ocr_info.as_str(),int_vec[15],Some(ocr_info.len())); 
                }
            }
        }
        
        if(keyword.to_string() == "Certification"){
            let a = int_vec.clone();
            if(a[7]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[7]);
                if(x != None){
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[7],x);
                }
                else{
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[7],Some(ocr_info.len())); 
                }
            }
            else if(a[19]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[19]);
                if(x != None){
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[19],x);
                }
                else{
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[19],Some(ocr_info.len())); 
                }
            }
            else if(a[24]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[24]);
                if(x != None){
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[24],x);
                }
                else{
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[24],Some(ocr_info.len())); 
                }
            }
            else if(a[25]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[25]);
                if(x != None){
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[25],x);
                }
                else{
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[25],Some(ocr_info.len())); 
                }
            }
        }

        if(keyword.to_string() == "Interests"){
            let a = int_vec.clone();
            if(a[26]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[26]);
                if(x != None){
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[26],x);
                }
                else{
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[26],Some(ocr_info.len())); 
                }
            }
            else if(a[27]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[27]);
                if(x != None){
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[27],x);
                }
                else{
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[27],Some(ocr_info.len())); 
                }
            }
            else if(a[28]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[28]);
                if(x != None){
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[28],x);
                }
                else{
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[28],Some(ocr_info.len())); 
                }
            }
            else if(a[29]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[29]);
                if(x != None){
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[29],x);
                }
                else{
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[29],Some(ocr_info.len())); 
                }
            }
        }


        if(keyword.to_string() == "Achievements"){
            let a = int_vec.clone();
            if(a[32]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[32]);
                if(x != None){
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[32],x);
                }
                else{
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[32],Some(ocr_info.len())); 
                }
            }
            else if(a[33]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[33]);
                if(x != None){
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[33],x);
                }
                else{
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[33],Some(ocr_info.len())); 
                }
            }
            else if(a[34]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[34]);
                if(x != None){
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[34],x);
                }
                else{
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[34],Some(ocr_info.len())); 
                }
            }
            else if(a[35]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[35]);
                if(x != None){
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[35],x);
                }
                else{
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[35],Some(ocr_info.len())); 
                }
            }
        }

        if(keyword.to_string() == "Summary"){
            let a = int_vec.clone();
            if(a[36]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[36]);
                if(x != None){
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[36],x);
                }
                else{
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[36],Some(ocr_info.len())); 
                }
            }
            else if(a[37]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[37]);
                if(x != None){
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[37],x);
                }
                else{
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[37],Some(ocr_info.len())); 
                }
            }
            else if(a[38]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[38]);
                if(x != None){
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[38],x);
                }
                else{
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[38],Some(ocr_info.len())); 
                }
            }
            else if(a[39]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[39]);
                if(x != None){
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[39],x);
                }
                else{
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[39],Some(ocr_info.len())); 
                }
            }
        }


        if(keyword.to_string() == "Languages"){
            let a = int_vec.clone();
            if(a[44]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[44]);
                if(x != None){
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[44],x);
                }
                else{
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[44],Some(ocr_info.len())); 
                }
            }
            else if(a[45]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[45]);
                if(x != None){
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[45],x);
                }
                else{
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[45],Some(ocr_info.len())); 
                }
            }
        }
        if(keyword.to_string() == "Personal"){
            let a = int_vec.clone();
            if(a[46]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[46]);
                if(x != None){
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[46],x);
                }
                else{
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[46],Some(ocr_info.len())); 
                }
            }
            else if(a[47]!=None){
                let x= find_next_smallest(int_vec.clone(), int_vec[47]);
                if(x != None){
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[47],x);
                }
                else{
                    fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[47],Some(ocr_info.len())); 
                }
            }
    }
    
    if(keyword.to_string() == "Leadership"){
        let a = int_vec.clone();
        if(a[48]!=None){
            let x= find_next_smallest(int_vec.clone(), int_vec[48]);
            if(x != None){
                fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[48],x);
            }
            else{
                fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[48],Some(ocr_info.len())); 
            }
        }
        else if(a[49]!=None){
            let x= find_next_smallest(int_vec.clone(), int_vec[49]);
            if(x != None){
                fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[49],x);
            }
            else{
                fin_cer=slice_in_between(&ocr_info.as_str(),int_vec[49],Some(ocr_info.len())); 
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
    vec.into_iter().flatten().filter(|&x| x > val.unwrap_or(usize::MAX)).min()
}


fn find_word(s: &str, word: &str) -> Option<usize> {
    let found = s.find(word);
    match found {
        Some(pos) => Some(pos + word.len()),
        None => None,
    }
}




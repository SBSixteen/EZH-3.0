#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

/*
(Skeler Remix)
3 a.m., drinkin' with this pistol on my lap, uh
Six medications, guess there ain't no fixin' that
It's that broken motherfucker, knucklin' 'til I relapse
If I die, they might cry 'til they inherit my stacks
What a cold fuckin' world, tell me what's the word?
Only time I speak my heart is when the message come out slurred, uh
People-pleasing, always eager to fix a motherfucker's problems
Leave me at the bottom, more comfortable with Gomorrah and Sodom

I chew on fent' and rot my teeth
All for moments of peace and slow heartbeats
Got offered a God I dissed, who gon' save me? (Heh)
All I could do is laugh and say "c'est la vie" (whoa, ye)
Woe is me, I know you get the picture, uh
This type of pain will earn you seven figures
Check my DNA, my RNA come with predictors
This shit is scripture and for what I got, there's no elixirs, uh

Feel like I'm not enough, uh
Find myself playing my dad like Honey Boy and Shia LaBeouf, uh
My mama crying as she watch the clock
Text my bro I love him, even though he got my number blocked
I would give up everything to see my brothers clean
No second thoughts, it's fuck the cost or take me, leave 'em be
Take my money, say you love me, even if it's lies
That connection's so depressing but it's all I got (when I die)

When I die just play this fucking song
I was never meant for this, been tortured just to carry on (when I die)
Couple coupes, lotta zeroes and a couple homes
None of it did shit for me, cock the pistol and now I'm- (when I die)

Ayy, head in the clouds, it look like it might rain again
Always holdin' back tears, it's how I manage to pay the rent
Pay the bills, place the bet so I don't have to chase the check
Verified through $uicide, the glitz and glamour came and went
Wash my fucking soul and still that one stain is kept
I just wanna be loved, the root of all my pain
Except the type that comes with age
In death, I can finally lay and rest
I'm owed a little peace and I'm ready to erase the debt (I- I- I-)

I never planned on showing the world the face that hides behind the mask
I always thought the last thing I would hear would be the gun's blast
Fill in the black hole in my chest with sex and drugs, but it never lasts, it never lasts
Shit, two years ago Big Pharma should've just cashed me out
Now I got a deviated septum, I'mma just rat me out
Pat me down, ask me how I'm still depressed!
All they see is my set, all they see is lack of debt
All they see is what they wanna see

Calling me a wannabe
I don't wanna be in misery, exit the pharmacy
Get a girl I wanna see, I wanna see how hard I bleed
When she rips out my fucking heart, enter the pharmacy
I'm a private person and take pride in my verses
Fuck your whole opinion, I don't care if it worsens
I'll ruin my reputation and make sure it's on purpose
Fuck this shit!

When I die just play this fucking song
I was never meant for this, been tortured just to carry on (when I die)
Couple coupes, a lot of zeros and a couple homes
None of it is shit for me, cock the pistol and now I'm-


 */

pub mod LogReg;
pub mod SMTP_EZH;
pub mod PDF_EZH;
pub mod CloudStorage_EZH;
pub mod Single_Column;
mod NER_EZH;
use CloudStorage_EZH::{generate_PDF_queue_report, generate_dir};
use serde::{Deserialize, Serialize};
use reqwest::{header::{USER_AGENT, HeaderMap, HeaderValue, AUTHORIZATION}, Client};
use regex::Regex;
use serde_json::{json, Value};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    email: String,
    name: String,
    password: String,
    sub_type: i32, //0 = free, 1 = paid, 2 =pro
    twofa: bool,
    verified: bool,
    datasets: String,
}

struct filepath {
    path: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Confirmation {
    value: bool,
    response: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Confirmation_Login {
    value: bool,
    response: String,
    TWO_FA: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct User_Fetch {
    value: bool,
    user: User,
}


#[derive(Serialize, Deserialize, Debug)]
struct SMTP_CRED{
    user:String,
    pass:String,
}
struct User_Login {
    value: bool,
    twofa: bool,
    hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct V_Code{

    email:String,
    vcode:String

}

#[tokio::main]
async fn main() {

    // //Generate your cloud storage directories
    // generate_dir(String::from("monkaw.gmail.com")).await;

    // //Start PDF Processing. <Put PDF in TEMP_PDF> and set to true
    // generate_PDF_queue_report(String::from("nabeelmirza79@gmail.com"), false).await;

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![LogReg::create_user, LogReg::login_user, LogReg::match_vcode, LogReg::match_2fa, LogReg::remember_me_token])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    // let re = Regex::new(r"\+?\(?\d*\)? ?\(?\d+\)?\d*([\s./-]?\d{2,})+").unwrap();

    // let mut headers = HeaderMap::new();
    // headers.insert(USER_AGENT, HeaderValue::from_static("SBSixteen"));
    // headers.insert(AUTHORIZATION,HeaderValue::from_static("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") );

    // let client = Client::builder().default_headers(headers).build().unwrap();

    // //let temp = NER_EZH::Git_Repo_Info::automatic(&client, String::from("SBSixteen"), String::from("Academia")).await;
    // let r1 = client.get("https://api.github.com/users/SBSixteen/repos").send().await.unwrap().text().await.unwrap();

    // let value:Value = serde_json::from_str(&r1).unwrap();

    // println!("{}", &value.is_array());

    // let temp = value.as_array().unwrap().len();

    // for i in 0..temp{
    //     println!("{}",value[i]["name"]);
    // }

    //LogReg::toggle_2fa(String::from("nabeelmirza80@gmail.com")).await;
    //println!("{}", LogReg::create_user(String::from("nabeelmirza80@gmail.com"), String::from("Nabeel Mirza"), String::from("12345678")).await);
    //LogReg::create_user(String::from("nabeelmirza79@gmail.com"), String::from("Nabeel Mirza"), String::from("12345678")).await;
    //LogReg::toggle_2fa(String::from("nabeelmirza79@gmail.com")).await;
    //LogReg::generate_2fa(String::from("nabeelmirza80@gmail.com")).await;
    //LogReg::toggle_blocked(String::from("nabeelmirza79@gmail.com")).await;
    //println!("{}",login_user(String::from("nabeelmirza79@gmail.com"), String::from("12345678")).await);
    //match_2fa(String::from("nabeelmirza79@gmail.com"), String::from("BksQ")).await;
    //match_vcode(String::from("nabeelmirza79@gmail.com"), String::from("UOQYsuB")).await;

}

fn string_to_response(s: &str) -> Response {
    serde_json::from_str(s).unwrap()
}

fn string_to_user(s: &str) -> User {
    serde_json::from_str(s).unwrap()
}

async fn perform_batch_process(account:String,){



}

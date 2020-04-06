use crate::*;

pub fn print_time() {
    let start = SystemTime::now();
    let datetime: DateTime<Utc> = start.into();
    println!("{}", datetime.format("%d/%m/%Y %T"));
}

/// Iterates through ZoomResponse and inserts to database
pub fn save_all_meetings(data: &ZoomResponse) {
    for meeting in &data.meetings {
        // println!("{:#?}", &meeting);
        insert_meeting(meeting.clone());
    }
}

pub fn print_stats(data: &ZoomResponse, current_page: usize) {
    println!("°");
    println!("├── Start \t | {}", data.from);
    println!("├── End \t | {}", data.to);
    println!("├── Pages \t | {:?}", data.page_count);
    println!("├── Current \t | {:?}", current_page);
    println!("├── Per Page \t | {:?}", data.page_size);
    println!("├── Total \t | {:?}", data.total_records);
    println!(
        "└── Remaining \t | {:?}",
        data.total_records - (data.page_count * current_page)
    );

    let first_item = data.meetings.first().unwrap();

    println!("°");
    println!("├── {}", &first_item.uuid);
    println!("├── {}", &first_item.topic);
    println!("├── {}", &first_item.host);
    println!("├── {}", &first_item.start_time);
    println!("└── {}", &first_item.end_time);

    let last_item = data.meetings.last().unwrap();

    println!("°");
    println!("├── {}", &last_item.uuid);
    println!("├── {}", &last_item.topic);
    println!("├── {}", &last_item.host);
    println!("├── {}", &last_item.start_time);
    println!("└── {}", &last_item.end_time);
}

#[derive(FromArgs)]
/// Reach new heights.
pub struct GoUp {
    /// the location of your config file
    #[argh(option, short = 'c')]
    pub config: String,
}

pub fn execute(key: String, secret: String, seconds_between_calls: u64) {
    println!(
        r#"                                                                                                     
                                                                                                      
 .oooo.o oooo    ooo ooo. .oo.    .ooooo.   .oooo.     oooooooo  .ooooo.   .ooooo.  ooo. .oo.  .oo.   
d88(  "8  `88.  .8'  `888P"Y88b  d88' `"Y8 `P  )88b   d'""7d8P  d88' `88b d88' `88b `888P"Y88bP"Y88b  
`"Y88b.    `88..8'    888   888  888        .oP"888     .d8P'   888   888 888   888  888   888   888  
o.  )88b    `888'     888   888  888   .o8 d8(  888   .d8P'  .P 888   888 888   888  888   888   888  
8""888P'     .8'     o888o o888o `Y8bod8P' `Y888""8o d8888888P  `Y8bod8P' `Y8bod8P' o888o o888o o888o 
         .o..P'                                                                                       
         `Y8P'                                                                                        
"#
    );
    let zm_response = fetch_zoom_data(&key, &secret, None);
    let mut next_page_token;
    match zm_response {
        Ok(data) => {
            println!(
                "Total Estimated Runtime {} mins\n\n",
                (data.page_count * seconds_between_calls as usize) / 60
            );
            println!("─── Log");
            print_stats(&data, 1);
            save_all_meetings(&data);

            next_page_token = data.next_page_token.clone();
            println!("°");
            println!("└── Next Page Token: {:?}\n\n", next_page_token);
            thread::sleep(Duration::from_secs(seconds_between_calls)); // before we do next

            for _n in 2..data.page_count + 1 {
                let zm_response = fetch_zoom_data(&key, &secret, Some(&next_page_token));
                let successful_resp: bool = match zm_response {
                    Ok(response) => {
                        println!("─── Log");
                        print_stats(&response, _n);
                        save_all_meetings(&response);
                        next_page_token = response.next_page_token.clone();
                        let mut _did = true;
                        if next_page_token == "" {
                            _did = false;
                        } else {
                            println!("°");
                            println!("└── Next Page Token: {:?}\n\n", next_page_token);
                            // wait before out next call
                            thread::sleep(Duration::from_secs(seconds_between_calls));
                        }
                        _did
                    }
                    Err(err) => {
                        println!("{:#?}", err);
                        false // if fail - should stop
                    }
                };
                if !successful_resp {
                    println!("We had an issue at page {}", _n);
                    break;
                }
            }
        }
        Err(err) => println!("{:#?}", err), // if fail - should stop
    };

    println!("-");
}

pub fn send_slack_message(webhook: &str, message: &str) -> String {
    let mut final_url = webhook.to_string();
    println!("Slack URL\t | {}", final_url);
    let slack_message = format!("{{ \"text\": \"{}\" }}", message);
    let response = minreq::post(final_url)
        .with_header("Content-type", "application/json")
        .with_body(slack_message)
        .send()
        .unwrap();
    response.json().unwrap_or(String::from(""))
}

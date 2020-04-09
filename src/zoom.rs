use crate::*;

const URL: &str = "https://api.zoom.us/v2/metrics/meetings?type=past&page_size=300";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    iss: String,
    exp: usize,
}

pub fn get_ms_time() -> usize {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_millis() as usize
}

pub fn generate_jwt(key: &String, secret: &String) -> String {
    let my_claims = Claims {
        iss: key.to_string(),
        exp: get_ms_time(),
    };
    encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}

pub fn fetch_zoom_data(
    key: &String,
    secret: &String,
    next_page_token: Option<&String>,
) -> std::result::Result<ZoomResponse, ZoomError> {
    let token = generate_jwt(key, secret);

    let mut final_url = URL.to_string();

    if next_page_token.is_some() {
        final_url = format!("{}&next_page_token={}", final_url, next_page_token.unwrap())
    }

    if false {
        final_url = format!("{}&from={}&to={}", final_url, "2020-04-03", "2020-04-03")
    }

    // println!("-- DOWNLOADING --");
    println!("URL \t\t | {}", final_url);

    let response = minreq::get(final_url)
        .with_header("authorization", format!("Bearer {}", token))
        .send()
        .unwrap();

    let data: Value = response.json().unwrap();
    let is_error = data.as_object().unwrap().contains_key("code");

    if is_error {
        return Err(response.json().unwrap());
    };
    Ok(response.json().unwrap())
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZoomError {
    pub code: i64,
    pub message: String,
}

impl fmt::Display for ZoomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl error::Error for ZoomError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}

impl Default for ZoomResponse {
    fn default() -> Self {
        ZoomResponse {
            from: String::from(""),
            to: String::from(""),
            page_count: 0,
            page_size: 0,
            total_records: 0,
            next_page_token: String::from(""),
            meetings: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZoomResponse {
    pub from: String,
    pub to: String,
    pub page_count: usize,
    pub page_size: usize,
    pub total_records: usize,
    pub next_page_token: String,
    pub meetings: MeetingMetrics,
}

pub type MeetingMetrics = Vec<Meeting>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Meeting {
    pub uuid: String,
    pub duration: String,
    pub email: String,
    pub end_time: String,
    pub has_3rd_party_audio: bool,
    pub has_pstn: bool,
    pub has_recording: bool,
    pub has_screen_share: bool,
    pub has_sip: bool,
    pub has_video: bool,
    pub has_voip: bool,
    pub host: String,
    pub id: i64,
    pub participants: i64,
    pub start_time: String,
    pub topic: String,
    pub user_type: String,
}

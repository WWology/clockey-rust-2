use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Match2 {
    date: String,
    stream: Stream,
    #[serde(rename = "match2id")]
    match2_id: String,
    pagename: String,
    namespace: i64,
    #[serde(rename = "match2opponents")]
    match2_opponents: Vec<Match2Opponent>,
    wiki: String,
}

#[derive(Serialize, Deserialize)]
pub struct Match2Opponent {
    id: i64,
    #[serde(rename = "type")]
    match2_opponent_type: String,
    name: String,
    template: String,
    icon: String,
    score: i64,
    status: String,
    placement: i64,
    #[serde(rename = "match2players")]
    match2_players: Vec<Match2Player>,
    extradata: Vec<Option<serde_json::Value>>,
    teamtemplate: Teamtemplate,
}

#[derive(Serialize, Deserialize)]
pub struct Match2Player {
    id: i64,
    opid: i64,
    name: String,
    displayname: String,
    flag: String,
    extradata: Vec<Option<serde_json::Value>>,
}

#[derive(Serialize, Deserialize)]
pub struct Teamtemplate {
    template: String,
    page: String,
    name: String,
    shortname: String,
    bracketname: String,
    image: String,
    imagedark: String,
    legacyimage: String,
    legacyimagedark: String,
    imageurl: String,
    imagedarkurl: String,
    legacyimageurl: String,
    legacyimagedarkurl: String,
}

#[derive(Serialize, Deserialize)]
pub struct Stream {
    twitch_en_1: String,
    twitch: String,
}

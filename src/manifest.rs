use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct WebIcon<'a> {
    pub src: String,
    pub sizes: &'a str,
    #[serde(rename = "type")]
    pub typ: &'a str
}

#[derive(Serialize, Debug)]
pub struct WebManifest<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    pub start_url: &'a str,
    pub display: &'a str,
    pub theme_color: &'a str,
    pub name: &'a str,
    pub icons: Vec<WebIcon<'a>>
}
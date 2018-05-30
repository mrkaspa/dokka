use serde_json;
use serde_json::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Docker {
    #[serde(rename = "Command")]
    pub command: Option<String>,

    #[serde(rename = "CreatedAt")]
    pub created_at: Option<String>,

    #[serde(rename = "ID")]
    pub id: Option<String>,

    #[serde(rename = "Image")]
    pub image: Option<String>,

    #[serde(rename = "Labels")]
    pub labels: Option<String>,

    #[serde(rename = "LocalVolumes")]
    pub local_volumes: Option<String>,

    #[serde(rename = "Mounts")]
    pub mounts: Option<String>,

    #[serde(rename = "Names")]
    pub names: Option<String>,

    #[serde(rename = "Networks")]
    pub networks: Option<String>,

    #[serde(rename = "Ports")]
    pub ports: Option<String>,

    #[serde(rename = "RunningFor")]
    pub running_for: Option<String>,

    #[serde(rename = "Size")]
    pub size: Option<String>,

    #[serde(rename = "Status")]
    pub status: Option<String>,
}

pub fn parse_cmd(out: String) -> Result<Vec<Docker>, Error> {
    let mut dockers: Vec<Docker> = vec![];
    let docker_cads: Vec<&str> = out.split('\n').collect();
    for dc in docker_cads {
        if dc == "" {
            break;
        }
        let dc = dc.replace("'", "").to_owned();
        let d: Docker = serde_json::from_str(&dc)?;
        dockers.push(d);
    }
    Ok(dockers)
}

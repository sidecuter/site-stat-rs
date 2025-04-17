use serde::Deserialize;

#[derive(Clone, Deserialize, Debug, Default)]
pub(crate) struct DataDto {
    pub locations: Vec<LocationDto>,
    pub corpuses: Vec<CourpusDto>,
    pub plans: Vec<PlanDto>
}

#[derive(Clone, Deserialize, Debug, Default)]
pub(crate) struct CrossingDto(pub String, pub String, pub f32);

#[derive(Clone, Deserialize, Debug, Default)]
pub(crate) struct LocationDto {
    pub id: String,
    pub title: String,
    pub short: String,
    pub  available: bool,
    pub  address: String,
    pub crossings: Option<Vec<CrossingDto>>
}

#[derive(Clone, Deserialize, Debug, Default)]
pub(crate) struct CourpusDto {
    pub id: String,
    #[serde(rename(deserialize = "locationId"))]
    pub location_id: String,
    pub title: String,
    pub available: bool,
    pub stairs: Option<Vec<Vec<String>>>
}

#[derive(Clone, Deserialize, Debug, Default)]
pub(crate) struct EntranceDto(pub String, pub String);

#[derive(Clone, Deserialize, Debug, Default)]
pub(crate) struct PlanDto {
    pub id: String,
    #[serde(rename(deserialize = "corpusId"))]
    pub corpus_id: String,
    pub floor: String,
    pub  available: bool,
    #[serde(rename(deserialize = "wayToSvg"), default)]
    pub way_to_svg: String,
    pub graph: Option<Vec<GraphDto>>,
    pub  entrances: Option<Vec<EntranceDto>>,
    pub  nearest: NearestDto
}

#[derive(Clone, Deserialize, Debug, Default)]
pub(crate) struct NearestDto {
    #[serde(default)]
    pub enter: String,
    pub wm: Option<String>,
    pub  ww: Option<String>,
    pub ws: Option<String>,
}

#[derive(Clone, Deserialize, Debug, Default)]
pub(crate) struct NeighborDto(pub String, pub f32);

#[derive(Clone, Deserialize, Debug, Default)]
pub(crate) struct GraphDto {
    pub id: String,
    pub x: f32,
    pub y: f32,
    #[serde(rename(deserialize = "type"))]
    pub type_: String,
    #[serde(rename(deserialize = "neighborData"))]
    pub neighbor_data: Vec<NeighborDto>
}

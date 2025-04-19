use std::collections::HashMap;
use std::sync::{Arc, MutexGuard};
use crate::schemas::graph::Graph;
use crate::schemas::dto::{GraphDto, DataDto, LocationDto, CorpusDto, PlanDto};

#[derive(Clone, Default, Debug)]
pub struct DataEntry {
    pub locations: Vec<Arc<LocationData>>,
    pub corpuses: Vec<Arc<CorpusData>>,
    pub plans: Vec<Arc<PlanData>>,
}

#[derive(Clone, Default, Debug)]
pub struct Crossing(pub String, pub String, pub f32);

#[derive(Clone, Default, Debug)]
pub struct LocationData {
    pub id: String,
    pub title: String,
    pub short: String,
    pub available: bool,
    pub address: String,
    pub crossings: Vec<Crossing>
}

#[derive(Clone, Debug)]
pub struct CorpusData {
    pub id: String,
    pub title: String,
    pub available: bool,
    pub location: Arc<LocationData>,
    pub stairs: Vec<Vec<String>>,
}

type Id = String;
type RoomId = String;
type CircleId = Id;

#[derive(Clone, Default, Debug)]
pub struct PlanEntrance(pub RoomId, pub CircleId);

#[derive(Clone, Debug)]
pub struct PlanData {
    pub id: String,
    pub floor: u8,
    pub available: bool,
    pub way_to_svg: String,
    pub graph: Vec<GraphDto>,
    pub entrances: Vec<PlanEntrance>,
    pub corpus: Arc<CorpusData>
}

pub async fn parse_data() -> Result<DataEntry, Box<dyn std::error::Error>> {
    const DATA_URL: &str = "https://mospolynavigation.github.io/polyna-preprocess/locationsV2.json";

    let data_dto = fetch_data(DATA_URL).await?;
    let locations = parse_locations(&data_dto.locations);
    let corpuses = parse_corpuses(&data_dto.corpuses, &locations);
    let plans = parse_plans(&data_dto.plans, &corpuses);

    Ok(DataEntry {
        locations,
        corpuses,
        plans,
    })
}

fn parse_locations(locations_dto: &[LocationDto]) -> Vec<Arc<LocationData>> {
    locations_dto.iter()
        .map(|dto| Arc::new(LocationData {
            id: dto.id.clone(),
            title: dto.title.clone(),
            short: dto.short.clone(),
            available: dto.available,
            address: dto.address.clone(),
            crossings: dto.crossings.as_deref().map_or_else(Vec::new, |v|
                v.iter().map(|c| Crossing(c.0.clone(), c.1.clone(), c.2)).collect()
            )
        }))
            .collect()
}

fn parse_corpuses(corpuses_dto: &[CorpusDto], locations: &[Arc<LocationData>]) -> Vec<Arc<CorpusData>> {
    let location_map: HashMap<_, _> = locations.iter()
        .map(|loc| (loc.id.as_str(), loc))
        .collect();

    corpuses_dto.iter()
        .filter_map(|dto| {
            let location = location_map.get(dto.location_id.as_str())?;
            Some(Arc::new(CorpusData {
                id: dto.id.clone(),
                title: dto.title.clone(),
                available: dto.available,
                location: Arc::clone(location),
                stairs: dto.stairs.clone().unwrap_or_default(),
            }))
        })
        .collect()
}

fn parse_plans(plans_dto: &[PlanDto], corpuses: &[Arc<CorpusData>]) -> Vec<Arc<PlanData>> {
    let corpus_map: HashMap<_, _> = corpuses.iter()
        .map(|corpus| (corpus.id.as_str(), corpus))
        .collect();

    plans_dto.iter()
        .filter_map(|dto| {
            let corpus = corpus_map.get(dto.corpus_id.as_str())?;
            Some(Arc::new(PlanData {
                id: dto.id.clone(),
                floor: dto.floor.parse().ok()?,
                available: dto.available,
                way_to_svg: dto.way_to_svg.clone(),
                graph: dto.graph.clone().unwrap_or_default(),
                entrances: dto.entrances.as_deref().map_or_else(Vec::new, |v|
                    v.iter().map(|e| PlanEntrance(e.0.clone(), e.1.clone())).collect()),
                corpus: Arc::clone(corpus),
            }))
        })
        .collect()
}

pub async fn fetch_data(url: &str) -> Result<DataDto, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; WOW64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.5666.197 Safari/537.36")
        .send()
        .await?;
    
    match response.error_for_status_ref() { 
        Ok(_) => response.json().await.map_err(Into::into),
        Err(e) => Err(format!("Request failed with status: {e}").into())
    }
}

pub async fn get_graph(
    data: MutexGuard<'_, DataEntry>,
    loc_id: &str
) -> Option<Graph> {
    let location = data.locations.iter().find(|loc| loc.id == loc_id)?;

    let plans = data.plans.iter()
        .filter(|plan| plan.corpus.location.id == loc_id)
        .cloned()
        .collect();

    let corpuses = data.corpuses.iter()
        .filter(|corpus| corpus.location.id == loc_id)
        .cloned()
        .collect();

    Some(Graph::new(location.clone(), plans, corpuses))
}

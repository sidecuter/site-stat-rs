use std::sync::{Arc, MutexGuard};
use reqwest::StatusCode;
use crate::schemas::dto::{GraphDto, DataDto, LocationDto};
use crate::schemas::graph::Graph;

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
    pub location: Arc<LocationData>,  // Shared владение
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

    // 1. Создаем все локации и оборачиваем в Arc
    let locations: Vec<Arc<LocationData>> = data_dto.locations
        .iter()
        .map(|dto| Arc::new(parse_location(dto)))
        .collect();

    // 2. Создаем корпуса с shared ссылками на локации
    let corpuses: Vec<Arc<CorpusData>> = data_dto.corpuses
        .iter()
        .map(|dto| {
            let location = locations.iter()
                .find(|loc| loc.id == dto.location_id)
                .expect("Location not found")
                .clone();  // Клонируем Arc

            Arc::new(CorpusData {
                id: dto.id.clone(),
                title: dto.title.clone(),
                location,
                available: dto.available,
                stairs: dto.stairs.clone().unwrap_or_default(),
            })
        })
        .collect();

    // 3. Создаем планы с shared ссылками на корпуса
    let plans: Vec<Arc<PlanData>> = data_dto.plans
        .iter()
        .map(|dto| {
            let corpus = corpuses.iter()
                .find(|c| c.id == dto.corpus_id)
                .expect("Corpus not found")
                .clone();  // Клонируем Arc

            Arc::new(PlanData {
                id: dto.id.clone(),
                floor: dto.floor.parse().expect("Invalid floor format"),
                available: dto.available,
                way_to_svg: dto.way_to_svg.clone(),
                graph: dto.graph.clone().unwrap_or_default(),
                entrances: dto.entrances.as_ref().map_or_else(
                    Vec::new,
                    |v| v.iter().map(|e| PlanEntrance(e.0.clone(), e.1.clone())).collect()
                ),
                corpus,
            })
        })
        .collect();

    Ok(DataEntry {
        locations,
        corpuses,
        plans,
    })
}

fn parse_location(dto: &LocationDto) -> LocationData {
    LocationData {
        id: dto.id.clone(),
        title: dto.title.clone(),
        short: dto.short.clone(),
        available: dto.available,
        address: dto.address.clone(),
        crossings: dto.crossings.as_ref().map_or_else(
            Vec::new,
            |v| v.iter()
                .map(|c| Crossing(c.0.clone(), c.1.clone(), c.2))
                .collect()
        ),
    }
}

pub async fn fetch_data(url: &str) -> Result<DataDto, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; WOW64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.5666.197 Safari/537.36")
        .send()
        .await?;

    match response.status() {
        StatusCode::OK => response.json::<DataDto>()
            .await.map_err(|e| e.into()),
        status => Err(
            format!("Request failed with status: {status}").into(),
        ),
    }
}

pub async fn get_graph(
    data: MutexGuard<'_, DataEntry>,
    loc_id: &str
) -> Option<Graph> {
    // Находим нужную локацию
    let location = data.locations.iter()
        .find(|loc| loc.id == loc_id)
        .map(Arc::clone)?;

    // Фильтруем планы для данной локации
    let plans: Vec<Arc<PlanData>> = data.plans.iter()
        .filter(|plan| plan.corpus.location.id == loc_id)
        .map(Arc::clone)
        .collect();

    // Фильтруем корпуса для данной локации
    let corpuses: Vec<Arc<CorpusData>> = data.corpuses.iter()
        .filter(|corpus| corpus.location.id == loc_id)
        .map(Arc::clone)
        .collect();

    Some(Graph::new(
        location,
        plans,
        corpuses,
    ))
}

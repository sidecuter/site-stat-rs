use std::sync::Arc;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Reverse;
use std::str::FromStr;
use std::time::Instant;
use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_web::body::BoxBody;
use serde::Serialize;
use utoipa::{PartialSchema, ToSchema};
use utoipa::openapi::{ObjectBuilder, RefOr, Schema};
use crate::schemas::data::{CorpusData, LocationData, PlanData};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum VertexType {
    Hallway,
    EntrancesToAu,
    Stair,
    Crossing,
    CrossingSpace,
    Lift,
}

impl VertexType {
    pub fn as_str(&self) -> &str {
        match self {
            VertexType::Hallway => "hallway",
            VertexType::EntrancesToAu => "entrancesToAu",
            VertexType::Stair => "stair",
            VertexType::Crossing => "crossing",
            VertexType::CrossingSpace => "crossingSpace",
            VertexType::Lift => "lift",
        }
    }
}

impl FromStr for VertexType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "hallway" => Ok(VertexType::Hallway),
            "entrancesToAu" => Ok(VertexType::EntrancesToAu),
            "stair" => Ok(VertexType::Stair),
            "crossing" => Ok(VertexType::Crossing),
            "crossingSpace" => Ok(VertexType::CrossingSpace),
            "lift" => Ok(VertexType::Lift),
            _ => Err(format!("Invalid vertex type: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Vertex {
    pub id: String,
    pub x: f32,
    pub y: f32,
    #[serde(rename(serialize = "type"))]
    #[schema(rename = "type")]
    pub type_: VertexType,
    #[serde(skip_serializing)]
    pub neighbor_data: Vec<(String, f32)>,
    #[serde(skip_serializing)]
    pub plan: Arc<PlanData>,
}

#[derive(Debug, Serialize)]
pub struct ShortestWay {
    pub way: Vec<Arc<Vertex>>,
    pub distance: i32,
}

impl PartialSchema for ShortestWay {
    fn schema() -> RefOr<Schema> {
        RefOr::T(Schema::Object(
            ObjectBuilder::new()
                .property("way", Vec::<Vertex>::schema())
                .property("distance", i32::schema())
                .build()
        ))
    }
}

impl ToSchema for ShortestWay {}

impl Responder for ShortestWay {
    type Body = BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}

pub struct Graph {
    pub location: Arc<LocationData>,
    pub plans: Vec<Arc<PlanData>>,
    pub corpuses: Vec<Arc<CorpusData>>,
    pub vertexes: HashMap<String, Arc<Vertex>>,
}

impl Graph {
    pub fn new(
        location: Arc<LocationData>,
        plans: Vec<Arc<PlanData>>,
        corpuses: Vec<Arc<CorpusData>>,
    ) -> Self {
        let mut graph = Self {
            location,
            plans,
            corpuses,
            vertexes: HashMap::new(),
        };
        graph.fill_vertexes_by_raw_vertexes();
        graph.add_stairs();
        graph.add_crossings();
        graph
    }

    fn fill_vertexes_by_raw_vertexes(&mut self) {
        let plans_of_loc: Vec<_> = self.plans
            .iter()
            .filter(|plan| plan.corpus.location.id == self.location.id)
            .collect();

        for plan in plans_of_loc {
            for raw_vertex in &plan.graph {
                let vertex = Arc::new(Vertex {
                    id: raw_vertex.id.clone(),
                    x: raw_vertex.x,
                    y: raw_vertex.y,
                    type_: VertexType::from_str(&raw_vertex.type_).unwrap(),
                    neighbor_data: raw_vertex.neighbor_data
                        .iter()
                        .map(|n| (n.0.clone(), n.1))
                        .collect(),
                    plan: Arc::clone(plan),
                });
                self.vertexes.insert(vertex.id.clone(), vertex);
            }
        }
    }

    fn add_stairs(&mut self) {
        let corpuses_of_loc: Vec<_> = self.corpuses
            .iter()
            .filter(|corpus| corpus.location.id == self.location.id)
            .collect();

        for corpus in corpuses_of_loc {
            for stairs_group in &corpus.stairs {
                for pair in stairs_group.windows(2) {
                    let stair1 = &pair[0];
                    let stair2 = &pair[1];

                    if let Some(v1) = self.vertexes.get_mut(stair1) {
                        Arc::make_mut(v1).neighbor_data.push((stair2.clone(), 1085.));
                    }
                    if let Some(v2) = self.vertexes.get_mut(stair2) {
                        Arc::make_mut(v2).neighbor_data.push((stair1.clone(), 916.));
                    }
                }
            }
        }
    }

    fn add_crossings(&mut self) {
        for crossing in &self.location.crossings {
            let c1 = &crossing.0;
            let c2 = &crossing.1;
            let dist = crossing.2;

            if let Some(v1) = self.vertexes.get_mut(c1) {
                Arc::make_mut(v1).neighbor_data.push((c2.clone(), dist));
            }
            if let Some(v2) = self.vertexes.get_mut(c2) {
                Arc::make_mut(v2).neighbor_data.push((c1.clone(), dist));
            }
        }
    }

    pub fn find_vertex_by_id(&self, id: &str) -> Option<Arc<Vertex>> {
        Some(Arc::clone(self.vertexes.get(id)?))
    }

    pub fn has_vertex(&self, id: &str) -> bool {
        self.vertexes.contains_key(id)
    }

    pub fn get_shortest_way_from_to(&self, start: &str, end: &str) -> ShortestWay {
        let start_time = Instant::now();
        let allowed_types: HashSet<&str> = [
            "hallway", "lift", "stair", "corpusTransition", "crossingSpace"
        ].into_iter().collect();

        let valid_ids: HashSet<_> = self.vertexes
            .iter()
            .filter(|(vid, v)| {
                allowed_types.contains(v.type_.as_str()) ||
                    *vid == start || *vid == end ||
                    vid.contains("crossing")
            })
            .map(|(vid, _)| vid.clone())
            .collect();

        let mut distances: HashMap<String, f32> = valid_ids.iter()
            .map(|vid| (vid.clone(), f32::INFINITY))
            .collect();
        distances.insert(start.to_string(), 0.0);

        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0, start.to_string())));

        let mut previous: HashMap<String, Option<String>> = HashMap::new();
        let mut visited = HashSet::new();

        while let Some(Reverse((_, current_id))) = heap.pop() {
            if current_id == end {
                break;
            }

            if visited.contains(&current_id) {
                continue;
            }
            visited.insert(current_id.clone());

            let current = match self.vertexes.get(&current_id) {
                Some(v) => v,
                None => continue,
            };

            for (neighbor, dist) in &current.neighbor_data {
                if !valid_ids.contains(neighbor) {
                    continue;
                }

                let new_dist = distances[&current_id] + dist;
                let neighbor_dist = distances.entry(neighbor.clone())
                    .or_insert(f32::INFINITY);

                if new_dist < *neighbor_dist {
                    *neighbor_dist = new_dist;
                    previous.insert(neighbor.clone(), Some(current_id.clone()));
                    heap.push(Reverse((new_dist.floor() as i32, neighbor.clone())));
                }
            }
        }

        let mut path = Vec::new();
        let mut current = end.to_string();
        while let Some(Some(prev)) = previous.get(&current) {
            path.push(current.clone());
            current = prev.clone();
        }
        path.push(start.to_string());
        path.reverse();

        let way: Vec<Arc<Vertex>> = path.iter()
            .filter_map(|id| self.vertexes.get(id).map(Arc::clone))
            .collect();

        let distance = distances.get(end)
            .map(|d| d.floor() as i32)
            .unwrap_or(i32::MAX);

        let duration = start_time.elapsed();
        println!("The task took {:.4} seconds to complete.", duration.as_secs_f64());

        ShortestWay { way, distance }
    }

    pub fn get_distance_between_vertexes(&self, v1: &Vertex, v2_id: &str) -> f32 {
        v1.neighbor_data.iter()
            .find(|(id, _)| id == v2_id)
            .expect("Neighbor not found")
            .1
    }
}

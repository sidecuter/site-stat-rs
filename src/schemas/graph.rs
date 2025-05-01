use crate::impl_responder;
use crate::schemas::data::{CorpusData, LocationData, PlanData};
use ordered_float::OrderedFloat;
use serde::Serialize;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Instant;
use utoipa::openapi::{ObjectBuilder, RefOr, Schema};
use utoipa::{PartialSchema, ToSchema};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, ToSchema)]
#[cfg_attr(test, derive(bincode::Decode))]
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
    #[allow(clippy::missing_const_for_fn)]
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Hallway => "hallway",
            Self::EntrancesToAu => "entrancesToAu",
            Self::Stair => "stair",
            Self::Crossing => "crossing",
            Self::CrossingSpace => "crossingSpace",
            Self::Lift => "lift",
        }
    }
}

impl FromStr for VertexType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "hallway" => Ok(Self::Hallway),
            "entrancesToAu" => Ok(Self::EntrancesToAu),
            "stair" => Ok(Self::Stair),
            "crossing" => Ok(Self::Crossing),
            "crossingSpace" => Ok(Self::CrossingSpace),
            "lift" => Ok(Self::Lift),
            _ => Err(format!("Invalid vertex type: {s}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[cfg_attr(test, derive(bincode::Decode))]
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
                .build(),
        ))
    }
}

impl ToSchema for ShortestWay {}

#[derive(Clone, Debug)]
#[cfg_attr(test, derive(bincode::Decode))]
pub struct Graph {
    pub location: Arc<LocationData>,
    pub plans: Vec<Arc<PlanData>>,
    pub corpuses: Vec<Arc<CorpusData>>,
    pub vertexes: HashMap<String, Arc<Vertex>>,
}

impl Graph {
    #[must_use]
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
        self.plans
            .iter()
            .filter(|plan| plan.corpus.location.id == self.location.id)
            .for_each(|plan| {
                plan.graph.iter().for_each(|raw_vertex| {
                    self.vertexes.insert(
                        raw_vertex.id.clone(),
                        Arc::new(Vertex {
                            id: raw_vertex.id.clone(),
                            x: raw_vertex.x,
                            y: raw_vertex.y,
                            type_: VertexType::from_str(&raw_vertex.type_).unwrap(),
                            neighbor_data: raw_vertex
                                .neighbor_data
                                .iter()
                                .map(|n| (n.0.clone(), n.1))
                                .collect(),
                            plan: Arc::clone(plan),
                        }),
                    );
                });
            });
    }

    fn add_stairs(&mut self) {
        self.corpuses
            .iter()
            .filter(|corpus| corpus.location.id == self.location.id)
            .for_each(|corpus| {
                corpus.stairs.iter().for_each(|stairs_group| {
                    stairs_group.windows(2).for_each(|pair| {
                        let stair1 = &pair[0];
                        let stair2 = &pair[1];
                        Arc::make_mut(self.vertexes.get_mut(stair1).unwrap())
                            .neighbor_data
                            .push((stair2.clone(), 1085.));
                        Arc::make_mut(self.vertexes.get_mut(stair2).unwrap())
                            .neighbor_data
                            .push((stair1.clone(), 916.));
                    });
                });
            });
    }

    fn add_crossings(&mut self) {
        self.location.crossings.iter().for_each(|crossing| {
            let c1 = &crossing.0;
            let c2 = &crossing.1;
            let dist = crossing.2;
            Arc::make_mut(self.vertexes.get_mut(c1).unwrap())
                .neighbor_data
                .push((c2.clone(), dist));
            Arc::make_mut(self.vertexes.get_mut(c2).unwrap())
                .neighbor_data
                .push((c1.clone(), dist));
        });
    }

    #[must_use]
    pub fn has_vertex(&self, id: &str) -> bool {
        self.vertexes.contains_key(id)
    }
    
    #[allow(clippy::cast_possible_truncation)]
    pub fn get_shortest_way_from_to(&self, start: &str, end: &str) -> ShortestWay {
        let start_time = Instant::now();

        // Подготовка структур данных
        let (id_to_index, index_to_vertex) = self.prepare_valid_vertices(start, end);
        let start_idx = id_to_index[start];
        let end_idx = id_to_index[end];

        // Инициализация алгоритма
        let mut distances = vec![f32::INFINITY; index_to_vertex.len()];
        distances[start_idx] = 0.0;

        let mut heap = BinaryHeap::new();
        heap.push(Reverse((OrderedFloat(0.0), start_idx)));

        let mut prev = vec![None; index_to_vertex.len()];
        let mut visited = vec![false; index_to_vertex.len()];

        // Основной цикл Дейкстры
        while let Some(Reverse((OrderedFloat(dist), idx))) = heap.pop() {
            if idx == end_idx {
                break;
            }
            if visited[idx] {
                continue;
            }
            visited[idx] = true;

            for (neighbor_id, edge_dist) in &index_to_vertex[idx].neighbor_data {
                if let Some(&neighbor_idx) = id_to_index.get(neighbor_id) {
                    let new_dist = dist + edge_dist;
                    if new_dist < distances[neighbor_idx] {
                        distances[neighbor_idx] = new_dist;
                        prev[neighbor_idx] = Some(idx);
                        heap.push(Reverse((OrderedFloat(new_dist), neighbor_idx)));
                    }
                }
            }
        }

        tracing::info!(
            "The task took {:.4} seconds",
            start_time.elapsed().as_secs_f64()
        );

        // Формирование результата
        ShortestWay {
            way: Self::reconstruct_path(start_idx, end_idx, &prev, &index_to_vertex),
            distance: distances[end_idx].floor() as i32,
        }
    }

    fn prepare_valid_vertices(
        &self,
        start: &str,
        end: &str,
    ) -> (HashMap<String, usize>, Vec<Arc<Vertex>>) {
        let allowed_types: HashSet<&str> = [
            "hallway",
            "lift",
            "stair",
            "corpusTransition",
            "crossingSpace",
        ]
        .into();

        let mut mapping = HashMap::new();
        let mut vertices = Vec::new();

        for v in self.vertexes.values() {
            if allowed_types.contains(v.type_.as_str())
                || v.id == start
                || v.id == end
                || v.id.contains("crossing")
            {
                mapping.insert(v.id.clone(), vertices.len());
                vertices.push(v.clone());
            }
        }

        (mapping, vertices)
    }

    fn reconstruct_path(
        start_idx: usize,
        end_idx: usize,
        prev: &[Option<usize>],
        vertices: &[Arc<Vertex>],
    ) -> Vec<Arc<Vertex>> {
        let mut path = Vec::new();
        let mut current = end_idx;

        while current != start_idx {
            path.push(vertices[current].clone());
            current = prev[current].unwrap_or(start_idx);
        }
        path.push(vertices[start_idx].clone());
        path.reverse();
        path
    }
}

impl_responder!(ShortestWay);

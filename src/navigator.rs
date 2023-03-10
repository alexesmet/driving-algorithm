use std::{rc::Rc, collections::HashMap};

mod map {
    use crate::model::Road;

    pub type RoadIndex = usize;

    #[derive(Debug)]
    pub struct RoadNode { pub road: Road, pub next: Vec<RoadIndex>}

    #[derive(Debug)]
    pub struct RoadMap {
        nodes: Vec<RoadNode>,
    }
    #[derive(Debug)]
    pub enum RoadMapError {
        NoRoadsPresent,
        DeadEndPresent,
        NextIndexOutOfBounds
    }

    impl RoadMap {
        pub fn new(nodes: Vec<RoadNode>) -> Result<Self, RoadMapError> {
            if nodes.is_empty() { return Err(RoadMapError::NoRoadsPresent)}
            for node in nodes.iter() {
                if node.next.is_empty() { return Err(RoadMapError::DeadEndPresent) }
                for next in node.next.iter() {
                    if next > &nodes.len() { return Err(RoadMapError::NextIndexOutOfBounds)}
                }
            }

            return Ok( Self { nodes });
        }
        pub fn get_road_by_id(&self, id: RoadIndex) -> &Road {
            &self.nodes[id].road
        }
        pub fn get_next_roads(&self, id: RoadIndex) -> &[RoadIndex] {
            &self.nodes[id].next
        }
        pub fn get_amount_of_roads(&self) -> usize {
            self.nodes.len()
        }
        pub fn get_roads(&self) -> impl Iterator<Item=&Road> + '_ {
            self.nodes.iter().map(|n| &n.road)
        }
    }
}

pub use map::{RoadMap,RoadMapError,RoadNode};
use map::*;
use crate::model::Road;

pub struct Navigator {
    map: Rc<RoadMap>,
    current_id: RoadIndex
}

#[derive(Debug)]
pub enum NavigatorCreationError {
    CurrentIndexOutOfBounds
}

impl Navigator {

    pub fn new(map: Rc<RoadMap>, current_id: RoadIndex) -> Result<Self, NavigatorCreationError> {
        if current_id > map.get_amount_of_roads() { 
            Err(NavigatorCreationError::CurrentIndexOutOfBounds) 
        } else {
            Ok(Self { map, current_id })
        }
    }
    pub fn get_road(&self) -> &Road {
        self.map.get_road_by_id(self.current_id)
    }
    pub fn get_next_road(&self) -> &Road {
        return self.map.get_road_by_id(self.map.get_next_roads(self.current_id)[0]);
    }
    pub fn switch_to_next_road(&mut self) {
        self.current_id = self.map.get_next_roads(self.current_id)[0];
    }
}


#[derive(Debug)]
pub enum RoadMapDeserializationError {
    /// Provided string is not a valid TOML
    InvalidFormat { error: toml::de::Error },
    MissingField { path: String },
    InvalidFieldType { path: String },
    UnknowRoadType { path: String, value: String },
    InvalidFractionNotation { path: String, error: FractionNotationError },
    InvalidDirectionNotation { path: String },
    UnknownRoadReferece { reference: String },
    RoadMapError { error: RoadMapError }

}

// TODO: Would be great to have a macro instead of a function
macro_rules! get_toml_field {
    ($table:expr, $field:expr, $as_type:expr, $error_context:expr) => { {
        let value = $table.get($field).ok_or_else(|| RoadMapDeserializationError::MissingField { path: format!("{}.{}",$error_context,$field) })?;
        $as_type(value).ok_or_else(|| RoadMapDeserializationError::InvalidFieldType { path: format!("{}.{}",$error_context,$field) })
    } };
}

pub fn road_nodes_from_toml(toml_str: &str) -> Result<RoadMap, RoadMapDeserializationError>{


    let table: toml::Table = toml::from_str(toml_str)
        .map_err(|error| RoadMapDeserializationError::InvalidFormat { error })?;

    let roads = get_toml_field!(&table, "roads", toml::Value::as_table, "")?;
    let nodes = get_toml_field!(&table, "nodes", toml::Value::as_table, "")?;

    let mut roads_ordered: Vec<(Road, Vec<_>)> = Vec::with_capacity(roads.len());
    let mut map_keys_to_indicies: HashMap<String, usize> = HashMap::with_capacity(roads.len());


    for (key, entry) in roads.iter() {
        let error_context = format!("roads.{}",key);
        let entry = entry.as_table().ok_or_else(|| RoadMapDeserializationError::InvalidFieldType{ path: (&error_context).clone() })?;
        let road = match get_toml_field!(entry, "type", toml::Value::as_str, &error_context)? {
            "Line" => {
                let start_x = get_toml_field!(entry, "start_x", toml::Value::as_float, &error_context)? as f32;
                let start_y = get_toml_field!(entry, "start_y", toml::Value::as_float, &error_context)? as f32;
                let end_x   = get_toml_field!(entry, "end_x",   toml::Value::as_float, &error_context)? as f32;
                let end_y   = get_toml_field!(entry, "end_y",   toml::Value::as_float, &error_context)? as f32;
                Ok(Road::Line { start: (start_x, start_y), end: (end_x, end_y) })
            },
            "Turn" => {
                let x = get_toml_field!(entry, "x", toml::Value::as_float, &error_context)? as f32;
                let y = get_toml_field!(entry, "y", toml::Value::as_float, &error_context)? as f32;
                let r = get_toml_field!(entry, "r", toml::Value::as_float, &error_context)? as f32;
                let start = get_toml_field!(entry, "start", toml::Value::as_str, &error_context)?;
                let end   = get_toml_field!(entry, "end",   toml::Value::as_str, &error_context)?;
                let dir   = get_toml_field!(entry, "dir",   toml::Value::as_str, &error_context)?;
                let start_angle = fraction_notation_to_angle(start)
                    .map_err(|error| RoadMapDeserializationError::InvalidFractionNotation { path: format!("{}.start",&error_context), error })?;
                let end_angle =   fraction_notation_to_angle(end)
                    .map_err(|error| RoadMapDeserializationError::InvalidFractionNotation { path: format!("{}.end",  &error_context), error })?;
                let direction = match dir {
                    "CCW" => Ok(crate::model::RoadTurnDirection::CCW),
                    "CW" => Ok(crate::model::RoadTurnDirection::CW),
                    _ => Err(RoadMapDeserializationError::InvalidDirectionNotation { path: format!("{}.dir", &error_context) })
                }?;
                Ok(Road::Turn { coordinates: (x,y), radius: r, start_angle, end_angle, direction })
            },
            unknown => Err(RoadMapDeserializationError::UnknowRoadType { path: format!("{}.type",&error_context), value: unknown.to_string() })
        }?;

        let error_context = format!("nodes.{}",key);
        let next_roads: Vec<String> = get_toml_field!(nodes, key, toml::Value::as_array, &error_context)?
            .iter()
            .enumerate()
            .map(|(i,v)| v.as_str().map(|v|v.to_string()).or_else(|| v.as_integer().map(|v|v.to_string()))
                 .ok_or(RoadMapDeserializationError::InvalidFieldType { path: format!("{}[{}]", &error_context, i) }))
            .collect::<Result<Vec<String>,RoadMapDeserializationError>>()?;

        let this_road_index = roads_ordered.len();
        map_keys_to_indicies.insert(key.to_owned(), this_road_index);
        roads_ordered.push((road, next_roads));
    }

    let road_nodes = roads_ordered.into_iter()
        .map(|(road,nexts)| { 
            Ok(RoadNode { road, next: nexts.into_iter()
                .map(|k| map_keys_to_indicies.get(&k).ok_or_else(|| RoadMapDeserializationError::UnknownRoadReferece { reference: k.to_owned() }) )
                .map(|r| r.map(|v| *v))
                .collect::<Result<Vec<usize>,RoadMapDeserializationError>>()? })
        })
        .collect::<Result<Vec<RoadNode>,RoadMapDeserializationError>>()?;

    RoadMap::new(road_nodes).map_err(|error| RoadMapDeserializationError::RoadMapError { error })
}


#[derive(Debug)]
pub enum FractionNotationError {
    DivisionSymbolAbsent,
    InvalidNumerator,
    InvalidDenominator,
}

fn fraction_notation_to_angle(s: &str) -> Result<f32, FractionNotationError> {
    use FractionNotationError::*;
    let (a,b) = s.split_once('/').ok_or(DivisionSymbolAbsent)?;
    Ok(std::f32::consts::PI * a.parse::<f32>().map_err(|_| InvalidNumerator)? / b.parse::<f32>().map_err(|_| InvalidDenominator)? )
}



use std::fmt::Debug;
impl Debug for Navigator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Navigator").field("map", &"hidden").field("current_id", &self.current_id).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_follow_circular_road() {

        let road_1 = Road::Turn { coordinates: ( 10., 10.), radius: 10., start_angle: (0.), end_angle: (0.), direction: crate::model::RoadTurnDirection::CCW };
        let road_2 = Road::Turn { coordinates: ( 10.,-10.), radius: 10., start_angle: (0.), end_angle: (0.), direction: crate::model::RoadTurnDirection::CCW };
        let road_3 = Road::Turn { coordinates: (-10.,-10.), radius: 10., start_angle: (0.), end_angle: (0.), direction: crate::model::RoadTurnDirection::CCW };
        let road_4 = Road::Turn { coordinates: (-10., 10.), radius: 10., start_angle: (0.), end_angle: (0.), direction: crate::model::RoadTurnDirection::CCW };

        let road_map = RoadMap::new(vec![
             RoadNode { road: road_1, /* 0 */ next: vec![1] },
             RoadNode { road: road_2, /* 1 */ next: vec![2] },
             RoadNode { road: road_3, /* 2 */ next: vec![3] },
             RoadNode { road: road_4, /* 3 */ next: vec![0] }
        ]).expect("Should have created RoadMap");

        let mut navigator = Navigator::new(Rc::new(road_map), 0).expect("Should have created the navigator");

        assert_eq!(navigator.current_id, 0);
        navigator.switch_to_next_road();
        assert_eq!(navigator.current_id, 1);
        navigator.switch_to_next_road();
        assert_eq!(navigator.current_id, 2);
        navigator.switch_to_next_road();
        assert_eq!(navigator.current_id, 3);
        navigator.switch_to_next_road();
        assert_eq!(navigator.current_id, 0);
    }

}

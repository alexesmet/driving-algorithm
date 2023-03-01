use std::rc::Rc;


mod map {
    use crate::model::Road;

    pub type RoadIndex = usize;

    pub struct RoadNode { pub road: Road, pub next: Vec<RoadIndex>}

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

pub use map::{RoadMap,RoadNode};
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
        if current_id > map.get_amount_of_roads() { return Err(NavigatorCreationError::CurrentIndexOutOfBounds) }
        return Ok(Self { map, current_id })
    }
    pub fn get_road(&self) -> &Road {
        self.map.get_road_by_id(self.current_id)
    }

    pub fn switch_to_next_road(&mut self) {
        self.current_id = self.map.get_next_roads(self.current_id)[0];
    }
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

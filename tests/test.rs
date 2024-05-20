#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use starbot::{Base, Cell, Map, MAP_WIDTH};

    #[test]
    fn test_to_char() {
        assert_eq!(Cell::Unknown.to_char(), '?');
        assert_eq!(Cell::Empty.to_char(), '.');
        assert_eq!(Cell::Base.to_char(), 'B');
        assert_eq!(Cell::Obstacle.to_char(), '#');
        assert_eq!(Cell::Mineral.to_char(), 'M');
        assert_eq!(Cell::Energy.to_char(), 'E');
        assert_eq!(Cell::SciencePOI.to_char(), 'S');
        assert_eq!(Cell::Robot(1).to_char(), '1');
        assert_eq!(Cell::Robot(9).to_char(), '9');
        assert_eq!(Cell::Robot(10).to_char(), 'X'); // Beyond digit range, should return 'X'
    }

    #[test]
    fn test_add_resource() {
        let map = Map::new();
        let base_map = Arc::new(Mutex::new(vec![
            vec![Cell::Unknown; MAP_WIDTH];
            starbot::MAP_HEIGHT
        ]));
        let base_position = map.find_base_position().unwrap();
        base_map.lock().unwrap()[base_position.1][base_position.0] = Cell::Base;
        let mut base = Base::new(base_position, base_map.clone());

        assert!(base.collected_resources.is_empty());

        base.add_resource(Cell::Mineral);
        assert_eq!(base.collected_resources.len(), 1);
        assert_eq!(base.collected_resources[0], Cell::Mineral);

        base.add_resource(Cell::Energy);
        assert_eq!(base.collected_resources.len(), 2);
        assert_eq!(base.collected_resources[1], Cell::Energy);
    }

    #[test]
    fn test_is_empty_or_walkable() {
        let map = Map::new();

        assert!(!map.is_empty_or_walkable((0, 0)));
    }
}

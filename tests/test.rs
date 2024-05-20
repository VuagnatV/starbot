

#[cfg(test)]
mod tests {

    #[test]
    fn cell_type_display() {
        use starbot::map::Cell;
        assert_eq!(format!("{}", Cell::Unknown), "?");
        assert_eq!(format!("{}", Cell::Robot(0)), "@");
        assert_eq!(format!("{}", Cell::Obstacle), "#");
        assert_eq!(format!("{}", Cell::Base), "B");
        assert_eq!(format!("{}", Cell::Energy), "E");
        assert_eq!(format!("{}", Cell::Mineral), "M");
    }
}
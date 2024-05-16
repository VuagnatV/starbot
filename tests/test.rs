

#[cfg(test)]
mod tests {

    #[test]
    fn cell_type_display() {
        use starbot::map::CellType;
        assert_eq!(format!("{}", CellType::Blank), " ");
        assert_eq!(format!("{}", CellType::Robot(0)), "@");
        assert_eq!(format!("{}", CellType::Obstacle), "X");
        assert_eq!(format!("{}", CellType::Base), "B");
        assert_eq!(format!("{}", CellType::Energie), "E");
        assert_eq!(format!("{}", CellType::Minerai), "M");
    }
}
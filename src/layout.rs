use yoga_sys::YGFlexDirection;

#[derive(Debug)]
pub enum FlexDirection {
    Column = 0,
    ColumnReverse = 1,
    Row = 2,
    RowReverse = 3,
}

impl FlexDirection {
    pub(crate) fn into_yoga(self) -> YGFlexDirection {
        use self::YGFlexDirection::*;

        match self {
            FlexDirection::Row => YGFlexDirectionRow,
            FlexDirection::RowReverse => YGFlexDirectionRowReverse,
            FlexDirection::Column => YGFlexDirectionColumn,
            FlexDirection::ColumnReverse => YGFlexDirectionColumnReverse,
        }
    }
}

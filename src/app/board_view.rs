use shared::{Position, Team};

/// Converts board coordinates without transforming upright artwork or game rules.
#[derive(Clone, Copy)]
pub(crate) struct BoardView {
    pub team: Team,
    pub width: i32,
    pub height: i32,
}

impl BoardView {
    pub fn point(self, (x, y): (f64, f64)) -> (f64, f64) {
        (
            x,
            if self.team == Team::Blue {
                self.height as f64 - 1.0 - y
            } else {
                y
            },
        )
    }

    pub fn tile(self, position: Position) -> Position {
        let (x, y) = self.point((position.0 as f64, position.1 as f64));
        Position(x as i8, y as i8)
    }

    pub fn direction(self, position: Position) -> Position {
        Position(
            position.0,
            if self.team == Team::Blue {
                -position.1
            } else {
                position.1
            },
        )
    }

    pub fn location(
        self,
        location: (i32, i32),
        offset: (i32, i32),
        scale: (i32, i32),
    ) -> Option<Position> {
        let x = location.0 as i64 - offset.0 as i64;
        let y = location.1 as i64 - offset.1 as i64;
        if scale.0 <= 0
            || scale.1 <= 0
            || x < 0
            || y < 0
            || x >= self.width as i64 * scale.0 as i64
            || y >= self.height as i64 * scale.1 as i64
        {
            return None;
        }
        Some(self.tile(Position(
            (x / scale.0 as i64) as i8,
            (y / scale.1 as i64) as i8,
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coordinates_and_boundaries() {
        for (width, height) in [(8, 8), (5, 7), (7, 5)] {
            for team in [Team::Red, Team::Blue] {
                let view = BoardView {
                    team,
                    width,
                    height,
                };
                for point in [(0.0, 0.0), (2.25, 3.75), (-0.5, height as f64)] {
                    assert_eq!(view.point(view.point(point)), point);
                    assert_eq!(view.point(point).0, point.0);
                }
                let expected_y = if team == Team::Blue { height - 1 } else { 0 };
                assert_eq!(
                    view.location((10, 20), (10, 20), (32, 24)),
                    Some(Position(0, expected_y as i8))
                );
                assert!(view.location((9, 20), (10, 20), (32, 24)).is_none());
                assert!(view.location((10, 19), (10, 20), (32, 24)).is_none());
                assert!(view
                    .location((10 + width * 32, 20), (10, 20), (32, 24))
                    .is_none());
                assert!(view
                    .location((10, 20 + height * 24), (10, 20), (32, 24))
                    .is_none());
                assert!(view
                    .location((i32::MAX, i32::MAX), (10, 20), (32, 24))
                    .is_none());
                assert_eq!(
                    view.location((9 + width * 32, 19 + height * 24), (10, 20), (32, 24)),
                    Some(view.tile(Position(width as i8 - 1, height as i8 - 1)))
                );
                assert_eq!(
                    view.direction(Position(1, 1)),
                    Position(1, if team == Team::Blue { -1 } else { 1 })
                );
            }
        }
    }
}

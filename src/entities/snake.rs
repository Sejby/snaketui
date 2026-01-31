#[derive(Clone, Copy, Debug)]
pub struct Segment {
    pub x: f64,
    pub y: f64,
}

impl Segment {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SnakeDirection {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
pub struct Snake {
    segments: Vec<Segment>,
    direction: SnakeDirection,
}

impl Snake {
    pub fn new(initial_segments: Vec<Segment>, direction: SnakeDirection) -> Self {
        Self {
            segments: initial_segments,
            direction,
        }
    }

    pub fn segments(&self) -> &[Segment] {
        &self.segments
    }

    pub fn head(&self) -> Segment {
        self.segments[0]
    }

    pub fn set_direction(&mut self, direction: SnakeDirection) {
        self.direction = direction;
    }

    pub fn len(&self) -> usize {
        self.segments.len()
    }

    pub fn calculate_next_head(&self, step_size: f64) -> Segment {
        let head = self.head();
        match self.direction {
            SnakeDirection::Left => Segment {
                x: head.x - step_size,
                y: head.y,
            },
            SnakeDirection::Right => Segment {
                x: head.x + step_size,
                y: head.y,
            },
            SnakeDirection::Up => Segment {
                x: head.x,
                y: head.y + step_size,
            },
            SnakeDirection::Down => Segment {
                x: head.x,
                y: head.y - step_size,
            },
        }
    }

    pub fn move_forward(&mut self, new_head: Segment, grow: bool) {
        self.segments.insert(0, new_head);
        if !grow {
            self.segments.pop();
        }
    }

    pub fn collides_with_self(&self, head: &Segment) -> bool {
        self.segments[1..]
            .iter()
            .any(|seg| (head.x - seg.x).abs() < 1e-6 && (head.y - seg.y).abs() < 1e-6)
    }

    pub fn overlaps(&self, x: f64, y: f64, box_size: f64) -> bool {
        self.segments
            .iter()
            .any(|s| (s.x - x).abs() < box_size * 0.8 && (s.y - y).abs() < box_size * 0.8)
    }
}

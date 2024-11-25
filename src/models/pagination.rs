use std::collections::VecDeque;

use crate::state::Track;

pub struct PaginatedQueue<'a> {
    queue: &'a VecDeque<Track>,
    page: usize,
    limit: usize,
    total: usize,
}

impl<'a> PaginatedQueue<'a> {
    pub fn new(queue: &'a VecDeque<Track>, total: usize, page: usize) -> Self {
        Self {
            queue,
            page,
            limit: 10,
            total,
        }
    }

    fn start_idx(&self) -> usize {
        (self.page - 1) * self.limit
    }

    fn end_idx(&self) -> usize {
        (self.start_idx() + self.limit).min(self.total)
    }

    pub fn total_pages(&self) -> usize {
        (self.total + self.limit - 1) / self.limit
    }

    pub fn get_fields(&self) -> impl IntoIterator<Item = (String, String, bool)> + '_ {
        let start = self.start_idx();
        let end = self.end_idx();
        self.queue
            .iter()
            .skip(start)
            .take(end - start)
            .enumerate()
            .map(move |(index, song)| {
                if index == 0 {
                    (
                        format!(
                            "{}. {} - {}[{}] ⬅️",
                            index + 1 + start,
                            song.artist,
                            song.name,
                            song.duration
                        ),
                        "".to_string(),
                        false,
                    )
                } else {
                    (
                        format!(
                            "{}. {} - {}[{}]",
                            index + 1 + start,
                            song.artist,
                            song.name,
                            song.duration
                        ),
                        "".to_string(),
                        false,
                    )
                }
            })
    }
}

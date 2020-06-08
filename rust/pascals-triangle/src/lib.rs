pub struct PascalsTriangle {
    row_count: u32
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count: row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rows: Vec<Vec<u32>> = Vec::new();
  
        for count in 0..self.row_count {
            let mut row: Vec<u32> = Vec::new();

            for index in 0..count+1 {
                if index == 0 || count == index {
                    row.push(1)
                } else if let Some(last_vec) = rows.last() {
                    let index = index as usize;
                    row.push(last_vec[index - 1] + last_vec[index]);
                }
            }
            rows.push(row);
        }
        rows
    }
}

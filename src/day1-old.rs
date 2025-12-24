use crate::reuse;

#[derive(Debug)]
struct Data {

    second_list: Vec<i32>,
    gaps: Option<Vec<i32>>,
    similarity_score: Option<Vec<i32>>,
}

impl Data {
    fn sort_lists(&mut self) {
        self.first_list.sort();
        self.second_list.sort();
    }

    fn get_gaps(&mut self) {
        let mut gaps: Vec<i32> = Vec::new();
        for (first, second) in self.first_list.iter().zip(self.second_list.iter()) {
            let gap = second - first;
            let absolute_gap: i32 = i32::abs(gap);
            gaps.push(absolute_gap)
        }
        self.gaps = Some(gaps);
    }

    fn sum_geps(&self) -> i32 {
        match &self.gaps {
            None => panic!("No gaps"),
            Some(v) => return v.iter().sum(),
        }
    }

    fn get_instances_in_second_list(&mut self) {
        let mut similarity_score: Vec<i32> = Vec::new();
        for i in self.first_list.iter() {
            let hits_in_second_list: Vec<i32> = self
                .second_list
                .iter()
                .copied()
                .filter(|x| x == i)
                .collect();
            dbg!(hits_in_second_list.len());
            let score = i * hits_in_second_list.len() as i32;
            similarity_score.push(score);
        }
        self.similarity_score = Some(similarity_score);
    }
}

pub fn main(input: &str) -> i32 {
    let input_list = reuse::split_input_by_line(input);
    let mut data: Data = loop_through_input_list(input_list);
    data.sort_lists();
    dbg!(&data);
    data.get_gaps();
    dbg!(&data.gaps);
    data.get_instances_in_second_list();
    match data.similarity_score{
        None => panic!("No similarity score"),
        Some(v) => return v.iter().sum()
    }
    
}

fn loop_through_input_list(input: Vec<String>) -> Data {
    let mut data: Data = Data {
        first_list: Vec::new(),
        second_list: Vec::new(),
        gaps: None,
        similarity_score: None,
    };
    for line in input {
        let strings = reuse::split_string_to_list(&line, ' ');
        dbg!(&strings);
        data.first_list.push(reuse::string_to_i32(strings[0]));
        data.second_list.push(reuse::string_to_i32(strings[3]));
    }
    return data;
}

use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let chars: Arc<Vec<_>> = Arc::new(input.iter().map(|c| c.to_string()).collect());
    
    let handles = (0..worker_count).map(|i| {
        let chars_ref = chars.clone();
        thread::spawn(move || {
            chars_ref
            .iter()
            .skip(i)
            .step_by(worker_count)
            .flat_map(|s| s.chars())
            .filter(|c| c.is_alphabetic())
            .map(|c| c.to_ascii_lowercase())
            .fold(HashMap::new(), |mut map, c| {
                *map.entry(c).or_insert(0) += 1;
                map
            })
        })
    });
    
    let mut map: HashMap<char, usize> = HashMap::new();
    for handle in handles {
        for (key, value) in handle.join().unwrap() {
            *map.entry(key).or_insert(0) += value;
        }
    }
    
    map
}

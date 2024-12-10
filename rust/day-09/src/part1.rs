use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
enum SpaceType {
    File(u64),
    Empty,
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let spaces_shorthand = input
        .chars()
        .filter_map(|char| match char.to_string().parse::<u64>() {
            Ok(num) => Some(num),
            Err(_) => None,
        })
        .collect_vec();

    let mut spaces = vec![];
    for (i, (file_size, empty_size)) in spaces_shorthand.iter().tuples().enumerate() {
        spaces.extend(vec![SpaceType::File(i as u64); *file_size as usize]);
        spaces.extend(vec![SpaceType::Empty; *empty_size as usize]);
    }

    if let Some(last_file_size) = spaces_shorthand.last() {
        spaces.extend(vec![
            SpaceType::File((spaces_shorthand.len() / 2) as u64);
            *last_file_size as usize
        ]);
    }

    let mut number_files = 0;
    let mut empty_indexes = vec![];
    for (i, space) in spaces.iter().rev().enumerate() {
        if space == &SpaceType::Empty {
            empty_indexes.push(spaces.len() - 1 - i);
        } else {
            number_files += 1;
        }
    }

    for space in spaces.clone().iter().rev() {
        if let &SpaceType::File(file_id) = space {
            let empty_index = match empty_indexes.pop() {
                Some(index) => index,
                None => break,
            };
            spaces[empty_index] = SpaceType::File(file_id);
        }
    }
    let files = spaces[0..number_files as usize].to_vec();

    let mut total = 0;
    for (i, file) in files.iter().enumerate() {
        let id = match file {
            SpaceType::File(id) => id,
            SpaceType::Empty => continue,
        };
        total += id * i as u64;
    }

    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "2333133121414131402";
        assert_eq!("1928", process(input));
    }
}

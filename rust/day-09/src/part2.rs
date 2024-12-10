use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
struct File {
    id: usize,
    size: usize,
    offset: usize,
}

#[derive(Debug, Clone, PartialEq)]
struct Space {
    size: usize,
    offset: usize,
}

#[derive(Debug, Clone, PartialEq)]
enum Block {
    File(File),
    Fpace(Space),
}

fn parse(input: &str) -> (Vec<File>, Vec<Space>) {
    let mut id = 0;
    let mut offset = 0;

    let mut chars = input.trim().chars();
    let mut files = Vec::new();
    let mut spaces = Vec::new();

    while let Some(file_char) = chars.next() {
        if let Some(size) = file_char.to_digit(10) {
            let size = size as usize;
            files.push(File { id, size, offset });
            id += 1;
            offset += size;
        }

        if let Some(empty_char) = chars.next() {
            if let Some(size) = empty_char.to_digit(10) {
                let size = size as usize;
                spaces.push(Space { size, offset });
                offset += size;
            }
        }
    }

    (files, spaces)
}

#[tracing::instrument]
pub fn process(input: &str) -> String {
    let (mut files, mut spaces) = parse(input);
    let mut result = 0;

    for file in files.iter_mut().rev() {
        for space in spaces.iter_mut() {
            if space.size >= file.size {
                file.offset = space.offset;
                space.size -= file.size;
                space.offset += file.size;

                if space.size == 0 {
                    spaces.retain(|s| s.size > 0);
                }
                break;
            }
        }
    }

    for file in files.iter().sorted_by_key(|f| f.offset) {
        for number in 0..file.size {
            result += file.id * (file.offset + number);
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "2333133121414131402";
        assert_eq!("2858 ", process(input));
    }
}

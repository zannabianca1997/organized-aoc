use std::iter;

use nonmax::NonMaxU16;

#[derive(Debug, Clone, Copy)]
struct Block(NonMaxU16);

fn parse1(input: &str) -> Box<[Option<Block>]> {
    input
        .trim()
        .bytes()
        .map(|ch| ch - b'0')
        .scan((true, 0), |(file, id), len| {
            let res = if *file {
                let fid = *id;
                *id += 1;
                Some(Block(NonMaxU16::new(fid).unwrap()))
            } else {
                None
            };
            *file = !*file;
            Some((len, res))
        })
        .flat_map(|(blocks, block)| iter::repeat_n(block, blocks as _))
        .collect()
}

pub fn part1(input: &str) -> usize {
    let mut input = parse1(input);

    let mut defrag_start = 0;
    let mut defrag_end = input.len() - 1;

    // Putting the start on a empty space and the end on a full one
    while input.get(defrag_start).copied().flatten().is_some() {
        defrag_start += 1
    }
    while defrag_start < defrag_end && input.get(defrag_end).unwrap().is_none() {
        defrag_end -= 1
    }

    while defrag_start < defrag_end {
        input[defrag_start] = input[defrag_end].take();
        defrag_start += 1;
        defrag_end -= 1;

        // Putting the start on a empty space and the end on a full one
        while defrag_start < defrag_end && input.get(defrag_start).copied().flatten().is_some() {
            defrag_start += 1
        }
        while defrag_start < defrag_end && input.get(defrag_end).unwrap().is_none() {
            defrag_end -= 1
        }
    }

    input
        .into_vec()
        .into_iter()
        .take_while(Option::is_some)
        .flatten()
        .enumerate()
        .map(|(pos, Block(id))| pos * id.get() as usize)
        .sum()
}

#[derive(Debug, Clone, Copy)]
struct File {
    len: u8,
    id: u16,
    pos: usize,
}
#[derive(Debug, Clone, Copy)]
struct Space {
    len: u8,
    pos: usize,
}

fn parse2(input: &str) -> (Vec<File>, Vec<Space>) {
    let mut files = vec![];
    let mut spaces = vec![];
    let mut pos = 0usize;
    for (block_num, len) in input.trim().bytes().map(|ch| ch - b'0').enumerate() {
        let res = if block_num % 2 == 0 {
            files.push(File {
                len,
                id: (block_num / 2) as _,
                pos,
            });
        } else {
            spaces.push(Space { len, pos })
        };
        pos += len as usize;
        res
    }
    (files, spaces)
}

pub fn part2(input: &str) -> usize {
    let (files, mut spaces) = parse2(input);
    let mut first_space = 0;
    let mut last_space = spaces.len() - 1;

    let files = files.into_iter().rev().map(|file| {
        while last_space >= first_space && spaces[first_space].len == 0 {
            first_space += 1;
        }
        while last_space >= first_space
            && (spaces[last_space].pos > file.pos || spaces[last_space].len == 0)
        {
            last_space -= 1;
        }
        if last_space < first_space {
            return file;
        }

        if let Some(space) = spaces[first_space..=last_space]
            .iter_mut()
            .find(|s| s.len >= file.len)
        {
            let mut moved_file = file;
            moved_file.pos = space.pos;

            space.len -= file.len;
            space.pos += file.len as usize;

            moved_file
        } else {
            file
        }
    });

    files
        .map(|f| {
            let mut file_total = 0;
            for pos in f.pos..(f.pos + f.len as usize) {
                file_total += pos
            }
            f.id as usize * file_total
        })
        .sum()
}

#[cfg(test)]
#[test]
fn example_part2() {
    assert_eq!(part2("2333133121414131402"), 2858)
}

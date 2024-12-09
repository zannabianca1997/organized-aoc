use std::iter;

use nonmax::NonMaxU16;

#[derive(Debug, Clone, Copy)]
struct Block(NonMaxU16);

fn parse1(input: &str) -> Box<[Option<Block>]> {
    input
        .trim()
        .chars()
        .map(|ch| ch as u8 - '0' as u8)
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
    input
        .trim()
        .chars()
        .map(|ch| ch as u8 - b'0')
        .enumerate()
        .for_each(|(block_num, len)| {
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
        });
    (files, spaces)
}

pub fn part2(input: &str) -> usize {
    let (files, mut spaces) = parse2(input);

    let files = files.into_iter().rev().map(|mut file| {
        if let Some(space) = spaces
            .iter_mut()
            .skip_while(|s| s.len < file.len)
            .take_while(|s| s.pos < file.pos)
            .next()
        {
            file.pos = space.pos;

            space.len -= file.len;
            space.pos += file.len as usize;
        }
        file
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
fn print_files<'a>(
    files: impl IntoIterator<Item = &'a File>,
    spaces: impl IntoIterator<Item = &'a Space>,
) {
    use core::str;

    let files: Vec<_> = files.into_iter().collect();
    let spaces: Vec<_> = spaces.into_iter().collect();

    let len = files
        .iter()
        .map(|f| f.pos + f.len as usize)
        .chain(spaces.iter().map(|s| s.pos + s.len as usize))
        .max()
        .unwrap();

    let mut printing = vec![b'x'; len].into_boxed_slice();

    for File { len, id, pos } in files {
        let id = *id as u8 + b'0';
        for c_pos in *pos..(*pos + *len as usize) {
            printing[c_pos] = id
        }
    }
    for Space { len, pos } in spaces {
        for c_pos in *pos..(*pos + *len as usize) {
            printing[c_pos] = b'.'
        }
    }

    println!("{}", str::from_utf8(&printing).unwrap())
}

#[cfg(test)]
#[test]
fn example_part2() {
    assert_eq!(part2("2333133121414131402"), 2858)
}

use std::{collections::HashMap, fs, iter::repeat};

#[derive(PartialEq, Clone, Debug)]
struct File {
    pos: usize,
    id: i32,
    size: usize,
}

fn get_files(disk: &Vec<Option<i32>>) -> (Vec<File>, Vec<File>) {
    let mut files: Vec<File> = Vec::new();
    let mut voids: Vec<File> = Vec::new();
    let mut file: Option<File> = None;
    let mut void: Option<File> = None;
    for (pos, id) in disk.iter().enumerate() {
        match id {
            Some(id) => {
                if let Some(v) = void {
                    voids.push(v.clone());
                    void = None;
                }
                match file {
                    None => {
                        file = Some(File {
                            pos: pos,
                            size: 1,
                            id: *id,
                        })
                    }
                    Some(ref mut f) => {
                        if f.id == *id {
                            f.size += 1;
                        } else {
                            files.push(f.clone());
                            file = Some(File {
                                pos: pos,
                                size: 1,
                                id: *id,
                            });
                        }
                    }
                }
            }
            None => {
                if let Some(f) = file {
                    files.push(f);
                    file = None;
                }
                match void {
                    None => {
                        void = Some(File {
                            pos: pos,
                            id: -1,
                            size: 1,
                        })
                    }
                    Some(ref mut v) => v.size += 1,
                }
            }
        }
    }
    if let Some(f) = file {
        files.push(f)
    }
    if let Some(v) = void {
        voids.push(v)
    }
    return (files, voids);
}

fn defragment(disk: &Vec<Option<i32>>) -> (Vec<File>, Vec<File>) {
    let (mut files, mut voids) = get_files(disk);
    for i in (0..files.len()).rev() {
        let file = files.get_mut(i).unwrap();
        let mut void_idx: usize = 0;
        while void_idx < voids.len() {
            if file.pos < voids[void_idx].pos {
                break;
            }
            if file.size > voids[void_idx].size {
                void_idx += 1;
                continue;
            }

            // found void big enough to fit file
            voids.push(File {
                pos: file.pos,
                id: -1,
                size: file.size,
            });
            file.pos = voids[void_idx].pos;
            voids[void_idx].pos += file.size;
            voids[void_idx].size -= file.size;
            break;
        }
    }
    return (files, voids);
}

fn compact(disk: &mut Vec<Option<i32>>) {
    let mut next_free: usize = 0;
    'next_block: for i in (0..disk.len()).rev() {
        if disk[i] == None {
            continue;
        }
        if next_free >= i {
            return;
        }
        for j in next_free..disk.len() {
            if disk[j] == None {
                disk.swap(i, j);
                next_free = j + 1;
                continue 'next_block;
            }
        }
    }
}

fn checksum(disk: &Vec<Option<i32>>) -> i64 {
    disk.iter().enumerate().fold(0i64, |acc, (idx, x)| match x {
        None => acc,
        Some(id) => acc + *id as i64 * idx as i64,
    })
}
fn file_checksum(files: &Vec<File>) -> i64 {
    files.iter().fold(0i64, |acc, x| {
        acc + x.id as i64 * (x.pos..x.pos + x.size).fold(0i64, |acc, x| acc + x as i64)
    })
}

fn load_input(file: &str) -> Vec<Option<i32>> {
    let input = fs::read_to_string(file).unwrap().replace("\n", "");
    let mut disk: Vec<Option<i32>> = Vec::new();
    let mut id: i32 = 0;
    let mut chars = input.chars();
    loop {
        if let (Some(sz), fr) = (chars.next(), chars.next()) {
            let size = sz as usize - '0' as usize;
            disk.append(&mut repeat(Some(id)).take(size).collect());
            if let Some(fr) = fr {
                let free = fr as usize - '0' as usize;
                disk.append(&mut repeat(None).take(free).collect());
            }
        } else {
            break;
        }
        id += 1;
    }
    return disk;
}

fn main() {
    let disk = load_input("src/day9/input.txt");
    let mut disk1 = disk.clone();
    compact(&mut disk1);
    println!("part1: {}", checksum(&disk1));
    let (files, _) = defragment(&disk);
    println!("part2: {}", file_checksum(&files));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dump(disk: &Vec<Option<i32>>) {
        for d in disk {
            match d {
                None => print!("."),
                Some(id) => print!("{id}"),
            }
        }
        println!("");
    }
    fn dump_map(files: &Vec<File>, voids: &Vec<File>) {
        let mut disk: Vec<Option<i32>> = vec![
            None;
            files.iter().fold(0, |acc, x| acc + x.size)
                + voids.iter().fold(0, |acc, x| acc + x.size)
        ];
        for f in files {
            for i in 0..f.size {
                disk[f.pos + i] = Some(f.id)
            }
        }
        for v in voids {
            if v.size == 0 {
                continue;
            }
            for i in 0..v.size {
                disk[v.pos + i] = None
            }
        }
        dump(&disk);
    }
    #[test]
    fn check1() {
        let mut disk = load_input("src/day9/test_input.txt");
        dump(&disk);
        let (files, voids) = get_files(&disk);
        println!("Before compaction:\nfile: {:?}\nvoids: {:?}", files, voids);
        compact(&mut disk);
        let (files, voids) = get_files(&disk);
        println!("After compaction:\nfile: {:?}\nvoids: {:?}", files, voids);
        dump(&disk);
        dump_map(&files, &voids);
        assert_eq!(checksum(&disk), 1928);
        assert_eq!(file_checksum(&files), 1928);
    }

    #[test]
    fn check2() {
        let mut disk = load_input("src/day9/test_input.txt");
        let (files, voids) = get_files(&disk);
        dump_map(&files, &voids);
        println!("files: {:?}\nvoids: {:?}", files, voids);
        let (files, voids) = defragment(&mut disk);
        // println!(
        //     "==== after defrag ====\nfiles: {:?}\nvoids: {:?}",
        //     files, voids
        // );
        // dump_map(&files, &voids);
        assert_eq!(file_checksum(&files), 2858);
    }
}

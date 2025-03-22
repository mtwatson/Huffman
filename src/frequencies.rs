use rayon::prelude::*;
use std::collections::HashMap;

pub fn word_frequencies(lines: &Vec<String>) -> HashMap<String, u64> {
    lines
        .par_iter()
        .fold(
            || HashMap::new(),
            |mut freqs: HashMap<_, _>, line: &String| {
                for word in line.split_ascii_whitespace() {
                    *freqs.entry(word.to_string()).or_insert(0) += 1;
                }
                freqs
            },
        )
        .reduce(
            || HashMap::new(),
            |mut freqs1, freqs2| {
                freqs2
                    .into_iter()
                    .for_each(|(word, n)| *freqs1.entry(word).or_insert(0) += n);
                freqs1
            },
        )
}

pub fn char_frequencies(lines: &Vec<String>) -> HashMap<char, u64> {
    lines
        .par_iter()
        .fold(
            || HashMap::new(),
            |mut freqs: HashMap<_, _>, line: &String| {
                for ch in line.chars() {
                    *freqs.entry(ch).or_insert(0) += 1;
                }
                freqs
            },
        )
        .reduce(
            || HashMap::new(),
            |mut freqs1, freqs2| {
                freqs2
                    .into_iter()
                    .for_each(|(ch, n)| *freqs1.entry(ch).or_insert(0) += n);
                freqs1
            },
        )
}
// only for testing provides a better interface for testing
#[cfg(test)]
pub fn get_char_frequencies(text: &str) -> HashMap<char, u64>
{
    // copied the important bits from compress.rs
    let lines: Vec<_> = text.chars().map(|x| x.to_string()).collect();
    char_frequencies(&lines)
}

#[cfg(test)]
mod test
{
    use std::collections::HashMap;
    use std::fs;

    use super::get_char_frequencies;

    // ascii chart:
    // https://cdn.shopify.com/s/files/1/1014/5789/files/Standard-ASCII-Table_large.jpg
    #[test]
    fn test_frequencies_hello_world()
    {
        let text = String::from("Hello World!");
        let counts = get_char_frequencies(&text);
        let expected = HashMap::from([('H', 1),
                                      ('e', 1),
                                      ('l', 3),
                                      ('o', 2),
                                      (' ', 1),
                                      ('W', 1),
                                      ('r', 1),
                                      ('d', 1),
                                      ('!', 1)]);

        for c in 0..256u32
        {
            let c = char::from_u32(c).unwrap();
            assert!(expected.get(&c) == counts.get(&c),
                    "for character {:?}, expected count: {:?}, found count: {:?}",
                    c,
                    expected.get(&c),
                    counts.get(&c));
        }
    }

    #[test]
    fn test_frequencies_random()
    {
        use rand::prelude::*;

        // do 100 iterations
        for _ in 0..100
        {
            // create a string with space to hold 39k characters
            let mut text = String::with_capacity(39000);

            // initialize rng
            let mut rng = rand::rng();
            let mut expected = HashMap::new();

            // for each possible character
            //   1) generate a random number between 50-150(inclusive)
            //   2) add that many copies to the text
            //   3) set that as the expected count for that character
            for c in 0..256u32
            {
                let c = char::from_u32(c).unwrap();
                let expected_count = rng.random_range(50..=150) as u64;
                expected.insert(c, expected_count);
                text.push_str(&std::iter::repeat(c).take(expected_count as usize)
                                                   .collect::<String>());
            }

            // run the counting function
            let counts = get_char_frequencies(&text);

            // make sure results match
            for c in 0..256u32
            {
                let c = char::from_u32(c).unwrap();
                assert!(expected.get(&c) == counts.get(&c),
                        "for character {:?}, expected count: {:?}, found count: {:?}",
                        c,
                        expected.get(&c),
                        counts.get(&c));
            }
        }
    }

    #[test]
    fn test_frequencies_asyoulik_txt()
    {
        let text = fs::read_to_string("asyoulik.txt").unwrap();
        let counts = get_char_frequencies(&text);
        let expected = HashMap::from([('H', 1),
                                      ('\t', 2895),
                                      ('\n', 4122),
                                      ('\r', 4122),
                                      (' ', 19359),
                                      ('!', 130),
                                      ('&', 5),
                                      ('\'', 496),
                                      ('(', 8),
                                      (')', 8),
                                      (',', 1948),
                                      ('-', 159),
                                      ('.', 958),
                                      (':', 382),
                                      (';', 369),
                                      ('?', 292),
                                      ('A', 1055),
                                      ('B', 169),
                                      ('C', 407),
                                      ('D', 535),
                                      ('E', 831),
                                      ('F', 120),
                                      ('G', 83),
                                      ('H', 250),
                                      ('I', 1563),
                                      ('J', 93),
                                      ('K', 124),
                                      ('L', 730),
                                      ('M', 136),
                                      ('N', 660),
                                      ('O', 953),
                                      ('P', 84),
                                      ('Q', 76),
                                      ('R', 660),
                                      ('S', 717),
                                      ('T', 593),
                                      ('U', 332),
                                      ('V', 105),
                                      ('W', 260),
                                      ('X', 5),
                                      ('Y', 139),
                                      ('[', 127),
                                      (']', 127),
                                      ('a', 6032),
                                      ('b', 1182),
                                      ('c', 1509),
                                      ('d', 3284),
                                      ('e', 10380),
                                      ('f', 1744),
                                      ('g', 1352),
                                      ('h', 5653),
                                      ('i', 4943),
                                      ('j', 43),
                                      ('k', 720),
                                      ('l', 3715),
                                      ('m', 2392),
                                      ('n', 5363),
                                      ('o', 7375),
                                      ('p', 1066),
                                      ('q', 72),
                                      ('r', 5155),
                                      ('s', 5157),
                                      ('t', 7509),
                                      ('u', 3036),
                                      ('v', 937),
                                      ('w', 1992),
                                      ('x', 107),
                                      ('y', 2352),
                                      ('z', 30),
                                      ('|', 14)]);

        for c in 0..256u32
        {
            let c = char::from_u32(c).unwrap();
            assert!(expected.get(&c) == counts.get(&c),
                    "for character {:?}, expected count: {:?}, found count: {:?}",
                    c,
                    expected.get(&c),
                    counts.get(&c));
        }
    }
}

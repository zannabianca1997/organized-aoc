pub fn part<const WINDOW: usize>(input: &str) -> usize {
    let input: Vec<_> = input.chars().collect();
    'outer: for (pos, window) in input.windows(WINDOW).enumerate() {
        for i in 0..window.len() {
            for j in 0..i {
                if window[i] == window[j] {
                    continue 'outer;
                }
            }
        }
        return pos + WINDOW;
    }
    panic!("Marker not found")
}

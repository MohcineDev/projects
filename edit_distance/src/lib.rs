pub fn edit_distance(source: &str, target: &str) -> usize {
    return distance(source, target, source.chars().count(), target.chars().count());
}

pub fn distance(s: &str, t: &str, s_len: usize, t_len: usize) -> usize {
    if s_len == 0 {
        return t_len;
    }
    if t_len == 0 {
        return s_len;
    }

    if s.chars().nth(s_len - 1) ==t.chars().nth(t_len - 1) {
        return distance(s, t, s_len - 1, t_len - 1);
    }
    let min;
    let a = distance(s, t, t_len, s_len - 1);
    let b = distance(s, t, t_len - 1, s_len);
    let c = distance(s, t, t_len - 1, s_len - 1);

    if a <= b && a <= c {
        min = a
    } else if b <= a && b <= c {
        min = b
    } else {
        min = c
    }

    return 1 + min;
}

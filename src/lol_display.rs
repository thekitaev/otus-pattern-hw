use colored::*;
use std::fmt::Display;
use std::fmt::Write as FmtWrite;

const START_COLOR: Color = Color {
    r: 100,
    g: 50,
    b: 5,
};

struct LolDisplay<T: Display> {
    inner: T,
}

#[derive(Clone, Copy, Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    #[allow(dead_code)]
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    fn next(&self) -> Self {
        let (r, g, b) = (
            self.r.overflowing_add(20).0,
            self.g.overflowing_add(10).0,
            self.b.overflowing_add(5).0,
        );
        Self { r, g, b }
    }
}

impl<T: Display> LolDisplay<T> {
    #[allow(dead_code)]
    fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: Display> Display for LolDisplay<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::new();
        write!(&mut buf, "{}", self.inner)?;

        let mut color = START_COLOR;

        for c in buf.chars() {
            if c == '\n' {
                write!(f, "{c}")?;
            } else {
                write!(
                    f,
                    "{}",
                    format!("{}", c).truecolor(color.r, color.g, color.b)
                )?;
                color = color.next();
            }
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod test {
    use colored::*;

    use super::{LolDisplay, START_COLOR};
    use std::fmt::Write as FmtWrite;

    #[test]
    fn test_rainbow() {
        let rainbow = LolDisplay::new("st".to_string());

        let mut have_buf = String::new();
        write!(&mut have_buf, "{}", rainbow).unwrap();

        let s_color = START_COLOR;
        let t_color = s_color.next();

        let want = format!(
            "{}{}",
            "s".truecolor(s_color.r, s_color.g, s_color.b),
            "t".truecolor(t_color.r, t_color.g, t_color.b)
        );
        assert_eq!(have_buf, want);
    }
}

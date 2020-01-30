use crate::{Error, Serde, Unit};

impl Serde<Unit, String> for Unit {
    fn de(s: String) -> Result<Unit, Error> {
        let t = s.trim();
        let st: Vec<&str> = t.split(|p: char| p.is_ascii_alphabetic()).collect();
        if st.len() != 2 {
            return Err(Error::DeserializeHtmlError(format!(
                "prase unit failed: {:#?}",
                st.join(""),
            )));
        }

        Ok(match st[1].trim().to_ascii_lowercase().as_str() {
            "ch" => Unit::Ch(st[0].parse().unwrap()),
            "cm" => Unit::Cm(st[0].parse().unwrap()),
            "dpcm" => Unit::Dpcm(st[0].parse().unwrap()),
            "dpi" => Unit::Dpi(st[0].parse().unwrap()),
            "dppx" => Unit::Dppx(st[0].parse().unwrap()),
            "em" => Unit::Em(st[0].parse().unwrap()),
            "fr" => Unit::Fr(st[0].parse().unwrap()),
            "in" => Unit::In(st[0].parse().unwrap()),
            "mm" => Unit::Mm(st[0].parse().unwrap()),
            "pc" => Unit::Pc(st[0].parse().unwrap()),
            "pt" => Unit::Pt(st[0].parse().unwrap()),
            "px" => Unit::Px(st[0].parse().unwrap()),
            "q" => Unit::Q(st[0].parse().unwrap()),
            "rem" => Unit::Rem(st[0].parse().unwrap()),
            "vh" => Unit::Vh(st[0].parse().unwrap()),
            "vmax" => Unit::Vmax(st[0].parse().unwrap()),
            "vmin" => Unit::Vmin(st[0].parse().unwrap()),
            "vw" => Unit::Vw(st[0].parse().unwrap()),
            _ => Unit::Em(1.0),
        })
    }

    fn ser(self) -> String {
        match self {
            Unit::Ch(n) => format!("{}ch", n),
            Unit::Cm(n) => format!("{}cm", n),
            Unit::Dpcm(n) => format!("{}dpcm", n),
            Unit::Dpi(n) => format!("{}dpi", n),
            Unit::Dppx(n) => format!("{}dppx", n),
            Unit::Em(n) => format!("{}em", n),
            Unit::Fr(n) => format!("{}fr", n),
            Unit::In(n) => format!("{}in", n),
            Unit::Mm(n) => format!("{}mm", n),
            Unit::Pc(n) => format!("{}pc", n),
            Unit::Pt(n) => format!("{}pt", n),
            Unit::Px(n) => format!("{}px", n),
            Unit::Q(n) => format!("{}Q", n),
            Unit::Rem(n) => format!("{}rem", n),
            Unit::Vh(n) => format!("{}vh", n),
            Unit::Vmax(n) => format!("{}vmax", n),
            Unit::Vmin(n) => format!("{}vmin", n),
            Unit::Vw(n) => format!("{}vw", n),
        }
    }
}

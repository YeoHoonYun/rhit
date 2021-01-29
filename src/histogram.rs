use {
    crate::*,
    chrono::{self, Date, FixedOffset},
    minimad::OwningTemplateExpander,
    termimad::*,
};

static MD: &str = r#"
|:-|-:|:-
|**date**|**hits**|**${scale}**
|:-|-:|:-
${bars
|${date}|${hits}|*${bar}*
}
|-:
"#;

pub struct Bar {
    date: Date<FixedOffset>,
    count: usize,
}

pub struct Histogram {
    bars: Vec<Bar>,
}
impl Histogram {
    pub fn of_days(log_base: &LogBase) -> Self {
        let mut bars = Vec::new();
        let mut cur_bar: Option<Bar> = None;
        for line in &log_base.lines {
            let date = line.time_local.date();
            if let Some(bar) = &mut cur_bar {
                if bar.date == date {
                    bar.count += 1;
                    continue;
                } else {
                    bars.push(cur_bar.take().unwrap());
                }
            }
            cur_bar = Some(Bar { date, count: 1 });
        }
        Self { bars }
    }
    pub fn print(&self, skin: &MadSkin) {
        let mut expander = OwningTemplateExpander::new();
        let max_hits = self.bars.iter().map(|b| b.count).max().unwrap();
        expander.set(
            "scale",
            format!("0               {:>4}", file_size::fit_4(max_hits as u64)),
        );
        let max_hits = max_hits as f32;
        for bar in &self.bars {
            let part = (bar.count as f32) / max_hits;
            expander.sub("bars")
                .set("date", bar.date.format("%a %Y/%m/%d"))
                .set("hits", bar.count)
                .set("bar", ProgressBar::new(part, 20));
        }
        md::print(expander, MD, skin);
    }
}
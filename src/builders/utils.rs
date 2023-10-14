use crate::helpers::capitalize;
use genpdf::{elements::*, render, style, Alignment, Element, Margins, Position};
use serde::{Deserialize, Serialize};

pub fn section_title(section: &String) -> PaddedElement<LinearLayout> {
    let mut layout = LinearLayout::vertical();

    let mut para = Paragraph::default();
    para.set_alignment(Alignment::Left);
    let cap = capitalize(section);
    para.push(cap);
    layout.push(
        para.styled(
            style::Style::new()
                .with_color(style::Color::Rgb(54, 125, 162))
                .with_font_size(12)
                .bold()
                .with_line_spacing(1.0),
        ),
    );
    let margins = Margins::trbl(0, 12, 0, 32);
    PaddedElement::new(layout, margins)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Skill {
    pub section: String,
    pub skills: Vec<String>,
}

pub fn subskill(s: Skill, mut layout: LinearLayout) -> LinearLayout {
    let mut para = Paragraph::default();
    para.set_alignment(Alignment::Left);
    para.push_styled(s.section, style::Style::new().bold());
    let list = s.skills.join(", ");
    para.push(list);
    layout.push(
        para.styled(
            style::Style::new()
                .with_color(style::Color::Rgb(50, 50, 50))
                .with_font_size(9)
                .with_line_spacing(1.0),
        ),
    );
    layout
}

pub fn skills(skills: Vec<Skill>) -> PaddedElement<LinearLayout> {
    let mut layout = LinearLayout::vertical();
    for skill in skills {
        layout = subskill(skill, layout);
    }
    let margins = Margins::trbl(0, 12, 0, 32);
    let ele = PaddedElement::new(layout, margins);
    ele
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct WorkExperience {
    pub position: String,
    pub company: String,
    pub location: String,
    pub start_date: String,
    pub end_date: String,
    pub highlights: Vec<String>,
}

pub fn work_experience(expr: WorkExperience, mut layout: LinearLayout) -> LinearLayout {
    let mut para = Paragraph::default();
    para.set_alignment(Alignment::Left);
    para.push_styled(
        format!("{}, {}", expr.position, expr.company),
        style::Style::new().bold(),
    );
    para.push(format!(
        "; {}  —  {} - {}",
        expr.location, expr.start_date, expr.end_date
    ));
    layout.push(
        para.styled(
            style::Style::new()
                .with_color(style::Color::Rgb(54, 125, 162))
                .with_font_size(10)
                .with_line_spacing(1.0),
        ),
    );

    let mut list = UnorderedList::with_bullet("•");

    for item in expr.highlights {
        list.push(Paragraph::new(item));
    }

    layout.push(
        list.styled(
            style::Style::new()
                .with_color(style::Color::Rgb(50, 50, 50))
                .with_font_size(9)
                .with_line_spacing(1.0),
        ),
    );
    layout
}

pub fn employment(jobs: Vec<WorkExperience>) -> PaddedElement<LinearLayout> {
    let mut layout = LinearLayout::vertical();

    for job in jobs {
        layout = work_experience(job, layout);
    }

    let margins = Margins::trbl(0, 12, 0, 32);
    let ele = PaddedElement::new(layout, margins);
    ele
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Degree {
    pub university: String,
    pub location: String,
    pub degree: String,
    pub year: String,
    pub description: String,
}

pub fn degree(deg: Degree, mut layout: LinearLayout) -> LinearLayout {
    let mut para = Paragraph::default();
    para.set_alignment(Alignment::Left);
    para.push_styled(
        format!("{}, {}", deg.university, deg.degree,),
        style::Style::new().bold(),
    );
    para.push(format!("  —  {}, {}", deg.location, deg.year));
    layout.push(
        para.styled(
            style::Style::new()
                .with_color(style::Color::Rgb(54, 125, 162))
                .with_font_size(10)
                .with_line_spacing(1.0),
        ),
    );

    let mut para = Paragraph::default();
    para.set_alignment(Alignment::Left);
    para.push(deg.description);
    layout.push(
        para.styled(
            style::Style::new()
                .with_color(style::Color::Rgb(50, 50, 50))
                .with_font_size(9)
                .with_line_spacing(1.0),
        ),
    );
    layout
}

pub fn education(degrees: Vec<Degree>) -> PaddedElement<LinearLayout> {
    let mut layout = LinearLayout::vertical();

    for deg in degrees {
        layout = degree(deg, layout);
    }

    let margins = Margins::trbl(0, 12, 0, 32);
    let ele = PaddedElement::new(layout, margins);
    ele
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Project {
    pub nickname: Option<String>,
    pub title: String,
    pub organization: String,
    pub year: String,
}

pub fn project_cell(proj: &Project, mut row: Vec<Box<dyn Element>>) -> Vec<Box<dyn Element>> {
    let mut layout = LinearLayout::vertical();
    let mut para = Paragraph::default();
    if proj.nickname.is_some() {
        para.push_styled(
            format!("{} - ", proj.nickname.clone().unwrap()),
            style::Style::new()
                .italic()
                .with_font_size(10)
                .with_line_spacing(1.0)
                .with_color(style::Color::Rgb(54, 125, 162)),
        )
    }
    para.push_styled(
        proj.title.clone(),
        style::Style::new()
            .with_font_size(10)
            .with_line_spacing(1.0)
            .with_color(style::Color::Rgb(54, 125, 162)),
    );
    layout.push(para);
    let mut para = Paragraph::default();
    para.push_styled(
        format!("{} - {}", proj.organization, proj.year),
        style::Style::new()
            .with_color(style::Color::Rgb(50, 50, 50))
            .with_font_size(9)
            .with_line_spacing(1.0),
    );
    layout.push(para);
    let margins = Margins::trbl(0, 0, 4, 0);
    let ele = PaddedElement::new(layout, margins);
    row.push(Box::new(ele));
    row
}

pub fn projects(projs: Vec<Project>) -> PaddedElement<TableLayout> {
    let mut table = TableLayout::new(vec![1, 1, 1]);
    for i in 0..projs.len() % 3 {
        let mut row: Vec<Box<dyn Element>> = Vec::new();
        if i + 3 < projs.len() {
            for proj in &projs[i..i + 3] {
                row = project_cell(proj, row);
            }
        } else {
            for proj in &projs[i..] {
                row = project_cell(proj, row);
            }
        }
        table.push_row(row).ok();
    }
    let margins = Margins::trbl(0, 12, 0, 32);
    let ele = PaddedElement::new(table, margins);
    ele
}

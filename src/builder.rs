use chrono::Local;
use genpdf::{
    elements::*, fonts, style, Alignment, Document, Element as _, Margins, PaperSize, Scale,
    SimplePageDecorator,
};
use image::io::Reader;
use include_dir::{include_dir, Dir};
use pdfshrink::gs_command;
use std::env;
use std::io::{self, Cursor, Write};
use std::path::PathBuf;
use tempfile;

use crate::helpers;

static FONTS_DIR: Dir<'_> = include_dir!("./fonts");
static FILES_DIR: Dir<'_> = include_dir!("./static");

pub fn build(name: String, company: String, location: String, position: String, out: PathBuf) {
    let output: PathBuf;

    if !out.ends_with(".pdf") {
        output = PathBuf::from(format!("{}.pdf", out.to_str().unwrap()).to_owned());
    } else {
        output = out.to_owned();
    }

    // Load a font from the file system
    let regular = fonts::FontData::new(
        FONTS_DIR
            .get_file("Rubik/Rubik-Regular.ttf")
            .unwrap()
            .contents()
            .to_vec(),
        None,
    )
    .unwrap();
    let bold = fonts::FontData::new(
        FONTS_DIR
            .get_file("Rubik/Rubik-Bold.ttf")
            .unwrap()
            .contents()
            .to_vec(),
        None,
    )
    .unwrap();
    let italic = fonts::FontData::new(
        FONTS_DIR
            .get_file("Rubik/Rubik-Italic.ttf")
            .unwrap()
            .contents()
            .to_vec(),
        None,
    )
    .unwrap();
    let bold_italic = fonts::FontData::new(
        FONTS_DIR
            .get_file("Rubik/Rubik-BoldItalic.ttf")
            .unwrap()
            .contents()
            .to_vec(),
        None,
    )
    .unwrap();
    let font_family = fonts::FontFamily {
        regular,
        bold,
        italic,
        bold_italic,
    };
    // Create a document and set the default font family
    let mut doc = Document::new(font_family);
    doc.set_font_size(10);
    doc.set_line_spacing(1.3);
    doc.set_paper_size(PaperSize::Letter);
    // Change the default settings
    doc.set_title("Cover Letter");
    // Set margins

    // Customize the pages
    let decorator = SimplePageDecorator::new();
    doc.set_page_decorator(decorator);

    // Add one or more elements

    let mut layout = LinearLayout::vertical();

    let mut para = Paragraph::default();
    para.set_alignment(Alignment::Center);
    para.push("FROM THE DESK OF");
    layout.push(
        para.styled(
            style::Style::new()
                .bold()
                .with_color(style::Color::Rgb(129, 173, 187))
                .with_font_size(8)
                .with_line_spacing(1.5),
        ),
    );

    let cap: String = helpers::capitalize(&name);

    let mut para = Paragraph::default();
    para.set_alignment(Alignment::Center);
    para.push(style::StyledString::new(
        cap,
        style::Style::new().with_font_size(32),
    ));
    layout.push(
        para.styled(
            style::Style::new()
                .bold()
                .with_color(style::Color::Rgb(26, 92, 113))
                .with_font_size(26),
        ),
    );

    let margins = Margins::trbl(4, 44, 0, 44);
    let ele = PaddedElement::new(layout, margins);
    doc.push(ele);

    doc.push(Break::new(0.5));

    let image = Image::from_dynamic_image(
        Reader::new(Cursor::new(
            FILES_DIR.get_file("line.jpg").unwrap().contents(),
        ))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap(),
    )
    .expect("Failed to load test image")
    .with_alignment(genpdf::Alignment::Center)
    .with_scale(Scale { x: 1.0, y: 1.0 }); // Center the image on the page.
    doc.push(image);
    doc.push(Break::new(1));

    let mut layout = LinearLayout::vertical();

    let now = format!("{}", Local::now().format("%B %e, %Y"));
    let para = Paragraph::new(now);
    layout.push(para);

    layout.push(Break::new(1));

    let letter_head_to = format!("{}", company.clone());
    let para = Paragraph::new(letter_head_to);
    layout.push(para);

    let letter_head_at = format!("{}", location);
    let para = Paragraph::new(letter_head_at);
    layout.push(para);

    layout.push(Break::new(1));

    let greeting = format!("Dear {},", company.clone());
    let para = Paragraph::new(greeting);
    layout.push(para);

    layout.push(Break::new(1));

    let para1 = format!("I am writing to express my sincere interest in the {} opportunity to work at {}. My journey in the realm of machine learning and software engineering has ignited a deep passion for innovation and problem-solving. I am excited about the prospect of contributing to your dynamic team's achievements.", position, company.clone());
    let para = Paragraph::new(para1);
    layout.push(para);

    layout.push(Break::new(1));

    let para2 = "My background is firmly rooted in machine learning, where I have designed and developed a range of models, from object recognition to genetic algorithms. These experiences have honed my ability to tackle complex challenges with creative solutions. One notable achievement was at HHMI, where I led a project resulting in a remarkable 50x performance improvement in ETL processes. This optimization facilitated the integration of facial recognition outputs with neuron segmentation data, enabling groundbreaking joint analyses of behavioral and microscopy data.";
    let para = Paragraph::new(para2);
    layout.push(para);

    layout.push(Break::new(1));

    let para3 = format!("My journey through computational publications, bioinformatics education, and comprehensive software engineering training has equipped me with a unique skill set. I am impressed by {}’s reputation for innovation and its unwavering commitment to excellence. Your focus on experimentation and quality aligns seamlessly with my personal values and professional aspirations.", company);
    let para = Paragraph::new(para3);
    layout.push(para);

    layout.push(Break::new(1));

    let closing = "Enclosed, please find my resume, which provides a comprehensive overview of my qualifications and accomplishments. I am genuinely eager to explore how my background and experience can contribute to your organization's continued growth. I welcome the opportunity to discuss how we can work together to achieve your strategic goals.";
    let para = Paragraph::new(closing);
    layout.push(para);

    layout.push(Break::new(1));

    let para = Paragraph::new("Thank you for considering my application.");
    layout.push(para);

    layout.push(Break::new(1));

    let para = Paragraph::new("Warm regards,");
    layout.push(para);

    layout.push(Break::new(1));

    let para = Paragraph::new(name);
    layout.push(para);

    let margins = Margins::trbl(0, 34.5, 0, 34.5);
    let ele = PaddedElement::new(layout, margins);
    doc.push(ele);

    doc.push(Break::new(1));

    let mut para = Paragraph::default();
    para.set_alignment(Alignment::Center);
    para.push("CONTACT");
    doc.push(
        para.styled(
            style::Style::new()
                .bold()
                .with_color(style::Color::Rgb(129, 173, 187))
                .with_font_size(8)
                .with_line_spacing(1.5),
        ),
    );

    doc.push(Break::new(0.33));

    let image = Image::from_dynamic_image(
        Reader::new(Cursor::new(
            FILES_DIR.get_file("line.jpg").unwrap().contents(),
        ))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap(),
    )
    .expect("Failed to load test image")
    .with_alignment(genpdf::Alignment::Center)
    .with_scale(Scale { x: 2.0, y: 0.5 }); // Center the image on the page.

    doc.push(image);

    doc.push(Break::new(0.33));

    let mut para = Paragraph::default();
    para.set_alignment(Alignment::Center);
    para.push(r#"+1 (319) 310-2338 | annie.ehler.4@gmail.com | linkedin.com/in/annie444 | github.com/annie444 | annieehler.com"#);
    doc.push(
        para.styled(
            style::Style::new()
                .bold()
                .with_color(style::Color::Rgb(26, 92, 113))
                .with_font_size(8)
                .with_line_spacing(1.0),
        ),
    );

    let out = tempfile::Builder::new()
        .suffix(".pdf")
        .tempfile()
        .expect("Unable to reach tempfile");

    // Render the document and write it to a file
    doc.render_to_file(out.path())
        .expect("Failed to write PDF file");

    let cmd = gs_command(out.path(), output)
        .output()
        .expect("failed to execute process");

    io::stdout().write_all(&cmd.stdout).unwrap();
    io::stderr().write_all(&cmd.stderr).unwrap();

    out.close().expect("Unable to get rid of tempfile");
}

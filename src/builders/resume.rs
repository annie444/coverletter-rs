use crate::builders::{FILES_DIR, FONTS_DIR};
use genpdf::{
    elements::*, fonts, style, Alignment, Document, Element as _, Margins, PaperSize, Scale,
    SimplePageDecorator,
};
use image::{io::Reader, ImageFormat};
use pdfshrink::gs_command;
use std::io::{self, Cursor, Write};
use std::{ffi::OsStr, path::PathBuf};
use tempfile;

use crate::builders::utils::*;
use crate::helpers;

pub fn build(name: String, position: String, out: PathBuf) {
    let output: PathBuf;

    if !(out.extension().is_none() || out.extension() == Some(OsStr::new("pdf"))) {
        output = PathBuf::from(format!("{}.pdf", out.display()).to_owned());
    } else {
        output = PathBuf::from(out).to_owned();
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
    doc.set_font_size(9);
    doc.set_line_spacing(1.2);
    doc.set_paper_size(PaperSize::Letter);
    // Change the default settings
    doc.set_title("Resume");
    // Set margins

    // Customize the pages
    let decorator = SimplePageDecorator::new();
    doc.set_page_decorator(decorator);

    // Add one or more elements

    let mut layout = LinearLayout::vertical();

    let cap: String = helpers::capitalize(&name);

    let mut para = Paragraph::default();
    para.set_alignment(Alignment::Left);
    para.push(style::StyledString::new(
        cap,
        style::Style::new().with_font_size(27),
    ));
    layout.push(
        para.styled(
            style::Style::new()
                .bold()
                .with_color(style::Color::Rgb(26, 92, 113))
                .with_font_size(26),
        ),
    );

    let margins = Margins::trbl(0, 12, 0, 12);
    let ele = PaddedElement::new(layout, margins);
    doc.push(ele);

    let image = Image::from_dynamic_image(
        Reader::with_format(
            Cursor::new(FILES_DIR.get_file("line.jpg").unwrap().contents()),
            ImageFormat::Jpeg,
        )
        .decode()
        .expect("Failed to load test image"),
    )
    .expect("Failed to load test image")
    .with_alignment(genpdf::Alignment::Center)
    .with_scale(Scale { x: 4.0, y: 1.0 }); // Center the image on the page.
    doc.push(image);

    let mut layout = LinearLayout::vertical();
    let mut para = Paragraph::default();
    para.set_alignment(Alignment::Left);
    para.push(r#"+1 (319) 310-2338 | annie.ehler.4@gmail.com | linkedin.com/in/annie444 | github.com/annie444 | annieehler.com"#);
    layout.push(
        para.styled(
            style::Style::new()
                .with_color(style::Color::Rgb(96, 96, 96))
                .with_font_size(10)
                .with_line_spacing(1.0),
        ),
    );
    let margins = Margins::trbl(0, 12, 0, 12);
    let ele = PaddedElement::new(layout, margins);
    doc.push(ele);

    doc.push(Break::new(0.5));

    doc.push(section_title(&position));

    let mut layout = LinearLayout::vertical();

    let mut para = Paragraph::default();
    para.set_alignment(Alignment::Left);
    let intro = "Experienced ".to_string();
    para.push(intro);
    let intro = "machine learning and artificial intelligence engineer ".to_string();
    para.push_styled(intro, style::Style::new().bold());
    let intro = "with expertise in ".to_string();
    para.push(intro);
    let intro =
        "data processing, design, testing, optimization, and deployment of ML models. ".to_string();
    para.push_styled(intro, style::Style::new().bold());
    let intro = "Equipped with a diverse and promising skill-set with ".to_string();
    para.push(intro);
    let intro = "4+ years of measurable expertise. ".to_string();
    para.push_styled(intro, style::Style::new().bold());
    let intro = "Experienced with the latest cutting edge development algorithms, big data, supervised and unsupervised learning, classification, regression, discriminative modeling, and model performance analysis. Effective in self-managing independent projects, as well as collaborating as part of a productive team.".to_string();
    para.push(intro);

    layout.push(
        para.styled(
            style::Style::new()
                .with_color(style::Color::Rgb(50, 50, 50))
                .with_font_size(9)
                .with_line_spacing(1.0),
        ),
    );
    let margins = Margins::trbl(0, 12, 0, 32);
    let ele = PaddedElement::new(layout, margins);
    doc.push(ele);

    doc.push(Break::new(0.5));

    doc.push(section_title(&"SKILLS".to_string()));

    doc.push(skills(vec![
        Skill {
            section: "Languages: ".to_string(),
            skills: vec![
                "C/C++".to_string(),
                "Python".to_string(),
                "R".to_string(),
                "MATLAB".to_string(),
                "Shell Scripting".to_string(),
                "PHP".to_string(),
                "HTML".to_string(),
                "CSS".to_string(),
                "JavaScript".to_string(),
                "Java".to_string(),
                "Rust".to_string(),
                "SQL".to_string(),
            ],
        },
        Skill {
            section: "AI Concepts: ".to_string(),
            skills: vec![
                "Deep Learning".to_string(),
                "Neural Networks".to_string(),
                "Convolution".to_string(),
                "LSTMs".to_string(),
                "Regressive Algorithms".to_string(),
                "Unsupervised Learning".to_string(),
                "Genetic Programming".to_string(),
                "Advanced Image Processing".to_string(),
                "Facial Recognition".to_string(),
                "Reinforcement Learning".to_string(),
                "LLMs".to_string(),
                "Transformer models".to_string(),
                "ChatGPT/Bard".to_string(),
                "A/B testing and Multi-arm testing".to_string(),
                "and Distributed Production".to_string(),
            ],
        },
        Skill {
            section: "CS Concepts".to_string(),
            skills: vec![
                "Tensorflow".to_string(),
                "PyTorch".to_string(),
                "MLOps".to_string(),
                "Kubernetes".to_string(),
                "CUDA".to_string(),
                "Keras".to_string(),
                "memory management".to_string(),
                "NumPy".to_string(),
                "Pandas".to_string(),
            ],
        },
        Skill {
            section: "Leadership".to_string(),
            skills: vec![
                "Innovative".to_string(),
                "Collaborative".to_string(),
                "Creative".to_string(),
                "Flexible".to_string(),
                "Problem-solver".to_string(),
                "Attention to Detail".to_string(),
                "Efficient".to_string(),
                "Leader".to_string(),
            ],
        },
    ]));

    doc.push(Break::new(0.5));

    doc.push(section_title(&"EXPERIENCE".to_string()));

    doc.push(employment(vec![
        WorkExperience {
            position: "Senior Software Engineer".to_string(),
            company: "Wardrobe Depot".to_string(),
            location: "Los Angeles".to_string(),
            start_date: "Oct 2021".to_string(),
            end_date: "Present".to_string(),
            highlights: vec![
                "Moved stack from Shopify to a customized full-stack web application utilizing a micro-services architecture on Kubernetes that easily scaled their customer bandwidth by 500% with an overall decrease in operating costs.".to_string(),
                "Conducted cross cloud cost analysis on AWS EC2 and S3, GCP GKE, and Akamai Suite to identify the cheapest architectures for each average monthly user saving over $1,200 in cloud costs monthly.".to_string(),
                "Designed git-triggered CI/CD pipelines with automated docker container optimization decreasing the time to production for new features by 3 weeks.".to_string(),
            ]
        },
        WorkExperience {
            position: "Computational Analyst".to_string(),
            company: "HHMI".to_string(),
            location: "San Diego".to_string(),
            start_date: "June 2022".to_string(),
            end_date: "Feb 2023".to_string(),
            highlights: vec![
                "Led and managed multiple projects focused on the integration cutting-edge AI & ML technologies into their facial recognition and 2-photon neural microscopy analysis pipeline decreasing time to publication by 6 months.".to_string(),
                "Integrated neural and facial recognition pipelines creating an automated workflow increasing the number of experiments processed per day by over 200x.".to_string(),
                "My novel procedures are now in-review for publish in 6 high-impact academic journals including Nature, Cell, and Science Signaling.".to_string(),
            ]
        },
        WorkExperience {
            position: "Bioinformatics Engineer".to_string(),
            company: "Carver College of Medicine".to_string(),
            location: "Iowa City".to_string(),
            start_date: "May 2016".to_string(),
            end_date: "Aug 2018".to_string(),
            highlights: vec![
               "Designed multiple data analysis pipelines for histology analysis, confocal microscopy, CT/MRI data, and alignment & quantification for genomics, transcriptomics, & proteomics shaving an average of 1 year off the data analysis timeline.".to_string(),
               "Designed novel ML models for object recognition throughout microscopy and CT imaging with an accuracy of 95%, roughly twice as good as the previous approach.".to_string(),
               "Integrated all data processing with the university’s computational core, increasing data bandwidth by 500x.".to_string(),
            ]
        }
    ]));

    doc.push(Break::new(0.5));

    doc.push(section_title(&"EDUCATION".to_string()));

    doc.push(education(vec![
        Degree {
            university: "Hampshire College".to_string(),
            location: "Amherst, MA".to_string(),
            degree: "Bachelors of the Arts".to_string(),
            year: "2022".to_string(),
            description: "Thesis focused on computational neuroscience with relevant courses including Epigenetics, Machine Learning, Research in Artificial Intelligence, Bioinformatics & Computational Molecular Biology, and Engineering Computing".to_string(),
        }
    ]));

    doc.push(Break::new(0.5));

    doc.push(section_title(&"PROJECTS".to_string()));

    doc.push(projects(vec![
        Project {
            nickname: None,
            title: "Lexicase Selection of Deep Neural Network Weights and Biases".to_string(),
            organization: "Hampshire College".to_string(),
            year: "2019".to_string(),
        },
        Project {
            nickname: None,
            title: "Non-Canonical Alignment and Quantification Techniques".to_string(),
            organization: "Smith College".to_string(),
            year: "2020".to_string(),
        },
        Project {
            nickname: Some("Poo Wi-Fi".to_string()),
            title: "Trash based public wireless internet".to_string(),
            organization: "IowaBIG".to_string(),
            year: "2016".to_string(),
        },
        Project {
            nickname: Some("SLEAPyFaces".to_string()),
            title: "Combine neural and facial data from across experiments in one line of code"
                .to_string(),
            organization: "HHMI".to_string(),
            year: "2023".to_string(),
        },
        Project {
            nickname: Some("InstaCrawl".to_string()),
            title: "Instagram post scraper and algorithm performance analytics".to_string(),
            organization: "Personal".to_string(),
            year: "2023".to_string(),
        },
    ]));

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

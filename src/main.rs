use pdfium_render::prelude::*;
use std::{env, error::Error, process};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input-pdf> <output-pdf>", args[0]);
        process::exit(1);
    }

    let input_pdf = &args[1];
    let output_pdf = &args[2];

    let pdfium = Pdfium::default();
    let mut doc = pdfium.load_pdf_from_file(input_pdf, None)?;

    let mut rect: Option<PdfRect> = None;
    for page in doc.pages().iter() {
        let text = page.text()?;
        rect = text.segments().iter().map(|seg| seg.bounds()).fold(
            rect,
            |acc, rect| acc.map_or(Some(rect), |acc| Some(rect_union(acc, rect))),
        );
    }

    match rect {
        None => println!("No text found in the document"),
        Some(rect) => {
            println!("Bounding box: {:?}", rect);
            for mut page in doc.pages_mut().iter() {

                page.boundaries_mut().set_crop(rect.clone())?;
            }
            doc.save_to_file(output_pdf)?;
        }
    }
    Ok(())
}

fn rect_union(a: PdfRect, b: PdfRect) -> PdfRect {
    PdfRect::new(
        PdfPoints::new(a.bottom.value.min(b.bottom.value)),
        PdfPoints::new(a.left.value.min(b.left.value)),
        PdfPoints::new(a.top.value.max(b.top.value)),
        PdfPoints::new(a.right.value.max(b.right.value)),
    )
}
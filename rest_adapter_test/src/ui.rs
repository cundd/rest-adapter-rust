use rest_adapter::*;
use ansi_term::Colour;
use term;
use std::io::Write;
use crate::cli_error::CliError;
use std::fmt::Debug;

type Result<T> = ::std::result::Result<T, CliError>;

pub fn print_error(error: CliError) -> () {
    let mut term = get_output_writer().unwrap();

    writeln!(term, "{}", color(&format!("{}", error), Colour::Red)).unwrap();
}

pub fn print_contents(contents: Vec<Content>) -> Result<()> {
    for content in contents {
        print_content(&content)?
    }

    Ok(())
}

pub fn print_page_tree(page: &Page) -> Result<()> {
    print_page_internal(page, 0)
}

pub fn print_data<T: Debug>(data: &T) -> Result<()> {
    let mut term = get_output_writer()?;

    Ok(writeln!(term, "{:#?}", data)?)
}

pub fn print_content(content: &Content) -> Result<()> {
    let mut term = get_output_writer()?;
    writeln!(term, "Identifier .......... {}", color(&format!("#{}", content.id()), Colour::Green))?;
    writeln!(term, "PID ................. {}", color(&format!("#{}", content.pid()), Colour::Green))?;
    writeln!(term, "Created ............. {}", color(&format!("{}", content.creation_time()), Colour::Green))?;
    writeln!(term, "Modified ............ {}", color(&format!("{}", content.modification_time()), Colour::Green))?;
    writeln!(term, "Content:")?;

    let html_content = content.content().trim();
    let length = html_content.len().min(120);

    writeln!(term, "{}", color(&format!("{}", &html_content[0..length]), Colour::Green))?;
    writeln!(term, "")?;

    Ok(())
}

fn print_page_internal(page: &Page, level: usize) -> Result<()> {
    let mut term = get_output_writer()?;
    writeln!(term, "{}Identifier .......... {}", indent(level), color(&format!("#{}", page.id()), Colour::Green))?;
    writeln!(term, "{}Title ............... {}", indent(level), color(page.title(), Colour::Green))?;
    writeln!(term, "{}PID ................. {}", indent(level), color(&format!("#{}", page.pid()), Colour::Green))?;
    writeln!(term, "{}Created ............. {}", indent(level), color(&format!("{}", page.creation_time()), Colour::Green))?;
    writeln!(term, "{}Modified ............ {}", indent(level), color(&format!("{}", page.modification_time()), Colour::Green))?;
    writeln!(term, "{}Author .............. {}", indent(level), page.author())?;
    writeln!(term, "{}Author email ........ {}", indent(level), page.author_email())?;
    writeln!(term, "{}Navigation title .... {}", indent(level), page.navigation_title())?;
    writeln!(term, "{}SEO title ........... {}", indent(level), page.seo_title())?;
    writeln!(term, "{}no-index ............ {}", indent(level), page.no_index())?;
    writeln!(term, "{}no-follow ........... {}", indent(level), page.no_follow())?;
    writeln!(term, "{}og-title ............ {}", indent(level), page.og_title())?;
    writeln!(term, "{}og-desc ............. {}", indent(level), page.og_description().unwrap_or("None"))?;
    writeln!(term, "{}Twitter title ....... {}", indent(level), page.twitter_title())?;
    writeln!(term, "{}Twitter desc ........ {}", indent(level), page.twitter_description().unwrap_or("None"))?;
    // writeln!(term, "{}Twitter Image     {}", indent(level), page.twitter_image())?;
    writeln!(term, "{}Canonical URI ....... {}", indent(level), page.canonical_link())?;

    let number_of_children = page.children().len();
    if number_of_children > 0 {
        writeln!(term, "{}Children {}:", indent(level), number_of_children)?;
        for child in page.children() {
            print_page_internal(child, level + 1)?;
        }
    }
    writeln!(term, "")?;

    Ok(())
}

fn indent(level: usize) -> String {
    "   ".repeat(level)
}


fn get_output_writer() -> Result<impl std::io::Write> {
    match term::stdout() {
        Some(t) => Ok(t),
        None => Err(CliError::output_error("Could not open output terminal for writing"))
    }
}


fn color(message: &str, color: Colour) -> String {
    if output_supports_color() {
        color.paint(message).to_string()
    } else {
        message.to_string()
    }
}

fn output_supports_color() -> bool {
    match term::stdout() {
        Some(t) => t.supports_color(),
        None => false,
    }
}

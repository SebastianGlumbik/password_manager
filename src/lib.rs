mod database;
mod utils;

use crate::database::models::traits::Id;
use crate::database::*;
use crate::models::{basic, specific, Category, Content, Record};

//TODO Comments for whole project
//TODO csv export/import, cloud backup

pub async fn run() -> Result<(), &'static str> {
    let mut database = Database::open("password")?;

    let mut record1 = Record::new("Example".to_string(), Category::Login);
    database
        .save_record(&mut record1)
        .map_err(|_| "Failed to save record")?;

    let mut record2 = Record::new(
        "Testing".to_string(),
        Category::Custom("Testing".to_string()),
    );
    database
        .save_record(&mut record2)
        .map_err(|_| "Failed to save record")?;

    let mut record1_content1 = Content::Email(specific::Email::new(
        "email".to_string(),
        true,
        "example@email.com".to_string(),
    )?);
    database
        .save_content(record1.id(), &mut record1_content1)
        .map_err(|_| "Failed to save content")?;

    let mut record1_content2 = Content::Url(specific::Url::new(
        "website".to_string(),
        true,
        "www.example.com".to_string(),
    )?);
    database
        .save_content(record1.id(), &mut record1_content2)
        .map_err(|_| "Failed to save content")?;

    let pg = utils::password::PasswordGenerator {
        length: 8,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: false,
        spaces: false,
        exclude_similar_characters: false,
        strict: true,
    };
    let mut record1_content3 = Content::Password(specific::Password::new(
        "password".to_string(),
        true,
        pg.generate_one()?,
    ));
    database
        .save_content(record1.id(), &mut record1_content3)
        .map_err(|_| "Failed to save content")?;

    for record in database
        .get_all_records()
        .map_err(|_| "Failed to get records")?
    {
        println!("{:?}", record);
    }

    for content in database
        .get_all_content_for_record(record1.id())
        .map_err(|_| "Failed to get contents")?
    {
        println!("{:?}", content);
    }

    let mut record1_content4 = Content::Text(basic::Text::new(
        "note".to_string(),
        false,
        "This is a note".to_string(),
        basic::TextType::Normal,
    ));
    database
        .save_content(record1.id(), &mut record1_content4)
        .map_err(|_| "Failed to save content")?;

    for content in database
        .get_all_content_for_record(record1.id())
        .map_err(|_| "Failed to get contents")?
    {
        println!("{:?}", content);

        if let Content::Password(password) = &content {
            println!(
                "Exposed password: {}",
                utils::password::is_exposed(password.value()).await.unwrap()
            );
            println!(
                "Password score: {}",
                utils::password::score(&utils::password::analyze(password.value()))
            );
        }
    }
    Ok(())
}

// TODO some struct password manager

//TODO Tests
#[cfg(test)]
mod tests {
    use super::*;
}

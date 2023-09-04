
use base64::{Engine as _, engine::general_purpose};
use image::DynamicImage;

pub fn create_diagram(description: &str) -> Result<DynamicImage, &str> {

    let origin = description.as_bytes();
    let encoded: String = general_purpose::STANDARD_NO_PAD.encode(origin);
    let url = format!("https://mermaid.ink/img/{}", encoded);

    // Download image 
    let img_data = match reqwest::blocking::get(url){
        Ok(res) => res,
        Err(_) => return Err("Download image problem."),
    };
    
    // Image to bytes
    let img_bytes = match img_data.bytes(){
        Ok(res) => res,
        Err(_) => return Err("Encoding image problem."),
    }; 

    // Create image object
    match image::load_from_memory(&img_bytes){
        Ok(res) => return Ok(res),
        Err(_) => return Err("Image object creation error."),
    };

}

pub fn save_diagram<'a>(description: &'a str, path: &'a str) -> Result<(), &'a str> {

    // Retrieve image with create_diagram function
    let image = match create_diagram(description){
        Ok(res) => res,
        Err(err) => return Err(err),
    };

    // Save image to file
    match image.save(path){
        Ok(_) => return Ok(()),
        Err(_) => return Err("Image saving error."),
    };
}


pub async fn async_create_diagram(description: &str) -> Result<DynamicImage, &str> {

    let origin = description.as_bytes();
    let encoded: String = general_purpose::STANDARD_NO_PAD.encode(origin);
    let url = format!("https://mermaid.ink/img/{}", encoded);

    // Download image 
    let img_data = match reqwest::get(url).await {
        Ok(res) => res,
        Err(_) => return Err("Download image problem."),
    };
    
    let img_bytes = img_data.bytes().await;
    // Image to bytes
    let img_bytes = match img_bytes {
        Ok(res) => res,
        Err(_) => return Err("Encoding image problem."),
    }; 

    // Create image object
    match image::load_from_memory(&img_bytes){
        Ok(res) => return Ok(res),
        Err(_) => return Err("Image object creation error."),
    };

}

pub async fn async_save_diagram<'a>(description: &'a str, path: &'a str) -> Result<(), &'a str> {

    // Retrieve image with create_diagram function
    let image = match async_create_diagram(description).await {
        Ok(res) => res,
        Err(err) => return Err(err),
    };

    // Save image to file
    match image.save(path){
        Ok(_) => return Ok(()),
        Err(_) => return Err("Image saving error."),
    };
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_create_diagram() {

        let description = "graph LR;
            A--> B & C & D;
            B--> A & E;
            C--> A & E;
            D--> A & E;
            E--> B & C & D;
        ";
        let result = create_diagram(description);
        assert!(result.is_ok());
    }

    #[test]
    fn test_save_diagram() {

        let description = "graph LR;
            A--> B & C & D;
            B--> A & E;
            C--> A & E;
            D--> A & E;
            E--> B & C & D;
        ";
        let result = save_diagram(description, "test.jpg");
        assert!(result.is_ok());
    }
}
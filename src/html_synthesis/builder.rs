/*
basiac page consist of:
    * first lines with: 
                        <!DOCTYPE html>
                        <html lang="en">
    * head
    * body, consisting:
        * header
        * main
        * footer
    
    parts which differ among pages are:
        * head
        * main - in body
    parts which are the same among pages are:
        * header - in body
        * footer - in body
        * first lines


    IMPORTNANT NOTE
    there are 2 baisc pages types. logged in and logged out. they have different headers.

*/

pub trait Synthesizable {
    fn synthesize(&self) -> String;
}


pub struct Body{
    pub header: String, 
    pub individual: String,
    pub footer: String,
    pub scripts: String,
}

pub struct HtmlPage{
    pub head: String,
    pub body: Body,
}


impl Synthesizable for Body{
    fn synthesize(&self) -> String {
        let mut result = "<body>".to_string();
        result.push_str(&self.header);
        result.push_str(&self.individual);
        result.push_str(&self.footer);
        result.push_str(&self.scripts);
        result.push_str("</body>");
        result
    }
}


impl Synthesizable for HtmlPage{
    fn synthesize(&self) -> String{
        let mut result = String::new();
        result.push_str("<!DOCTYPE html>\n<html lang=\"en\">");
        result.push_str(&self.head);
        result.push_str(&self.body.synthesize());
        result.push_str("</html>");
        result
    }
}
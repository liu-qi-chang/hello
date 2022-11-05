use std::{collections::HashMap, io::Write};


#[derive(Debug,Clone,PartialEq)]
pub struct HttpReponse<'a>{
    pub version: &'a str,
    pub status_code: &'a str,
    pub status_text: &'a str,
    pub headers: Option<HashMap<&'a str, &'a str>>,
    pub body: Option<String>,
}

impl<'a> Default for HttpReponse<'a> {
    fn default() -> Self {
        Self { 
            version: "HTTP/1.1".into(),
            status_code: "200".into(), 
            status_text: "OK".into(), 
            headers: None, 
            body: None 
        }
    }
}

impl<'a> From<HttpReponse<'a>> for String {
    fn from(res: HttpReponse<'a>) -> Self {

        let resq = res.clone();
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            &resq.version(),
            &resq.status_code(),
            &resq.status_text(),
            &resq.headers(),
            &res.body.unwrap().len(),
            &resq.body()
        )
    }
}

impl<'a> HttpReponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> HttpReponse<'a> {

        let mut reponse: HttpReponse<'a> = HttpReponse::default();
        if status_code != "200"{
            reponse.status_code = status_code.into();
        }
        reponse.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };
        reponse.status_text = match reponse.status_code {
            "200" => "OK".into(),
            "400" => "Bad Request".into(),
            "404" => "Not Found".into(),
            "500" => "Ineternal Server Error".into(),
            _ => "Not Found".into(),
        };
        reponse.body = body;
        reponse
    }
    pub fn send_reponse(&self, write_stream: &mut impl Write) -> Result<(),()> {
        let res = self.clone();
        let reponse_string = String::from(res);
        let _ = write!(write_stream, "{}", reponse_string);
        Ok(())
    }

    fn version(&self) -> &str {
        self.version
    }

    fn status_code(&self) -> &str {
        self.status_code
    }

    fn status_text(&self) -> &str {
        self.status_text
    }

    fn headers(&self) -> String {
        let map : HashMap<&str, &str> = self.headers.clone().unwrap();
        let mut header_string : String = "".into();
        for(k, v) in map.iter() {
            header_string = format!("{}{}:{}\r\n", header_string, k, v);
        }
        header_string
    }

    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn test_response() {
        let response_actual = HttpReponse::new(
            "200",
            None,
            Some("xxx".into())
        );
        let res_excepted = HttpReponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxx".into()),
        };
        assert_eq!(response_actual, res_excepted);
    }
}
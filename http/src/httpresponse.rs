use std::collections::HashMap;
use std::io::{Result, Write};

#[derive(Debug, PartialEq, Clone)]
//'a 是生命周期参数，表示结构体中引用的生命周期。
pub struct HttpResponse<'a> {
    pub version: &'a str,
    pub status_code: &'a str,
    pub status_text: &'a str,
    pub headers: Option<HashMap<&'a str, &'a str>>,
    pub body: Option<String>,
}
// 这意味着 version、status_code、status_text 和 headers 中的字符串切片必须在 HttpResponse 结构体存在期间保持有效。
impl<'a> Default for HttpResponse<'a> {// 为类型提供一个默认值
    fn default() -> Self {
        Self {
            version: "HTTP/1.1".into(),//版本号
            status_code: "200".into(),//状态码
            status_text: "OK".into(),//状态文本
            headers: None,// HTTP 响应头
            body: None,// HTTP 响应体
        }
    }
    
}

impl<'a> From<HttpResponse<'a>> for String {//从 HttpResponse 类型转换为 String 类型
    fn from(res: HttpResponse<'a>) -> String {
        let res1 = res.clone();
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            &res1.version(),
            &res1.status_code(),
            &res1.status_text(),
            &res1.headers(),
            &res.body.unwrap().len(),
            &res1.body()
        )
    }
    
    
}


impl<'a> HttpResponse<'a> {
    pub fn from_str(res: &'a str ) -> HttpResponse<'a> {
        let mut lines = res.split("\r\n");
        let status_line = lines.next().unwrap_or_default();
        let mut headers = HashMap::new();
        let mut body = None;
    
        // Parse the status line
        let parts: Vec<&str> = status_line.split_whitespace().collect();
        let version = parts.get(0).unwrap_or(&"");
        let status_code = parts.get(1).unwrap_or(&"");
        let status_text = parts.get(2).unwrap_or(&"");
    
        // Parse headers
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            let parts: Vec<&str> = line.splitn(2, ": ").collect();
            if parts.len() == 2 {
                headers.insert(parts[0], parts[1]);
            }
        }
    
        // Parse body
        body = Some(lines.collect::<Vec<&str>>().join("\r\n"));
        let mut resp=HttpResponse::default();
        resp.version=&version;
        resp.status_code=&status_code;
        resp.status_text=&status_text;
        resp.headers=Some(headers);
        resp.body=body;
        
        
        resp
    
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> HttpResponse<'a> {
        let mut response: HttpResponse<'a> = HttpResponse::default();
        if status_code != "200" {
            response.status_code = status_code.into();
        };
        
        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };
        response.status_text = match response.status_code {
            "200" => "OK".into(),
            "400" => "Bad Request".into(),
            "404" => "Not Found".into(),
            "500" => "Internal Server Error".into(),
            _ => "Not Found".into(),
        };

        response.body = body;
        //返回构造好的响应
        response
    }

    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
        let res = self.clone();
        let response_string: String = String::from(res);
        let _ = write!(write_stream, "{}", response_string);
        // println!("{}", response_string);

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
        let map: HashMap<&str, &str> = self.headers.clone().unwrap();
        let mut header_string: String = "".into();
        for (k, v) in map.iter() {
            header_string = format!("{}{}:{}\r\n", header_string, k, v);
        }
        header_string
    }

    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),//将 String 转换为 &str 
            None => "",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_struct_creation_200() {
        let response_actual = HttpResponse::new("200", None, Some("xxxx".into()));
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxxx".into()),
        };
        assert_eq!(response_actual, response_expected);
    }

    #[test]
    fn test_response_struct_creation_404() {
        let response_actual = HttpResponse::new("404", None, Some("xxxx".into()));
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxxx".into()),
        };
        assert_eq!(response_actual, response_expected);
    }

    #[test]
    fn test_http_response_creation() {
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_text: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxxx".into()),
        };
        let http_string: String = response_expected.into();
        let actual_string =
            "HTTP/1.1 404 Not Found\r\nContent-Type:text/html\r\nContent-Length: 4\r\n\r\nxxxx";
        assert_eq!(http_string, actual_string);
    }
}


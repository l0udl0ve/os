use service::stub;
use service::message::Message;
use crypto::rsa_crypto;
use rsa::{RsaPrivateKey, RsaPublicKey};
pub fn handle_client(req_data: &String, private_key: &RsaPrivateKey,client_public_key: &RsaPublicKey) -> String
{
    let req_data=match parse_to_vec_u8(req_data) {
                Ok(vec) => vec, // 输出: [15, 69, 40]
                Err(e) => {
                    println!("解析错误: {}", e);
                    return  String::new();
                }
            };
    let req_data=rsa_crypto::decrypt_u8(&private_key,&req_data);
    let mes=serde_json::from_slice::<Message>(&req_data);
    
    //获取服务类型
    match mes {
        Ok(mes) => {
            //服务匹配
            println!("{:?}", mes);
            let res= match_service(mes);
            println!("结果 {:?}",res);
            // stream.write(res.as_bytes()).unwrap();
            // let res= rsa_crypto::encrypt_str(&client_public_key,&res);
            
            return res;
        },
        Err(e) => {
            let error_message = format!("Error parsing message: {}", e);
            // stream.write(error_message.as_bytes()).unwrap();
            return error_message
        }
    };
}

pub fn match_service(mes: Message) -> String {
    let mes_sname = mes.service_name.trim();
    let mes_data: Vec<&str> = mes.data.trim().split(' ').collect();
    // let mes_data=mes.data.trim().split(' ').collect();
    let res=match mes_sname {
        "add" => format!("{}", stub::add(mes_data[0].parse::<i32>().unwrap_or(0), mes_data[1].parse::<i32>().unwrap_or(0))),
        "subtract" => format!("{}", stub::subtract(mes_data[0].parse::<i32>().unwrap_or(0), mes_data[1].parse::<i32>().unwrap_or(0))),
        _ => "Unknown request".to_string(),
    };
    return res;
} 

fn parse_to_vec_u8(input: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
    // 去掉方括号
    let trimmed = input.trim_matches(|c| c == '[' || c == ']');
    // 分割字符串，去掉空格和逗号
    let parts: Vec<&str> = trimmed.split(|c: char| c.is_whitespace() || c == ',').filter(|s| !s.is_empty()).collect();
    // 将字符串部分转换为 u8
    let result: Result<Vec<u8>, _> = parts.iter().map(|s| s.parse::<u8>()).collect();
    
    result

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_to_vec_u8() {
        let input = "[15, 69, 40]";
        let result = parse_to_vec_u8(input);
        assert_eq!(result, Ok(vec![15, 69, 40]));
    }

    
    
    #[test]
    fn test_match_service() {
        let mes = Message {
            service_name: "add".to_string(),
            data_len: 2,
            data: "1 2".to_string(),
        };

        let result = match_service(mes);
        assert_eq!(result, "3");
    }
    
    #[test]
    fn test_match_service_unknown_request() {
        let mes = Message {
            service_name: "unknown".to_string(),
            data_len: 2,
            data: "1 2".to_string(),
        };

        let result = match_service(mes);
        assert_eq!(result, "Unknown request");
    }
    
    #[test]
    fn test_handle_client() {
        let req_data = "{\"service_name\":\"add\",\"data_len\":2,\"data\":\"1 2\"}";
        let (priv_key, pub_key) = rsa_crypto::generate_keys();
        let req_data=rsa_crypto::encrypt_str(&pub_key,req_data);
        //转化为字符串
        let mut string = String::from("[");
        for (i, &value) in req_data.iter().enumerate() {
            if i != 0 {
                string.push_str(", ");
            }
            string.push_str(&value.to_string());
        }
        string.push_str("]");
        
        let result = handle_client(&string, &priv_key, &pub_key);
        assert_eq!(result, "3");
    }
}
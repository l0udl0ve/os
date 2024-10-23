use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub service_name: String,
    pub data_len: u8,
    pub data: String,
}

fn serde_obj(mes: Message) -> String {
    
    let request = serde_json::to_string(&mes ).expect("Failed to serialize to JSON");
    return request;
}

fn deserder_obj(request: String) -> Message {
    let request = serde_json::from_str(&request).expect("Failed to deserialize from JSON");
    return request;
}
//数组
fn serde_vec(numbers: Vec<u8>) -> String {
    // 将数组序列化为 JSON 字符串
    let json_string = serde_json::to_string(&numbers).expect("Failed to serialize to JSON");
    // println!("Serialized JSON: {}", json_string);
    return json_string;
}

fn deserder_vec(json_string: String) -> Vec<u8> {
    // 将 JSON 字符串反序列化为数组
    let numbers: Vec<u8> = serde_json::from_str(&json_string).expect("Failed to deserialize from JSON");
    // println!("Deserialized numbers: {:?}", numbers);
    return numbers;
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_objserde() {
        let mes =Message {
            service_name: "add".to_string(),
            data_len: 3,
            data: "1 2".to_string(),
        };
    
        // 将结构体序列化为 JSON 字符串
        let json_string =serde_obj(mes);
    
        println!("Serialized JSON: {}", json_string);
    }

    #[test]
    fn test_objdeserde() {
        let json_string = r#"{
            "service_name": "add",
            "data_len": 3,
            "data": "1 2"
        }"#;
    
        // 将 JSON 字符串反序列化为结构体
        let person: Message = deserder_obj(json_string.to_string());
    
        println!("Deserialized struct: {:?}", person);
    }
    
    #[test]
    fn test_vecserde() {
        let numbers: Vec<u8> = [108, 123, 122].to_vec();

    // 将数组序列化为 JSON 字符串
        let json_string =serde_vec(numbers);

        println!("Serialized JSON: {}", json_string);
    }
    
    #[test]
    fn test_vecdeserde() {
        let json_string = r#"[108, 123, 122]"#;

    // 将 JSON 字符串反序列化为数组
        let numbers: Vec<u8> = deserder_vec(json_string.to_string());

        println!("Deserialized array: {:?}", numbers);
    }
}
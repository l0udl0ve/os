# 项目设计报告: 基于Rust语言的RPC设计和实现

## 1. 目录

- [1.目录](#1-目录)
- [2.目标描述](#2-目标描述)
    - [2.1.赛题分析](#21-赛题分析)
        - [2.1.1.赛题目的](#211-赛题目的)
        - [2.1.2.内容介绍](#212-内容介绍)
        - [2.1.3.序列化和反序列化方法](#213-序列化和反序列化方法)
    - [2.2. 相关调研](#22-相关调研)
        - [2.2.1.RPC调用](#221-RPC调用)
        - [2.2.2.RPC传输协议](#222-RPC传输协议)
        - [2.2.3.赛题目的](#223-赛题目的)
- [3.开发计划](#3-开发计划)
- [4.系统框架设计](#4-系统框架设计)
    - [4.1.系统设计介绍](#41-系统设计介绍)
    - [4.2.模块设计](#42-模块设计)
        - [4.2.1.通信模块设计](#421-通信模块设计)
        - [4.2.2.序列化模块设计](#422-序列化模块设计)
        - [4.2.3.服务调用模块设计](#423-服务调用模块设计)
        - [4.2.4.加解密模块设计](#424-加解密模块设计)
- [5.项目结构](#5-项目结构)

  
## 2. 目标描述

### 2.1.赛题分析

#### 2.1.1.赛题目的
基于Rust语言实现远程过程调用RPC机制，为操作系统提供分布式计算基础设施;通过模块化设计，实现功能的灵活部署。

#### 2.1.2.内容介绍
远程过程调用(RPC)是分布式计算的基础设施，它是利用网络从远程计算机请求服务，可以理解为将程序的一部分功能放在远程的计算机上。通过网络通信请求发送至远程计算机后，利用远程计算机的系统资源执行这部分程序，最终返回远程计算机上的执行结果。RPC核心组成包括四个部分client(服务调用者)、stub(本地存根)、server(服务器提供者)和RPCruntime(RPC通信者)。client、调用端的stub及其中一个RPC通信包的实例位于调用端的机器上，而server、服务提供端的stub及另一个RPC通信包的实例位于被调用的机器上。

#### 2.1.3.赛题要求
基于Rust语言实现远程过程调用基础设施，包括以下基础特性:采用模块化的设计方法，将RPC基础设施按照功能划分为不同的模块，如client、server、stub、RPCruntime；可基于Linux内核或如意内核运行。此外，可考虑额外加分特色功能，包括但不限于：加解密(实现远程通信的安全保证)、多体系结构(X86、ARM、RiSv)支持、远程服务管理中心(支持服务发布、查询等功能)、设计IDL接口描述语言等。

### 2.2.赛题调研
#### 2.2.1.RPC调用

RPC（Remote Procedure Call）—远程过程调用，它是一种通过网络从远程计算机程序上请求服务，而不需要了解底层网络技术的协议。比如两个不同的服务 A、B 部署在两台不同的机器上，那么服务 A 如果想要调用服务 B 中的某个方法该怎么办呢？使用 HTTP请求 当然可以，但是可能会比较慢而且一些优化做的并不好。 RPC 的出现就是为了解决这个问题。
从上面对 RPC 介绍的内容中，概括来讲RPC 主要解决了：让分布式或者微服务系统中不同服务之间的调用像本地调用一样简单。


服务消费方（client）调用以本地调用方式调用服务；
client stub接收到调用后负责将方法、参数等组装成能够进行网络传输的消息体；
client stub找到服务地址，并将消息发送到服务端；
server stub收到消息后进行解码；
server stub根据解码结果调用本地的服务；
本地服务执行并将结果返回给server stub；
server stub将返回结果打包成消息并发送至消费方；
client stub接收到消息，并进行解码；
服务消费方得到最终结果。

<font color="#ff0000">常见的 RPC 框架总结</font>

| 名称      | 描述                                                                                                                                 |
| :-----: | :--------------------------------------------------------------------------------------------------------------------------------: |
| RMI     | JDK自带的RPC，有很多局限性，不推荐使用。                                                                                                            |
| Dubbo   | 阿里巴巴公司开源的一个高性能优秀的服务框架，使得应用可通过高性能的 RPC 实现服务的输出和输入功能，可以和 Spring框架无缝集成。目前 Dubbo 已经成为 Spring Cloud Alibaba 中的官方组件。                     |
| gRPC    | 是可以在任何环境中运行的现代开源高性能RPC框架。它可以通过可插拔的支持来有效地连接数据中心内和跨数据中心的服务，以实现负载平衡，跟踪，运行状况检查和身份验证。它也适用于分布式计算的最后一英里，以将设备，移动应用程序和浏览器连接到后端服务。           |
| Hessian | 一个轻量级的remotingonhttp工具，使用简单的方法提供了RMI的功能。 相比WebService，Hessian更简单、快捷。采用的是二进制RPC协议，因为采用的是二进制协议，所以它很适合于发送二进制数据。                       |
| Thrift  | Apache Thrift是Facebook开源的跨语言的RPC通信框架，目前已经捐献给Apache基金会管理，由于其跨语言特性和出色的性能，在很多互联网公司得到应用，有能力的公司甚至会基于thrift研发一套分布式服务框架，增加诸如服务注册、服务发现等功能。 |

#### 2.2.2.RPC传输协议
##### (1) HTTP
HTTP（超文本传输协议）是一种用于从WWW服务器传输超文本到本地浏览器的传输协议，它可以使浏览器更加高效，使网络传输减少。它不仅保证计算机正确快速地传输超文本文档，还确定传输文档中的哪一部分，以及哪部分内容首先显示。

在RPC中，HTTP可以作为一种传输协议使用。RPC允许程序调用另一台计算机上的子程序，而不需要开发人员处理这个调用过程中涉及的底层通信细节。当RPC基于HTTP协议实现时，可以利用HTTP的请求-响应模型来执行远程方法调用。这种方式下，客户端通过发送一个HTTP请求到服务器来启动一个远程过程，服务器处理该请求并返回一个HTTP响应，其中包含了调用结果或错误信息。

##### (2) Socket
Socket，也称为套接字，是计算机网络中用于实现不同主机之间通信的一种编程接口。它提供了一组函数，允许程序员在网络中的两台主机之间建立连接、发送数据和接收数据。Socket编程是基于传输层协议（如TCP或UDP）进行的，使用IP地址和端口号来唯一标识网络中的主机和进程。

Socket在RPC的作用主要体现在以下几个方面：

- **底层通信机制**：Socket是实现RPC的基础，它负责在网络上的主机之间建立连接，并通过网络传输数据。在RPC框架中，客户端通过Socket向服务器发送请求，服务器通过Socket接收请求并返回响应。
    
- **数据传输**：在RPC调用过程中，客户端需要将调用的方法名、参数等信息序列化为二进制流，并通过Socket发送给服务器。服务器接收到数据后，进行反序列化操作，然后执行相应的方法，并将结果序列化后通过Socket返回给客户端。
    
- **连接管理**：在RPC中，Socket还负责管理客户端和服务器之间的连接。这包括建立连接、维护连接以及断开连接等操作。对于需要长时间保持连接的应用，Socket提供了长连接的选项，以减少频繁建立和断开连接带来的开销。
    
- **灵活性**：由于Socket提供了对底层网络通信的直接控制，它使得RPC可以实现更加灵活的通信策略。例如，可以根据应用的需求选择不同的传输协议（TCP或UDP），或者实现自定义的通信协议以满足特定的性能要求。
    
- **安全性**：虽然Socket本身不提供安全性，但可以通过在应用层实现安全机制来确保RPC通信的安全性。例如，可以使用SSL/TLS协议对Socket进行封装，实现加密通信。


#### 2.2.3.序列化和反序列化方法
**在RPC框架中，序列化和反序列化的作用是确保数据能够有效地在网络上传输并被正确解析**。序列化和反序列化是一种数据转化的技术，从数据的用途来看，序列化就是为了将数据按照规定的格式就行重组，在保持原有的数据语义不变的情况下，达到存储或者传输的目的；反序列化则是为了将序列化后的数据重新还原成具有语义的数据，以达到重新使用或者复用原有数据的目的。

##### (1) Json
JSON，全称是 JavaScript Object Notation，即 JavaScript对象标记法，是一种轻量级（Light-Meight)、基于文本的(Text-Based)、可读的(Human-Readable)格式。
JSON无论对于人，还是对于机器来说，都是十分便于阅读和书写的，而且相比 XML(另一种常见的数据交换格式)，文件更小，因此迅速成为网络上十分流行的交换格式。
因为JSON本身就是参考JavaScript 对象的规则定义的，其语法与JavaScript定义对象的语法几乎完全相同。JSON格式的创始人声称此格式永远不升级，这就表示这种格式具有长时间的稳定性，10 年前写的文件，10年后也能用,没有任何兼容性问题。

JSON 的语法规则十分简单，可称得上“优雅完美”，总结起来有：
- 数组（Array）用方括号(“\[ \]”)表示。
- 对象（0bject）用大括号(“{ }”)表示。
- 名称/值对(name/value）组合成数组和对象。
- 名称(name）置于双引号中，值（value）有字符串、数值、布尔值、null、对象和数组。
并列的数据之间用逗号(“,”)分隔

##### (2) XML
 XML全称是Extensible Markup Language,意思是可扩展的标记语言，XML的标签是可以由用户自定义的。但是为了限定XML的内容，需要使用xml约束（DTD或Schema）,为了获取xml的内容，我们需要用dom4j（常用）进行解析。

XML语法比json复杂，总结起来有以下七点：
1. **文档说明**
```xml
<?xml version="1.0" encoding="utf-8"?>
<!--这里的version是必须有的，后面的encoding是可有可无的，但是必须是小写-->
```
- 文档的声明必须以`<?xml开头 ，以?>`结束；
- 文档声明必须是从文档的0行0列的位置开始；
- 文档声明的只有三个属性：
    - 区分大小写
    - version：指明XML的版本，必须是属性，因为我们是不会选择1.1，只会选择1.0
    - encoding：指定当前文档的编码。可选择属性，默认值是utf-8

2. **元素**
元素是XML文件的基本构成单位，可以包含文本、属性和其他元素。元素通过开始标签和结束标签来定义。
```xml
<element>Content</element>
```

元素可以嵌套：
```xml
<parent>
    <child>Child content</child>
</parent>
```

元素的命名：
- 区分大小写
- 不能使用空格，冒号
- 不建议使用XML、xml、Xml开头

每个XML文件必须有且仅有一个根元素，所有其他元素都必须包含在这个根元素内。

3. **属性**
```xml
<web-app version="2.5">
```
- 属性是元素的一部分，它必须出现在元素的开始标签中
- 属性的定义格式：属性名=属性值，其中属性值必须使用单引或双引
- 一个元素可以有0~N个属性，但一个元素中不能出现同名属性
- 属性名不能使用空格、冒号等特殊字符，且必须以字母开头

4. **注释**
XML的注释与 HTML相同，即以`<!--`开始，以`-->`结束。注释内容会被 XML解析器忽略!

5. **转义字符**
XML中的转义字符与 HTML一样。
因为很多符号已经被XML文档结构所使用，所以在元素体或属性值中想使用这些符号就必须使用转义字符，例如：`<`、`>`、`’`、`”`、`&`。

| 字符  | 字符引用<br><font color="#bfbfbf">（十进制）</font> | 字符引用<br><font color="#bfbfbf">（十六进制）</font> | 预定义实体引用  |
| :-: | :----------------------------------------: | :-----------------------------------------: | :------: |
| `<` |                  `&#60;`                   |                  `&#x3c;`                   |  `&lt;`  |
| `>` |                  `&#62;`                   |                  `&#x3e;`                   |  `&gt;`  |
| `"` |                  `&#34;`                   |                  `&#x22;`                   | `&quot;` |
| `'` |                  `&#39;`                   |                  `&#x27;`                   | `&apos;` |
| `&` |                  `&#38;`                   |                  `&#x26;`                   | `&amp;`  |

6. **CDATA区**
CDATA区块用于包含不需要解析的文本数据，特别是包含特殊字符的文本。
```xml
<element><![CDATA[Some unparsed text]]></element>
```

## 3.开发计划

- [x] **通信框架（已完成）**
- [x] **序列化和反序列化（已完成）**
- [x] **服务调用（已完成）**
- [x] **加解密模块（已完成）**
- [ ] **IDL接口描述语言**<font color="#ff0000">（进行中）</font>
- [ ] **远程服务管理中心**<font color="#ff0000">（进行中）</font>
- [ ] **跨语言支持**
- [ ] **多体系结构支持**

## 4. 系统框架设计

### 4.1. 系统设计介绍
本项目构建的RPC系统，以**Rust**语言为基础语言，搭建**HTTP服务器客户端**作为通信的基础框架，兼顾稳定和高效搭建了以**json**为核心的**序列化和反序列化模块**，通过RSA算法进行数据的加解密，保证了用户的数据安全。目前采用编写trait的方式设计服务存根函数，在下一步的工作中考虑使用protobuf作为IDL描述语言，搭建一个和编程语言无关和OS无关的RPC项目，并且搭建远程服务管理中心，让用户更加便捷地增删服务。

### 4.2. 模块设计
#### 4.2.1.通信模块设计

##### (1) 搭建Socket框架

Socket（套接字）是对TCP/IP协议的封装，它提供了一种方式，使得应用层可以通过调用Socket接口来实现网络通信。Socket通信是通信框架的基础，要实现HTTP等架构于应用层的通信框架，首先必须实现Socket通信框架。
###### a. 服务端
```rust
use std::io::{Read, Write};
use std::net::TcpListener;

fn main() 
{
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000...");

    // let result = listener.accept().unwrap(); // 只接收一次请求

    for stream in listener.incoming() 
    {
        let mut stream = stream.unwrap();
        println!("Connection established!");
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }
}
```

###### b. 客户端
```TypeScript
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() 
{
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    stream.write("Hello".as_bytes()).unwrap();

    let mut buffer = [0; 5];
    stream.read(&mut buffer).unwrap();

    println!(
        "Response from server: {:?}",
        str::from_utf8(&buffer).unwrap()
    );
}
```

在Cargo.toml中配置工作区的包

```
[workspace]

members = ["tcpserver", "tcpclient"]
```

##### (2) 搭建http服务器
使用HTTP搭建RPC而不直接使用socket通信搭建的好处主要体现在以下几个方面：

1. **简单易用**：HTTP协议隐藏了底层网络细节，使得开发者可以专注于业务逻辑的实现，而无需过多关注网络通信的具体实现。这种简单易用的特性降低了开发门槛，提高了开发效率。

2. **跨平台和跨语言**：HTTP协议几乎适用于所有的平台和语言，这使得基于HTTP的RPC服务可以轻松地与不同平台和语言的应用进行交互。这种跨平台和跨语言的特性为系统的集成和扩展提供了极大的便利。

3. **灵活性高**：HTTP还支持多种数据格式（如JSON、XML等），方便数据的传输和解析。

4. **易于调试和测试**：由于HTTP协议的广泛应用和成熟性，存在大量的工具和库用于HTTP服务的调试和测试。

5. **利用现有基础设施**：许多企业和组织已经部署了大量的HTTP代理、负载均衡器和防火墙等基础设施来处理HTTP流量。通过使用HTTP搭建RPC，可以利用这些现有基础设施来实现服务的负载均衡、故障转移和安全防护等功能。

6. **易于集成第三方服务**：许多第三方服务（如API网关、监控工具等）都支持HTTP协议。通过使用HTTP搭建RPC，可以方便地将这些第三方服务集成到系统中，以实现更丰富的功能和更好的可观测性。

###### a. 消息流动
客户端的请求 -> Server（HTTP Library） -> Router -> Handlers ->处理请求回复客户端

###### b. 各部分功能

- Server：监听进来的TCP字节流

- Router：接受HTTP请求，并决定调用哪个Handler

- Handler：处理HTTP请求，构建HTTP响应

- HTTP Library：1、解释字节流，把它转化为HTTP请求；把HTTP响应转化回字节流

###### c. 构建步骤

- 1.解析HTTP请求消息
    
- 2.构建HTTP响应消息
    
- 3.路由与Handler
    
- 4.测试Web Server

##### (3) 解析 HTTP 请求

###### a. 数据结构

| **数据结构名称**  | **数据类型** |    **描述**    |
| :---------: | :------: | :----------: |
| HttpRequest |  struct  |   表示HTTP请求   |
|   Method    |   enum   | 指定所允许的HTTP方法 |
|   Version   |   enum   | 指定所允许的HTTP版本 |

###### b. 具体实现

http/lib.rs

```rust
pub mod httprequest;
```

Cargo.toml 修改配置如下

```
[workspace]

members = ["tcpserver", "tcpclient", "http", "httpserver"]
```

http/src/httprequest.rs

```rust
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
//debug  {:?} 格式化输出
//自动派生 PartialEq 特性，使得枚举类型可以进行相等性比较
//enum 关键字用于定义枚举类型
pub enum Method 
{
    Get,
    Post,
    Uninitialized,
}

//impl 关键字用于实现某个特性（trait）。
//From<&str> 是一个标准库中的特性，表示可以从 &str 类型转换为 Method 类型
impl From<&str> for Method 
{
    fn from(s: &str) -> Method 
    {
        match s 
        {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
            //其他任何值都会返回 Method::Uninitialized
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version 
{
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version 
{
    fn from(s: &str) -> Version 
    {
        match s 
        {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            _ => Version::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,// 请求方法           
    pub version: Version,// HTTP 版本
    pub resource: Resource,// 请求资源
    pub headers: HashMap<String, String>,// 请求头
    pub msg_body: String, // 请求体
}

impl From<String> for HttpRequest 
{
    fn from(req: String) -> Self 
    {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";
        
        for line in req.lines() 
        {
            if line.contains("HTTP") 
            {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } 
            else if line.contains(":") 
            {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            } 
            else if line.len() == 0 {} 
            else 
            {
                parsed_msg_body = line;
                // parsed_msg_body = parsed_msg_body + line;
            }
        }
        
        HttpRequest 
        {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body.to_string(),
        }
    }
}

fn process_req_line(s: &str) -> (Method, Resource, Version) 
{
    //将字符串按空白字符（空格、制表符、换行符等）分割成一个迭代器
    let mut words = s.split_whitespace();
    //words.next() 方法从迭代器中获取下一个元素。
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();

    //构造返回值
    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )
}

fn process_header_line(s: &str) -> (String, String) 
{
    let mut header_items = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");
    if let Some(k) = header_items.next() 
    {
        key = k.to_string();
    }
    if let Some(v) = header_items.next() 
    {
        value = v.to_string();
    }

    (key, value)
}

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn test_method_into() 
    {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
    }

    #[test]
    fn test_version_into() 
    {
        let v: Version = "HTTP/1.1".into();
        assert_eq!(v, Version::V1_1);
    }

    #[test]
    fn test_read_http() 
    {
        let s: String = String::from("GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.71.1\r\nAccept: */*\r\n\r\n");
        let mut headers_expected = HashMap::new();
        headers_expected.insert("Host".into(), " localhost:3000".into());
        headers_expected.insert("Accept".into(), " */*".into());
        headers_expected.insert("User-Agent".into(), " curl/7.71.1".into());
        let req: HttpRequest = s.into();

        assert_eq!(Method::Get, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
        assert_eq!(headers_expected, req.headers);
    }
}
```

##### (4) 构建HTTP响应

###### a. 需求分析

| **需要实现的方法或trait** | **用途**                   |
| :-----------------: | :------------------------: |
| Default trait     | 指定成员的默认值                 |
| new()             | 使用默认值创建一个新的结构体           |
| send_response()   | 构建响应，将原始字节通过TCP传送        |
| getter 方法         | 获得成员的值                   |
| From trait        | 能够将HttpResponse转化为String |

###### b. 代码实现

http/lib.rs

```rust
pub mod httprequest;
pub mod httpresponse;
```

http/src/httpresponse.rs

```rust
use std::collections::HashMap;
use std::io::{Result, Write};

#[derive(Debug, PartialEq, Clone)]
//'a 是生命周期参数，表示结构体中引用的生命周期。
pub struct HttpResponse<'a> 
{
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}
// 这意味着 version、status_code、status_text 和 headers 中的字符串切片必须在 HttpResponse 结构体存在期间保持有效。
impl<'a> Default for HttpResponse<'a> // 为类型提供一个默认值
{
    fn default() -> Self 
    {
        Self {
            version: "HTTP/1.1".into(),//版本号
            status_code: "200".into(),//状态码
            status_text: "OK".into(),//状态文本
            headers: None,// HTTP 响应头
            body: None,// HTTP 响应体
        }
    }
}

impl<'a> From<HttpResponse<'a>> for String //从 HttpResponse 类型转换为 String 类型
{
    fn from(res: HttpResponse<'a>) -> String 
    {
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

impl<'a> HttpResponse<'a> 
{
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> HttpResponse<'a> 
    {
        let mut response: HttpResponse<'a> = HttpResponse::default();
        if status_code != "200" 
        {
            response.status_code = status_code.into();
        };
        
        response.headers = match &headers 
        {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };
        response.status_text = match response.status_code 
        {
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

    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> 
    {
        let res = self.clone();
        let response_string: String = String::from(res);
        let _ = write!(write_stream, "{}", response_string);
        // println!("{}", response_string);

        Ok(())
    }

    fn version(&self) -> &str 
    {
        self.version
    }

    fn status_code(&self) -> &str 
    {
        self.status_code
    }

    fn status_text(&self) -> &str 
    {
        self.status_text
    }

    fn headers(&self) -> String 
    {
        let map: HashMap<&str, &str> = self.headers.clone().unwrap();
        let mut header_string: String = "".into();
        for (k, v) in map.iter() 
        {
            header_string = format!("{}{}:{}\r\n", header_string, k, v);
        }
        header_string
    }

    pub fn body(&self) -> &str 
    {
        match &self.body 
        {
            Some(b) => b.as_str(),//将 String 转换为 &str 
            None => "",
        }
    }
}
```

##### (5) 构建server模块

在httpserver/Cargo.toml中配置

```
[package]
name = "httpserver"
version = "0.1.0"
edition = "2021"

[dependencies]
http = {path = "../http"}
```

httpserver/main.rs 负责启动服务器

```rust
mod handler;
mod router;
mod server;

use server::Server;

fn main() 
{
    let server = Server::new("localhost:3000");
    server.run();
}
```

httpserver/server.rs 接受处理响应请求

```TypeScript
use super::router::Router;
use http::httprequest::HttpRequest;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;

pub struct Server<'a> 
{
    socket_addr: &'a str,//服务器套接字地址
}

impl<'a> Server<'a> 
{
    //构造函数
    pub fn new(socket_addr: &'a str) -> Self 
    {
        Server { socket_addr }
    }

    pub fn run(&self) 
    {
        
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Running on {}", self.socket_addr);

        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established");

            let mut read_buffer = [0; 200];
            stream.read(&mut read_buffer).unwrap();
            
             // 打印原始的HTTP请求
            let request_str = match str::from_utf8(&read_buffer) 
            {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
            println!("Received HTTP request:\n{}\n", request_str.trim_end_matches(char::from(0)));

            //将数据转化成http包格式
            let req= HttpRequest::from(request_str.to_string());
            println!("{:?}", req);
            let req: HttpRequest = String::from_utf8(read_buffer.to_vec()).unwrap().into();
            // println!("{:?}", req);
            Router::route(req, &mut stream);
        }
    }
}
```

##### (6) 构建 router 和 handler 模块

router模块负责分析请求内容，handler模块负责处理消息 在httpserver/Cargo.toml中配置

```
[package]
name = "httpserver"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
http = {path = "../http"}
serde = {version="1.0.163", features = ["derive"]}
serde_json = "1.0.96"
```

httpserver/src/handler.rs

```rust
use http::{httprequest::HttpRequest, httpresponse::HttpResponse};
use serde::{Deserialize, Serialize};//用于序列化和反序列化 JSON 数据
use std::collections::HashMap;
use std::env;
use std::fs;

pub trait Handler 
{
    //定义了一个抽象方法，用于处理 HTTP 请求并返回 HttpResponse。
    fn handle(req: &HttpRequest) -> HttpResponse;
    
    fn load_file(file_name: &str) -> Option<String> 
    {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", public_path, file_name);

        let contents = fs::read_to_string(full_path);
        //读取文件内容，并返回 Option<String>。
        contents.ok()
    }
}

pub struct StaticPageHandler;
pub struct PageNotFoundHandler;
pub struct WebServiceHandler;

#[derive(Serialize, Deserialize)]
pub struct OrderStatus 
{
    order_id: i32,
    order_date: String,
    order_status: String,
}

impl Handler for PageNotFoundHandler 
{
    fn handle(_req: &HttpRequest) -> HttpResponse 
    {
        HttpResponse::new("404", None, Self::load_file("404.html"))
    }
}

impl Handler for StaticPageHandler 
{
    fn handle(req: &HttpRequest) -> HttpResponse 
    {
        let http::httprequest::Resource::Path(s) = &req.resource;
        let route: Vec<&str> = s.split("/").collect();
        match route[1] 
        {
            "index" => HttpResponse::new("200", None, Self::load_file("index.html")),
            "health" => HttpResponse::new("200", None, Self::load_file("health.html")),
            
            path => match Self::load_file(path) 
            {
                Some(contents) => 
                {
                    let mut map: HashMap<&str, &str> = HashMap::new();
                    if path.ends_with(".css") 
                    {
                        map.insert("Content-Type", "text/css");
                    } 
                    else if path.ends_with(".js") 
                    {
                        map.insert("Content-Type", "text/javascript");
                    } 
                    else 
                    {
                        map.insert("Content-Type", "text/html");
                    }
                    HttpResponse::new("200", Some(map), Some(contents))
                }
                None => HttpResponse::new("404", None, Self::load_file("404.html")),
            },
        }
    }
}

impl WebServiceHandler 
{
    fn load_json() -> Vec<OrderStatus> 
    {
        let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", data_path, "orders.json");
        let json_contents = fs::read_to_string(full_path);
        let orders: Vec<OrderStatus> =
            serde_json::from_str(json_contents.unwrap().as_str()).unwrap();
        orders
    }
}

impl Handler for WebServiceHandler 
{
    fn handle(req: &HttpRequest) -> HttpResponse 
    {
        //获取资源路径 服务类型
        let http::httprequest::Resource::Path(s) = &req.resource;
        // 获取请求体 数据
        // let req_data =&req.msg_body;
        // 将路径分割成数组
        let route: Vec<&str> = s.split("/").collect();
        // localhost:3000/api/shipping/orders
        match route[2] 
        {
            //果 route[2] 是 "shipping" 并且路径有至少四个部分（即 route.len() > 2），并且第四部分 route[3] 是 "orders"。
            "shipping" if route.len() > 2 && route[3] == "orders" => 
            {
                let body = Some(serde_json::to_string(&Self::load_json()).unwrap());
                let mut headers: HashMap<&str, &str> = HashMap::new();
                headers.insert("Content-Type", "application/json");
                HttpResponse::new("200", Some(headers), body)
            }
            _ => HttpResponse::new("404", None, Self::load_file("404.html")),
        }
    }
}
```

httpserver/src/router.rs

```rust
use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use http::{httprequest, httprequest::HttpRequest, httpresponse::HttpResponse};
use std::io::prelude::*;

pub struct Router;

impl Router 
{
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () 
    {
        match req.method 
        {
            httprequest::Method::Get => match &req.resource 
            {
                httprequest::Resource::Path(s) => 
                {
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] 
                    {
                        "api" => 
                        {
                            let resp: HttpResponse = WebServiceHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                        _ => 
                        {
                            let resp: HttpResponse = StaticPageHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            },
            _ => 
            {
                let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}
```

httpserver/data/orders.json

```
[
    {
        "order_id": 1,
        "order_date": "21 Jan 2024",
        "order_status": "Delivered"
    },
    {
        "order_id": 2,
        "order_date": "2 Feb 2024",
        "order_status": "Pending"
    }
]
```

##### (7) 编写测试示例
简单写了网页进行测试

httpserver/public/404.html

```TypeScript
<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Not Found!</title>
</head>

<body>
    <h1>404 Error</h1>
    <p>Sorry the requested page does not exist</p>
</body>

</html>
```

httpserver/public/health.html

```html
<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Health!</title>
</head>

<body>
    <h1>Hello welcome to health page!</h1>
    <p>This site is perfectly fine</p>
</body>

</html>

```

httpserver/public/index.html

```html
<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link rel="stylesheet" href="styles.css">
    <title>Index!</title>
</head>

<body>
    <h1>Hello, welcome to home page</h1>
    <p>This is the index page for the web site</p>
</body>

</html>

```

httpserver/public/styles.css

```css
h1 
{
    color: red;
    margin-left: 25px;
}
```

#### 4.2.2.序列化模块设计
经过前期调研发现使用json为基础来设计序列化，相比于xml而言，能在保证准确性的同时，较大程度地提升效率。
在一次RPC调用的过程中**客户端**（服务调用方）需要提交**服务名称**（service_name）、**数据长度**（data_len）、**服务参数**（data）三个字段。由序列化模块转化为json格式添加并到HTTP报文的body中，序列化的数据会在**服务端**（服务提供方）进行反序列化。

```rust
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
```

#### 4.2.3.服务调用模块设计
在服务端接收到客户端所传递的参数之后，将其反序列化，之后进行服务匹配，从所提供服务的列表中找到对应服务的函数地址，并进行调用。
```rust
use service::stub;
use service::message::Message;


pub fn handle_client(req_data: &String,private_key: &[u8]) -> String
{
    let mes=serde_json::from_slice::<Message>(req_data.as_bytes());
    //获取服务类型
    match mes {
        Ok(mes) => {
            //服务匹配
            let res= match_service(mes,private_key);
            // stream.write(res.as_bytes()).unwrap();
            return res
        },
        Err(e) => {
            let error_message = format!("Error parsing message: {}", e);
            // stream.write(error_message.as_bytes()).unwrap();
            return error_message
        }
    };


}

pub fn match_service(mes: Message,private_key: &[u8]) -> String {
    let mes_sname = mes.service_name.trim();
    let mut mes_data=crypto::decrypto(private_key,mes.data);
    let mes_data: Vec<&str> = mes.data.trim().split(' ').collect();
    // let mes_data=mes.data.trim().split(' ').collect();
    let res=match mes_sname {
        "add" => format!("{}", stub::add(mes_data[0].parse::<i32>().unwrap_or(0), mes_data[1].parse::<i32>().unwrap_or(0))),
        "subtract" => format!("{}", stub::subtract(mes_data[0].parse::<i32>().unwrap_or(0), mes_data[1].parse::<i32>().unwrap_or(0))),
        _ => "Unknown request".to_string(),
    };
    return res;
} 
```

#### 4.2.4.加解密模块设计
在本项目所构建的RPC框架下，采用RSA算法对所需要传输的数据进行加解密。
##### (1) 加密模块
```rust
use rsa::{RsaPrivateKey, RsaPublicKey, PaddingScheme};
use rsa::rand::SeedableRng;
use rsa::rand::rngs::StdRng;

pub fn generate_keys() -> (RsaPrivateKey, Vec<u8>) {
    let rng = &mut StdRng::from_entropy();
    let bits = 2048;
    let private_key = RsaPrivateKey::new(rng, bits).unwrap();
    let public_key = RsaPublicKey::from(&private_key);
    (private_key, public_key.to_bytes())
}

pub fn decrypt_message(private_key: &RsaPrivateKey, ciphertext: &[u8]) -> Vec<u8> {
    let padding = PaddingScheme::PKCS1v15Encrypt;
    private_key.decrypt(padding, ciphertext).unwrap()
}
```

##### (2) 解密模块
```rust
use rsa::{Rsa, PublicKey, PaddingScheme};

pub fn encrypt_message(public_key: &[u8], message: &[u8]) -> Vec<u8> {
    let rsa = Rsa::from_public_key(public_key).unwrap();
    let padding = PaddingScheme::PKCS1v15Encrypt;
    rsa.public_key().encrypt(&mut rand::thread_rng(), padding, message).unwrap()
}
```

## 5. 项目结构

```
/ (根目录)
├── README.md                         # 项目概述
├── code                              # 代码文件夹
│   ├── crypto                        # 加解密模块
|   |   ├──src              
|   |   |  ├──lib.rs
|   |   |  └── rsa_crypto.rs
|   |   └── Cargo.toml
│   ├── http                          # http请求响应模块
|   |   ├──src
|   |   |  ├── lib.rs
|   |   |  ├── httprequest.rs
|   |   |  └── httpresponse.rs
|   |   └── Cargo.toml
│   ├── httpserver                    # http服务器
│   |   ├── src
│   |   |   └── handler
│   |   |   |   └── call.rs
|   |   |   ├── handler.rs
|   |   |   ├── router.rs
|   |   |   ├── main.rs
│   |   |   └── server.rs
│   |   ├── public                    # http测试网页文件
|   |   |   ├── 404.html
|   |   |   ├── health.html
|   |   |   ├── index.html
|   |   |   └── styles.css
│   |   └── Cargo.toml
│   ├── service                       # 服务调用和序列化模块
|   |   ├──src
|   |   |  ├── lib.rs
|   |   |  ├── message.rs
|   |   |  ├── serialization_json.rs
|   |   |  └── stub.rs
│   |   └── Cargo.toml
│   ├── src
│   |   └── main.rs                    # 项目主文件
│   ├── tcpclient                      # TCP客户端
│   |   ├── src
│   |   |   └── main.rs
│   |   └── Cargo.toml
│   ├── tcpserver                      # TCP服务器
│   |   ├── src
|   |   |   ├── call.rs
│   |   |   └── main.rs
│   |   └── Cargo.toml
│   ├──.gitignore
│   ├── Cargo.lock
│   └── Cargo.toml
└── 项目功能说明书.pdf
```


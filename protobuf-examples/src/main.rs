mod protos;

use protobuf::YYPMessage;
// use protobuf::Message;

use protobuf::MessageField;

// For demonstration we take `GetRequest` messages from a source generated
// with pure-rust codegen, and `GetResponse` with `protoc`-based codegen.
// This is not needed in practice, done here for demonstration purposes.
use protos::generated_with_native::example::get_response::Status;
use protos::generated_with_native::example::GetResponse;
use protos::generated_with_pure::example::GetRequest;
use protos::generated_with_native::example::file_descriptor;
use protos::generated_with_native::uri::exts::uri;
use protos::generated_with_native::example::Bone;
use protos::generated_with_native::example::Cat;
use protos::generated_with_native::example::Dog;
use protos::generated_with_native::yypspecial::Uint8;
use protos::generated_with_native::yypspecial::Uint16;
use protos::generated_with_native::example::YYPSmallNumber;

fn try_rust() {
    println!("--------try--------");
}

fn test_yypspecial() {
    println!("\n--------hello yypspeical--------");
    println!("--------test Uint8--------");
    {
        let mut msg = Uint8::new();
        msg.data = "150".to_string();
        println!("msg: {:#?}", msg);

        let out_bytes: Vec<u8> = msg.write_to_bytes().unwrap();
        println!("out_bytes: {:?}", out_bytes);
        let in_msg = Uint8::parse_from_bytes(&out_bytes).unwrap();
        println!("in_msg: {:#?}", in_msg);
    }
    println!("--------test Uint16--------");
    {
        let mut msg = Uint16::new();
        msg.data = "6501".to_string();
        println!("msg: {:#?}", msg);

        let out_bytes: Vec<u8> = msg.write_to_bytes().unwrap();
        println!("out_bytes: {:?}", out_bytes);
        let in_msg = Uint16::parse_from_bytes(&out_bytes).unwrap();
        println!("in_msg: {:#?}", in_msg);
    }
    println!("--------test YYPSmallNumber--------");
    {
        let mut msg = YYPSmallNumber::new();
        {
            let mut n8 = Uint8::new();
            n8.data = "3".to_string();
            msg.n8 = MessageField::some(n8);
        }
        {
            let mut n8 = Uint8::new();
            n8.data = "7".to_string();
            msg.rep8.push(n8);
        }
        {
            let mut n8 = Uint8::new();
            n8.data = "8".to_string();
            msg.rep8.push(n8);
        }
        {
            let mut n8 = Uint8::new();
            n8.data = "8".to_string();
            msg.map8.insert(5, n8);
        }

        {
            let mut n16 = Uint16::new();
            n16.data = "7".to_string();
            msg.rep16.push(n16);
        }
        {
            let mut n16 = Uint16::new();
            n16.data = "8".to_string();
            msg.rep16.push(n16);
        }
        {
            let mut n16 = Uint16::new();
            n16.data = "8".to_string();
            msg.map16.insert(5, n16);
        }

        let mut n16 = Uint16::new();
        n16.data = "30617".to_string();
        msg.n16 = MessageField::some(n16);
        println!("msg: {:#?}", msg);

        let out_bytes: Vec<u8> = msg.write_to_bytes().unwrap();
        println!("out_bytes: {:?}", out_bytes);
        let in_msg = YYPSmallNumber::parse_from_bytes(&out_bytes).unwrap();
        println!("in_msg: {:#?}", in_msg);
    }

}

fn test_uri() {
    println!("uri: {:?}", uri.field_number);
    use protobuf::Message;
    // method 1 to get uri:
    println!("yy.uri: {:#?}", file_descriptor()
             .message_by_package_relative_name("GetRequest").unwrap()
             .get_proto().options.get_or_default().get_unknown_fields()
             .get(uri.field_number).unwrap().varint.first().unwrap());
    // method 2 to get uri:
    println!("yy.uri: {:#?}", <GetRequest as Message>::descriptor_static()
             .get_proto().options.get_or_default().get_unknown_fields()
             .get(uri.field_number).unwrap().varint.first().unwrap());
}

fn test_fileds() {
    let mut bone = Bone::new();
    bone.name = "hello!".to_string();
    let bone_msg_len = bone.compute_size();
    println!("bone_msg_len: {}", bone_msg_len);

    let mut bone2 = Bone::new();
    bone2.name = "12".to_string();

    let mut bone3 = Bone::new();
    bone3.name = "jcj".to_string();

    let mut cat = Cat::new();
    cat.bone = MessageField::some(bone);
    cat.map1.insert(100 as i32, bone2);
    cat.map1.insert(102 as i32, bone3);
    let cat_msg_len = cat.compute_size();
    println!("cat_msg_len: {}", cat_msg_len);

    {
        let mut bone3 = Bone::new();
        bone3.name = "hello!".to_string();
        let out_bytes: Vec<u8> = bone3.write_to_bytes().unwrap();
        println!("out_bytes: {:?}", out_bytes);
    }
    {
        println!("cat: {:#?}", cat);
        let out_bytes: Vec<u8> = cat.write_to_bytes().unwrap();
        println!("cat out_bytes: {:?}", out_bytes);
        let in_msg = Cat::parse_from_bytes(&out_bytes).unwrap();
        println!("cat in_msg: {:#?}", in_msg);
    }

    {
        println!("=============DOG===============");
        let dog = Dog::new();
        let out_bytes: Vec<u8> = dog.write_to_bytes().unwrap();
        println!("dog out_bytes: {:?}", out_bytes);
        let in_msg = Dog::parse_from_bytes(&out_bytes).unwrap();
        println!("dog in_msg: {:#?}", in_msg);
    }
}

fn test() {
    // Encode example request
    let mut out_msg = GetRequest::new();
    out_msg.name = "John Smith".to_string();
    out_msg.age = 25;
    out_msg.features.push("one".to_string());
    out_msg.features.push("two".to_string());

    let out_bytes: Vec<u8> = out_msg.write_to_bytes().unwrap();

    // Decode example request
    let in_msg = GetRequest::parse_from_bytes(&out_bytes).unwrap();

    let in_name = in_msg.name;

    assert_eq!(in_name, "John Smith");

    //////////////////////////////////

    // Encode example response
    let mut out_resp = GetResponse::new();
    out_resp.status = Status::OK.into();
    out_resp.address = "1243 main street".to_string();
    out_resp.city = "anytown".to_string();
    out_resp.zipcode = 54321;

    let out_bytes: Vec<u8> = out_resp.write_to_bytes().unwrap();

    // Decode example response
    let in_resp = GetResponse::parse_from_bytes(&out_bytes).unwrap();

    assert_eq!(in_resp.status, out_resp.status);
    assert_eq!(in_resp.zipcode, out_resp.zipcode);
    assert_eq!(in_resp.address, out_resp.address);
}

fn main() {
    println!("hello example!");

    test_uri();

    test_fileds();

    test();

    test_yypspecial();

    try_rust();
}

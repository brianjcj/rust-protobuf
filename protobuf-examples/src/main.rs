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

fn main() {
    println!("hello main");
    println!("uri: {:?}", uri.field_number);
    {
        use protobuf::Message;
        // method 1:
        println!("yy.uri: {:#?}", file_descriptor()
                 .message_by_package_relative_name("GetRequest").unwrap()
                 .get_proto().options.get_or_default().get_unknown_fields()
                 .get(uri.field_number).unwrap().varint.first().unwrap());
        // method 2:
        println!("yy.uri: {:#?}", <GetRequest as Message>::descriptor_static()
                 .get_proto().options.get_or_default().get_unknown_fields()
                 .get(uri.field_number).unwrap().varint.first().unwrap());
    }

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
        println!("cat in_mnsg: {:#?}", in_msg);
    }

    {
        println!("=============DOG===============");
        let dog = Dog::new();
        let out_bytes: Vec<u8> = dog.write_to_bytes().unwrap();
        println!("dog out_bytes: {:?}", out_bytes);
        let in_msg = Dog::parse_from_bytes(&out_bytes).unwrap();
        println!("dog in_mnsg: {:#?}", in_msg);
    }

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

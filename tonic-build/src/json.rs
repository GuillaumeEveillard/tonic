use super::{client, server};
use proc_macro2::TokenStream;
use quote::ToTokens;
use std::io;
use std::path::{Path, PathBuf};
use crate::{Service, Method};
use std::fs::File;
use std::io::Write;
use quote::quote;

pub fn compile_protos_json(proto: impl AsRef<Path>) -> io::Result<()> {
    let proto_path: &Path = proto.as_ref();

    // directory the main .proto file resides in
    let proto_dir = proto_path
        .parent()
        .expect("proto file should reside in a directory");

    compile(&[proto_path], &[proto_dir])?;

    Ok(())
}

struct DummyService {
    methods: Vec<DummyMethod>,
    comments: Vec<String>
}

struct DummyMethod {
    name: String,
    identifier: String,

}

impl Method for DummyMethod {
    const CODEC_PATH: &'static str = "tonic::codec::JsonCodec";
    type Comment = String;

    fn name(&self) -> &str {
        return &self.name;
    }

    fn identifier(&self) -> &str {
        return &self.identifier;
    }

    fn client_streaming(&self) -> bool {
        false
    }

    fn server_streaming(&self) -> bool {
        false
    }

    fn comment(&self) -> &[Self::Comment] {
        &[]
    }

    fn request_response_name(&self, proto_path: &str) -> (TokenStream, TokenStream) {
        (quote!{super::HelloRequest}, quote!{super::HelloReply})
    }
}

impl Service for DummyService {
    const CODEC_PATH: &'static str = "mon-json-codec";
    type Comment = String;
    type Method = DummyMethod;

    fn name(&self) -> &str {
        return "JsonGreeter";
    }

    fn package(&self) -> &str {
        return "monpackage";
    }

    fn identifier(&self) -> &str {
        return "identifier";
    }

    fn methods(&self) -> &[Self::Method] {
        return &self.methods;
    }

    fn comment(&self) -> &[Self::Comment] {
        return &self.comments;
    }
}

pub fn compile<P>(
    protos: &[P],
    includes: &[P],
) -> io::Result<()>
    where
        P: AsRef<Path>,
{

    // Il faut 3 choses
    // - générer les struts (pas fait)
    // - générer les implémentation server et client (fait)
    // - le decoder json JsonCodec (fait)

    let structs = quote! {
        use serde::{Serialize, Deserialize};

        /// The request message containing the user's name.
        #[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
        pub struct HelloRequest {
            pub name: std::string::String,
        }
        /// The response message containing the greetings
        #[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
        pub struct HelloReply {
            pub message: std::string::String,
        }
    };


    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    // let proto_path = protos.as_ref().to_str().unwrap();

    let service = DummyService{methods: vec![DummyMethod{name: "method1".to_string(), identifier: "id1".to_string()}], comments: Vec::new()};

    let it = protos.as_ref().into_iter();

    for proto in protos.as_ref().into_iter() {
        let proto_path = proto.as_ref().to_str().unwrap();
        let server = server::generate(&service, &proto_path);
        let client = client::generate(&service, &proto_path);

        let mut file = File::create(out_dir.join("helloworld_json.rs"))?;
        file.write_all(structs.to_string().as_bytes())?;
        file.write_all(server.to_string().as_bytes())?;
        file.write_all(client.to_string().as_bytes())?;
    }





    // config.out_dir(out_dir.clone());
    // for (proto_path, rust_path) in self.extern_path.iter() {
    //     config.extern_path(proto_path, rust_path);
    // }
    // for (prost_path, attr) in self.field_attributes.iter() {
    //     config.field_attribute(prost_path, attr);
    // }
    // for (prost_path, attr) in self.type_attributes.iter() {
    //     config.type_attribute(prost_path, attr);
    //
    //
    // // config.service_generator(Box::new(ServiceGenerator::new(self)));
    //
    // config.compile_protos(protos, includes)?;


    Ok(())
}
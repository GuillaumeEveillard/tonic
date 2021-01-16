use super::{client, server};
use proc_macro2::TokenStream;
use quote::ToTokens;
use std::io;
use std::path::{Path, PathBuf};
use crate::{Service, Method};
use std::fs::File;
use std::io::Write;

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

}

impl Method for DummyMethod {
    const CODEC_PATH: &'static str = "";
    type Comment = String;

    fn name(&self) -> &str {
        unimplemented!()
    }

    fn identifier(&self) -> &str {
        unimplemented!()
    }

    fn client_streaming(&self) -> bool {
        unimplemented!()
    }

    fn server_streaming(&self) -> bool {
        unimplemented!()
    }

    fn comment(&self) -> &[Self::Comment] {
        unimplemented!()
    }

    fn request_response_name(&self, proto_path: &str) -> (TokenStream, TokenStream) {
        unimplemented!()
    }
}

impl Service for DummyService {
    const CODEC_PATH: &'static str = "mon-json-codec";
    type Comment = String;
    type Method = DummyMethod;

    fn name(&self) -> &str {
        return "monservice";
    }

    fn package(&self) -> &str {
        return "monpackage";
    }

    fn identifier(&self) -> &str {
        return "id";
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
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    // let proto_path = protos.as_ref().to_str().unwrap();

    let service = DummyService{methods: Vec::new(), comments: Vec::new()};

    let it = protos.as_ref().into_iter();

    for proto in protos.as_ref().into_iter() {
        let proto_path = proto.as_ref().to_str().unwrap();
        let server = server::generate(&service, &proto_path);
        let client = client::generate(&service, &proto_path);

        let mut file = File::create(out_dir.join("mon-server-stp.rs"))?;
        file.write_all(server.to_string().as_bytes())?;

        let mut file = File::create(out_dir.join("mon-client-stp.rs"))?;
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
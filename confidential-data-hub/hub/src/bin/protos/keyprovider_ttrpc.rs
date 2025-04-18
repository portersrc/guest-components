// This file is generated by ttrpc-compiler 0.7.0. Do not edit
// @generated

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unknown_lints)]
#![allow(clipto_camel_casepy)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
#![allow(clippy::all)]
use protobuf::{CodedInputStream, CodedOutputStream, Message};
use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;

#[derive(Clone)]
pub struct KeyProviderServiceClient {
    client: ::ttrpc::r#async::Client,
}

impl KeyProviderServiceClient {
    pub fn new(client: ::ttrpc::r#async::Client) -> Self {
        KeyProviderServiceClient {
            client,
        }
    }

    pub async fn wrap_key(&self, ctx: ttrpc::context::Context, req: &super::keyprovider::KeyProviderKeyWrapProtocolInput) -> ::ttrpc::Result<super::keyprovider::KeyProviderKeyWrapProtocolOutput> {
        let mut cres = super::keyprovider::KeyProviderKeyWrapProtocolOutput::new();
        ::ttrpc::async_client_request!(self, ctx, req, "keyprovider.KeyProviderService", "WrapKey", cres);
    }

    pub async fn un_wrap_key(&self, ctx: ttrpc::context::Context, req: &super::keyprovider::KeyProviderKeyWrapProtocolInput) -> ::ttrpc::Result<super::keyprovider::KeyProviderKeyWrapProtocolOutput> {
        let mut cres = super::keyprovider::KeyProviderKeyWrapProtocolOutput::new();
        ::ttrpc::async_client_request!(self, ctx, req, "keyprovider.KeyProviderService", "UnWrapKey", cres);
    }
}

struct WrapKeyMethod {
    service: Arc<dyn KeyProviderService + Send + Sync>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for WrapKeyMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<::ttrpc::Response> {
        ::ttrpc::async_request_handler!(self, ctx, req, keyprovider, KeyProviderKeyWrapProtocolInput, wrap_key);
    }
}

struct UnWrapKeyMethod {
    service: Arc<dyn KeyProviderService + Send + Sync>,
}

#[async_trait]
impl ::ttrpc::r#async::MethodHandler for UnWrapKeyMethod {
    async fn handler(&self, ctx: ::ttrpc::r#async::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<::ttrpc::Response> {
        ::ttrpc::async_request_handler!(self, ctx, req, keyprovider, KeyProviderKeyWrapProtocolInput, un_wrap_key);
    }
}

#[async_trait]
pub trait KeyProviderService: Sync {
    async fn wrap_key(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _: super::keyprovider::KeyProviderKeyWrapProtocolInput) -> ::ttrpc::Result<super::keyprovider::KeyProviderKeyWrapProtocolOutput> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/keyprovider.KeyProviderService/WrapKey is not supported".to_string())))
    }
    async fn un_wrap_key(&self, _ctx: &::ttrpc::r#async::TtrpcContext, _: super::keyprovider::KeyProviderKeyWrapProtocolInput) -> ::ttrpc::Result<super::keyprovider::KeyProviderKeyWrapProtocolOutput> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/keyprovider.KeyProviderService/UnWrapKey is not supported".to_string())))
    }
}

pub fn create_key_provider_service(service: Arc<dyn KeyProviderService + Send + Sync>) -> HashMap<String, ::ttrpc::r#async::Service> {
    let mut ret = HashMap::new();
    let mut methods = HashMap::new();
    let streams = HashMap::new();

    methods.insert("WrapKey".to_string(),
                    Box::new(WrapKeyMethod{service: service.clone()}) as Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    methods.insert("UnWrapKey".to_string(),
                    Box::new(UnWrapKeyMethod{service: service.clone()}) as Box<dyn ::ttrpc::r#async::MethodHandler + Send + Sync>);

    ret.insert("keyprovider.KeyProviderService".to_string(), ::ttrpc::r#async::Service{ methods, streams });
    ret
}

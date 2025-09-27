use anyhow::Result;
use deno_permissions::{
  AllowRunDescriptor, AllowRunDescriptorParseResult, DenyRunDescriptor, EnvDescriptor,
  EnvDescriptorParseError, FfiDescriptor, ImportDescriptor, NetDescriptor, NetDescriptorParseError,
  PathDescriptor, PathQueryDescriptor, PathResolveError, PermissionDescriptorParser,
  ReadDescriptor, RunDescriptorParseError, RunQueryDescriptor, SpecialFilePathQueryDescriptor,
  SysDescriptor, SysDescriptorParseError, WriteDescriptor,
};
use std::{
  borrow::Cow,
  path::{Path},
};
use sys_traits;

#[derive(Debug, Clone)]
pub struct TestPermissionDescriptorParser;
impl TestPermissionDescriptorParser {
}
impl PermissionDescriptorParser for TestPermissionDescriptorParser {
  fn parse_read_descriptor(&self, text: &str) -> Result<ReadDescriptor, PathResolveError> {
    Ok(ReadDescriptor(PathDescriptor::new_known_absolute(
      Cow::from(Path::new(text)),
    )))
  }

  fn parse_write_descriptor(&self, text: &str) -> Result<WriteDescriptor, PathResolveError> {
    Ok(WriteDescriptor(PathDescriptor::new_known_absolute(
      Cow::from(Path::new(text)),
    )))
  }

  fn parse_net_descriptor(&self, text: &str) -> Result<NetDescriptor, NetDescriptorParseError> {
    NetDescriptor::parse_for_query(text)
  }

  fn parse_import_descriptor(
    &self,
    text: &str,
  ) -> Result<ImportDescriptor, NetDescriptorParseError> {
    ImportDescriptor::parse_for_list(text)
  }

  fn parse_env_descriptor(&self, text: &str) -> Result<EnvDescriptor, EnvDescriptorParseError> {
    Ok(EnvDescriptor::new(Cow::Borrowed(text)))
  }

  fn parse_sys_descriptor(&self, text: &str) -> Result<SysDescriptor, SysDescriptorParseError> {
    SysDescriptor::parse(text.to_string())
  }

  fn parse_allow_run_descriptor(
    &self,
    text: &str,
  ) -> Result<AllowRunDescriptorParseResult, RunDescriptorParseError> {
    Ok(AllowRunDescriptorParseResult::Descriptor(
      AllowRunDescriptor(PathDescriptor::new_known_absolute(Cow::from(Path::new(
        text,
      )))),
    ))
  }

  fn parse_deny_run_descriptor(&self, text: &str) -> Result<DenyRunDescriptor, PathResolveError> {
    if text.contains("/") {
      Ok(DenyRunDescriptor::Path(PathDescriptor::new_known_absolute(
        Cow::from(Path::new(text)),
      )))
    } else {
      Ok(DenyRunDescriptor::Name(text.to_string()))
    }
  }

  fn parse_ffi_descriptor(&self, text: &str) -> Result<FfiDescriptor, PathResolveError> {
    Ok(FfiDescriptor(PathDescriptor::new_known_absolute(
      Cow::from(Path::new(text)),
    )))
  }

  fn parse_path_query<'a>(
    &self,
    path: Cow<'a, Path>,
  ) -> Result<PathQueryDescriptor<'a>, PathResolveError> {
    Ok(PathQueryDescriptor::new_known_absolute(path))
  }

  fn parse_run_query<'a>(
    &self,
    requested: &'a str,
  ) -> Result<RunQueryDescriptor<'a>, RunDescriptorParseError> {
    RunQueryDescriptor::parse(requested, &sys_traits::impls::RealSys).map_err(Into::into)
  }

  fn parse_special_file_descriptor<'a>(
    &self,
    _path: PathQueryDescriptor<'a>,
  ) -> Result<SpecialFilePathQueryDescriptor<'a>, PathResolveError> {
    todo!()
  }

  fn parse_net_query(&self, _text: &str) -> Result<NetDescriptor, NetDescriptorParseError> {
    todo!()
  }
}
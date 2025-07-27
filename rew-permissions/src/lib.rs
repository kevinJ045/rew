use anyhow::Result;
use deno_permissions::{
  AllowRunDescriptor, AllowRunDescriptorParseResult, DenyRunDescriptor, EnvDescriptor,
  EnvDescriptorParseError, FfiDescriptor, ImportDescriptor, NetDescriptor, NetDescriptorParseError,
  PathQueryDescriptor, PathResolveError, PermissionDescriptorParser,
  ReadDescriptor, RunDescriptorParseError, RunQueryDescriptor, SysDescriptor,
  SysDescriptorParseError, WriteDescriptor,
};
use std::path::{PathBuf};

#[derive(Debug, Clone)]
pub struct TestPermissionDescriptorParser;
impl TestPermissionDescriptorParser {
  fn join_path_with_root(&self, path: &str) -> PathBuf {
    if path.starts_with("C:\\") {
      PathBuf::from(path)
    } else {
      PathBuf::from("/").join(path)
    }
  }
}
impl PermissionDescriptorParser for TestPermissionDescriptorParser {
  fn parse_read_descriptor(&self, text: &str) -> Result<ReadDescriptor, PathResolveError> {
    Ok(ReadDescriptor(self.join_path_with_root(text)))
  }
  fn parse_write_descriptor(&self, text: &str) -> Result<WriteDescriptor, PathResolveError> {
    Ok(WriteDescriptor(self.join_path_with_root(text)))
  }
  fn parse_net_descriptor(&self, text: &str) -> Result<NetDescriptor, NetDescriptorParseError> {
    NetDescriptor::parse(text)
  }
  fn parse_import_descriptor(
    &self,
    text: &str,
  ) -> Result<ImportDescriptor, NetDescriptorParseError> {
    ImportDescriptor::parse(text)
  }
  fn parse_env_descriptor(&self, text: &str) -> Result<EnvDescriptor, EnvDescriptorParseError> {
    Ok(EnvDescriptor::new(text))
  }
  fn parse_sys_descriptor(&self, text: &str) -> Result<SysDescriptor, SysDescriptorParseError> {
    SysDescriptor::parse(text.to_string())
  }
  fn parse_allow_run_descriptor(
    &self,
    text: &str,
  ) -> Result<AllowRunDescriptorParseResult, RunDescriptorParseError> {
    Ok(AllowRunDescriptorParseResult::Descriptor(
      AllowRunDescriptor(self.join_path_with_root(text)),
    ))
  }
  fn parse_deny_run_descriptor(&self, text: &str) -> Result<DenyRunDescriptor, PathResolveError> {
    if text.contains("/") {
      Ok(DenyRunDescriptor::Path(self.join_path_with_root(text)))
    } else {
      Ok(DenyRunDescriptor::Name(text.to_string()))
    }
  }
  fn parse_ffi_descriptor(&self, text: &str) -> Result<FfiDescriptor, PathResolveError> {
    Ok(FfiDescriptor(self.join_path_with_root(text)))
  }
  fn parse_path_query(&self, path: &str) -> Result<PathQueryDescriptor, PathResolveError> {
    Ok(PathQueryDescriptor {
      resolved: self.join_path_with_root(path),
      requested: path.to_string(),
    })
  }
  fn parse_run_query(
    &self,
    requested: &str,
  ) -> Result<RunQueryDescriptor, RunDescriptorParseError> {
    RunQueryDescriptor::parse(requested).map_err(Into::into)
  }
}

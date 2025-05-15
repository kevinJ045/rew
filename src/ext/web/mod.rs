use super::ExtensionTrait;
use deno_core::{extension, Extension};
use std::sync::Arc;

mod options;
pub use options::WebOptions;

mod permissions;
pub(crate) use permissions::PermissionsContainer;
pub use permissions::{
    AllowlistWebPermissions, DefaultWebPermissions, PermissionDeniedError, SystemsPermissionKind,
    WebPermissions,
};

extension!(
    init_web,
    esm_entry_point = "ext:init_web/init_web.js",
    esm = [ dir "src/ext/web", "init_web.js", "init_errors.js" ],
    options = {
        permissions: Arc<dyn WebPermissions>
    },
    state = |state, config| state.put(PermissionsContainer(config.permissions)),
);
impl ExtensionTrait<WebOptions> for init_web {
    fn init(options: WebOptions) -> Extension {
        init_web::init_ops_and_esm(options.permissions)
    }
}

impl ExtensionTrait<WebOptions> for deno_web::deno_web {
    fn init(options: WebOptions) -> Extension {
        deno_web::deno_web::init_ops_and_esm::<PermissionsContainer>(
            options.blob_store,
            options.base_url,
        )
    }
}

pub fn extensions(options: WebOptions, is_snapshot: bool) -> Vec<Extension> {
    vec![
        deno_web::deno_web::build(options.clone(), is_snapshot),
        init_web::build(options.clone(), is_snapshot),
    ]
}

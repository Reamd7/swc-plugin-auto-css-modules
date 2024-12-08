use swc_core::{
    ecma::ast::{Pass, Program},
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

#[plugin_transform]
pub fn auto_css_modules(mut program: Program, data: TransformPluginProgramMetadata) -> Program {
    let config = serde_json::from_str(
        &data
            .get_transform_plugin_config()
            .expect("failed to get plugin config for auto_css_modules"),
    )
    .expect("invalid packages");

    auto_css_modules::auto_css_modules(config).process(&mut program);
    program
    // program.fold_with(tranformer)
}

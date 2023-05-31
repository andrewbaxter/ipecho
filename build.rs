use std::{
    env,
    fs,
    path::PathBuf,
};
use serde_json::json;
use terrars::{
    primvec,
    BuildStack,
    BuildVariable,
};
use terrars_andrewbaxter_localrun::{
    BuildDataRun,
    BuildProviderLocalrun,
};
use terrars_hashicorp_aws::{
    BuildIamRole,
    BuildLambdaFunction,
    BuildProviderAws,
    BuildLambdaFunctionUrl,
    BuildCloudwatchLogGroup,
    BuildIamRoleInlinePolicyEl,
};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let root = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());
    let deploy_root = root.join("deploy");
    let tf_root = deploy_root.join("tf");
    fs::create_dir_all(&tf_root).unwrap();
    let stack = &mut BuildStack {}.build();

    // Input vars
    let aws_region = &BuildVariable { tf_id: "aws_region".into() }.build(stack);
    let aws_access_key = BuildVariable { tf_id: "aws_public_key".into() }.build(stack);
    let aws_secret_key = BuildVariable { tf_id: "aws_secret_key".into() }.build(stack).set_sensitive(true);

    // Auth
    BuildProviderLocalrun {}.build(stack);
    BuildProviderAws {}
        .build(stack)
        .set_region(aws_region)
        .set_access_key(&aws_access_key)
        .set_secret_key(&aws_secret_key);

    // Resources
    let rust =
        BuildDataRun {
            tf_id: "z22WPM6IT".into(),
            command: primvec!["cargo", "lambda", "build", "--arm64", "--release", "--output-format=zip"].into(),
        }
            .build(stack)
            .set_working_dir(root.to_str().unwrap())
            .set_outputs(primvec![root.join("target/lambda/ipecho/bootstrap.zip").to_str().unwrap()]);
    let name = "ipecho";
    BuildLambdaFunctionUrl {
        tf_id: "zK9KY4I8C".into(),
        authorization_type: "NONE".into(),
        function_name: BuildLambdaFunction {
            tf_id: "zJFE3CJPM".into(),
            function_name: "ipecho".into(),
            role: BuildIamRole {
                tf_id: "z0S0XLX58".into(),
                assume_role_policy: serde_json::to_string(&json!({
                    "Version": "2012-10-17",
                    "Statement":[{
                        "Action": "sts:AssumeRole",
                        "Effect": "Allow",
                        "Sid": "",
                        "Principal": {
                            "Service": "lambda.amazonaws.com",
                        }
                    }]
                })).unwrap().into(),
            }
                .build(stack)
                .set_inline_policy(
                    vec![
                        BuildIamRoleInlinePolicyEl {}
                            .build()
                            .set_name(format!("lambda-{}-allow-logging", name))
                            .set_policy(serde_json::to_string(&json!({
                                "Version": "2012-10-17",
                                "Statement":[{
                                    "Action":["logs:CreateLogStream", "logs:PutLogEvents"],
                                    "Effect": "Allow",
                                    "Resource": "arn:aws:logs:*:*:*",
                                }]
                            })).unwrap())
                    ],
                )
                .depends_on(
                    &BuildCloudwatchLogGroup { tf_id: "z00UHO0AG".into() }
                        .build(stack)
                        .set_name(format!("/aws/lambda/{}", name))
                        .set_retention_in_days(1f64),
                )
                .arn()
                .into(),
        }
            .build(stack)
            .set_filename(rust.outputs().get(0))
            .set_source_code_hash(stack.func("filebase64sha256").e(&rust.outputs().get(0)))
            .set_handler("bootstrap")
            .set_runtime("provided.al2")
            .set_architectures(primvec!["arm64"])
            .function_name()
            .into(),
    }.build(stack);

    // Save the stack file
    fs::write(tf_root.join("stack.tf.json"), stack.serialize(&tf_root.join("state.json")).unwrap()).unwrap();
}

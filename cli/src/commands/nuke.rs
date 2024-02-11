use assemblylift_tools::terraform;
use clap::ArgMatches;
use dialoguer::Confirm;

pub fn command(_matches: Option<&ArgMatches>) {
    if Confirm::new()
        .with_prompt(
            "Are you sure you want to destroy ALL provisioned infrastructure?\nThis is PERMANENT!",
        )
        .interact()
        .unwrap()
    {
        let tf = terraform::Terraform::default();
        tf.destroy();
    }
}

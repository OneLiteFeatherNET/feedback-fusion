#![no_main]
use arbitrary::{Arbitrary, Unstructured};
use feedback_fusion_common::proto::{CreateFieldRequest, CreatePromptRequest, CreateTargetRequest};
use lazy_static::lazy_static;
use libfuzzer_sys::fuzz_target;

#[derive(Debug)]
struct FuzzInput {
    targets: Vec<TargetWithPrompts>,
}

#[derive(Debug)]
struct TargetWithPrompts {
    target_request: CreateTargetRequest,
    prompts: Vec<PromptWithFields>,
}

#[derive(Debug)]
struct PromptWithFields {
    prompt_request: CreatePromptRequest,
    fields: Vec<CreateFieldRequest>,
}

impl Arbitrary<'_> for FuzzInput {
    fn arbitrary(u: &mut Unstructured) -> arbitrary::Result<Self> {
        let num_targets = u.int_in_range(0..=3)?;

        let targets = (0..num_targets)
            .map(|_| {
                let target_request: CreateTargetRequest = u.arbitrary()?;

                let num_prompts = u.int_in_range(0..=3)?;

                let prompts = (0..num_prompts)
                    .map(|_| {
                        let prompt_request: CreatePromptRequest = u.arbitrary()?;

                        let num_fields = u.int_in_range(0..=4)?;

                        let fields = (0..num_fields)
                            .map(|_| u.arbitrary::<CreateFieldRequest>())
                            .collect::<Result<Vec<_>, _>>()?;

                        Ok(PromptWithFields {
                            prompt_request,
                            fields,
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(TargetWithPrompts {
                    target_request,
                    prompts,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(FuzzInput { targets })
    }
}

lazy_static! {
    static ref RUNTIME: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();
}

fuzz_target!(|data: &[u8]| {
    let mut u = Unstructured::new(data);

    if let Ok(fuzz_input) = FuzzInput::arbitrary(&mut u) {
        RUNTIME.block_on(async {
            let (mut client, _) = feedback_fusion_common::connect!();

            for target in fuzz_input.targets {
                match client.create_target(target.target_request.clone()).await {
                    Ok(target_response) => {
                        let target_id = target_response.into_inner().id;

                        for prompt in target.prompts {
                            let mut prompt_request = prompt.prompt_request.clone();
                            prompt_request.target = target_id.clone();

                            match client.create_prompt(prompt_request.clone()).await {
                                Ok(prompt_response) => {
                                    let prompt_id = prompt_response.into_inner().id;

                                    for mut field in prompt.fields {
                                        field.prompt = prompt_id.clone();

                                        if let Err(e) = client.create_field(field.clone()).await {
                                            handle_grpc_error(e);
                                        }
                                    }
                                }
                                Err(e) => handle_grpc_error(e),
                            }
                        }
                    }
                    Err(e) => handle_grpc_error(e),
                }
            }
        });
    }
});

fn handle_grpc_error(e: tonic::Status) {
    match e.code() {
        tonic::Code::Internal => panic!("Internal error: {}", e.message()),
        tonic::Code::PermissionDenied => panic!("Permission denied: {}", e.message()),
        tonic::Code::Unauthenticated => panic!("Unauthenticated: {}", e.message()),
        _ => {}
    }
}

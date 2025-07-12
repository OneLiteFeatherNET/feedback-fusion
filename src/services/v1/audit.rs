//SPDX-FileCopyrightText: 2025 OneLiteFeatherNet
//SPDX-License-Identifier: MIT

//MIT License

// Copyright (c) 2025 OneLiteFeatherNet

//Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
//associated documentation files (the "Software"), to deal in the Software without restriction,
//including without limitation the rights to use, copy, modify, merge, publish, distribute,
//sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
//furnished to do so, subject to the following conditions:

//The above copyright notice and this permission notice (including the next paragraph) shall be
//included in all copies or substantial portions of the Software.

//THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
//NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
//NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
//DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use std::borrow::Cow;

use feedback_fusion_common::{
    common::ProtoResourceKind,
    database::schema::audit::AuditVersion,
    proto::{
        AuditVersionPage, GetAuditVersionsRequest, ProtoAuditVersion, ProtoResource,
        RollbackResourceRequest, proto_resource::Inner,
    },
};
use prost::Message;

use crate::{
    database::schema::{
        authorization::ResourceKind,
        feedback::{Field, Prompt, Target},
        user::UserContext,
    },
    prelude::{v1::FeedbackFusionV1Context, *},
};

#[instrument(skip_all)]
pub async fn get_audit_versions(
    context: &FeedbackFusionV1Context<'_>,
    request: Request<GetAuditVersionsRequest>,
    user_context: UserContext,
) -> Result<Response<AuditVersionPage>> {
    let connection = context.connection();
    let data = request.into_inner();
    let page_request = data.page_request();

    let resource_kind =
        ResourceKind::from(&ProtoResourceKind::try_from(data.resource_type).unwrap());

    // check wether the user is actually authorized to read the specified resource
    if user_context
        .authorize(
            connection,
            &EndpointScopeSelector::Specific(Cow::Borrowed(data.resource_id.as_str()))
                .with_resource_kind(&resource_kind),
            &Permission::Read,
        )
        .await
        .is_ok()
    {
        // now we know for sure that the user is authorized and therefore we now can select the
        // page
        let audit_versions = database_request!(
            AuditVersion::select_page_by_resource_type_and_resource_id(
                connection,
                &page_request,
                &resource_kind,
                data.resource_id.as_str()
            )
            .await,
            "Select page of audit versions"
        )?;

        Ok(Response::new(AuditVersionPage {
            page_token: page_request.page_no().try_into()?,
            next_page_token: TryInto::<i32>::try_into(page_request.page_no())? + 1i32,
            page_size: page_request.page_size().try_into()?,
            total: audit_versions.total.try_into()?,
            audit_versions: audit_versions
                .records
                .into_iter()
                .filter_map(|version| version.try_into().ok())
                .collect::<Vec<ProtoAuditVersion>>(),
        }))
    } else {
        Err(FeedbackFusionError::Unauthorized)
    }
}

#[instrument(skip_all)]
pub async fn rollback_resource(
    context: &FeedbackFusionV1Context<'_>,
    request: Request<RollbackResourceRequest>,
    user_context: UserContext,
) -> Result<Response<()>> {
    let connection = context.connection();
    let data = request.into_inner();

    let resource_kind =
        ResourceKind::from(&ProtoResourceKind::try_from(data.resource_type).unwrap());

    // check wether the user is actually authorized to read the specified resource
    if user_context
        .authorize(
            connection,
            &EndpointScopeSelector::Specific(Cow::Borrowed(data.resource_id.as_str()))
                .with_resource_kind(&resource_kind),
            &Permission::Write,
        )
        .await
        .is_ok()
    {
        // try to fetch the rollback version
        let version: Vec<AuditVersion> = database_request!(
            AuditVersion::select_by_map(connection, value!{ "resource_id": data.resource_id, "resource_type": data.resource_type, "version": data.version }).await,
            "Fetch the rollback data"
        )?;

        if let Some(version) = version.into_iter().next() {
            // decode the resource
            let resource = ProtoResource::decode(version.data.into_inner().as_slice())?;

            if let Some(inner) = resource.inner {
                match inner {
                    Inner::Unknown(()) => Err(FeedbackFusionError::BadRequest(
                        "Can't rollback to a delete version".to_owned(),
                    )),
                    Inner::Target(proto_target) => {
                        let target = Target::from(proto_target);
                        database_request!(
                            Target::update_by_map(connection, &target, value! {"id": target.id()})
                                .await,
                            "Rollback target"
                        )?;

                        Ok(Response::new(()))
                    }
                    Inner::Prompt(proto_prompt) => {
                        let prompt = Prompt::from(proto_prompt);
                        database_request!(
                            Prompt::update_by_map(connection, &prompt, value! {"id": prompt.id()})
                                .await,
                            "Rollback prompt"
                        )?;

                        Ok(Response::new(()))
                    }
                    Inner::Field(proto_field) => {
                        let field = TryInto::<Field>::try_into(proto_field)?;
                        database_request!(
                            Field::update_by_map(connection, &field, value! {"id": field.id()})
                                .await,
                            "Rollback field"
                        )?;

                        Ok(Response::new(()))
                    }
                }
            } else {
                Err(FeedbackFusionError::BadRequest(
                    "Missing inner resource data".to_owned(),
                ))
            }
        } else {
            Err(FeedbackFusionError::BadRequest(
                "Rollback version not found".to_owned(),
            ))
        }
    } else {
        Err(FeedbackFusionError::Unauthorized)
    }
}

/*
 * Copyright (c) 2023 OneLiteFeatherNET
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */

use feedback_fusion_common::proto::{ProtoResource, proto_resource::Inner};

use crate::{
    database::schema::feedback::{Field, Prompt, Target},
    error::FeedbackFusionError,
};

pub mod schema;

#[derive(Debug)]
pub enum Resource {
    Target(Target),
    Prompt(Prompt),
    Field(Field),
}

impl TryInto<Resource> for ProtoResource {
    type Error = FeedbackFusionError;

    fn try_into(self) -> Result<Resource, Self::Error> {
        if let Some(inner) = self.inner {
            match inner {
                Inner::Target(target) => Ok(Resource::Target(Target::from(target))),
                Inner::Prompt(prompt) => Ok(Resource::Prompt(Prompt::from(prompt))),
                Inner::Field(field) => Ok(Resource::Field(field.try_into()?)),
                unknown => Err(FeedbackFusionError::BadRequest(format!(
                    "Got unknown inner resource {unknown:?} while parsing resource"
                ))),
            }
        } else {
            Err(FeedbackFusionError::BadRequest(
                "Missing inner resource data".to_owned(),
            ))
        }
    }
}

macro_rules! proto_resource_from_type {
    ($type:path, $variant:ident) => {
        impl From<$type> for ProtoResource {
            fn from(value: $type) -> Self {
                Self {
                    inner: Some(Inner::$variant(value.into())),
                }
            }
        }
    };
}

proto_resource_from_type!(Target, Target);
proto_resource_from_type!(Prompt, Prompt);
proto_resource_from_type!(Field, Field);

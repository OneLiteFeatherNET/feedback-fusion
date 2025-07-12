# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [common/resource.proto](#common_resource-proto)
    - [ProtoResourceKind](#common-ProtoResourceKind)
  
- [feedback-fusion-event-v1/event/batch.proto](#feedback-fusion-event-v1_event_batch-proto)
    - [ProtoEvent](#feedback_fusion_event_v1-ProtoEvent)
    - [ProtoEventBatch](#feedback_fusion_event_v1-ProtoEventBatch)
  
    - [ProtoEventType](#feedback_fusion_event_v1-ProtoEventType)
  
- [feedback-fusion-event-v1/event/resource_modified.proto](#feedback-fusion-event-v1_event_resource_modified-proto)
    - [ProtoResourceModifiedEvent](#feedback_fusion_event_v1-ProtoResourceModifiedEvent)
  
    - [ProtoResourceModificationOperation](#feedback_fusion_event_v1-ProtoResourceModificationOperation)
  
- [feedback-fusion-event-v1/service.proto](#feedback-fusion-event-v1_service-proto)
    - [FeedbackFusionIndexerV1](#feedback_fusion_event_v1-FeedbackFusionIndexerV1)
  
- [feedback-fusion-v1/audit.proto](#feedback-fusion-v1_audit-proto)
    - [AuditVersionPage](#feedback_fusion_v1-AuditVersionPage)
    - [GetAuditVersionsRequest](#feedback_fusion_v1-GetAuditVersionsRequest)
    - [ProtoAuditVersion](#feedback_fusion_v1-ProtoAuditVersion)
    - [RollbackResourceRequest](#feedback_fusion_v1-RollbackResourceRequest)
  
    - [ProtoAuditAction](#feedback_fusion_v1-ProtoAuditAction)
  
- [feedback-fusion-v1/export.proto](#feedback-fusion-v1_export-proto)
    - [DataExportRequest](#feedback_fusion_v1-DataExportRequest)
    - [DataExportResponse](#feedback_fusion_v1-DataExportResponse)
  
- [feedback-fusion-v1/authorization.proto](#feedback-fusion-v1_authorization-proto)
    - [CreateResourceAuthorizationRequest](#feedback_fusion_v1-CreateResourceAuthorizationRequest)
    - [DeleteResourceAuthorizationRequest](#feedback_fusion_v1-DeleteResourceAuthorizationRequest)
    - [ExportResourceAuthorizationsRequest](#feedback_fusion_v1-ExportResourceAuthorizationsRequest)
    - [GetResourceAuthorizationRequest](#feedback_fusion_v1-GetResourceAuthorizationRequest)
    - [GetResourceAuthorizationsRequest](#feedback_fusion_v1-GetResourceAuthorizationsRequest)
    - [ProtoResourceAuthorization](#feedback_fusion_v1-ProtoResourceAuthorization)
    - [ProtoResourceAuthorizationData](#feedback_fusion_v1-ProtoResourceAuthorizationData)
    - [ResourceAuthorizationExportResponse](#feedback_fusion_v1-ResourceAuthorizationExportResponse)
    - [ResourceAuthorizationList](#feedback_fusion_v1-ResourceAuthorizationList)
    - [ResourceAuthorizationPage](#feedback_fusion_v1-ResourceAuthorizationPage)
    - [UpdateResourceAuthorizationRequest](#feedback_fusion_v1-UpdateResourceAuthorizationRequest)
  
    - [ProtoAuthorizationGrant](#feedback_fusion_v1-ProtoAuthorizationGrant)
    - [ProtoAuthorizationType](#feedback_fusion_v1-ProtoAuthorizationType)
  
- [feedback-fusion-v1/target.proto](#feedback-fusion-v1_target-proto)
    - [CreateTargetRequest](#feedback_fusion_v1-CreateTargetRequest)
    - [DeleteTargetRequest](#feedback_fusion_v1-DeleteTargetRequest)
    - [GetTargetRequest](#feedback_fusion_v1-GetTargetRequest)
    - [GetTargetsRequest](#feedback_fusion_v1-GetTargetsRequest)
    - [ProtoTarget](#feedback_fusion_v1-ProtoTarget)
    - [TargetPage](#feedback_fusion_v1-TargetPage)
    - [UpdateTargetRequest](#feedback_fusion_v1-UpdateTargetRequest)
  
- [feedback-fusion-v1/response.proto](#feedback-fusion-v1_response-proto)
    - [CheckboxResponse](#feedback_fusion_v1-CheckboxResponse)
    - [CreateResponsesRequest](#feedback_fusion_v1-CreateResponsesRequest)
    - [CreateResponsesRequest.DataEntry](#feedback_fusion_v1-CreateResponsesRequest-DataEntry)
    - [FieldResponseList](#feedback_fusion_v1-FieldResponseList)
    - [GetResponsesRequest](#feedback_fusion_v1-GetResponsesRequest)
    - [NumberResponse](#feedback_fusion_v1-NumberResponse)
    - [ProtoFieldResponse](#feedback_fusion_v1-ProtoFieldResponse)
    - [ProtoPromptResponse](#feedback_fusion_v1-ProtoPromptResponse)
    - [RangeResponse](#feedback_fusion_v1-RangeResponse)
    - [RatingResponse](#feedback_fusion_v1-RatingResponse)
    - [ResponseData](#feedback_fusion_v1-ResponseData)
    - [ResponsePage](#feedback_fusion_v1-ResponsePage)
    - [ResponsePage.DataEntry](#feedback_fusion_v1-ResponsePage-DataEntry)
    - [SelectionResponse](#feedback_fusion_v1-SelectionResponse)
    - [TextResponse](#feedback_fusion_v1-TextResponse)
    - [UserInfoResponse](#feedback_fusion_v1-UserInfoResponse)
    - [UserInfoResponse.PermissionsEntry](#feedback_fusion_v1-UserInfoResponse-PermissionsEntry)
  
- [feedback-fusion-v1/service.proto](#feedback-fusion-v1_service-proto)
    - [FeedbackFusionV1](#feedback_fusion_v1-FeedbackFusionV1)
    - [PublicFeedbackFusionV1](#feedback_fusion_v1-PublicFeedbackFusionV1)
  
- [feedback-fusion-v1/field.proto](#feedback-fusion-v1_field-proto)
    - [CreateFieldRequest](#feedback_fusion_v1-CreateFieldRequest)
    - [DeleteFieldRequest](#feedback_fusion_v1-DeleteFieldRequest)
    - [FieldPage](#feedback_fusion_v1-FieldPage)
    - [GetFieldsRequest](#feedback_fusion_v1-GetFieldsRequest)
    - [ProtoCheckboxOptions](#feedback_fusion_v1-ProtoCheckboxOptions)
    - [ProtoField](#feedback_fusion_v1-ProtoField)
    - [ProtoFieldOptions](#feedback_fusion_v1-ProtoFieldOptions)
    - [ProtoNumberOptions](#feedback_fusion_v1-ProtoNumberOptions)
    - [ProtoRangeOptions](#feedback_fusion_v1-ProtoRangeOptions)
    - [ProtoRatingOptions](#feedback_fusion_v1-ProtoRatingOptions)
    - [ProtoSelectionOptions](#feedback_fusion_v1-ProtoSelectionOptions)
    - [ProtoTextOptions](#feedback_fusion_v1-ProtoTextOptions)
    - [UpdateFieldRequest](#feedback_fusion_v1-UpdateFieldRequest)
  
    - [ProtoCheckboxStyle](#feedback_fusion_v1-ProtoCheckboxStyle)
    - [ProtoFieldType](#feedback_fusion_v1-ProtoFieldType)
  
- [feedback-fusion-v1/prompt.proto](#feedback-fusion-v1_prompt-proto)
    - [CreatePromptRequest](#feedback_fusion_v1-CreatePromptRequest)
    - [DeletePromptRequest](#feedback_fusion_v1-DeletePromptRequest)
    - [GetPromptRequest](#feedback_fusion_v1-GetPromptRequest)
    - [GetPromptsRequest](#feedback_fusion_v1-GetPromptsRequest)
    - [PromptPage](#feedback_fusion_v1-PromptPage)
    - [ProtoPrompt](#feedback_fusion_v1-ProtoPrompt)
    - [UpdatePromptRequest](#feedback_fusion_v1-UpdatePromptRequest)
  
- [feedback-fusion-v1/resource.proto](#feedback-fusion-v1_resource-proto)
    - [ProtoResource](#feedback_fusion_v1-ProtoResource)
  
- [Scalar Value Types](#scalar-value-types)



<a name="common_resource-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## common/resource.proto


 


<a name="common-ProtoResourceKind"></a>

### ProtoResourceKind


| Name | Number | Description |
| ---- | ------ | ----------- |
| UNKNOWN | 0 |  |
| TARGET | 1 |  |
| PROMPT | 2 |  |
| FIELD | 3 |  |
| EXPORT | 4 |  |
| AUTHORIZE | 5 |  |
| RESPONSE | 6 |  |


 

 

 



<a name="feedback-fusion-event-v1_event_batch-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## feedback-fusion-event-v1/event/batch.proto



<a name="feedback_fusion_event_v1-ProtoEvent"></a>

### ProtoEvent



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  |  |
| event_type | [ProtoEventType](#feedback_fusion_event_v1-ProtoEventType) |  |  |
| resource_modified_event | [ProtoResourceModifiedEvent](#feedback_fusion_event_v1-ProtoResourceModifiedEvent) |  |  |






<a name="feedback_fusion_event_v1-ProtoEventBatch"></a>

### ProtoEventBatch



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| events | [ProtoEvent](#feedback_fusion_event_v1-ProtoEvent) | repeated |  |





 


<a name="feedback_fusion_event_v1-ProtoEventType"></a>

### ProtoEventType


| Name | Number | Description |
| ---- | ------ | ----------- |
| UNKNOWN | 0 |  |
| RESOURCE_MODIFIED | 1 |  |


 

 

 



<a name="feedback-fusion-event-v1_event_resource_modified-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## feedback-fusion-event-v1/event/resource_modified.proto



<a name="feedback_fusion_event_v1-ProtoResourceModifiedEvent"></a>

### ProtoResourceModifiedEvent



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| operation | [ProtoResourceModificationOperation](#feedback_fusion_event_v1-ProtoResourceModificationOperation) |  |  |
| id | [string](#string) |  |  |
| resource_kind | [common.ProtoResourceKind](#common-ProtoResourceKind) |  |  |
| data | [bytes](#bytes) |  |  |
| made_by | [string](#string) |  |  |





 


<a name="feedback_fusion_event_v1-ProtoResourceModificationOperation"></a>

### ProtoResourceModificationOperation


| Name | Number | Description |
| ---- | ------ | ----------- |
| CREATE | 0 |  |
| UPDATE | 1 |  |
| DELETE | 2 |  |


 

 

 



<a name="feedback-fusion-event-v1_service-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## feedback-fusion-event-v1/service.proto


 

 

 


<a name="feedback_fusion_event_v1-FeedbackFusionIndexerV1"></a>

### FeedbackFusionIndexerV1


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| SendBatch | [ProtoEventBatch](#feedback_fusion_event_v1-ProtoEventBatch) | [.google.protobuf.Empty](#google-protobuf-Empty) |  |

 



<a name="feedback-fusion-v1_audit-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## feedback-fusion-v1/audit.proto



<a name="feedback_fusion_v1-AuditVersionPage"></a>

### AuditVersionPage



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| page_token | [int32](#int32) |  |  |
| next_page_token | [int32](#int32) |  |  |
| page_size | [int32](#int32) |  |  |
| total | [int32](#int32) |  |  |
| audit_versions | [ProtoAuditVersion](#feedback_fusion_v1-ProtoAuditVersion) | repeated |  |






<a name="feedback_fusion_v1-GetAuditVersionsRequest"></a>

### GetAuditVersionsRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| page_token | [int32](#int32) |  |  |
| page_size | [int32](#int32) |  |  |
| resource_id | [string](#string) |  |  |
| resource_type | [common.ProtoResourceKind](#common-ProtoResourceKind) |  |  |






<a name="feedback_fusion_v1-ProtoAuditVersion"></a>

### ProtoAuditVersion



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |
| resource_id | [string](#string) |  |  |
| resource_type | [common.ProtoResourceKind](#common-ProtoResourceKind) |  |  |
| action | [ProtoAuditAction](#feedback_fusion_v1-ProtoAuditAction) |  |  |
| data | [ProtoResource](#feedback_fusion_v1-ProtoResource) |  |  |
| version | [int32](#int32) |  |  |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  |  |
| made_by | [string](#string) |  |  |






<a name="feedback_fusion_v1-RollbackResourceRequest"></a>

### RollbackResourceRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| resource_id | [string](#string) |  |  |
| resource_type | [common.ProtoResourceKind](#common-ProtoResourceKind) |  |  |
| version | [int32](#int32) |  |  |





 


<a name="feedback_fusion_v1-ProtoAuditAction"></a>

### ProtoAuditAction


| Name | Number | Description |
| ---- | ------ | ----------- |
| UNKNOWN | 0 |  |
| CREATE | 1 |  |
| UPDATE | 2 |  |
| DELETE | 3 |  |


 

 

 



<a name="feedback-fusion-v1_export-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## feedback-fusion-v1/export.proto



<a name="feedback_fusion_v1-DataExportRequest"></a>

### DataExportRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| targets | [string](#string) | repeated |  |






<a name="feedback_fusion_v1-DataExportResponse"></a>

### DataExportResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| export | [string](#string) |  |  |





 

 

 

 



<a name="feedback-fusion-v1_authorization-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## feedback-fusion-v1/authorization.proto



<a name="feedback_fusion_v1-CreateResourceAuthorizationRequest"></a>

### CreateResourceAuthorizationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| resource_kind | [common.ProtoResourceKind](#common-ProtoResourceKind) |  |  |
| resource_id | [string](#string) | repeated |  |
| authorization_data | [ProtoResourceAuthorizationData](#feedback_fusion_v1-ProtoResourceAuthorizationData) |  |  |






<a name="feedback_fusion_v1-DeleteResourceAuthorizationRequest"></a>

### DeleteResourceAuthorizationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |






<a name="feedback_fusion_v1-ExportResourceAuthorizationsRequest"></a>

### ExportResourceAuthorizationsRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| ids | [string](#string) | repeated |  |






<a name="feedback_fusion_v1-GetResourceAuthorizationRequest"></a>

### GetResourceAuthorizationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |






<a name="feedback_fusion_v1-GetResourceAuthorizationsRequest"></a>

### GetResourceAuthorizationsRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| page_token | [int32](#int32) |  |  |
| page_size | [int32](#int32) |  |  |
| value | [string](#string) | optional |  |
| authorization_type | [ProtoAuthorizationType](#feedback_fusion_v1-ProtoAuthorizationType) | optional |  |






<a name="feedback_fusion_v1-ProtoResourceAuthorization"></a>

### ProtoResourceAuthorization



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |
| resource_kind | [common.ProtoResourceKind](#common-ProtoResourceKind) |  |  |
| resource_id | [string](#string) | optional |  |
| authorization_type | [ProtoAuthorizationType](#feedback_fusion_v1-ProtoAuthorizationType) |  |  |
| authorization_grant | [ProtoAuthorizationGrant](#feedback_fusion_v1-ProtoAuthorizationGrant) |  |  |
| value | [string](#string) |  |  |
| updated_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  |  |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  |  |






<a name="feedback_fusion_v1-ProtoResourceAuthorizationData"></a>

### ProtoResourceAuthorizationData



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| type | [ProtoAuthorizationType](#feedback_fusion_v1-ProtoAuthorizationType) |  |  |
| grant | [ProtoAuthorizationGrant](#feedback_fusion_v1-ProtoAuthorizationGrant) | repeated |  |
| values | [string](#string) | repeated |  |






<a name="feedback_fusion_v1-ResourceAuthorizationExportResponse"></a>

### ResourceAuthorizationExportResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| export | [string](#string) |  |  |






<a name="feedback_fusion_v1-ResourceAuthorizationList"></a>

### ResourceAuthorizationList



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| authorizations | [ProtoResourceAuthorization](#feedback_fusion_v1-ProtoResourceAuthorization) | repeated |  |






<a name="feedback_fusion_v1-ResourceAuthorizationPage"></a>

### ResourceAuthorizationPage



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| page_token | [int32](#int32) |  |  |
| next_page_token | [int32](#int32) |  |  |
| page_size | [int32](#int32) |  |  |
| total | [int32](#int32) |  |  |
| authorizations | [ProtoResourceAuthorization](#feedback_fusion_v1-ProtoResourceAuthorization) | repeated |  |






<a name="feedback_fusion_v1-UpdateResourceAuthorizationRequest"></a>

### UpdateResourceAuthorizationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |
| resource_id | [string](#string) | optional |  |
| authorization_type | [ProtoAuthorizationType](#feedback_fusion_v1-ProtoAuthorizationType) | optional |  |
| authorization_grant | [ProtoAuthorizationGrant](#feedback_fusion_v1-ProtoAuthorizationGrant) | optional |  |
| value | [string](#string) | optional |  |





 


<a name="feedback_fusion_v1-ProtoAuthorizationGrant"></a>

### ProtoAuthorizationGrant


| Name | Number | Description |
| ---- | ------ | ----------- |
| WRITE | 0 |  |
| READ | 1 |  |
| List | 2 |  |
| All | 3 |  |



<a name="feedback_fusion_v1-ProtoAuthorizationType"></a>

### ProtoAuthorizationType


| Name | Number | Description |
| ---- | ------ | ----------- |
| TYPE_SCOPE | 0 |  |
| TYPE_GROUP | 1 |  |
| TYPE_SUBJECT | 2 |  |


 

 

 



<a name="feedback-fusion-v1_target-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## feedback-fusion-v1/target.proto



<a name="feedback_fusion_v1-CreateTargetRequest"></a>

### CreateTargetRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |
| description | [string](#string) | optional |  |






<a name="feedback_fusion_v1-DeleteTargetRequest"></a>

### DeleteTargetRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |






<a name="feedback_fusion_v1-GetTargetRequest"></a>

### GetTargetRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |






<a name="feedback_fusion_v1-GetTargetsRequest"></a>

### GetTargetsRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| page_token | [int32](#int32) |  |  |
| page_size | [int32](#int32) |  |  |
| query | [string](#string) |  |  |






<a name="feedback_fusion_v1-ProtoTarget"></a>

### ProtoTarget



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |
| name | [string](#string) |  |  |
| description | [string](#string) | optional |  |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  |  |
| updated_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  |  |






<a name="feedback_fusion_v1-TargetPage"></a>

### TargetPage



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| page_token | [int32](#int32) |  |  |
| next_page_token | [int32](#int32) |  |  |
| page_size | [int32](#int32) |  |  |
| total | [int32](#int32) |  |  |
| targets | [ProtoTarget](#feedback_fusion_v1-ProtoTarget) | repeated |  |






<a name="feedback_fusion_v1-UpdateTargetRequest"></a>

### UpdateTargetRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |
| name | [string](#string) | optional |  |
| description | [string](#string) | optional |  |





 

 

 

 



<a name="feedback-fusion-v1_response-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## feedback-fusion-v1/response.proto



<a name="feedback_fusion_v1-CheckboxResponse"></a>

### CheckboxResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| checked | [bool](#bool) |  |  |






<a name="feedback_fusion_v1-CreateResponsesRequest"></a>

### CreateResponsesRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| data | [CreateResponsesRequest.DataEntry](#feedback_fusion_v1-CreateResponsesRequest-DataEntry) | repeated |  |
| prompt | [string](#string) |  |  |






<a name="feedback_fusion_v1-CreateResponsesRequest-DataEntry"></a>

### CreateResponsesRequest.DataEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [ResponseData](#feedback_fusion_v1-ResponseData) |  |  |






<a name="feedback_fusion_v1-FieldResponseList"></a>

### FieldResponseList



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| data | [ProtoFieldResponse](#feedback_fusion_v1-ProtoFieldResponse) | repeated |  |






<a name="feedback_fusion_v1-GetResponsesRequest"></a>

### GetResponsesRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| page_token | [int32](#int32) |  |  |
| page_size | [int32](#int32) |  |  |
| prompt | [string](#string) |  |  |






<a name="feedback_fusion_v1-NumberResponse"></a>

### NumberResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| number | [int32](#int32) |  |  |






<a name="feedback_fusion_v1-ProtoFieldResponse"></a>

### ProtoFieldResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |
| response | [string](#string) |  |  |
| field | [string](#string) |  |  |
| data | [ResponseData](#feedback_fusion_v1-ResponseData) |  |  |






<a name="feedback_fusion_v1-ProtoPromptResponse"></a>

### ProtoPromptResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |
| prompt | [string](#string) |  |  |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  |  |






<a name="feedback_fusion_v1-RangeResponse"></a>

### RangeResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| start | [int32](#int32) |  |  |
| end | [int32](#int32) |  |  |






<a name="feedback_fusion_v1-RatingResponse"></a>

### RatingResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| rating | [int32](#int32) |  |  |






<a name="feedback_fusion_v1-ResponseData"></a>

### ResponseData



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| text | [TextResponse](#feedback_fusion_v1-TextResponse) |  |  |
| rating | [RatingResponse](#feedback_fusion_v1-RatingResponse) |  |  |
| checkbox | [CheckboxResponse](#feedback_fusion_v1-CheckboxResponse) |  |  |
| selection | [SelectionResponse](#feedback_fusion_v1-SelectionResponse) |  |  |
| range | [RangeResponse](#feedback_fusion_v1-RangeResponse) |  |  |
| number | [NumberResponse](#feedback_fusion_v1-NumberResponse) |  |  |






<a name="feedback_fusion_v1-ResponsePage"></a>

### ResponsePage



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| page_token | [int32](#int32) |  |  |
| next_page_token | [int32](#int32) |  |  |
| page_size | [int32](#int32) |  |  |
| total | [int32](#int32) |  |  |
| data | [ResponsePage.DataEntry](#feedback_fusion_v1-ResponsePage-DataEntry) | repeated |  |






<a name="feedback_fusion_v1-ResponsePage-DataEntry"></a>

### ResponsePage.DataEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [FieldResponseList](#feedback_fusion_v1-FieldResponseList) |  |  |






<a name="feedback_fusion_v1-SelectionResponse"></a>

### SelectionResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| values | [string](#string) | repeated |  |






<a name="feedback_fusion_v1-TextResponse"></a>

### TextResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| text | [string](#string) |  |  |






<a name="feedback_fusion_v1-UserInfoResponse"></a>

### UserInfoResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| permissions | [UserInfoResponse.PermissionsEntry](#feedback_fusion_v1-UserInfoResponse-PermissionsEntry) | repeated |  |






<a name="feedback_fusion_v1-UserInfoResponse-PermissionsEntry"></a>

### UserInfoResponse.PermissionsEntry



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| key | [string](#string) |  |  |
| value | [bool](#bool) |  |  |





 

 

 

 



<a name="feedback-fusion-v1_service-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## feedback-fusion-v1/service.proto


 

 

 


<a name="feedback_fusion_v1-FeedbackFusionV1"></a>

### FeedbackFusionV1
uses oidc authentication

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| CreateTarget | [CreateTargetRequest](#feedback_fusion_v1-CreateTargetRequest) | [ProtoTarget](#feedback_fusion_v1-ProtoTarget) |  |
| GetTarget | [GetTargetRequest](#feedback_fusion_v1-GetTargetRequest) | [ProtoTarget](#feedback_fusion_v1-ProtoTarget) |  |
| GetTargets | [GetTargetsRequest](#feedback_fusion_v1-GetTargetsRequest) | [TargetPage](#feedback_fusion_v1-TargetPage) |  |
| UpdateTarget | [UpdateTargetRequest](#feedback_fusion_v1-UpdateTargetRequest) | [ProtoTarget](#feedback_fusion_v1-ProtoTarget) |  |
| DeleteTarget | [DeleteTargetRequest](#feedback_fusion_v1-DeleteTargetRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) |  |
| CreatePrompt | [CreatePromptRequest](#feedback_fusion_v1-CreatePromptRequest) | [ProtoPrompt](#feedback_fusion_v1-ProtoPrompt) |  |
| GetPrompts | [GetPromptsRequest](#feedback_fusion_v1-GetPromptsRequest) | [PromptPage](#feedback_fusion_v1-PromptPage) |  |
| UpdatePrompt | [UpdatePromptRequest](#feedback_fusion_v1-UpdatePromptRequest) | [ProtoPrompt](#feedback_fusion_v1-ProtoPrompt) |  |
| DeletePrompt | [DeletePromptRequest](#feedback_fusion_v1-DeletePromptRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) |  |
| CreateField | [CreateFieldRequest](#feedback_fusion_v1-CreateFieldRequest) | [ProtoField](#feedback_fusion_v1-ProtoField) |  |
| GetFields | [GetFieldsRequest](#feedback_fusion_v1-GetFieldsRequest) | [FieldPage](#feedback_fusion_v1-FieldPage) |  |
| UpdateField | [UpdateFieldRequest](#feedback_fusion_v1-UpdateFieldRequest) | [ProtoField](#feedback_fusion_v1-ProtoField) |  |
| DeleteField | [DeleteFieldRequest](#feedback_fusion_v1-DeleteFieldRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) |  |
| GetResponses | [GetResponsesRequest](#feedback_fusion_v1-GetResponsesRequest) | [ResponsePage](#feedback_fusion_v1-ResponsePage) |  |
| GetUserInfo | [.google.protobuf.Empty](#google-protobuf-Empty) | [UserInfoResponse](#feedback_fusion_v1-UserInfoResponse) |  |
| ExportData | [DataExportRequest](#feedback_fusion_v1-DataExportRequest) | [DataExportResponse](#feedback_fusion_v1-DataExportResponse) |  |
| CreateResourceAuthorization | [CreateResourceAuthorizationRequest](#feedback_fusion_v1-CreateResourceAuthorizationRequest) | [ResourceAuthorizationList](#feedback_fusion_v1-ResourceAuthorizationList) |  |
| GetResourceAuthorizations | [GetResourceAuthorizationsRequest](#feedback_fusion_v1-GetResourceAuthorizationsRequest) | [ResourceAuthorizationPage](#feedback_fusion_v1-ResourceAuthorizationPage) |  |
| GetResourceAuthorization | [GetResourceAuthorizationRequest](#feedback_fusion_v1-GetResourceAuthorizationRequest) | [ProtoResourceAuthorization](#feedback_fusion_v1-ProtoResourceAuthorization) |  |
| UpdateResourceAuthorization | [UpdateResourceAuthorizationRequest](#feedback_fusion_v1-UpdateResourceAuthorizationRequest) | [ProtoResourceAuthorization](#feedback_fusion_v1-ProtoResourceAuthorization) |  |
| DeleteResourceAuthorization | [DeleteResourceAuthorizationRequest](#feedback_fusion_v1-DeleteResourceAuthorizationRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) |  |
| ExportResourceAuthorizations | [ExportResourceAuthorizationsRequest](#feedback_fusion_v1-ExportResourceAuthorizationsRequest) | [ResourceAuthorizationExportResponse](#feedback_fusion_v1-ResourceAuthorizationExportResponse) |  |
| GetAuditVersions | [GetAuditVersionsRequest](#feedback_fusion_v1-GetAuditVersionsRequest) | [AuditVersionPage](#feedback_fusion_v1-AuditVersionPage) |  |
| RollbackResource | [RollbackResourceRequest](#feedback_fusion_v1-RollbackResourceRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) |  |


<a name="feedback_fusion_v1-PublicFeedbackFusionV1"></a>

### PublicFeedbackFusionV1
does not use oidc authentication

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| GetActiveFields | [GetFieldsRequest](#feedback_fusion_v1-GetFieldsRequest) | [FieldPage](#feedback_fusion_v1-FieldPage) |  |
| GetPrompt | [GetPromptRequest](#feedback_fusion_v1-GetPromptRequest) | [ProtoPrompt](#feedback_fusion_v1-ProtoPrompt) |  |
| CreateResponses | [CreateResponsesRequest](#feedback_fusion_v1-CreateResponsesRequest) | [ProtoPromptResponse](#feedback_fusion_v1-ProtoPromptResponse) |  |

 



<a name="feedback-fusion-v1_field-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## feedback-fusion-v1/field.proto



<a name="feedback_fusion_v1-CreateFieldRequest"></a>

### CreateFieldRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| prompt | [string](#string) |  |  |
| title | [string](#string) |  |  |
| description | [string](#string) | optional |  |
| field_type | [ProtoFieldType](#feedback_fusion_v1-ProtoFieldType) |  |  |
| options | [ProtoFieldOptions](#feedback_fusion_v1-ProtoFieldOptions) |  |  |






<a name="feedback_fusion_v1-DeleteFieldRequest"></a>

### DeleteFieldRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |






<a name="feedback_fusion_v1-FieldPage"></a>

### FieldPage



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| page_token | [int32](#int32) |  |  |
| next_page_token | [int32](#int32) |  |  |
| page_size | [int32](#int32) |  |  |
| total | [int32](#int32) |  |  |
| fields | [ProtoField](#feedback_fusion_v1-ProtoField) | repeated |  |






<a name="feedback_fusion_v1-GetFieldsRequest"></a>

### GetFieldsRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| page_token | [int32](#int32) |  |  |
| page_size | [int32](#int32) |  |  |
| prompt | [string](#string) |  |  |






<a name="feedback_fusion_v1-ProtoCheckboxOptions"></a>

### ProtoCheckboxOptions



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| style | [ProtoCheckboxStyle](#feedback_fusion_v1-ProtoCheckboxStyle) |  |  |
| default_state | [bool](#bool) |  |  |






<a name="feedback_fusion_v1-ProtoField"></a>

### ProtoField



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |
| prompt | [string](#string) |  |  |
| title | [string](#string) |  |  |
| description | [string](#string) | optional |  |
| field_type | [ProtoFieldType](#feedback_fusion_v1-ProtoFieldType) |  |  |
| options | [ProtoFieldOptions](#feedback_fusion_v1-ProtoFieldOptions) |  |  |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  |  |
| updated_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  |  |






<a name="feedback_fusion_v1-ProtoFieldOptions"></a>

### ProtoFieldOptions



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| text | [ProtoTextOptions](#feedback_fusion_v1-ProtoTextOptions) |  |  |
| rating | [ProtoRatingOptions](#feedback_fusion_v1-ProtoRatingOptions) |  |  |
| checkbox | [ProtoCheckboxOptions](#feedback_fusion_v1-ProtoCheckboxOptions) |  |  |
| selection | [ProtoSelectionOptions](#feedback_fusion_v1-ProtoSelectionOptions) |  |  |
| range | [ProtoRangeOptions](#feedback_fusion_v1-ProtoRangeOptions) |  |  |
| number | [ProtoNumberOptions](#feedback_fusion_v1-ProtoNumberOptions) |  |  |






<a name="feedback_fusion_v1-ProtoNumberOptions"></a>

### ProtoNumberOptions



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| min | [int32](#int32) |  |  |
| max | [int32](#int32) |  |  |
| placeholder | [string](#string) |  |  |






<a name="feedback_fusion_v1-ProtoRangeOptions"></a>

### ProtoRangeOptions



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| min | [int32](#int32) |  |  |
| max | [int32](#int32) |  |  |






<a name="feedback_fusion_v1-ProtoRatingOptions"></a>

### ProtoRatingOptions



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| max | [int32](#int32) |  |  |






<a name="feedback_fusion_v1-ProtoSelectionOptions"></a>

### ProtoSelectionOptions



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| values | [string](#string) | repeated |  |
| multiple | [bool](#bool) |  |  |
| combobox | [bool](#bool) |  |  |






<a name="feedback_fusion_v1-ProtoTextOptions"></a>

### ProtoTextOptions



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| placeholder | [string](#string) |  |  |
| lines | [int32](#int32) |  |  |






<a name="feedback_fusion_v1-UpdateFieldRequest"></a>

### UpdateFieldRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| options | [ProtoFieldOptions](#feedback_fusion_v1-ProtoFieldOptions) |  |  |
| title | [string](#string) | optional |  |
| description | [string](#string) | optional |  |
| id | [string](#string) |  |  |





 


<a name="feedback_fusion_v1-ProtoCheckboxStyle"></a>

### ProtoCheckboxStyle


| Name | Number | Description |
| ---- | ------ | ----------- |
| NORMAL | 0 |  |
| SWITCH | 1 |  |



<a name="feedback_fusion_v1-ProtoFieldType"></a>

### ProtoFieldType


| Name | Number | Description |
| ---- | ------ | ----------- |
| TEXT | 0 |  |
| RATING | 1 |  |
| CHECKBOX | 2 |  |
| SELECTION | 3 |  |
| RANGE | 4 |  |
| NUMBER | 5 |  |


 

 

 



<a name="feedback-fusion-v1_prompt-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## feedback-fusion-v1/prompt.proto



<a name="feedback_fusion_v1-CreatePromptRequest"></a>

### CreatePromptRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| target | [string](#string) |  |  |
| title | [string](#string) |  |  |
| description | [string](#string) |  |  |
| active | [bool](#bool) |  |  |






<a name="feedback_fusion_v1-DeletePromptRequest"></a>

### DeletePromptRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |






<a name="feedback_fusion_v1-GetPromptRequest"></a>

### GetPromptRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |






<a name="feedback_fusion_v1-GetPromptsRequest"></a>

### GetPromptsRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| page_token | [int32](#int32) |  |  |
| page_size | [int32](#int32) |  |  |
| target | [string](#string) |  |  |






<a name="feedback_fusion_v1-PromptPage"></a>

### PromptPage



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| page_token | [int32](#int32) |  |  |
| next_page_token | [int32](#int32) |  |  |
| page_size | [int32](#int32) |  |  |
| total | [int32](#int32) |  |  |
| prompts | [ProtoPrompt](#feedback_fusion_v1-ProtoPrompt) | repeated |  |






<a name="feedback_fusion_v1-ProtoPrompt"></a>

### ProtoPrompt



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |
| title | [string](#string) |  |  |
| description | [string](#string) |  |  |
| target | [string](#string) |  |  |
| active | [bool](#bool) |  |  |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  |  |
| updated_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  |  |






<a name="feedback_fusion_v1-UpdatePromptRequest"></a>

### UpdatePromptRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |
| active | [bool](#bool) | optional |  |
| description | [string](#string) | optional |  |
| title | [string](#string) | optional |  |





 

 

 

 



<a name="feedback-fusion-v1_resource-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## feedback-fusion-v1/resource.proto



<a name="feedback_fusion_v1-ProtoResource"></a>

### ProtoResource



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| unknown | [google.protobuf.Empty](#google-protobuf-Empty) |  |  |
| target | [ProtoTarget](#feedback_fusion_v1-ProtoTarget) |  |  |
| prompt | [ProtoPrompt](#feedback_fusion_v1-ProtoPrompt) |  |  |
| field | [ProtoField](#feedback_fusion_v1-ProtoField) |  |  |





 

 

 

 



## Scalar Value Types

| .proto Type | Notes | C++ | Java | Python | Go | C# | PHP | Ruby |
| ----------- | ----- | --- | ---- | ------ | -- | -- | --- | ---- |
| <a name="double" /> double |  | double | double | float | float64 | double | float | Float |
| <a name="float" /> float |  | float | float | float | float32 | float | float | Float |
| <a name="int32" /> int32 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint32 instead. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="int64" /> int64 | Uses variable-length encoding. Inefficient for encoding negative numbers – if your field is likely to have negative values, use sint64 instead. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="uint32" /> uint32 | Uses variable-length encoding. | uint32 | int | int/long | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="uint64" /> uint64 | Uses variable-length encoding. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum or Fixnum (as required) |
| <a name="sint32" /> sint32 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int32s. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sint64" /> sint64 | Uses variable-length encoding. Signed int value. These more efficiently encode negative numbers than regular int64s. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="fixed32" /> fixed32 | Always four bytes. More efficient than uint32 if values are often greater than 2^28. | uint32 | int | int | uint32 | uint | integer | Bignum or Fixnum (as required) |
| <a name="fixed64" /> fixed64 | Always eight bytes. More efficient than uint64 if values are often greater than 2^56. | uint64 | long | int/long | uint64 | ulong | integer/string | Bignum |
| <a name="sfixed32" /> sfixed32 | Always four bytes. | int32 | int | int | int32 | int | integer | Bignum or Fixnum (as required) |
| <a name="sfixed64" /> sfixed64 | Always eight bytes. | int64 | long | int/long | int64 | long | integer/string | Bignum |
| <a name="bool" /> bool |  | bool | boolean | boolean | bool | bool | boolean | TrueClass/FalseClass |
| <a name="string" /> string | A string must always contain UTF-8 encoded or 7-bit ASCII text. | string | String | str/unicode | string | string | string | String (UTF-8) |
| <a name="bytes" /> bytes | May contain any arbitrary sequence of bytes. | string | ByteString | str | []byte | ByteString | string | String (ASCII-8BIT) |


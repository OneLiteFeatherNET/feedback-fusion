# Protocol Documentation
<a name="top"></a>

## Table of Contents

- [feedback-fusion-event-v1.proto](#feedback-fusion-event-v1-proto)
    - [Event](#feedback_fusion_event_v1-Event)
    - [EventBatch](#feedback_fusion_event_v1-EventBatch)
    - [ResourceModifiedEvent](#feedback_fusion_event_v1-ResourceModifiedEvent)
  
    - [EventType](#feedback_fusion_event_v1-EventType)
    - [ResourceKind](#feedback_fusion_event_v1-ResourceKind)
    - [ResourceModificationOperation](#feedback_fusion_event_v1-ResourceModificationOperation)
  
    - [FeedbackFusionIndexerV1](#feedback_fusion_event_v1-FeedbackFusionIndexerV1)
  
- [feedback-fusion-v1.proto](#feedback-fusion-v1-proto)
    - [CheckboxOptions](#feedback_fusion_v1-CheckboxOptions)
    - [CheckboxResponse](#feedback_fusion_v1-CheckboxResponse)
    - [CreateFieldRequest](#feedback_fusion_v1-CreateFieldRequest)
    - [CreatePromptRequest](#feedback_fusion_v1-CreatePromptRequest)
    - [CreateResourceAuthorizationRequest](#feedback_fusion_v1-CreateResourceAuthorizationRequest)
    - [CreateResponsesRequest](#feedback_fusion_v1-CreateResponsesRequest)
    - [CreateResponsesRequest.DataEntry](#feedback_fusion_v1-CreateResponsesRequest-DataEntry)
    - [CreateTargetRequest](#feedback_fusion_v1-CreateTargetRequest)
    - [DataExportRequest](#feedback_fusion_v1-DataExportRequest)
    - [DataExportResponse](#feedback_fusion_v1-DataExportResponse)
    - [DeleteFieldRequest](#feedback_fusion_v1-DeleteFieldRequest)
    - [DeletePromptRequest](#feedback_fusion_v1-DeletePromptRequest)
    - [DeleteResourceAuthorizationRequest](#feedback_fusion_v1-DeleteResourceAuthorizationRequest)
    - [DeleteTargetRequest](#feedback_fusion_v1-DeleteTargetRequest)
    - [ExportResourceAuthorizationsRequest](#feedback_fusion_v1-ExportResourceAuthorizationsRequest)
    - [Field](#feedback_fusion_v1-Field)
    - [FieldOptions](#feedback_fusion_v1-FieldOptions)
    - [FieldPage](#feedback_fusion_v1-FieldPage)
    - [FieldResponse](#feedback_fusion_v1-FieldResponse)
    - [FieldResponseList](#feedback_fusion_v1-FieldResponseList)
    - [GetFieldsRequest](#feedback_fusion_v1-GetFieldsRequest)
    - [GetPromptRequest](#feedback_fusion_v1-GetPromptRequest)
    - [GetPromptsRequest](#feedback_fusion_v1-GetPromptsRequest)
    - [GetResourceAuthorizationRequest](#feedback_fusion_v1-GetResourceAuthorizationRequest)
    - [GetResourceAuthorizationsRequest](#feedback_fusion_v1-GetResourceAuthorizationsRequest)
    - [GetResponsesRequest](#feedback_fusion_v1-GetResponsesRequest)
    - [GetTargetRequest](#feedback_fusion_v1-GetTargetRequest)
    - [GetTargetsRequest](#feedback_fusion_v1-GetTargetsRequest)
    - [NumberOptions](#feedback_fusion_v1-NumberOptions)
    - [NumberResponse](#feedback_fusion_v1-NumberResponse)
    - [Prompt](#feedback_fusion_v1-Prompt)
    - [PromptPage](#feedback_fusion_v1-PromptPage)
    - [PromptResponse](#feedback_fusion_v1-PromptResponse)
    - [RangeOptions](#feedback_fusion_v1-RangeOptions)
    - [RangeResponse](#feedback_fusion_v1-RangeResponse)
    - [RatingOptions](#feedback_fusion_v1-RatingOptions)
    - [RatingResponse](#feedback_fusion_v1-RatingResponse)
    - [ResourceAuthorization](#feedback_fusion_v1-ResourceAuthorization)
    - [ResourceAuthorizationData](#feedback_fusion_v1-ResourceAuthorizationData)
    - [ResourceAuthorizationExportResponse](#feedback_fusion_v1-ResourceAuthorizationExportResponse)
    - [ResourceAuthorizationList](#feedback_fusion_v1-ResourceAuthorizationList)
    - [ResourceAuthorizationPage](#feedback_fusion_v1-ResourceAuthorizationPage)
    - [ResponseData](#feedback_fusion_v1-ResponseData)
    - [ResponsePage](#feedback_fusion_v1-ResponsePage)
    - [ResponsePage.DataEntry](#feedback_fusion_v1-ResponsePage-DataEntry)
    - [SelectionOptions](#feedback_fusion_v1-SelectionOptions)
    - [SelectionResponse](#feedback_fusion_v1-SelectionResponse)
    - [Target](#feedback_fusion_v1-Target)
    - [TargetPage](#feedback_fusion_v1-TargetPage)
    - [TextOptions](#feedback_fusion_v1-TextOptions)
    - [TextResponse](#feedback_fusion_v1-TextResponse)
    - [UpdateFieldRequest](#feedback_fusion_v1-UpdateFieldRequest)
    - [UpdatePromptRequest](#feedback_fusion_v1-UpdatePromptRequest)
    - [UpdateResourceAuthorizationRequest](#feedback_fusion_v1-UpdateResourceAuthorizationRequest)
    - [UpdateTargetRequest](#feedback_fusion_v1-UpdateTargetRequest)
    - [UserInfoResponse](#feedback_fusion_v1-UserInfoResponse)
    - [UserInfoResponse.PermissionsEntry](#feedback_fusion_v1-UserInfoResponse-PermissionsEntry)
  
    - [AuthorizationGrant](#feedback_fusion_v1-AuthorizationGrant)
    - [AuthorizationType](#feedback_fusion_v1-AuthorizationType)
    - [CheckboxStyle](#feedback_fusion_v1-CheckboxStyle)
    - [FieldType](#feedback_fusion_v1-FieldType)
    - [ResourceKind](#feedback_fusion_v1-ResourceKind)
  
    - [FeedbackFusionV1](#feedback_fusion_v1-FeedbackFusionV1)
    - [PublicFeedbackFusionV1](#feedback_fusion_v1-PublicFeedbackFusionV1)
  
- [Scalar Value Types](#scalar-value-types)



<a name="feedback-fusion-event-v1-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## feedback-fusion-event-v1.proto



<a name="feedback_fusion_event_v1-Event"></a>

### Event



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  |  |
| event_type | [EventType](#feedback_fusion_event_v1-EventType) |  |  |
| resource_modified_event | [ResourceModifiedEvent](#feedback_fusion_event_v1-ResourceModifiedEvent) |  |  |






<a name="feedback_fusion_event_v1-EventBatch"></a>

### EventBatch



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| events | [Event](#feedback_fusion_event_v1-Event) | repeated |  |






<a name="feedback_fusion_event_v1-ResourceModifiedEvent"></a>

### ResourceModifiedEvent



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| operation | [ResourceModificationOperation](#feedback_fusion_event_v1-ResourceModificationOperation) |  |  |
| id | [string](#string) |  |  |
| resource_kind | [ResourceKind](#feedback_fusion_event_v1-ResourceKind) |  |  |
| data | [bytes](#bytes) |  |  |





 


<a name="feedback_fusion_event_v1-EventType"></a>

### EventType


| Name | Number | Description |
| ---- | ------ | ----------- |
| UNKNOWN | 0 |  |
| RESOURCE_MODIFIED | 1 |  |



<a name="feedback_fusion_event_v1-ResourceKind"></a>

### ResourceKind


| Name | Number | Description |
| ---- | ------ | ----------- |
| TARGET | 0 |  |
| PROMPT | 1 |  |
| FIELD | 2 |  |



<a name="feedback_fusion_event_v1-ResourceModificationOperation"></a>

### ResourceModificationOperation


| Name | Number | Description |
| ---- | ------ | ----------- |
| CREATE | 0 |  |
| UPDATE | 1 |  |
| DELETE | 2 |  |


 

 


<a name="feedback_fusion_event_v1-FeedbackFusionIndexerV1"></a>

### FeedbackFusionIndexerV1


| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| SendBatch | [EventBatch](#feedback_fusion_event_v1-EventBatch) | [.google.protobuf.Empty](#google-protobuf-Empty) |  |

 



<a name="feedback-fusion-v1-proto"></a>
<p align="right"><a href="#top">Top</a></p>

## feedback-fusion-v1.proto



<a name="feedback_fusion_v1-CheckboxOptions"></a>

### CheckboxOptions



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| style | [CheckboxStyle](#feedback_fusion_v1-CheckboxStyle) |  |  |
| default_state | [bool](#bool) |  |  |






<a name="feedback_fusion_v1-CheckboxResponse"></a>

### CheckboxResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| checked | [bool](#bool) |  |  |






<a name="feedback_fusion_v1-CreateFieldRequest"></a>

### CreateFieldRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| prompt | [string](#string) |  |  |
| title | [string](#string) |  |  |
| description | [string](#string) | optional |  |
| field_type | [FieldType](#feedback_fusion_v1-FieldType) |  |  |
| options | [FieldOptions](#feedback_fusion_v1-FieldOptions) |  |  |






<a name="feedback_fusion_v1-CreatePromptRequest"></a>

### CreatePromptRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| target | [string](#string) |  |  |
| title | [string](#string) |  |  |
| description | [string](#string) |  |  |
| active | [bool](#bool) |  |  |






<a name="feedback_fusion_v1-CreateResourceAuthorizationRequest"></a>

### CreateResourceAuthorizationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| resource_kind | [ResourceKind](#feedback_fusion_v1-ResourceKind) |  |  |
| resource_id | [string](#string) | repeated |  |
| authorization_data | [ResourceAuthorizationData](#feedback_fusion_v1-ResourceAuthorizationData) |  |  |






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






<a name="feedback_fusion_v1-CreateTargetRequest"></a>

### CreateTargetRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| name | [string](#string) |  |  |
| description | [string](#string) | optional |  |






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






<a name="feedback_fusion_v1-DeleteFieldRequest"></a>

### DeleteFieldRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |






<a name="feedback_fusion_v1-DeletePromptRequest"></a>

### DeletePromptRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |






<a name="feedback_fusion_v1-DeleteResourceAuthorizationRequest"></a>

### DeleteResourceAuthorizationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |






<a name="feedback_fusion_v1-DeleteTargetRequest"></a>

### DeleteTargetRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |






<a name="feedback_fusion_v1-ExportResourceAuthorizationsRequest"></a>

### ExportResourceAuthorizationsRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| ids | [string](#string) | repeated |  |






<a name="feedback_fusion_v1-Field"></a>

### Field



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |
| prompt | [string](#string) |  |  |
| title | [string](#string) |  |  |
| description | [string](#string) | optional |  |
| field_type | [FieldType](#feedback_fusion_v1-FieldType) |  |  |
| options | [FieldOptions](#feedback_fusion_v1-FieldOptions) |  |  |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  |  |
| updated_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  |  |






<a name="feedback_fusion_v1-FieldOptions"></a>

### FieldOptions



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| text | [TextOptions](#feedback_fusion_v1-TextOptions) |  |  |
| rating | [RatingOptions](#feedback_fusion_v1-RatingOptions) |  |  |
| checkbox | [CheckboxOptions](#feedback_fusion_v1-CheckboxOptions) |  |  |
| selection | [SelectionOptions](#feedback_fusion_v1-SelectionOptions) |  |  |
| range | [RangeOptions](#feedback_fusion_v1-RangeOptions) |  |  |
| number | [NumberOptions](#feedback_fusion_v1-NumberOptions) |  |  |






<a name="feedback_fusion_v1-FieldPage"></a>

### FieldPage



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| page_token | [int32](#int32) |  |  |
| next_page_token | [int32](#int32) |  |  |
| page_size | [int32](#int32) |  |  |
| total | [int32](#int32) |  |  |
| fields | [Field](#feedback_fusion_v1-Field) | repeated |  |






<a name="feedback_fusion_v1-FieldResponse"></a>

### FieldResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |
| response | [string](#string) |  |  |
| field | [string](#string) |  |  |
| data | [ResponseData](#feedback_fusion_v1-ResponseData) |  |  |






<a name="feedback_fusion_v1-FieldResponseList"></a>

### FieldResponseList



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| data | [FieldResponse](#feedback_fusion_v1-FieldResponse) | repeated |  |






<a name="feedback_fusion_v1-GetFieldsRequest"></a>

### GetFieldsRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| page_token | [int32](#int32) |  |  |
| page_size | [int32](#int32) |  |  |
| prompt | [string](#string) |  |  |






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
| authorization_type | [AuthorizationType](#feedback_fusion_v1-AuthorizationType) | optional |  |






<a name="feedback_fusion_v1-GetResponsesRequest"></a>

### GetResponsesRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| page_token | [int32](#int32) |  |  |
| page_size | [int32](#int32) |  |  |
| prompt | [string](#string) |  |  |






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






<a name="feedback_fusion_v1-NumberOptions"></a>

### NumberOptions



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| min | [int32](#int32) |  |  |
| max | [int32](#int32) |  |  |
| placeholder | [string](#string) |  |  |






<a name="feedback_fusion_v1-NumberResponse"></a>

### NumberResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| number | [int32](#int32) |  |  |






<a name="feedback_fusion_v1-Prompt"></a>

### Prompt



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |
| title | [string](#string) |  |  |
| description | [string](#string) |  |  |
| target | [string](#string) |  |  |
| active | [bool](#bool) |  |  |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  |  |
| updated_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  |  |






<a name="feedback_fusion_v1-PromptPage"></a>

### PromptPage



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| page_token | [int32](#int32) |  |  |
| next_page_token | [int32](#int32) |  |  |
| page_size | [int32](#int32) |  |  |
| total | [int32](#int32) |  |  |
| prompts | [Prompt](#feedback_fusion_v1-Prompt) | repeated |  |






<a name="feedback_fusion_v1-PromptResponse"></a>

### PromptResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |
| prompt | [string](#string) |  |  |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  |  |






<a name="feedback_fusion_v1-RangeOptions"></a>

### RangeOptions



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| min | [int32](#int32) |  |  |
| max | [int32](#int32) |  |  |






<a name="feedback_fusion_v1-RangeResponse"></a>

### RangeResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| start | [int32](#int32) |  |  |
| end | [int32](#int32) |  |  |






<a name="feedback_fusion_v1-RatingOptions"></a>

### RatingOptions



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| max | [int32](#int32) |  |  |






<a name="feedback_fusion_v1-RatingResponse"></a>

### RatingResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| rating | [int32](#int32) |  |  |






<a name="feedback_fusion_v1-ResourceAuthorization"></a>

### ResourceAuthorization



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |
| resource_kind | [ResourceKind](#feedback_fusion_v1-ResourceKind) |  |  |
| resource_id | [string](#string) | optional |  |
| authorization_type | [AuthorizationType](#feedback_fusion_v1-AuthorizationType) |  |  |
| authorization_grant | [AuthorizationGrant](#feedback_fusion_v1-AuthorizationGrant) |  |  |
| value | [string](#string) |  |  |
| updated_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  |  |
| created_at | [google.protobuf.Timestamp](#google-protobuf-Timestamp) |  |  |






<a name="feedback_fusion_v1-ResourceAuthorizationData"></a>

### ResourceAuthorizationData



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| type | [AuthorizationType](#feedback_fusion_v1-AuthorizationType) |  |  |
| grant | [AuthorizationGrant](#feedback_fusion_v1-AuthorizationGrant) | repeated |  |
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
| authorizations | [ResourceAuthorization](#feedback_fusion_v1-ResourceAuthorization) | repeated |  |






<a name="feedback_fusion_v1-ResourceAuthorizationPage"></a>

### ResourceAuthorizationPage



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| page_token | [int32](#int32) |  |  |
| next_page_token | [int32](#int32) |  |  |
| page_size | [int32](#int32) |  |  |
| total | [int32](#int32) |  |  |
| authorizations | [ResourceAuthorization](#feedback_fusion_v1-ResourceAuthorization) | repeated |  |






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






<a name="feedback_fusion_v1-SelectionOptions"></a>

### SelectionOptions



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| values | [string](#string) | repeated |  |
| multiple | [bool](#bool) |  |  |
| combobox | [bool](#bool) |  |  |






<a name="feedback_fusion_v1-SelectionResponse"></a>

### SelectionResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| values | [string](#string) | repeated |  |






<a name="feedback_fusion_v1-Target"></a>

### Target



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
| targets | [Target](#feedback_fusion_v1-Target) | repeated |  |






<a name="feedback_fusion_v1-TextOptions"></a>

### TextOptions



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| placeholder | [string](#string) |  |  |
| lines | [int32](#int32) |  |  |






<a name="feedback_fusion_v1-TextResponse"></a>

### TextResponse



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| text | [string](#string) |  |  |






<a name="feedback_fusion_v1-UpdateFieldRequest"></a>

### UpdateFieldRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| options | [FieldOptions](#feedback_fusion_v1-FieldOptions) |  |  |
| title | [string](#string) | optional |  |
| description | [string](#string) | optional |  |
| id | [string](#string) |  |  |






<a name="feedback_fusion_v1-UpdatePromptRequest"></a>

### UpdatePromptRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |
| active | [bool](#bool) | optional |  |
| description | [string](#string) | optional |  |
| title | [string](#string) | optional |  |






<a name="feedback_fusion_v1-UpdateResourceAuthorizationRequest"></a>

### UpdateResourceAuthorizationRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |
| resource_id | [string](#string) | optional |  |
| authorization_type | [AuthorizationType](#feedback_fusion_v1-AuthorizationType) | optional |  |
| authorization_grant | [AuthorizationGrant](#feedback_fusion_v1-AuthorizationGrant) | optional |  |
| value | [string](#string) | optional |  |






<a name="feedback_fusion_v1-UpdateTargetRequest"></a>

### UpdateTargetRequest



| Field | Type | Label | Description |
| ----- | ---- | ----- | ----------- |
| id | [string](#string) |  |  |
| name | [string](#string) | optional |  |
| description | [string](#string) | optional |  |






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





 


<a name="feedback_fusion_v1-AuthorizationGrant"></a>

### AuthorizationGrant


| Name | Number | Description |
| ---- | ------ | ----------- |
| WRITE | 0 |  |
| READ | 1 |  |
| List | 2 |  |
| All | 3 |  |



<a name="feedback_fusion_v1-AuthorizationType"></a>

### AuthorizationType


| Name | Number | Description |
| ---- | ------ | ----------- |
| TYPE_SCOPE | 0 |  |
| TYPE_GROUP | 1 |  |
| TYPE_SUBJECT | 2 |  |



<a name="feedback_fusion_v1-CheckboxStyle"></a>

### CheckboxStyle


| Name | Number | Description |
| ---- | ------ | ----------- |
| NORMAL | 0 |  |
| SWITCH | 1 |  |



<a name="feedback_fusion_v1-FieldType"></a>

### FieldType


| Name | Number | Description |
| ---- | ------ | ----------- |
| TEXT | 0 |  |
| RATING | 1 |  |
| CHECKBOX | 2 |  |
| SELECTION | 3 |  |
| RANGE | 4 |  |
| NUMBER | 5 |  |



<a name="feedback_fusion_v1-ResourceKind"></a>

### ResourceKind


| Name | Number | Description |
| ---- | ------ | ----------- |
| RESOURCE_TARGET | 0 |  |
| RESOURCE_PROMPT | 1 |  |
| RESOURCE_FIELD | 2 |  |
| RESOURCE_EXPORT | 3 |  |
| RESOURCE_AUTHORIZE | 4 |  |
| RESOURCE_RESPONSE | 5 |  |


 

 


<a name="feedback_fusion_v1-FeedbackFusionV1"></a>

### FeedbackFusionV1
uses oidc authentication

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| CreateTarget | [CreateTargetRequest](#feedback_fusion_v1-CreateTargetRequest) | [Target](#feedback_fusion_v1-Target) |  |
| GetTarget | [GetTargetRequest](#feedback_fusion_v1-GetTargetRequest) | [Target](#feedback_fusion_v1-Target) |  |
| GetTargets | [GetTargetsRequest](#feedback_fusion_v1-GetTargetsRequest) | [TargetPage](#feedback_fusion_v1-TargetPage) |  |
| UpdateTarget | [UpdateTargetRequest](#feedback_fusion_v1-UpdateTargetRequest) | [Target](#feedback_fusion_v1-Target) |  |
| DeleteTarget | [DeleteTargetRequest](#feedback_fusion_v1-DeleteTargetRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) |  |
| CreatePrompt | [CreatePromptRequest](#feedback_fusion_v1-CreatePromptRequest) | [Prompt](#feedback_fusion_v1-Prompt) |  |
| GetPrompts | [GetPromptsRequest](#feedback_fusion_v1-GetPromptsRequest) | [PromptPage](#feedback_fusion_v1-PromptPage) |  |
| UpdatePrompt | [UpdatePromptRequest](#feedback_fusion_v1-UpdatePromptRequest) | [Prompt](#feedback_fusion_v1-Prompt) |  |
| DeletePrompt | [DeletePromptRequest](#feedback_fusion_v1-DeletePromptRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) |  |
| CreateField | [CreateFieldRequest](#feedback_fusion_v1-CreateFieldRequest) | [Field](#feedback_fusion_v1-Field) |  |
| GetFields | [GetFieldsRequest](#feedback_fusion_v1-GetFieldsRequest) | [FieldPage](#feedback_fusion_v1-FieldPage) |  |
| UpdateField | [UpdateFieldRequest](#feedback_fusion_v1-UpdateFieldRequest) | [Field](#feedback_fusion_v1-Field) |  |
| DeleteField | [DeleteFieldRequest](#feedback_fusion_v1-DeleteFieldRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) |  |
| GetResponses | [GetResponsesRequest](#feedback_fusion_v1-GetResponsesRequest) | [ResponsePage](#feedback_fusion_v1-ResponsePage) |  |
| GetUserInfo | [.google.protobuf.Empty](#google-protobuf-Empty) | [UserInfoResponse](#feedback_fusion_v1-UserInfoResponse) |  |
| ExportData | [DataExportRequest](#feedback_fusion_v1-DataExportRequest) | [DataExportResponse](#feedback_fusion_v1-DataExportResponse) |  |
| CreateResourceAuthorization | [CreateResourceAuthorizationRequest](#feedback_fusion_v1-CreateResourceAuthorizationRequest) | [ResourceAuthorizationList](#feedback_fusion_v1-ResourceAuthorizationList) |  |
| GetResourceAuthorizations | [GetResourceAuthorizationsRequest](#feedback_fusion_v1-GetResourceAuthorizationsRequest) | [ResourceAuthorizationPage](#feedback_fusion_v1-ResourceAuthorizationPage) |  |
| GetResourceAuthorization | [GetResourceAuthorizationRequest](#feedback_fusion_v1-GetResourceAuthorizationRequest) | [ResourceAuthorization](#feedback_fusion_v1-ResourceAuthorization) |  |
| UpdateResourceAuthorization | [UpdateResourceAuthorizationRequest](#feedback_fusion_v1-UpdateResourceAuthorizationRequest) | [ResourceAuthorization](#feedback_fusion_v1-ResourceAuthorization) |  |
| DeleteResourceAuthorization | [DeleteResourceAuthorizationRequest](#feedback_fusion_v1-DeleteResourceAuthorizationRequest) | [.google.protobuf.Empty](#google-protobuf-Empty) |  |
| ExportResourceAuthorizations | [ExportResourceAuthorizationsRequest](#feedback_fusion_v1-ExportResourceAuthorizationsRequest) | [ResourceAuthorizationExportResponse](#feedback_fusion_v1-ResourceAuthorizationExportResponse) |  |


<a name="feedback_fusion_v1-PublicFeedbackFusionV1"></a>

### PublicFeedbackFusionV1
does not use oidc authentication

| Method Name | Request Type | Response Type | Description |
| ----------- | ------------ | ------------- | ------------|
| GetActiveFields | [GetFieldsRequest](#feedback_fusion_v1-GetFieldsRequest) | [FieldPage](#feedback_fusion_v1-FieldPage) |  |
| GetPrompt | [GetPromptRequest](#feedback_fusion_v1-GetPromptRequest) | [Prompt](#feedback_fusion_v1-Prompt) |  |
| CreateResponses | [CreateResponsesRequest](#feedback_fusion_v1-CreateResponsesRequest) | [PromptResponse](#feedback_fusion_v1-PromptResponse) |  |

 



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


# PutFleetsFleetIdMembersMemberIdMovement

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**role** | **String** | If a character is moved to the `fleet_commander` role, neither `wing_id` or `squad_id` should be specified. If a character is moved to the `wing_commander` role, only `wing_id` should be specified. If a character is moved to the `squad_commander` role, both `wing_id` and `squad_id` should be specified. If a character is moved to the `squad_member` role, both `wing_id` and `squad_id` should be specified. | 
**squad_id** | Option<**i64**> | squad_id integer | [optional]
**wing_id** | Option<**i64**> | wing_id integer | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



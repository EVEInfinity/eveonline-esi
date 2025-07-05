# GetCorporationsCorporationIdStructures200Ok

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**corporation_id** | **i32** | ID of the corporation that owns the structure | 
**fuel_expires** | Option<**String**> | Date on which the structure will run out of fuel | [optional]
**name** | Option<**String**> | The structure name | [optional]
**next_reinforce_apply** | Option<**String**> | The date and time when the structure's newly requested reinforcement times (e.g. next_reinforce_hour and next_reinforce_day) will take effect | [optional]
**next_reinforce_hour** | Option<**i32**> | The requested change to reinforce_hour that will take effect at the time shown by next_reinforce_apply | [optional]
**profile_id** | **i32** | The id of the ACL profile for this citadel | 
**reinforce_hour** | Option<**i32**> | The hour of day that determines the four hour window when the structure will randomly exit its reinforcement periods and become vulnerable to attack against its armor and/or hull. The structure will become vulnerable at a random time that is +/- 2 hours centered on the value of this property | [optional]
**services** | Option<[**Vec<models::GetCorporationsCorporationIdStructuresService>**](get_corporations_corporation_id_structures_service.md)> | Contains a list of service upgrades, and their state | [optional]
**state** | **String** | state string | 
**state_timer_end** | Option<**String**> | Date at which the structure will move to it's next state | [optional]
**state_timer_start** | Option<**String**> | Date at which the structure entered it's current state | [optional]
**structure_id** | **i64** | The Item ID of the structure | 
**system_id** | **i32** | The solar system the structure is in | 
**type_id** | **i32** | The type id of the structure | 
**unanchors_at** | Option<**String**> | Date at which the structure will unanchor | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



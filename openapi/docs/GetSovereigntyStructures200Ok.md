# GetSovereigntyStructures200Ok

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**alliance_id** | **i32** | The alliance that owns the structure.  | 
**solar_system_id** | **i32** | Solar system in which the structure is located.  | 
**structure_id** | **i64** | Unique item ID for this structure. | 
**structure_type_id** | **i32** | A reference to the type of structure this is.  | 
**vulnerability_occupancy_level** | Option<**f32**> | The occupancy level for the next or current vulnerability window. This takes into account all development indexes and capital system bonuses. Also known as Activity Defense Multiplier from in the client. It increases the time that attackers must spend using their entosis links on the structure.  | [optional]
**vulnerable_end_time** | Option<**String**> | The time at which the next or current vulnerability window ends. At the end of a vulnerability window the next window is recalculated and locked in along with the vulnerabilityOccupancyLevel. If the structure is not in 100% entosis control of the defender, it will go in to 'overtime' and stay vulnerable for as long as that situation persists. Only once the defenders have 100% entosis control and has the vulnerableEndTime passed does the vulnerability interval expire and a new one is calculated.  | [optional]
**vulnerable_start_time** | Option<**String**> | The next time at which the structure will become vulnerable. Or the start time of the current window if current time is between this and vulnerableEndTime.  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



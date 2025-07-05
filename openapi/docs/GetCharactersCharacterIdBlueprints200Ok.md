# GetCharactersCharacterIdBlueprints200Ok

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**item_id** | **i64** | Unique ID for this item. | 
**location_flag** | **String** | Type of the location_id | 
**location_id** | **i64** | References a station, a ship or an item_id if this blueprint is located within a container. If the return value is an item_id, then the Character AssetList API must be queried to find the container using the given item_id to determine the correct location of the Blueprint. | 
**material_efficiency** | **i32** | Material Efficiency Level of the blueprint. | 
**quantity** | **i32** | A range of numbers with a minimum of -2 and no maximum value where -1 is an original and -2 is a copy. It can be a positive integer if it is a stack of blueprint originals fresh from the market (e.g. no activities performed on them yet). | 
**runs** | **i32** | Number of runs remaining if the blueprint is a copy, -1 if it is an original. | 
**time_efficiency** | **i32** | Time Efficiency Level of the blueprint. | 
**type_id** | **i32** | type_id integer | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



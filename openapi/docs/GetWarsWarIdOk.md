# GetWarsWarIdOk

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**aggressor** | [**models::GetWarsWarIdAggressor**](get_wars_war_id_aggressor.md) |  | 
**allies** | Option<[**Vec<models::GetWarsWarIdAlly>**](get_wars_war_id_ally.md)> | allied corporations or alliances, each object contains either corporation_id or alliance_id | [optional]
**declared** | **String** | Time that the war was declared | 
**defender** | [**models::GetWarsWarIdDefender**](get_wars_war_id_defender.md) |  | 
**finished** | Option<**String**> | Time the war ended and shooting was no longer allowed | [optional]
**id** | **i32** | ID of the specified war | 
**mutual** | **bool** | Was the war declared mutual by both parties | 
**open_for_allies** | **bool** | Is the war currently open for allies or not | 
**retracted** | Option<**String**> | Time the war was retracted but both sides could still shoot each other | [optional]
**started** | Option<**String**> | Time when the war started and both sides could shoot each other | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



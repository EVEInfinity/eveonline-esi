# GetCorporationsCorporationIdCustomsOffices200Ok

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**alliance_tax_rate** | Option<**f32**> | Only present if alliance access is allowed | [optional]
**allow_access_with_standings** | **bool** | standing_level and any standing related tax rate only present when this is true | 
**allow_alliance_access** | **bool** | allow_alliance_access boolean | 
**bad_standing_tax_rate** | Option<**f32**> | bad_standing_tax_rate number | [optional]
**corporation_tax_rate** | Option<**f32**> | corporation_tax_rate number | [optional]
**excellent_standing_tax_rate** | Option<**f32**> | Tax rate for entities with excellent level of standing, only present if this level is allowed, same for all other standing related tax rates | [optional]
**good_standing_tax_rate** | Option<**f32**> | good_standing_tax_rate number | [optional]
**neutral_standing_tax_rate** | Option<**f32**> | neutral_standing_tax_rate number | [optional]
**office_id** | **i64** | unique ID of this customs office | 
**reinforce_exit_end** | **i32** | reinforce_exit_end integer | 
**reinforce_exit_start** | **i32** | Together with reinforce_exit_end, marks a 2-hour period where this customs office could exit reinforcement mode during the day after initial attack | 
**standing_level** | Option<**String**> | Access is allowed only for entities with this level of standing or better | [optional]
**system_id** | **i32** | ID of the solar system this customs office is located in | 
**terrible_standing_tax_rate** | Option<**f32**> | terrible_standing_tax_rate number | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



# GetSovereigntyCampaigns200Ok

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attackers_score** | Option<**f32**> | Score for all attacking parties, only present in Defense Events.  | [optional]
**campaign_id** | **i32** | Unique ID for this campaign. | 
**constellation_id** | **i32** | The constellation in which the campaign will take place.  | 
**defender_id** | Option<**i32**> | Defending alliance, only present in Defense Events  | [optional]
**defender_score** | Option<**f32**> | Score for the defending alliance, only present in Defense Events.  | [optional]
**event_type** | **String** | Type of event this campaign is for. tcu_defense, ihub_defense and station_defense are referred to as \"Defense Events\", station_freeport as \"Freeport Events\".  | 
**participants** | Option<[**Vec<models::GetSovereigntyCampaignsParticipant>**](get_sovereignty_campaigns_participant.md)> | Alliance participating and their respective scores, only present in Freeport Events.  | [optional]
**solar_system_id** | **i32** | The solar system the structure is located in.  | 
**start_time** | **String** | Time the event is scheduled to start.  | 
**structure_id** | **i64** | The structure item ID that is related to this campaign.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



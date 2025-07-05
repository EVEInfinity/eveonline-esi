# GetCharactersCharacterIdIndustryJobs200Ok

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**activity_id** | **i32** | Job activity ID | 
**blueprint_id** | **i64** | blueprint_id integer | 
**blueprint_location_id** | **i64** | Location ID of the location from which the blueprint was installed. Normally a station ID, but can also be an asset (e.g. container) or corporation facility | 
**blueprint_type_id** | **i32** | blueprint_type_id integer | 
**completed_character_id** | Option<**i32**> | ID of the character which completed this job | [optional]
**completed_date** | Option<**String**> | Date and time when this job was completed | [optional]
**cost** | Option<**f64**> | The sume of job installation fee and industry facility tax | [optional]
**duration** | **i32** | Job duration in seconds | 
**end_date** | **String** | Date and time when this job finished | 
**facility_id** | **i64** | ID of the facility where this job is running | 
**installer_id** | **i32** | ID of the character which installed this job | 
**job_id** | **i32** | Unique job ID | 
**licensed_runs** | Option<**i32**> | Number of runs blueprint is licensed for | [optional]
**output_location_id** | **i64** | Location ID of the location to which the output of the job will be delivered. Normally a station ID, but can also be a corporation facility | 
**pause_date** | Option<**String**> | Date and time when this job was paused (i.e. time when the facility where this job was installed went offline) | [optional]
**probability** | Option<**f32**> | Chance of success for invention | [optional]
**product_type_id** | Option<**i32**> | Type ID of product (manufactured, copied or invented) | [optional]
**runs** | **i32** | Number of runs for a manufacturing job, or number of copies to make for a blueprint copy | 
**start_date** | **String** | Date and time when this job started | 
**station_id** | **i64** | ID of the station where industry facility is located | 
**status** | **String** | status string | 
**successful_runs** | Option<**i32**> | Number of successful runs for this job. Equal to runs unless this is an invention job | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



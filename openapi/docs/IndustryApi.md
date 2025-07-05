# \IndustryApi

All URIs are relative to *https://esi.evetech.net/latest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_characters_character_id_industry_jobs**](IndustryApi.md#get_characters_character_id_industry_jobs) | **GET** /characters/{character_id}/industry/jobs/ | List character industry jobs
[**get_characters_character_id_mining**](IndustryApi.md#get_characters_character_id_mining) | **GET** /characters/{character_id}/mining/ | Character mining ledger
[**get_corporation_corporation_id_mining_extractions**](IndustryApi.md#get_corporation_corporation_id_mining_extractions) | **GET** /corporation/{corporation_id}/mining/extractions/ | Moon extraction timers
[**get_corporation_corporation_id_mining_observers**](IndustryApi.md#get_corporation_corporation_id_mining_observers) | **GET** /corporation/{corporation_id}/mining/observers/ | Corporation mining observers
[**get_corporation_corporation_id_mining_observers_observer_id**](IndustryApi.md#get_corporation_corporation_id_mining_observers_observer_id) | **GET** /corporation/{corporation_id}/mining/observers/{observer_id}/ | Observed corporation mining
[**get_corporations_corporation_id_industry_jobs**](IndustryApi.md#get_corporations_corporation_id_industry_jobs) | **GET** /corporations/{corporation_id}/industry/jobs/ | List corporation industry jobs
[**get_industry_facilities**](IndustryApi.md#get_industry_facilities) | **GET** /industry/facilities/ | List industry facilities
[**get_industry_systems**](IndustryApi.md#get_industry_systems) | **GET** /industry/systems/ | List solar system cost indices



## get_characters_character_id_industry_jobs

> Vec<models::GetCharactersCharacterIdIndustryJobs200Ok> get_characters_character_id_industry_jobs(character_id, datasource, if_none_match, include_completed, token)
List character industry jobs

List industry jobs placed by a character  --- Alternate route: `/dev/characters/{character_id}/industry/jobs/`  Alternate route: `/legacy/characters/{character_id}/industry/jobs/`  Alternate route: `/v1/characters/{character_id}/industry/jobs/`  --- This route is cached for up to 300 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**include_completed** | Option<**bool**> | Whether to retrieve completed character industry jobs. Only includes jobs from the past 90 days |  |
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

[**Vec<models::GetCharactersCharacterIdIndustryJobs200Ok>**](get_characters_character_id_industry_jobs_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_characters_character_id_mining

> Vec<models::GetCharactersCharacterIdMining200Ok> get_characters_character_id_mining(character_id, datasource, if_none_match, page, token)
Character mining ledger

Paginated record of all mining done by a character for the past 30 days   --- Alternate route: `/dev/characters/{character_id}/mining/`  Alternate route: `/legacy/characters/{character_id}/mining/`  Alternate route: `/v1/characters/{character_id}/mining/`  --- This route is cached for up to 600 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**page** | Option<**i32**> | Which page of results to return |  |[default to 1]
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

[**Vec<models::GetCharactersCharacterIdMining200Ok>**](get_characters_character_id_mining_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_corporation_corporation_id_mining_extractions

> Vec<models::GetCorporationCorporationIdMiningExtractions200Ok> get_corporation_corporation_id_mining_extractions(corporation_id, datasource, if_none_match, page, token)
Moon extraction timers

Extraction timers for all moon chunks being extracted by refineries belonging to a corporation.   --- Alternate route: `/dev/corporation/{corporation_id}/mining/extractions/`  Alternate route: `/legacy/corporation/{corporation_id}/mining/extractions/`  Alternate route: `/v1/corporation/{corporation_id}/mining/extractions/`  --- This route is cached for up to 1800 seconds  --- Requires one of the following EVE corporation role(s): Station_Manager 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**corporation_id** | **i32** | An EVE corporation ID | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**page** | Option<**i32**> | Which page of results to return |  |[default to 1]
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

[**Vec<models::GetCorporationCorporationIdMiningExtractions200Ok>**](get_corporation_corporation_id_mining_extractions_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_corporation_corporation_id_mining_observers

> Vec<models::GetCorporationCorporationIdMiningObservers200Ok> get_corporation_corporation_id_mining_observers(corporation_id, datasource, if_none_match, page, token)
Corporation mining observers

Paginated list of all entities capable of observing and recording mining for a corporation   --- Alternate route: `/dev/corporation/{corporation_id}/mining/observers/`  Alternate route: `/legacy/corporation/{corporation_id}/mining/observers/`  Alternate route: `/v1/corporation/{corporation_id}/mining/observers/`  --- This route is cached for up to 3600 seconds  --- Requires one of the following EVE corporation role(s): Accountant 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**corporation_id** | **i32** | An EVE corporation ID | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**page** | Option<**i32**> | Which page of results to return |  |[default to 1]
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

[**Vec<models::GetCorporationCorporationIdMiningObservers200Ok>**](get_corporation_corporation_id_mining_observers_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_corporation_corporation_id_mining_observers_observer_id

> Vec<models::GetCorporationCorporationIdMiningObserversObserverId200Ok> get_corporation_corporation_id_mining_observers_observer_id(corporation_id, observer_id, datasource, if_none_match, page, token)
Observed corporation mining

Paginated record of all mining seen by an observer   --- Alternate route: `/dev/corporation/{corporation_id}/mining/observers/{observer_id}/`  Alternate route: `/legacy/corporation/{corporation_id}/mining/observers/{observer_id}/`  Alternate route: `/v1/corporation/{corporation_id}/mining/observers/{observer_id}/`  --- This route is cached for up to 3600 seconds  --- Requires one of the following EVE corporation role(s): Accountant 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**corporation_id** | **i32** | An EVE corporation ID | [required] |
**observer_id** | **i64** | A mining observer id | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**page** | Option<**i32**> | Which page of results to return |  |[default to 1]
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

[**Vec<models::GetCorporationCorporationIdMiningObserversObserverId200Ok>**](get_corporation_corporation_id_mining_observers_observer_id_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_corporations_corporation_id_industry_jobs

> Vec<models::GetCorporationsCorporationIdIndustryJobs200Ok> get_corporations_corporation_id_industry_jobs(corporation_id, datasource, if_none_match, include_completed, page, token)
List corporation industry jobs

List industry jobs run by a corporation  --- Alternate route: `/dev/corporations/{corporation_id}/industry/jobs/`  Alternate route: `/legacy/corporations/{corporation_id}/industry/jobs/`  Alternate route: `/v1/corporations/{corporation_id}/industry/jobs/`  --- This route is cached for up to 300 seconds  --- Requires one of the following EVE corporation role(s): Factory_Manager 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**corporation_id** | **i32** | An EVE corporation ID | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**include_completed** | Option<**bool**> | Whether to retrieve completed corporation industry jobs. Only includes jobs from the past 90 days |  |[default to false]
**page** | Option<**i32**> | Which page of results to return |  |[default to 1]
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

[**Vec<models::GetCorporationsCorporationIdIndustryJobs200Ok>**](get_corporations_corporation_id_industry_jobs_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_industry_facilities

> Vec<models::GetIndustryFacilities200Ok> get_industry_facilities(datasource, if_none_match)
List industry facilities

Return a list of industry facilities  --- Alternate route: `/dev/industry/facilities/`  Alternate route: `/legacy/industry/facilities/`  Alternate route: `/v1/industry/facilities/`  --- This route is cached for up to 3600 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |

### Return type

[**Vec<models::GetIndustryFacilities200Ok>**](get_industry_facilities_200_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_industry_systems

> Vec<models::GetIndustrySystems200Ok> get_industry_systems(datasource, if_none_match)
List solar system cost indices

Return cost indices for solar systems  --- Alternate route: `/dev/industry/systems/`  Alternate route: `/legacy/industry/systems/`  Alternate route: `/v1/industry/systems/`  --- This route is cached for up to 3600 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |

### Return type

[**Vec<models::GetIndustrySystems200Ok>**](get_industry_systems_200_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


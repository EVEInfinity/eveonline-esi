# \WarsApi

All URIs are relative to *https://esi.evetech.net/latest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_wars**](WarsApi.md#get_wars) | **GET** /wars/ | List wars
[**get_wars_war_id**](WarsApi.md#get_wars_war_id) | **GET** /wars/{war_id}/ | Get war information
[**get_wars_war_id_killmails**](WarsApi.md#get_wars_war_id_killmails) | **GET** /wars/{war_id}/killmails/ | List kills for a war



## get_wars

> Vec<i32> get_wars(datasource, if_none_match, max_war_id)
List wars

Return a list of wars  --- Alternate route: `/dev/wars/`  Alternate route: `/legacy/wars/`  Alternate route: `/v1/wars/`  --- This route is cached for up to 3600 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**max_war_id** | Option<**i32**> | Only return wars with ID smaller than this |  |

### Return type

**Vec<i32>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_wars_war_id

> models::GetWarsWarIdOk get_wars_war_id(war_id, datasource, if_none_match)
Get war information

Return details about a war  --- Alternate route: `/dev/wars/{war_id}/`  Alternate route: `/legacy/wars/{war_id}/`  Alternate route: `/v1/wars/{war_id}/`  --- This route is cached for up to 3600 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**war_id** | **i32** | ID for a war | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |

### Return type

[**models::GetWarsWarIdOk**](get_wars_war_id_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_wars_war_id_killmails

> Vec<models::GetWarsWarIdKillmails200Ok> get_wars_war_id_killmails(war_id, datasource, if_none_match, page)
List kills for a war

Return a list of kills related to a war  --- Alternate route: `/dev/wars/{war_id}/killmails/`  Alternate route: `/legacy/wars/{war_id}/killmails/`  Alternate route: `/v1/wars/{war_id}/killmails/`  --- This route is cached for up to 3600 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**war_id** | **i32** | A valid war ID | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**page** | Option<**i32**> | Which page of results to return |  |[default to 1]

### Return type

[**Vec<models::GetWarsWarIdKillmails200Ok>**](get_wars_war_id_killmails_200_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


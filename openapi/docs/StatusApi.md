# \StatusApi

All URIs are relative to *https://esi.evetech.net/latest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_status**](StatusApi.md#get_status) | **GET** /status/ | Retrieve the uptime and player counts



## get_status

> models::GetStatusOk get_status(datasource, if_none_match)
Retrieve the uptime and player counts

EVE Server status  --- Alternate route: `/dev/status/`  Alternate route: `/legacy/status/`  Alternate route: `/v1/status/`  Alternate route: `/v2/status/`  --- This route is cached for up to 30 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |

### Return type

[**models::GetStatusOk**](get_status_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


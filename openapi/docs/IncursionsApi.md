# \IncursionsApi

All URIs are relative to *https://esi.evetech.net/latest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_incursions**](IncursionsApi.md#get_incursions) | **GET** /incursions/ | List incursions



## get_incursions

> Vec<models::GetIncursions200Ok> get_incursions(datasource, if_none_match)
List incursions

Return a list of current incursions  --- Alternate route: `/dev/incursions/`  Alternate route: `/legacy/incursions/`  Alternate route: `/v1/incursions/`  --- This route is cached for up to 300 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |

### Return type

[**Vec<models::GetIncursions200Ok>**](get_incursions_200_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


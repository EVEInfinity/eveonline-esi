# \RoutesApi

All URIs are relative to *https://esi.evetech.net/latest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_route_origin_destination**](RoutesApi.md#get_route_origin_destination) | **GET** /route/{origin}/{destination}/ | Get route



## get_route_origin_destination

> Vec<i32> get_route_origin_destination(destination, origin, avoid, connections, datasource, flag, if_none_match)
Get route

Get the systems between origin and destination  --- Alternate route: `/dev/route/{origin}/{destination}/`  Alternate route: `/legacy/route/{origin}/{destination}/`  Alternate route: `/v1/route/{origin}/{destination}/`  --- This route is cached for up to 86400 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destination** | **i32** | destination solar system ID | [required] |
**origin** | **i32** | origin solar system ID | [required] |
**avoid** | Option<[**Vec<i32>**](i32.md)> | avoid solar system ID(s) |  |
**connections** | Option<[**Vec<Vec<i32>>**](Vec<i32>.md)> | connected solar system pairs |  |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**flag** | Option<**String**> | route security preference |  |[default to shortest]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |

### Return type

**Vec<i32>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


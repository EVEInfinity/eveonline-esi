# \InsuranceApi

All URIs are relative to *https://esi.evetech.net/latest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_insurance_prices**](InsuranceApi.md#get_insurance_prices) | **GET** /insurance/prices/ | List insurance levels



## get_insurance_prices

> Vec<models::GetInsurancePrices200Ok> get_insurance_prices(accept_language, datasource, if_none_match, language)
List insurance levels

Return available insurance levels for all ship types  --- Alternate route: `/dev/insurance/prices/`  Alternate route: `/legacy/insurance/prices/`  Alternate route: `/v1/insurance/prices/`  --- This route is cached for up to 3600 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept_language** | Option<**String**> | Language to use in the response |  |[default to en]
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**language** | Option<**String**> | Language to use in the response, takes precedence over Accept-Language |  |[default to en]

### Return type

[**Vec<models::GetInsurancePrices200Ok>**](get_insurance_prices_200_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


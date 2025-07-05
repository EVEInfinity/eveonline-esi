# \FittingsApi

All URIs are relative to *https://esi.evetech.net/latest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_characters_character_id_fittings_fitting_id**](FittingsApi.md#delete_characters_character_id_fittings_fitting_id) | **DELETE** /characters/{character_id}/fittings/{fitting_id}/ | Delete fitting
[**get_characters_character_id_fittings**](FittingsApi.md#get_characters_character_id_fittings) | **GET** /characters/{character_id}/fittings/ | Get fittings
[**post_characters_character_id_fittings**](FittingsApi.md#post_characters_character_id_fittings) | **POST** /characters/{character_id}/fittings/ | Create fitting



## delete_characters_character_id_fittings_fitting_id

> delete_characters_character_id_fittings_fitting_id(character_id, fitting_id, datasource, token)
Delete fitting

Delete a fitting from a character  --- Alternate route: `/dev/characters/{character_id}/fittings/{fitting_id}/`  Alternate route: `/legacy/characters/{character_id}/fittings/{fitting_id}/`  Alternate route: `/v1/characters/{character_id}/fittings/{fitting_id}/` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**fitting_id** | **i32** | ID for a fitting of this character | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

 (empty response body)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_characters_character_id_fittings

> Vec<models::GetCharactersCharacterIdFittings200Ok> get_characters_character_id_fittings(character_id, datasource, if_none_match, token)
Get fittings

Return fittings of a character  --- Alternate route: `/dev/characters/{character_id}/fittings/`  Alternate route: `/v2/characters/{character_id}/fittings/`  --- This route is cached for up to 300 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

[**Vec<models::GetCharactersCharacterIdFittings200Ok>**](get_characters_character_id_fittings_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_characters_character_id_fittings

> models::PostCharactersCharacterIdFittingsCreated post_characters_character_id_fittings(character_id, fitting, datasource, token)
Create fitting

Save a new fitting for a character  --- Alternate route: `/dev/characters/{character_id}/fittings/`  Alternate route: `/legacy/characters/{character_id}/fittings/`  Alternate route: `/v1/characters/{character_id}/fittings/`  Alternate route: `/v2/characters/{character_id}/fittings/` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**fitting** | [**PostCharactersCharacterIdFittingsFitting**](PostCharactersCharacterIdFittingsFitting.md) | Details about the new fitting | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

[**models::PostCharactersCharacterIdFittingsCreated**](post_characters_character_id_fittings_created.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


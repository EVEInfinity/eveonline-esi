# \SearchApi

All URIs are relative to *https://esi.evetech.net/latest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_characters_character_id_search**](SearchApi.md#get_characters_character_id_search) | **GET** /characters/{character_id}/search/ | Search on a string



## get_characters_character_id_search

> models::GetCharactersCharacterIdSearchOk get_characters_character_id_search(categories, character_id, search, accept_language, datasource, if_none_match, language, strict, token)
Search on a string

Search for entities that match a given sub-string.  --- Alternate route: `/dev/characters/{character_id}/search/`  Alternate route: `/legacy/characters/{character_id}/search/`  Alternate route: `/v3/characters/{character_id}/search/`  --- This route is cached for up to 3600 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**categories** | [**Vec<String>**](String.md) | Type of entities to search for | [required] |
**character_id** | **i32** | An EVE character ID | [required] |
**search** | **String** | The string to search on | [required] |
**accept_language** | Option<**String**> | Language to use in the response |  |[default to en]
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**language** | Option<**String**> | Language to use in the response, takes precedence over Accept-Language |  |[default to en]
**strict** | Option<**bool**> | Whether the search should be a strict match |  |[default to false]
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

[**models::GetCharactersCharacterIdSearchOk**](get_characters_character_id_search_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


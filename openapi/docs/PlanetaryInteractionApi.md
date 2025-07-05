# \PlanetaryInteractionApi

All URIs are relative to *https://esi.evetech.net/latest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_characters_character_id_planets**](PlanetaryInteractionApi.md#get_characters_character_id_planets) | **GET** /characters/{character_id}/planets/ | Get colonies
[**get_characters_character_id_planets_planet_id**](PlanetaryInteractionApi.md#get_characters_character_id_planets_planet_id) | **GET** /characters/{character_id}/planets/{planet_id}/ | Get colony layout
[**get_corporations_corporation_id_customs_offices**](PlanetaryInteractionApi.md#get_corporations_corporation_id_customs_offices) | **GET** /corporations/{corporation_id}/customs_offices/ | List corporation customs offices
[**get_universe_schematics_schematic_id**](PlanetaryInteractionApi.md#get_universe_schematics_schematic_id) | **GET** /universe/schematics/{schematic_id}/ | Get schematic information



## get_characters_character_id_planets

> Vec<models::GetCharactersCharacterIdPlanets200Ok> get_characters_character_id_planets(character_id, datasource, if_none_match, token)
Get colonies

Returns a list of all planetary colonies owned by a character.  --- Alternate route: `/dev/characters/{character_id}/planets/`  Alternate route: `/legacy/characters/{character_id}/planets/`  Alternate route: `/v1/characters/{character_id}/planets/`  --- This route is cached for up to 600 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

[**Vec<models::GetCharactersCharacterIdPlanets200Ok>**](get_characters_character_id_planets_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_characters_character_id_planets_planet_id

> models::GetCharactersCharacterIdPlanetsPlanetIdOk get_characters_character_id_planets_planet_id(character_id, planet_id, datasource, token)
Get colony layout

Returns full details on the layout of a single planetary colony, including links, pins and routes. Note: Planetary information is only recalculated when the colony is viewed through the client. Information will not update until this criteria is met.  --- Alternate route: `/dev/characters/{character_id}/planets/{planet_id}/`  Alternate route: `/v3/characters/{character_id}/planets/{planet_id}/` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**planet_id** | **i32** | Planet id of the target planet | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

[**models::GetCharactersCharacterIdPlanetsPlanetIdOk**](get_characters_character_id_planets_planet_id_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_corporations_corporation_id_customs_offices

> Vec<models::GetCorporationsCorporationIdCustomsOffices200Ok> get_corporations_corporation_id_customs_offices(corporation_id, datasource, if_none_match, page, token)
List corporation customs offices

List customs offices owned by a corporation  --- Alternate route: `/dev/corporations/{corporation_id}/customs_offices/`  Alternate route: `/legacy/corporations/{corporation_id}/customs_offices/`  Alternate route: `/v1/corporations/{corporation_id}/customs_offices/`  --- This route is cached for up to 3600 seconds  --- Requires one of the following EVE corporation role(s): Director 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**corporation_id** | **i32** | An EVE corporation ID | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**page** | Option<**i32**> | Which page of results to return |  |[default to 1]
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

[**Vec<models::GetCorporationsCorporationIdCustomsOffices200Ok>**](get_corporations_corporation_id_customs_offices_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_universe_schematics_schematic_id

> models::GetUniverseSchematicsSchematicIdOk get_universe_schematics_schematic_id(schematic_id, datasource, if_none_match)
Get schematic information

Get information on a planetary factory schematic  --- Alternate route: `/dev/universe/schematics/{schematic_id}/`  Alternate route: `/legacy/universe/schematics/{schematic_id}/`  Alternate route: `/v1/universe/schematics/{schematic_id}/`  --- This route is cached for up to 3600 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**schematic_id** | **i32** | A PI schematic ID | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |

### Return type

[**models::GetUniverseSchematicsSchematicIdOk**](get_universe_schematics_schematic_id_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


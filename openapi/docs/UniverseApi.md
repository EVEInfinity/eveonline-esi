# \UniverseApi

All URIs are relative to *https://esi.evetech.net/latest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_universe_ancestries**](UniverseApi.md#get_universe_ancestries) | **GET** /universe/ancestries/ | Get ancestries
[**get_universe_asteroid_belts_asteroid_belt_id**](UniverseApi.md#get_universe_asteroid_belts_asteroid_belt_id) | **GET** /universe/asteroid_belts/{asteroid_belt_id}/ | Get asteroid belt information
[**get_universe_bloodlines**](UniverseApi.md#get_universe_bloodlines) | **GET** /universe/bloodlines/ | Get bloodlines
[**get_universe_categories**](UniverseApi.md#get_universe_categories) | **GET** /universe/categories/ | Get item categories
[**get_universe_categories_category_id**](UniverseApi.md#get_universe_categories_category_id) | **GET** /universe/categories/{category_id}/ | Get item category information
[**get_universe_constellations**](UniverseApi.md#get_universe_constellations) | **GET** /universe/constellations/ | Get constellations
[**get_universe_constellations_constellation_id**](UniverseApi.md#get_universe_constellations_constellation_id) | **GET** /universe/constellations/{constellation_id}/ | Get constellation information
[**get_universe_factions**](UniverseApi.md#get_universe_factions) | **GET** /universe/factions/ | Get factions
[**get_universe_graphics**](UniverseApi.md#get_universe_graphics) | **GET** /universe/graphics/ | Get graphics
[**get_universe_graphics_graphic_id**](UniverseApi.md#get_universe_graphics_graphic_id) | **GET** /universe/graphics/{graphic_id}/ | Get graphic information
[**get_universe_groups**](UniverseApi.md#get_universe_groups) | **GET** /universe/groups/ | Get item groups
[**get_universe_groups_group_id**](UniverseApi.md#get_universe_groups_group_id) | **GET** /universe/groups/{group_id}/ | Get item group information
[**get_universe_moons_moon_id**](UniverseApi.md#get_universe_moons_moon_id) | **GET** /universe/moons/{moon_id}/ | Get moon information
[**get_universe_planets_planet_id**](UniverseApi.md#get_universe_planets_planet_id) | **GET** /universe/planets/{planet_id}/ | Get planet information
[**get_universe_races**](UniverseApi.md#get_universe_races) | **GET** /universe/races/ | Get character races
[**get_universe_regions**](UniverseApi.md#get_universe_regions) | **GET** /universe/regions/ | Get regions
[**get_universe_regions_region_id**](UniverseApi.md#get_universe_regions_region_id) | **GET** /universe/regions/{region_id}/ | Get region information
[**get_universe_stargates_stargate_id**](UniverseApi.md#get_universe_stargates_stargate_id) | **GET** /universe/stargates/{stargate_id}/ | Get stargate information
[**get_universe_stars_star_id**](UniverseApi.md#get_universe_stars_star_id) | **GET** /universe/stars/{star_id}/ | Get star information
[**get_universe_stations_station_id**](UniverseApi.md#get_universe_stations_station_id) | **GET** /universe/stations/{station_id}/ | Get station information
[**get_universe_structures**](UniverseApi.md#get_universe_structures) | **GET** /universe/structures/ | List all public structures
[**get_universe_structures_structure_id**](UniverseApi.md#get_universe_structures_structure_id) | **GET** /universe/structures/{structure_id}/ | Get structure information
[**get_universe_system_jumps**](UniverseApi.md#get_universe_system_jumps) | **GET** /universe/system_jumps/ | Get system jumps
[**get_universe_system_kills**](UniverseApi.md#get_universe_system_kills) | **GET** /universe/system_kills/ | Get system kills
[**get_universe_systems**](UniverseApi.md#get_universe_systems) | **GET** /universe/systems/ | Get solar systems
[**get_universe_systems_system_id**](UniverseApi.md#get_universe_systems_system_id) | **GET** /universe/systems/{system_id}/ | Get solar system information
[**get_universe_types**](UniverseApi.md#get_universe_types) | **GET** /universe/types/ | Get types
[**get_universe_types_type_id**](UniverseApi.md#get_universe_types_type_id) | **GET** /universe/types/{type_id}/ | Get type information
[**post_universe_ids**](UniverseApi.md#post_universe_ids) | **POST** /universe/ids/ | Bulk names to IDs
[**post_universe_names**](UniverseApi.md#post_universe_names) | **POST** /universe/names/ | Get names and categories for a set of IDs



## get_universe_ancestries

> Vec<models::GetUniverseAncestries200Ok> get_universe_ancestries(accept_language, datasource, if_none_match, language)
Get ancestries

Get all character ancestries  --- Alternate route: `/legacy/universe/ancestries/`  Alternate route: `/v1/universe/ancestries/`  --- This route expires daily at 11:05

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept_language** | Option<**String**> | Language to use in the response |  |[default to en]
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**language** | Option<**String**> | Language to use in the response, takes precedence over Accept-Language |  |[default to en]

### Return type

[**Vec<models::GetUniverseAncestries200Ok>**](get_universe_ancestries_200_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_universe_asteroid_belts_asteroid_belt_id

> models::GetUniverseAsteroidBeltsAsteroidBeltIdOk get_universe_asteroid_belts_asteroid_belt_id(asteroid_belt_id, datasource, if_none_match)
Get asteroid belt information

Get information on an asteroid belt  --- Alternate route: `/legacy/universe/asteroid_belts/{asteroid_belt_id}/`  Alternate route: `/v1/universe/asteroid_belts/{asteroid_belt_id}/`  --- This route expires daily at 11:05

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asteroid_belt_id** | **i32** | asteroid_belt_id integer | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |

### Return type

[**models::GetUniverseAsteroidBeltsAsteroidBeltIdOk**](get_universe_asteroid_belts_asteroid_belt_id_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_universe_bloodlines

> Vec<models::GetUniverseBloodlines200Ok> get_universe_bloodlines(accept_language, datasource, if_none_match, language)
Get bloodlines

Get a list of bloodlines  --- Alternate route: `/legacy/universe/bloodlines/`  Alternate route: `/v1/universe/bloodlines/`  --- This route expires daily at 11:05

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept_language** | Option<**String**> | Language to use in the response |  |[default to en]
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**language** | Option<**String**> | Language to use in the response, takes precedence over Accept-Language |  |[default to en]

### Return type

[**Vec<models::GetUniverseBloodlines200Ok>**](get_universe_bloodlines_200_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_universe_categories

> Vec<i32> get_universe_categories(datasource, if_none_match)
Get item categories

Get a list of item categories  --- Alternate route: `/legacy/universe/categories/`  Alternate route: `/v1/universe/categories/`  --- This route expires daily at 11:05

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |

### Return type

**Vec<i32>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_universe_categories_category_id

> models::GetUniverseCategoriesCategoryIdOk get_universe_categories_category_id(category_id, accept_language, datasource, if_none_match, language)
Get item category information

Get information of an item category  --- Alternate route: `/legacy/universe/categories/{category_id}/`  Alternate route: `/v1/universe/categories/{category_id}/`  --- This route expires daily at 11:05

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category_id** | **i32** | An Eve item category ID | [required] |
**accept_language** | Option<**String**> | Language to use in the response |  |[default to en]
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**language** | Option<**String**> | Language to use in the response, takes precedence over Accept-Language |  |[default to en]

### Return type

[**models::GetUniverseCategoriesCategoryIdOk**](get_universe_categories_category_id_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_universe_constellations

> Vec<i32> get_universe_constellations(datasource, if_none_match)
Get constellations

Get a list of constellations  --- Alternate route: `/legacy/universe/constellations/`  Alternate route: `/v1/universe/constellations/`  --- This route expires daily at 11:05

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |

### Return type

**Vec<i32>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_universe_constellations_constellation_id

> models::GetUniverseConstellationsConstellationIdOk get_universe_constellations_constellation_id(constellation_id, accept_language, datasource, if_none_match, language)
Get constellation information

Get information on a constellation  --- Alternate route: `/legacy/universe/constellations/{constellation_id}/`  Alternate route: `/v1/universe/constellations/{constellation_id}/`  --- This route expires daily at 11:05

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**constellation_id** | **i32** | constellation_id integer | [required] |
**accept_language** | Option<**String**> | Language to use in the response |  |[default to en]
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**language** | Option<**String**> | Language to use in the response, takes precedence over Accept-Language |  |[default to en]

### Return type

[**models::GetUniverseConstellationsConstellationIdOk**](get_universe_constellations_constellation_id_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_universe_factions

> Vec<models::GetUniverseFactions200Ok> get_universe_factions(accept_language, datasource, if_none_match, language)
Get factions

Get a list of factions  --- Alternate route: `/dev/universe/factions/`  Alternate route: `/v2/universe/factions/`  --- This route expires daily at 11:05

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept_language** | Option<**String**> | Language to use in the response |  |[default to en]
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**language** | Option<**String**> | Language to use in the response, takes precedence over Accept-Language |  |[default to en]

### Return type

[**Vec<models::GetUniverseFactions200Ok>**](get_universe_factions_200_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_universe_graphics

> Vec<i32> get_universe_graphics(datasource, if_none_match)
Get graphics

Get a list of graphics  --- Alternate route: `/legacy/universe/graphics/`  Alternate route: `/v1/universe/graphics/`  --- This route expires daily at 11:05

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |

### Return type

**Vec<i32>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_universe_graphics_graphic_id

> models::GetUniverseGraphicsGraphicIdOk get_universe_graphics_graphic_id(graphic_id, datasource, if_none_match)
Get graphic information

Get information on a graphic  --- Alternate route: `/dev/universe/graphics/{graphic_id}/`  Alternate route: `/legacy/universe/graphics/{graphic_id}/`  Alternate route: `/v1/universe/graphics/{graphic_id}/`  --- This route expires daily at 11:05

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**graphic_id** | **i32** | graphic_id integer | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |

### Return type

[**models::GetUniverseGraphicsGraphicIdOk**](get_universe_graphics_graphic_id_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_universe_groups

> Vec<i32> get_universe_groups(datasource, if_none_match, page)
Get item groups

Get a list of item groups  --- Alternate route: `/legacy/universe/groups/`  Alternate route: `/v1/universe/groups/`  --- This route expires daily at 11:05

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**page** | Option<**i32**> | Which page of results to return |  |[default to 1]

### Return type

**Vec<i32>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_universe_groups_group_id

> models::GetUniverseGroupsGroupIdOk get_universe_groups_group_id(group_id, accept_language, datasource, if_none_match, language)
Get item group information

Get information on an item group  --- Alternate route: `/dev/universe/groups/{group_id}/`  Alternate route: `/legacy/universe/groups/{group_id}/`  Alternate route: `/v1/universe/groups/{group_id}/`  --- This route expires daily at 11:05

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i32** | An Eve item group ID | [required] |
**accept_language** | Option<**String**> | Language to use in the response |  |[default to en]
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**language** | Option<**String**> | Language to use in the response, takes precedence over Accept-Language |  |[default to en]

### Return type

[**models::GetUniverseGroupsGroupIdOk**](get_universe_groups_group_id_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_universe_moons_moon_id

> models::GetUniverseMoonsMoonIdOk get_universe_moons_moon_id(moon_id, datasource, if_none_match)
Get moon information

Get information on a moon  --- Alternate route: `/legacy/universe/moons/{moon_id}/`  Alternate route: `/v1/universe/moons/{moon_id}/`  --- This route expires daily at 11:05

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**moon_id** | **i32** | moon_id integer | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |

### Return type

[**models::GetUniverseMoonsMoonIdOk**](get_universe_moons_moon_id_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_universe_planets_planet_id

> models::GetUniversePlanetsPlanetIdOk get_universe_planets_planet_id(planet_id, datasource, if_none_match)
Get planet information

Get information on a planet  --- Alternate route: `/legacy/universe/planets/{planet_id}/`  Alternate route: `/v1/universe/planets/{planet_id}/`  --- This route expires daily at 11:05

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**planet_id** | **i32** | planet_id integer | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |

### Return type

[**models::GetUniversePlanetsPlanetIdOk**](get_universe_planets_planet_id_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_universe_races

> Vec<models::GetUniverseRaces200Ok> get_universe_races(accept_language, datasource, if_none_match, language)
Get character races

Get a list of character races  --- Alternate route: `/dev/universe/races/`  Alternate route: `/legacy/universe/races/`  Alternate route: `/v1/universe/races/`  --- This route expires daily at 11:05

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept_language** | Option<**String**> | Language to use in the response |  |[default to en]
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**language** | Option<**String**> | Language to use in the response, takes precedence over Accept-Language |  |[default to en]

### Return type

[**Vec<models::GetUniverseRaces200Ok>**](get_universe_races_200_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_universe_regions

> Vec<i32> get_universe_regions(datasource, if_none_match)
Get regions

Get a list of regions  --- Alternate route: `/legacy/universe/regions/`  Alternate route: `/v1/universe/regions/`  --- This route expires daily at 11:05

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |

### Return type

**Vec<i32>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_universe_regions_region_id

> models::GetUniverseRegionsRegionIdOk get_universe_regions_region_id(region_id, accept_language, datasource, if_none_match, language)
Get region information

Get information on a region  --- Alternate route: `/legacy/universe/regions/{region_id}/`  Alternate route: `/v1/universe/regions/{region_id}/`  --- This route expires daily at 11:05

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**region_id** | **i32** | region_id integer | [required] |
**accept_language** | Option<**String**> | Language to use in the response |  |[default to en]
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**language** | Option<**String**> | Language to use in the response, takes precedence over Accept-Language |  |[default to en]

### Return type

[**models::GetUniverseRegionsRegionIdOk**](get_universe_regions_region_id_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_universe_stargates_stargate_id

> models::GetUniverseStargatesStargateIdOk get_universe_stargates_stargate_id(stargate_id, datasource, if_none_match)
Get stargate information

Get information on a stargate  --- Alternate route: `/legacy/universe/stargates/{stargate_id}/`  Alternate route: `/v1/universe/stargates/{stargate_id}/`  --- This route expires daily at 11:05

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**stargate_id** | **i32** | stargate_id integer | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |

### Return type

[**models::GetUniverseStargatesStargateIdOk**](get_universe_stargates_stargate_id_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_universe_stars_star_id

> models::GetUniverseStarsStarIdOk get_universe_stars_star_id(star_id, datasource, if_none_match)
Get star information

Get information on a star  --- Alternate route: `/legacy/universe/stars/{star_id}/`  Alternate route: `/v1/universe/stars/{star_id}/`  --- This route expires daily at 11:05

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**star_id** | **i32** | star_id integer | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |

### Return type

[**models::GetUniverseStarsStarIdOk**](get_universe_stars_star_id_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_universe_stations_station_id

> models::GetUniverseStationsStationIdOk get_universe_stations_station_id(station_id, datasource, if_none_match)
Get station information

Get information on a station  --- Alternate route: `/dev/universe/stations/{station_id}/`  Alternate route: `/v2/universe/stations/{station_id}/`  --- This route expires daily at 11:05

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**station_id** | **i32** | station_id integer | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |

### Return type

[**models::GetUniverseStationsStationIdOk**](get_universe_stations_station_id_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_universe_structures

> Vec<i64> get_universe_structures(datasource, filter, if_none_match)
List all public structures

List all public structures  --- Alternate route: `/dev/universe/structures/`  Alternate route: `/legacy/universe/structures/`  Alternate route: `/v1/universe/structures/`  --- This route is cached for up to 3600 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**filter** | Option<**String**> | Only list public structures that have this service online |  |
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |

### Return type

**Vec<i64>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_universe_structures_structure_id

> models::GetUniverseStructuresStructureIdOk get_universe_structures_structure_id(structure_id, datasource, if_none_match, token)
Get structure information

Returns information on requested structure if you are on the ACL. Otherwise, returns \"Forbidden\" for all inputs.  --- Alternate route: `/legacy/universe/structures/{structure_id}/`  Alternate route: `/v1/universe/structures/{structure_id}/`  Alternate route: `/v2/universe/structures/{structure_id}/`  --- This route is cached for up to 3600 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**structure_id** | **i64** | An Eve structure ID | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

[**models::GetUniverseStructuresStructureIdOk**](get_universe_structures_structure_id_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_universe_system_jumps

> Vec<models::GetUniverseSystemJumps200Ok> get_universe_system_jumps(datasource, if_none_match)
Get system jumps

Get the number of jumps in solar systems within the last hour ending at the timestamp of the Last-Modified header, excluding wormhole space. Only systems with jumps will be listed  --- Alternate route: `/legacy/universe/system_jumps/`  Alternate route: `/v1/universe/system_jumps/`  --- This route is cached for up to 3600 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |

### Return type

[**Vec<models::GetUniverseSystemJumps200Ok>**](get_universe_system_jumps_200_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_universe_system_kills

> Vec<models::GetUniverseSystemKills200Ok> get_universe_system_kills(datasource, if_none_match)
Get system kills

Get the number of ship, pod and NPC kills per solar system within the last hour ending at the timestamp of the Last-Modified header, excluding wormhole space. Only systems with kills will be listed  --- Alternate route: `/v2/universe/system_kills/`  --- This route is cached for up to 3600 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |

### Return type

[**Vec<models::GetUniverseSystemKills200Ok>**](get_universe_system_kills_200_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_universe_systems

> Vec<i32> get_universe_systems(datasource, if_none_match)
Get solar systems

Get a list of solar systems  --- Alternate route: `/dev/universe/systems/`  Alternate route: `/legacy/universe/systems/`  Alternate route: `/v1/universe/systems/`  --- This route expires daily at 11:05

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |

### Return type

**Vec<i32>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_universe_systems_system_id

> models::GetUniverseSystemsSystemIdOk get_universe_systems_system_id(system_id, accept_language, datasource, if_none_match, language)
Get solar system information

Get information on a solar system.  --- Alternate route: `/dev/universe/systems/{system_id}/`  Alternate route: `/v4/universe/systems/{system_id}/`  --- This route expires daily at 11:05

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | system_id integer | [required] |
**accept_language** | Option<**String**> | Language to use in the response |  |[default to en]
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**language** | Option<**String**> | Language to use in the response, takes precedence over Accept-Language |  |[default to en]

### Return type

[**models::GetUniverseSystemsSystemIdOk**](get_universe_systems_system_id_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_universe_types

> Vec<i32> get_universe_types(datasource, if_none_match, page)
Get types

Get a list of type ids  --- Alternate route: `/legacy/universe/types/`  Alternate route: `/v1/universe/types/`  --- This route expires daily at 11:05

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**page** | Option<**i32**> | Which page of results to return |  |[default to 1]

### Return type

**Vec<i32>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_universe_types_type_id

> models::GetUniverseTypesTypeIdOk get_universe_types_type_id(type_id, accept_language, datasource, if_none_match, language)
Get type information

Get information on a type  --- Alternate route: `/dev/universe/types/{type_id}/`  Alternate route: `/legacy/universe/types/{type_id}/`  Alternate route: `/v2/universe/types/{type_id}/`  Alternate route: `/v3/universe/types/{type_id}/`  --- This route expires daily at 11:05

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**type_id** | **i32** | An Eve item type ID | [required] |
**accept_language** | Option<**String**> | Language to use in the response |  |[default to en]
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**language** | Option<**String**> | Language to use in the response, takes precedence over Accept-Language |  |[default to en]

### Return type

[**models::GetUniverseTypesTypeIdOk**](get_universe_types_type_id_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_universe_ids

> models::PostUniverseIdsOk post_universe_ids(names, accept_language, datasource, language)
Bulk names to IDs

Resolve a set of names to IDs in the following categories: agents, alliances, characters, constellations, corporations factions, inventory_types, regions, stations, and systems. Only exact matches will be returned. All names searched for are cached for 12 hours  --- Alternate route: `/dev/universe/ids/`  Alternate route: `/legacy/universe/ids/`  Alternate route: `/v1/universe/ids/` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**names** | [**Vec<String>**](String.md) | The names to resolve | [required] |
**accept_language** | Option<**String**> | Language to use in the response |  |[default to en]
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**language** | Option<**String**> | Language to use in the response, takes precedence over Accept-Language |  |[default to en]

### Return type

[**models::PostUniverseIdsOk**](post_universe_ids_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_universe_names

> Vec<models::PostUniverseNames200Ok> post_universe_names(ids, datasource)
Get names and categories for a set of IDs

Resolve a set of IDs to names and categories. Supported ID's for resolving are: Characters, Corporations, Alliances, Stations, Solar Systems, Constellations, Regions, Types, Factions  --- Alternate route: `/dev/universe/names/`  Alternate route: `/legacy/universe/names/`  Alternate route: `/v2/universe/names/`  Alternate route: `/v3/universe/names/` 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ids** | [**Vec<i32>**](i32.md) | The ids to resolve | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]

### Return type

[**Vec<models::PostUniverseNames200Ok>**](post_universe_names_200_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


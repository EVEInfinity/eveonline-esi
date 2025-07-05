# GetCorporationsCorporationIdOrders200Ok

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**duration** | **i32** | Number of days for which order is valid (starting from the issued date). An order expires at time issued + duration | 
**escrow** | Option<**f64**> | For buy orders, the amount of ISK in escrow | [optional]
**is_buy_order** | Option<**bool**> | True if the order is a bid (buy) order | [optional]
**issued** | **String** | Date and time when this order was issued | 
**issued_by** | **i32** | The character who issued this order | 
**location_id** | **i64** | ID of the location where order was placed | 
**min_volume** | Option<**i32**> | For buy orders, the minimum quantity that will be accepted in a matching sell order | [optional]
**order_id** | **i64** | Unique order ID | 
**price** | **f64** | Cost per unit for this order | 
**range** | **String** | Valid order range, numbers are ranges in jumps | 
**region_id** | **i32** | ID of the region where order was placed | 
**type_id** | **i32** | The type ID of the item transacted in this order | 
**volume_remain** | **i32** | Quantity of items still required or offered | 
**volume_total** | **i32** | Quantity of items required or offered at time order was placed | 
**wallet_division** | **i32** | The corporation wallet division used for this order. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)



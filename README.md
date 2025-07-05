# eveonline-esi

EVE Online ESI for rust.

WIP.

## Development

### Update ESI Openapi

```bash
deno run -A --node-modules-dir --allow-scripts npm:@openapitools/openapi-generator-cli generate -g rust -i https://esi.evetech.net/latest/swagger.json -o openapi
```
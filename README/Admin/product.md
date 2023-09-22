# Rust API - Admin-Api

## Create Product
__Url:__ http://127.0.0.1:3000/admin-api/category/{id}/product \
__Method:__ POST \
__Required:__
* Bearer Token
* read_category and write_category scopes

__Body:__
```json
{
  "sku": "102",
  "description": "Test Prod 3",
  "variants": [
    {
      "sku": "102-001",
      "description": "Variante 1",
      "price": 99.99,
      "image_url": "/102-001.jpeg"
    },
    {
      "sku": "102-002",
      "description": "Variante 2",
      "price": 94.99,
      "image_url": "/102-002.jpeg"
    }
  ]
}
```
__Response:__
```json
{
  "_id": "b765fdf4-d036-4354-aab5-553409835431",
  "description": "Test Category",
  "products": [
    {
      "_id": "feb5f1c7-d527-4f04-bcf0-b80799ca9c17",
      "sku": "100",
      "description": "Test Prod",
      "variants": [
        {
          "_id": "17dc7f3f-1131-48a5-82ae-6a3357bbc7d8",
          "sku": "100-001",
          "description": "Variante 1",
          "price": 69.99,
          "sale_price": null,
          "image_url": "/100-001.jpeg"
        }
      ]
    },
    {
      "_id": "1f17b5a4-3bf0-416b-ac35-b1cf7a51928f",
      "sku": "102",
      "description": "Test Prod 3",
      "variants": [
        {
          "_id": "347c56c5-86da-4fe4-92e9-da9ec6dc79a9",
          "sku": "102-001",
          "description": "Variante 1",
          "price": 99.99,
          "sale_price": null,
          "image_url": "/102-001.jpeg"
        },
        {
          "_id": "eb752a35-6363-4528-a85a-6e12ca8779ab",
          "sku": "102-002",
          "description": "Variante 2",
          "price": 94.99,
          "sale_price": null,
          "image_url": "/102-002.jpeg"
        }
      ]
    }
  ]
}
```
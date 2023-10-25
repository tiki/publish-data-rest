# Microblink Receipt Taxonomy

## Receipt
Describes a single receipt
| Field | Type | Description | Example |
| ----- | ---- | ----------- | ------- |
| receiptDate | [StringType](#StringType) | The purchase date found on the receipt formatted as MM/dd/yyyy | 10/24/2023 |
| receiptTime | [StringType](#StringType) | The purchase time found on the receipt formatted as HH:mm | 15:50 |
| retailerId | [Retailer](#Retailer) | Retailer auto-detection identifier | See [Retailer](#Retailer) |
| products | \[[Product](#Product)] | An array of products that were detected on the receipt | See [Product](#Product) |
| coupons | \[[Coupon](#Coupon)] | An array of coupons that were detected on the receipt | See [Coupon](#Coupon) |
| total | [FloatType](#FloatType) | The total detected on the receipt | 10.10 |
| tip | [FloatType](#FloatType) | The tip detected on the receipt | 10.10 |
| subtotal | [FloatType](#FloatType) | The subtotal detected on the receipt | 10.10 |
| taxes | [FloatType](#FloatType) | The tax detected on the receipt | 10.10 |
| storeNumber | [StringType](#StringType) | The store number detected on the receipt | 42 |
| merchantName | [StringType](#StringType) | The store name detected on the receipt | Taco Bell |
| storeAddress | [StringType](#StringType) | The store street address detected on the receipt | 2101 Rosa L. Parks Blvd. |
| storeCity | [StringType](#StringType) | The store city detected on the receipt | Nashville |
| blinkReceiptId | string | Unique ID for this receipt which allows BlinkReceipt to correspond scan data to the correct receipt image stored by client | 4eb3a6ee-ace5-45ef-90d7-702f57e935ec |
| storeState | [StringType](#StringType) | The store state detected on the receipt | TN |
| storeZip | [StringType](#StringType) | The store zipcode detected on the receipt | 37228 |
| storeCountry | [StringType](#StringType) | The store country detected on the receipt | USA |
| storePhone | [StringType](#StringType) | The store phone number detected on the receipt | 615-256-3382 |
| cashierId | [StringType](#StringType) | The cashier ID detected on the receipt | abc123 |
| transactionId | [StringType](#StringType) | The transaction ID detected on the receipt | abc123 |
| registerId | [StringType](#StringType) | The register ID detected on the receipt | abc123 |
| paymentMethods | \[[PaymentMethod](#PaymentMethod)] | An array of payment methods found on the receipt | See [PaymentMethod](#PaymentMethod) |
| taxId | [StringType](#StringType) | The tax ID of the retailer | ABC-123 |
| mallName | [StringType](#StringType) | The mall name in which the retailer is located | The Mall at Green Hills |
| last4cc | [StringType](#StringType) | The last 4 digits of the credit card used | 4242 |
| ocrConfidence | float32 | An average confidence (between 0 and 100) for the OCR performed on this receipt | 100 |
| merchantSource | string | Value indicating which merchant detection method succeeded in identifying the current merchant | phone |
| foundTopEdge | boolean | Indicates whether a top edge was found on any frame that was scanned in this session | false |
| foundBottomEdge | boolean | Indicates whether a bottom edge was found on any frame that was scanned in this session | false |
| eReceiptOrderNumber | string | For an Amazon or e-receipt order, this is the order number | abc123 |
| eReceiptOrderStatus | string | For an e-receipt order, this is the order status | shipped |
| eReceiptRawHtml | string | For an Amazon or e-receipt order, this is the raw HTML that was parsed | \<html>...<\html> |
| eReceiptShippingAddress | string | For an e-receipt order, this is the shipping address | 1814 Hayes Street, Nashville TN, 37203 |
| shipments | \[[Shipment](#Shipment)] | An array shipments discovered in this order | See [Shipment](#Shipment) |
| longTransactionId | [StringType](#StringType) | Some receipts contain a longer transaction number in addition to the standard one | abcxyz123456789 |
| subtotalMatches | boolean | Indicates whether the subtotal matches the sum of the products and coupons | true |
| eReceiptEmailProvider | string | The name of the email provider from which this e-receipt was obtained | gmail |
| eReceiptEmailId | string | The internal identifier of this email message from the provider | abc123 |
| eReceiptAuthenticated | boolean | Whether this e-receipt could be authenticated via DKIM or SPF headers | true |
| instacartShopper | boolean | Whether the purchase was made by an Instacart shopper | false |
| eReceipt | boolean | Indicates if the receipt is an e-receipt | true |
| duplicate | boolean | Indicate whether the receipt is a duplicate | false |
| fraudulent | boolean | Indicate whether the receipt is believed to be fraudulent | false |
| receiptDateTime | int64 | The receipt date time as a Unix timestamp (seconds since epoch) | 1698189352 |
| duplicateBlinkReceiptIds | [string] | Contains the Blink Receipt ID(s) of any duplicate receipts | [abc123, xyz456] |
| merchantMatchGuess | [StringType](#StringType) | When no merchant is detected using standard methods, the parser will occasionally return a best guess for the merchant based on receipt heuristics. | Taco Bell |
| productsPendingLookup | int32 | Indicates how many products were still awaiting product intelligence results when the scan session ended | 0 |
| qualifiedPromotions | \[[Promotion](#Promotion)] | Contains all the promotions that were validated | See [Promotion](#Promotion) |
| unqualifiedPromotions | \[[Promotion](#Promotion)] | Contains all the promotions that were NOT validated | See [Promotion](#Promotion) |
| extendedFields | Map<string, string> | Additional receipt fields | {field1, val1} |
| eReceiptAdditionalFees | Map<string, string> | Additional fees / charges for this e-receipt, if any (like tips, bag / bottle fees, etc…) Each key-value pair, represents the name of the fee and its amount { “Tips”: “2.19”, “Bag Fee”: “0.05” } | {Tips, 2.19} |
| purchaseType | [StringType](#StringType) | Represents the type of purchase (i.e., online, in-store) |  online |
| loyaltyForBanner | boolean | Indicates if the receipt is part of a loyalty program | false |
| channel | [StringType](#StringType) | The channel through which the receipt was obtained (i.e., email, app) | email |
| submissionDate | int64 | The submission date of the receipt as a Unix timestamp | 1698189352  |
| eReceiptFulfilledBy | string | The name of the 3rd party service (like Instacart) that fulfilled this order | Instacart |
| eReceiptShippingStatus | string | The shipping status of the e-receipt | delivered |
| eReceiptPOSSystem | string | The point of sale system used for this order | Toast |
| eReceiptSubMerchant | string | The name of the sub-merchant (like “McDonald’s” for some Uber Eats orders) | Taco Bell |
| qualifiedSurveys | \[[Survey](#Survey)] | All the surveys that were validated | See [Survey](#Survey) |
| barcode | string | The barcode detected on the receipt ( Code 39, Code 128, and ITF formats) | ABC-1234 |
| eReceiptMerchantEmail | string | The merchant's email address associated with the e-receipt. | hello@tacobell.com |
| eReceiptEmailSubject | string | The subject of the e-receipt email. | Your order has shipped |
| eReceiptShippingCosts | float32 | The shipping costs associated with the e-receipt. | 10.10 |
| currencyCode | string | The currency code associated with the receipt. | USD |
| clientMerchantName | [StringType](#StringType) | If there is client-specific version of a particular merchant, it will be passed back in this property. | Taco Bell |
| loyaltyProgram | boolean | Whether there is an indication of a loyalty program found on the receipt. | false |
| merchantSources | [int32] | An array indicating which merchant detection method(s) succeeded in identifying the current merchant. | [1, 2] |
| paymentTerminalId | [StringType](#StringType) | The unique identifier associated with the payment processor’s terminal | abc123 |
| paymentTransactionId | [StringType](#StringType) | The unique identifier associated with the payment processor’s transaction | abc123 |
| combinedRawText | [StringType](#StringType) | String representing cumulative set of raw results | d\nTellthebell.com\n\nSurvey Code: 9967 2032-1191 1427... |

## AdditionalLine
An additional line of text or information.

| Field | Type | Description | Example |
| ----- | ---- | ----------- | ------- |
| type | [StringType](#StringType) | Type or category of the additional line| item |
| text | string | The text content of the additional line | Chicken Quesadilla 0.00 |
| lineNumber | int32 | The unique identifier or line number | 1013 |

## Coupon
A coupon applied to a purchase.

| Field | Type | Description | Example |
| ----- | ---- | ----------- | ------- |
| type | string | The type or category of the coupon | fixed rate |
| amount | [FloatType](#FloatType) | The amount associated with the coupon | 10.10 |
| sku | [StringType](#StringType) | The SKU (Stock Keeping Unit) | XYZ12345 |
| description | [StringType](#StringType) | A description or additional information | $5 off a purchase of $50 or more |
| relatedProductIndex | int32 | The index of the related product for which the coupon is applicable | 1013 |

## PaymentMethod
The metadata for the payment method used.

| Field | Type | Description | Example |
| ----- | ---- | ----------- | ------- |
| paymentMethod | [StringType](#StringType) | The type of payment method used for the transaction | Credit |
| cardType | [StringType](#StringType) | The type of card used for payment, if applicable | Mastercard |
| cardIssuer | [StringType](#StringType) | The issuer of the card, if applicable | Barclays |
| amount | [StringType](#StringType) | The amount associated with the payment method | 10.10 |

## Promotion
A promotion applied to the purchase.

| Field | Type | Description | Example |
| ----- | ---- | ----------- | ------- |
| id | int64 | The unique identifier | 1013 |
| slug | string | The slug or short identifier | promo-1 |
| reward | float32 | The amount of the reward offered | 10.10 |
| rewardCurrency | string | The currency in which the reward is provided | USD |
| errorCode | int32 | The error code associated with the promotion | 123 |
| errorMessage | string | A message describing the error | promotion expired |
| relatedProductIndexes | [int32] | An array of related product indexes that the promotion is linked to | [123, 456] |
| qualifications | [][int32] | An array of arrays containing qualification details | [[123], [456]] |

## Product
Describes one product on a receipt

| Field | Type | Description | Example |
| ----- | ---- | ----------- | ------- |
| productNumber | [StringType](#StringType) | Product number (SKU, UPC, or other) found on the receipt | 00123456789012 |
| description | [StringType](#StringType) | Description of the product | CHK QUES COMBO |
| quantity | [FloatType](#FloatType) | Product quantity found on the receipt (can be # of items or an amount if units are weight) | 1.0 |
| unitPrice | [FloatType](#FloatType) | Unit price of the item | 10.10 |
| unitOfMeasure | [StringType](#StringType) | Unit of measure (i.e. pounds, ounces, packs, etc) | pounds |
| totalPrice | [FloatType](#FloatType) | Total price paid for this line item, taking into account quantities, weights, and discounts | 10.10 |
| fullPrice | float32 | The total price before any savings were applied to this item (to compute the discount on this item, subtract totalPrice from fullPrice) | 10.10 |
| line | int32 | The line number for the item | 1 |
| productName | string | Name of the product | Soft Taco |
| brand | string | Brand of the product | Taco Bell |
| category | string | Category of the product | Tacos & Burritos |
| size | string | Size of the product | LG |
| rewardsGroup | string | Reward group identifier | abc123 |
| competitorRewardsGroup | string | Competitor reward group identifier  | xyz567 |
| upc | string | The UPC number for the item | 00123456789012 |
| imageUrl | string | The product thumbnail URL (size may vary) | https://d1ralsognjng37.cloudfront.net/ba026203-2472-4b5f-a426-c89d0cf1d599.jpeg |
| shippingStatus | string | The status of the product shipment | delivered |
| additionalLines | \[[AdditionalLine](#AdditionalLine)] | Any additional line items for the product | See [AdditionalLine](#AdditionalLine) |
| priceAfterCoupons | [FloatType](#FloatType) | The price of the product after applied coupons  | 10.10 |
| voided | boolean | Indicates whether this is a voided product | false |
| probability | float64 | If this product is part of a different product’s possibleProducts array, then this property indicates an estimate of how likely this particular possible product is to match the description of the parent | 0.5 |
| sensitive | boolean | Indicates whether the current product is from a sensitive category | false |
| possibleProducts | \[[Product](#Product)] | An array of possible products if an exact match is not found | See [Product](#Product) |
| subProducts | \[[Product](#Product)] | An array of sub products tied to the current product | See [Product](#Product) |
| added | boolean | Indicates if the items was added to the receipt | false |
| blinkReceiptBrand | string | The blink receipt brand identifier | TACO_BELL |
| blinkReceiptCategory | string | The blink receipt category identifier | Food_Tacos_Burritos_Meat_Tacos |
| extendedFields | Map<string, string> | Additional product fields | { field1, val1 } |
| fuelType | string | If this product is fuel, this will be the type of fuel (e.g. “Regular”, “Unleaded”, etc) | Unleaded |
| descriptionPrefix | [StringType](#StringType) | The prefix found before the product description | TB |
| descriptionPostfix | [StringType](#StringType) | The postfix found after the product description | TB |
| skuPrefix | [StringType](#StringType) | The prefix found before the product description | TB |
| skuPostfix | [StringType](#StringType) | The postfix found after the SKU | TB |
| attributes | Map<string, string> | Zero or more attributes related to the product | { attr1, val1 } |
| sector | string | The product sector | Food |
| department | string | The product department | Tacos & Burritos |
| majorCategory | string | The product major category | Meat Tacos |
| subCategory | string | The product sub category | Quesadilla |
| itemType | string | The product item type | Taco |

## Retailer
A retailer identifier and banner identifier

| Field | Type | Description | Example |
| ----- | ---- | ----------- | ------- |
| id | int32 | The unique identifier | abc123 |
| bannerId | int32 | The associated banner | xyz567 |

## Shipment
The status of the shipment of the purchase

| Field | Type | Description | Example |
| ----- | ---- | ----------- | ------- |
| status | string | The status of the shipment | delivered |
| products | \[[Product](#Product)] | Array of products included in the shipment | See [Product](#Product) |

## Survey
A survey consisting of a series of multiple choice or open ended questions.

| Field | Type | Description | Example |
| ----- | ---- | ----------- | ------- |
| clientUserId | string | The client-specific user identifier | abc123 |
| slug | string | The slug or short identifier | survey-1 |
| rewardValue | float32 | The value of the reward offered for completion | 10.10 |
| startDate | string | The survey start date | 2023-09-24T18:57:02Z |
| endDate | string | The survey end date | 2023-10-24T18:57:02Z |
| questions | \[[SurveyQuestion](#SurveyQuestion)] | An array of questions contained in the survey | See [SurveyQuestion](#SurveyQuestion) |

## SurveyAnswer
A multiple choice answer to a survey question

| Field | Type | Description | Example |
| ----- | ---- | ----------- | ------- |
| number | int32 | The identifier for the answer | 1 |
| text | string | The text content of the answer option | blue |
| nextQuestionIndex | [int32] | The index of the next question in the survey | [2] |

## SurveyQuestion
A survey question, either multiple choice or open-ended

| Field | Type | Description | Example |
| ----- | ---- | ----------- | ------- |
| myIndex | int32 | The index of the question within the survey | 1 |
| lastQuestion | boolean | If the question is the last question in the survey | false |
| nextQuestionIndex | int32 | Index of the next question in the survey | 2 |
| text | string | The text of the question | what color is the sky? |
| type | string | Type or category of the question | open-ended |
| answers | \[[SurveyAnswer](#SurveyAnswer)] | An array of multiple choice answers | See [SurveyAnswer](#SurveyAnswer) |
| multipleAnswers | boolean | If the question allows multiple answers | false |
| totalNumberOfQuestions | int32 | Total number of questions in the survey | 2 |
| userResponse | [SurveyResponse](#SurveyResponse) | User's response to the survey | See [SurveyResponse](#SurveyResponse) |

## SurveyResponse
A user response to a survey question

| Field | Type | Description | Example |
| ----- | ---- | ----------- | ------- |
| answersSelected | [int32] | List of selected answers in the survey | [1] |
| freeText | string | Free-text comments provided | I'm color blind. |

## StringType
A type that holds information about a string value along with the OCR confidence level.

| Field | Type | Description | Example |
| ----- | ---- | ----------- | ------- |
| confidence | float32 | OCR accuracy confidence (0-100) | 100 |
| value | string | The parsed text value | Chicken Quesadilla |

## FloatType
A type that holds information about a floating-point value along with the OCR confidence level.

| Field | Type | Description | Example |
| ----- | ---- | ----------- | ------- |
| confidence | float32 | OCR accuracy confidence (0-100) | 100 |
| value | string | The parsed numeric value | 10.10 |

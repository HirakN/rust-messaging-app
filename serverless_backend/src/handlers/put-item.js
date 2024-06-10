// Create clients and set shared const values outside of the handler.

// Create a DocumentClient that represents the query to add an item
const dynamodb = require('aws-sdk/clients/dynamodb');
let ddb = new dynamodb({
    region: 'eu-west-1'
});
const docClient = new dynamodb.DocumentClient({
    service: ddb
});


// Get the DynamoDB table name from environment variables
const tableName = process.env.TABLE_NAME;

/**
 * A simple example includes a HTTP post method to add one item to a DynamoDB table.
 */
exports.putItemHandler = async (event) => {
    if (event.httpMethod !== 'POST') {
        throw new Error(`postMethod only accepts POST method, you tried: ${event.httpMethod} method.`);
    }
    // All log statements are written to CloudWatch
    console.log('received:', event.body);

    // Get id and name from the body of the request
    const data = JSON.parse(event.body);
    const { content, timestamp, session_token, name } = data;
    // Creates a new item, or replaces an old item with a new item
    // https://docs.aws.amazon.com/AWSJavaScriptSDK/latest/AWS/DynamoDB/DocumentClient.html#put-property
    let response = {};

    try {
        const params = {
            TableName: tableName,
            Item: { name, content, session_token, timestamp }
        };

        const result = await docClient.put(params).promise();

        response = {
            statusCode: 200,
            body: JSON.stringify(data)
        };
    } catch (ResourceNotFoundException) {
        console.log("===========")
        console.log(ResourceNotFoundException)
        console.log("===========")
        response = {
            statusCode: 404,
            body: "Unable to call DynamoDB. Table resource not found."
        };
    }

    // All log statements are written to CloudWatch
    console.info(`response from: ${event.path} statusCode: ${response.statusCode} body: ${response.body}`);
    return response;
};

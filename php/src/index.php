<?php
require __DIR__ . '/../vendor/autoload.php';
use SpecDb\Query\SearchResult;
use SpecDb\Query\SearchResultList;
use SpecDb\Query\SpecType;

// Send POST request
$client = new \GuzzleHttp\Client();
$response = $client->get('http://localhost:8082/v1/protobuf/search/i7', [
    'headers' => [
        // 'Content-Type' => 'application/x-protobuf'
    ],
    'body' => $binaryData
]);

// Get response body
$responseBinary = $response->getBody()->getContents();

// if response is not 200, the api might return a different message 

// message ErrorResponse {
//     ErrorEnum code = 1;        // e.g., HTTP or app-specific code
//     string message = 2;    // Human-readable
//     repeated string details = 3; // Optional extra info
// }
// enum ErrorEnum {
//     NO_RESULTS = 0;
//     UNKNOWN = 7;
// }


// Create a SearchResult message
$searchResultList = new SearchResultList();
$searchResultList->mergeFromString($responseBinary);

// Iterate results
foreach ($searchResultList->getResults() as $result) {
    echo '<ul>';
    /** @var \SpecDb\Query\SearchResult $result */
    echo "<li>Name: " . $result->getName() . '</li>';
    echo "<li>SpecType: " . SpecType::name($result->getSpecType()) . '</li>';
    echo "<li>HumanName: " . $result->getHumanName() . '</li>';
    echo '</ul>';
}

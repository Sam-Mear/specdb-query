<?php 
use GuzzleHttp\Exception\ClientException;
use GuzzleHttp\Exception\GuzzleException;
use SpecDb\Query\Apu;
use SpecDb\Query\ApuArchitecture;
use SpecDb\Query\Cpu;
use SpecDb\Query\CpuArchitecture;
use SpecDb\Query\GenericContainer;
use SpecDb\Query\GraphicsArchitecture;
use SpecDb\Query\GraphicsCard;
use SpecDb\Query\SearchResult;
use SpecDb\Query\SearchResultFullList;
use SpecDb\Query\SearchResultList;
use SpecDb\Query\SpecType;

function getSearch(): SearchResultList
{
    $query = $_GET['query'] ?? 'root';
    // Send POST request
    $client = new \GuzzleHttp\Client();
    try {
        $response = $client->get('http://localhost:8082/v1/protobuf/search/'.$query);
    } catch(GuzzleException $e) {
        return new SearchResultList();
    }

    // Get response body
    $responseBinary = $response->getBody()->getContents();

    // Create a SearchResult message
    $searchResultList = new SearchResultList();
    $searchResultList->mergeFromString($responseBinary);
    return $searchResultList;
}

function getSearchWithFullSpecs(): SearchResultFullList
{
    
    $query = $_GET['query'] ?? 'root';
    // Send POST request
    $client = new \GuzzleHttp\Client();
    try {
        $response = $client->get('http://localhost:8082/v1/protobuf/search_full_specs/'.$query);
    } catch(GuzzleException $e) {
        return new SearchResultFullList();
    }

    // Get response body
    $responseBinary = $response->getBody()->getContents();

    // Create a SearchResult message
    $searchResultList = new SearchResultFullList();
    $searchResultList->mergeFromString($responseBinary);
    return $searchResultList;
}

function getCpuDetails(string $name): Cpu
{
    $client = new \GuzzleHttp\Client();
    $response = $client->get('http://localhost:8082/v1/protobuf/cpu/'.$name);

    // Get response body
    $responseBinary = $response->getBody()->getContents();

    // Create a SearchResult message
    $cpu = new Cpu();
    $cpu->mergeFromString($responseBinary);
    return $cpu;
}

function getGraphicsCardDetails(string $name): GraphicsCard
{
    $client = new \GuzzleHttp\Client();
    $response = $client->get('http://localhost:8082/v1/protobuf/graphics_card/'.$name);

    // Get response body
    $responseBinary = $response->getBody()->getContents();

    // Create a SearchResult message
    $graphicsCard = new GraphicsCard();
    $graphicsCard->mergeFromString($responseBinary);
    return $graphicsCard;
}

function getApuDetails(string $name): Apu
{
    $client = new \GuzzleHttp\Client();
    $response = $client->get('http://localhost:8082/v1/protobuf/apu/'.$name);

    // Get response body
    $responseBinary = $response->getBody()->getContents();

    // Create a SearchResult message
    $apu = new Apu();
    $apu->mergeFromString($responseBinary);
    return $apu;
}

function getCpuArchitectureDetails(string $name): CpuArchitecture
{
    $client = new \GuzzleHttp\Client();
    $response = $client->get('http://localhost:8082/v1/protobuf/cpu_architecture/'.$name);

    // Get response body
    $responseBinary = $response->getBody()->getContents();

    // Create a SearchResult message
    $cpuArchitecture = new CpuArchitecture();
    $cpuArchitecture->mergeFromString($responseBinary);
    return $cpuArchitecture;
}

function getGraphicsArchitectureDetails(string $name): GraphicsArchitecture
{
    $client = new \GuzzleHttp\Client();
    $response = $client->get('http://localhost:8082/v1/protobuf/graphics_architecture/'.$name);

    // Get response body
    $responseBinary = $response->getBody()->getContents();

    // Create a SearchResult message
    $graphicsArchitecture = new GraphicsArchitecture();
    $graphicsArchitecture->mergeFromString($responseBinary);
    return $graphicsArchitecture;
}

function getApuArchitectureDetails(string $name): ApuArchitecture
{
    $client = new \GuzzleHttp\Client();
    $response = $client->get('http://localhost:8082/v1/protobuf/apu_architecture/'.$name);

    // Get response body
    $responseBinary = $response->getBody()->getContents();

    // Create a SearchResult message
    $apuArchitecture = new ApuArchitecture();
    $apuArchitecture->mergeFromString($responseBinary);
    return $apuArchitecture;
}


function getGenericContainerDetails(string $name): GenericContainer
{
    $client = new \GuzzleHttp\Client();
    $response = $client->get('http://localhost:8082/v1/protobuf/generic_container/'.$name);

    // Get response body
    $responseBinary = $response->getBody()->getContents();

    // Create a SearchResult message
    $genericContainer = new GenericContainer();
    $genericContainer->mergeFromString($responseBinary);
    return $genericContainer;
}

<?php
// This whole file with a query "ryzen7" can take 24 ms!
require __DIR__ . '/../vendor/autoload.php';

use SpecDb\Query\Cpu;
use SpecDb\Query\SearchResult;
use SpecDb\Query\SearchResultList;
use SpecDb\Query\SpecType;

function getSearch(): SearchResultList
{
    $query = $_GET['query'] ?? 'i7';
    // Send POST request
    $client = new \GuzzleHttp\Client();
    $response = $client->get('http://localhost:8082/v1/protobuf/search/'.$query);

    // Get response body
    $responseBinary = $response->getBody()->getContents();

    // Create a SearchResult message
    $searchResultList = new SearchResultList();
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
?>

<form>
    <input type="text" name="query" placeholder="Enter your query..." required>
    <button type="submit">Submit</button>
</form>


<?php

// Iterate results
foreach (getSearch()->getResults() as $result) {
    /** @var \SpecDb\Query\SearchResult $result */
    echo "<h2>Name: " . $result->getName() . '</h2>';
    echo "<li>SpecType: " . SpecType::name($result->getSpecType()) . '</li>';
    echo "<li>HumanName: " . $result->getHumanName() . '</li>';
    if ($result->getSpecType() === SpecType::SPEC_TYPE_CPU) {
        $cpu = getCpuDetails($result->getName());
        ?>

<table style="width:100%; border-collapse: collapse; font-family: Arial, sans-serif;">
    <thead>
        <tr style="background-color: #f4f4f4;">
            <th>Property</th>
            <th>Value</th>
        </tr>
    </thead>
    <tbody>
        <tr><td style="">Core Count</td><td><?= htmlspecialchars($cpu->getCoreCount()) ?></td></tr>
        <tr><td>Thread Count</td><td><?= htmlspecialchars($cpu->getThreadCount()) ?></td></tr>
        <tr><td>Base Frequency</td><td><?= htmlspecialchars($cpu->getBaseFrequency()) ?></td></tr>
        <tr><td>TDP</td><td><?= htmlspecialchars($cpu->getTdp()) ?></td></tr>
        <tr><td>Boost Frequency</td><td><?= htmlspecialchars($cpu->getBoostFrequency()) ?></td></tr>
        <tr><td>XFR Frequency</td><td><?= htmlspecialchars($cpu->getXfrFrequency()) ?></td></tr>
        <tr><td>Socket</td><td><?= htmlspecialchars($cpu->getSocket()) ?></td></tr>
        <tr><td>Stepping</td><td><?= htmlspecialchars($cpu->getStepping()) ?></td></tr>
        <tr><td>L1 Cache Data</td><td><?= htmlspecialchars($cpu->getL1CacheData()) ?></td></tr>
        <tr><td>L1 Cache Instruction</td><td><?= htmlspecialchars($cpu->getL1CacheInstruction()) ?></td></tr>
        <tr><td>L2 Cache Total</td><td><?= htmlspecialchars($cpu->getL2CacheTotal()) ?></td></tr>
        <tr><td>L3 Cache Total</td><td><?= htmlspecialchars($cpu->getL3CacheTotal()) ?></td></tr>
        <tr><td>Memory Type</td><td><?= htmlspecialchars($cpu->getMemoryType()) ?></td></tr>
        <tr><td>PCIe 5.0 Lanes</td><td><?= htmlspecialchars($cpu->getPcie50Lanes()) ?></td></tr>
        <tr><td>PCIe 4.0 Lanes</td><td><?= htmlspecialchars($cpu->getPcie40Lanes()) ?></td></tr>
        <tr><td>PCIe 3.0 Lanes</td><td><?= htmlspecialchars($cpu->getPcie30Lanes()) ?></td></tr>
        <tr><td>PCIe 2.0 Lanes</td><td><?= htmlspecialchars($cpu->getPcie20Lanes()) ?></td></tr>
        <tr><td>PCIe 1.0 Lanes</td><td><?= htmlspecialchars($cpu->getPcie10Lanes()) ?></td></tr>
        <tr><td>AVX/SSE/MMX</td><td><?= htmlspecialchars($cpu->getAvxSseMmx()) ?></td></tr>
        <tr><td>FMA4</td><td><?= htmlspecialchars($cpu->getFma4()) ?></td></tr>
        <tr><td>FMA3</td><td><?= htmlspecialchars($cpu->getFma3()) ?></td></tr>
        <tr><td>BMI</td><td><?= htmlspecialchars($cpu->getBmi()) ?></td></tr>
        <tr><td>AES</td><td><?= htmlspecialchars($cpu->getAes()) ?></td></tr>
        <tr><td>SHA</td><td><?= htmlspecialchars($cpu->getSha()) ?></td></tr>
        <tr><td>Other Extensions</td><td><?= htmlspecialchars(implode(', ', iterator_to_array($cpu->getOtherExtensions()))) ?></td></tr>
        <tr><td>Unlocked</td><td><?= htmlspecialchars($cpu->getUnlocked()) ?></td></tr>
        <tr><td>XFR Support</td><td><?= htmlspecialchars($cpu->getXfrSupport()) ?></td></tr>
        <tr><td>Max Memory Channels</td><td><?= htmlspecialchars($cpu->getMaxMemoryChannels()) ?></td></tr>
        <tr><td>Max Memory Frequency</td><td><?= htmlspecialchars($cpu->getMaxMemoryFrequency()) ?></td></tr>
        <tr><td>Compatible Chipsets</td><td><?= htmlspecialchars(implode(', ', iterator_to_array($cpu->getCompatableChipsets()))) ?></td></tr>
        <tr><td>Performance Core Base Frequency</td><td><?= htmlspecialchars($cpu->getPerformanceCoreBaseFrequency()) ?></td></tr>
        <tr><td>Efficient Core Base Frequency</td><td><?= htmlspecialchars($cpu->getEfficientCoreBaseFrequency()) ?></td></tr>
        <tr><td>Performance Core Boost Frequency</td><td><?= htmlspecialchars($cpu->getPerformanceCoreBoostFrequency()) ?></td></tr>
        <tr><td>Efficient Core Boost Frequency</td><td><?= htmlspecialchars($cpu->getEfficientCoreBoostFrequency()) ?></td></tr>
        <tr><td>Performance Core Count</td><td><?= htmlspecialchars($cpu->getPerformanceCoreCount()) ?></td></tr>
        <tr><td>Efficient Core Count</td><td><?= htmlspecialchars($cpu->getEfficientCoreCount()) ?></td></tr>
        <tr><td>Performance Thread Count</td><td><?= htmlspecialchars($cpu->getPerformanceThreadCount()) ?></td></tr>
        <tr><td>Efficient Thread Count</td><td><?= htmlspecialchars($cpu->getEfficientThreadCount()) ?></td></tr>
        <tr><td>CTDP Support</td><td><?= htmlspecialchars($cpu->getCtdpSupport()) ?></td></tr>
        <tr><td>Efficient Core Architecture</td><td><?= htmlspecialchars($cpu->getEfficientCoreArchitecture()) ?></td></tr>
        <tr><td>Manufacturer</td><td><?= htmlspecialchars($cpu->getManufacturer()) ?></td></tr>
        <tr><td>Market</td><td><?= htmlspecialchars(implode(', ', iterator_to_array($cpu->getMarket()))) ?></td></tr>
        <tr><td>Architecture</td><td><?= htmlspecialchars($cpu->getArchitecture()) ?></td></tr>
        <tr><td>Lithography</td><td><?= htmlspecialchars($cpu->getLithography()) ?></td></tr>
        <tr><td>Release Date</td><td><?= htmlspecialchars($cpu->getReleaseDate()) ?></td></tr>
    </tbody>
</table>


        <?php
    }
    
    echo '<hr>';
}






?>
<style>
  td,tr {
    border: 1px solid #ccc; padding: 8px;
  }  
</style>
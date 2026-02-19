<?php
use GuzzleHttp\Exception\ClientException;
use GuzzleHttp\Exception\GuzzleException;
use SpecDb\Query\Apu;
use SpecDb\Query\Cpu;
use SpecDb\Query\CpuArchitecture;
use SpecDb\Query\GraphicsCard;
use SpecDb\Query\SearchResult;
use SpecDb\Query\SearchResultList;
use SpecDb\Query\SpecType;

include(getcwd().'/../bootstrap.php');

$specPrinter = new SpecDbPrinter();

// Iterate results
foreach (getSearchWithFullSpecs()->getSearchResultFull() as $result) {
    /** @var \SpecDb\Query\SearchResultFull $result */
    echo "<h2>Name: " . $result->getName() . '</h2>';
    echo "<li>HumanName: " . $result->getHumanName() . '</li>';
    try {
        match ($result->getFullSpecs()) {
            'graphics_card' => $specPrinter->printGraphicsCardDetails($result->getGraphicsCard()),
            'cpu' => $specPrinter->printCpuDetails($result->getCpu()),
            'apu' => $specPrinter->printApuDetails($result->getApu()),
            'cpu_architecture' => $specPrinter->printCpuArchitectureDetails($result->getCpuArchitecture()),
            'apu_architecture' => $specPrinter->printApuArchitectureDetails($result->getApuArchitecture()),
            'graphics_architecture' => $specPrinter->printGraphicsArchitectureDetails($result->getGraphicsArchitecture()),
            'generic_container' => $specPrinter->printGenericContainerDetails($result->getGenericContainer()),
            'hidden' => null
        };
    } catch (ClientException $exception) {
        echo '<span class="error">'.$exception->getMessage()."</span>";
    }
    
    echo '<hr>';
}


?>
<style>
    :root {
        --colour-main: #ff4545;
    }
  td,tr {
    border: 1px solid #ccc; padding: 8px;
  }
    body {
        font-family: Arial, sans-serif;
        background-color: #f9f9f9;
        margin: 20px;
        color: #333;
    }

    form {
        display: flex;
        gap: 10px;
        margin-bottom: 20px;
    }

    input[type="text"] {
        flex: 1;
        padding: 10px;
        border: 1px solid #ccc;
        border-radius: 4px;
        font-size: 14px;
    }

    button {
        padding: 10px 20px;
        background-color: var(--colour-main);
        color: #fff;
        border: none;
        border-radius: 4px;
        font-size: 14px;
        cursor: pointer;
    }

    button:hover {
        background-color: var(--colour-main);
    }

    h2 {
        margin-top: 30px;
        color: var(--colour-main);
        font-size: 20px;
    }

    li {
        list-style: none;
        margin: 4px 0;
        font-size: 14px;
    }

    table {
        width: 100%;
        border-collapse: collapse;
        margin-top: 10px;
        background-color: #fff;
        box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    }

    thead tr {
        background-color: var(--colour-main);
        color: #fff;
        text-align: left;
    }

    th, td {
        padding: 10px;
        border: 1px solid #ddd;
        font-size: 14px;
    }

    tbody tr:nth-child(even) {
        background-color: #f4f4f4;
    }

    hr {
        margin: 30px 0;
        border: none;
        border-top: 1px solid #ccc;
    }

    .error {
        display: block;
        margin-top: 10px;
        color: red;
        font-weight: bold;
    }

</style>
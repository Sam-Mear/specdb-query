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

// Handle AddExtra form submission
$addExtraMessage = null;
if ($_SERVER['REQUEST_METHOD'] === 'POST' && ($_POST['action'] ?? '') === 'add_extra') {
    try {
        $req = new \SpecDb\Query\AddExtraRequest();
        $req->setSpecName($_POST['spec_name'] ?? '');
        $req->setSectionHeader($_POST['section_header'] ?? '');
        $req->setKey($_POST['key'] ?? '');

        $extra = new \SpecDb\Query\Extra();
        if (!empty($_POST['namespace'])) {
            $extra->setNamespace($_POST['namespace']);
        }
        // For this example we only support string extras via the form
        if (isset($_POST['string_value'])) {
            $extra->setStringValue($_POST['string_value']);
        }
        $req->setExtra($extra);

        $client = new \GuzzleHttp\Client();
        $response = $client->post('http://localhost:8082/v1/protobuf/extra', [
            'body' => $req->serializeToString(),
            'headers' => ['Content-Type' => 'application/octet-stream']
        ]);

        $respBody = $response->getBody()->getContents();
        $addResp = new \SpecDb\Query\AddExtraResponse();
        $addResp->mergeFromString($respBody);
        $addExtraMessage = 'Added: ' . ($addResp->getOk() ? 'ok' : 'failed') . ' - ' . $addResp->getMessage();
    } catch (GuzzleException $e) {
        $addExtraMessage = 'Request failed: ' . $e->getMessage();
    }
}

$specPrinter = new SpecDbPrinter();

// Simple form for adding an extra
if ($addExtraMessage !== null) {
    echo '<div style="padding:10px;background:#efe;">' . htmlspecialchars($addExtraMessage) . '</div>';
}
?>
<h3>Add Extra (example)</h3>
<form method="post">
    <input type="hidden" name="action" value="add_extra">
    <input type="text" name="spec_name" placeholder="Spec name (exact)" required>
    <input type="text" name="section_header" placeholder="Section header (e.g. 'Performance')" required>
    <input type="text" name="key" placeholder="Key (e.g. 'bench.1')" required>
    <input type="text" name="namespace" placeholder="Namespace (optional)">
    <input type="text" name="string_value" placeholder="String value (example)">
    <button type="submit">Add Extra</button>
</form>
<?php

// Iterate results
foreach (getSearch()->getResults() as $result) {
    /** @var \SpecDb\Query\SearchResult $result */
    echo "<h2>Name: " . $result->getName() . '</h2>';
    echo "<li>SpecType: " . SpecType::name($result->getSpecType()) . '</li>';
    echo "<li>HumanName: " . $result->getHumanName() . '</li>';
    try {
        match ($result->getSpecType()) {
            SpecType::SPEC_TYPE_CPU => $specPrinter->printCpuDetails(getCpuDetails($result->getName())),
            SpecType::SPEC_TYPE_GRAPHICS_CARD => $specPrinter->printGraphicsCardDetails(getGraphicsCardDetails($result->getName())),
            SpecType::SPEC_TYPE_APU => $specPrinter->printApuDetails(getApuDetails($result->getName())),
            SpecType::SPEC_TYPE_CPU_ARCHITECTURE => $specPrinter->printCpuArchitectureDetails(getCpuArchitectureDetails($result->getName())),
            SpecType::SPEC_TYPE_APU_ARCHITECTURE => $specPrinter->printApuArchitectureDetails(getApuArchitectureDetails($result->getName())),
            SpecType::SPEC_TYPE_GRAPHICS_ARCHITECTURE => $specPrinter->printGraphicsArchitectureDetails(getGraphicsArchitectureDetails($result->getName())),
            SpecType::SPEC_TYPE_GENERIC_CONTAINER => $specPrinter->printGenericContainerDetails(getGenericContainerDetails($result->getName())),
            SpecType::SPEC_TYPE_HIDDEN => null
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
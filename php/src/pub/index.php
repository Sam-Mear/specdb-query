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
$importMessage = null;

// Export via GET ?export_spec=NAME -> stream download
if (isset($_GET['export_spec'])) {
    $specToExport = $_GET['export_spec'];
    try {
        $client = new \GuzzleHttp\Client();
        $response = $client->get('http://localhost:8082/v1/protobuf/extra/export/'.rawurlencode($specToExport));
        $body = $response->getBody()->getContents();
        header('Content-Type: application/octet-stream');
        header('Content-Disposition: attachment; filename="extras_'.basename($specToExport).'.pb"');
        echo $body;
        exit;
    } catch (GuzzleException $e) {
        $importMessage = 'Export failed: ' . $e->getMessage();
    }
}

// Export All via GET ?export_all=1 -> stream JSON download
if (isset($_GET['export_all'])) {
    try {
        $client = new \GuzzleHttp\Client();
        $response = $client->get('http://localhost:8082/v1/protobuf/extra/export_all');
        $body = $response->getBody()->getContents();
        header('Content-Type: application/json');
        header('Content-Disposition: attachment; filename="extras_all.json"');
        echo $body;
        exit;
    } catch (GuzzleException $e) {
        $importMessage = 'Export all failed: ' . $e->getMessage();
    }
}

// Handle import file upload or add_extra POST
if ($_SERVER['REQUEST_METHOD'] === 'POST') {
    if (($_POST['action'] ?? '') === 'add_extra') {
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

    if (($_POST['action'] ?? '') === 'import_extras') {
        // file upload handling
        if (!isset($_FILES['extras_file']) || $_FILES['extras_file']['error'] !== UPLOAD_ERR_OK) {
            $importMessage = 'No file uploaded';
        } else {
            $specName = $_POST['spec_name_import'] ?? '';
            $fileContents = file_get_contents($_FILES['extras_file']['tmp_name']);
            try {
                $client = new \GuzzleHttp\Client();
                $response = $client->post('http://localhost:8082/v1/protobuf/extra/import/'.rawurlencode($specName), [
                    'body' => $fileContents,
                    'headers' => ['Content-Type' => 'application/octet-stream']
                ]);
                $importMessage = 'Import response status: ' . $response->getStatusCode();
            } catch (GuzzleException $e) {
                $importMessage = 'Import failed: ' . $e->getMessage();
            }
        }
    }

    if (($_POST['action'] ?? '') === 'import_all_extras') {
        if (!isset($_FILES['extras_all_file']) || $_FILES['extras_all_file']['error'] !== UPLOAD_ERR_OK) {
            $importMessage = 'No file uploaded for import all';
        } else {
            $fileContents = file_get_contents($_FILES['extras_all_file']['tmp_name']);
            try {
                $client = new \GuzzleHttp\Client();
                $response = $client->post('http://localhost:8082/v1/protobuf/extra/import_all', [
                    'body' => $fileContents,
                    'headers' => ['Content-Type' => 'application/json']
                ]);
                $importMessage = 'Import all response status: ' . $response->getStatusCode();
            } catch (GuzzleException $e) {
                $importMessage = 'Import all failed: ' . $e->getMessage();
            }
        }
    }
}

$specPrinter = new SpecDbPrinter();

// Simple form for adding an extra
if ($addExtraMessage !== null) {
    echo '<div style="padding:10px;background:#efe;">' . htmlspecialchars($addExtraMessage) . '</div>';
}
if ($importMessage !== null) {
    echo '<div style="padding:10px;background:#eef;">' . htmlspecialchars($importMessage) . '</div>';
}
?>
<h3>Import/Export All Extras</h3>
<form method="get" style="margin-bottom:10px;">
    <input type="hidden" name="export_all" value="1">
    <button type="submit">Export All Extras (JSON)</button>
</form>
<form method="post" enctype="multipart/form-data" style="margin-bottom:20px;">
    <input type="hidden" name="action" value="import_all_extras">
    <input type="file" name="extras_all_file" required>
    <button type="submit">Import All Extras (JSON)</button>
</form>
<?php
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
    
    // Export / Import UI for this spec
    echo '<div style="margin:8px 0;">';
    // Export button (GET will trigger download)
    echo '<form method="get" style="display:inline-block;margin-right:8px;">';
    echo '<input type="hidden" name="export_spec" value="'.htmlspecialchars($result->getName()).'">';
    echo '<button type="submit">Export Extras</button>';
    echo '</form>';

    // Import form - upload file and POST to PHP which forwards to API
    echo '<form method="post" enctype="multipart/form-data" style="display:inline-block;">';
    echo '<input type="hidden" name="action" value="import_extras">';
    echo '<input type="hidden" name="spec_name_import" value="'.htmlspecialchars($result->getName()).'">';
    echo '<input type="file" name="extras_file" required style="display:inline-block;">';
    echo '<button type="submit">Import Extras</button>';
    echo '</form>';

    echo '</div>';

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
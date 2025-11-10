<?php
// This whole file with a query "ryzen7" can take 24 ms!
require __DIR__ . '/../vendor/autoload.php';

use SpecDb\Query\Apu;
use SpecDb\Query\Cpu;
use SpecDb\Query\GraphicsCard;
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

function printCpuDetails(Cpu $cpu) {
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

function printGraphicsCardDetails(GraphicsCard $graphicsCard) {
    ?>
    <table style="width:100%; border-collapse: collapse; font-family: Arial, sans-serif;">
        <thead>
            <tr style="background-color: #f4f4f4;">
                <th>Property</th>
                <th>Value</th>
            </tr>
        </thead>
        <tbody>
            <tr><td>VRAM Capacity</td><td><?= htmlspecialchars($graphicsCard->getVramCapacity()) ?></td></tr>
            <tr><td>Shader Processor Count</td><td><?= htmlspecialchars($graphicsCard->getShaderProcessorCount()) ?></td></tr>
            <tr><td>GPU Base Frequency</td><td><?= htmlspecialchars($graphicsCard->getGpuBaseFrequency()) ?></td></tr>
            <tr><td>Manufacturer</td><td><?= htmlspecialchars($graphicsCard->getManufacturer()) ?></td></tr>
            <tr><td>Vendor</td><td><?= htmlspecialchars($graphicsCard->getVendor()) ?></td></tr>
            <tr><td>Market</td><td><?= htmlspecialchars(implode(', ', iterator_to_array($graphicsCard->getMarket()))) ?></td></tr>
            <tr><td>Architecture</td><td><?= htmlspecialchars($graphicsCard->getArchitecture()) ?></td></tr>
            <tr><td>Lithography</td><td><?= htmlspecialchars($graphicsCard->getLithography()) ?></td></tr>
            <tr><td>Release Date</td><td><?= htmlspecialchars($graphicsCard->getReleaseDate()) ?></td></tr>
            <tr><td>DirectX Support</td><td><?= htmlspecialchars($graphicsCard->getDirectXSupport()) ?></td></tr>
            <tr><td>OpenGL Support</td><td><?= htmlspecialchars($graphicsCard->getOpenGlSupport()) ?></td></tr>
            <tr><td>OpenCL Support</td><td><?= htmlspecialchars($graphicsCard->getOpenClSupport()) ?></td></tr>
            <tr><td>Vulkan Support</td><td><?= htmlspecialchars($graphicsCard->getVulkanSupport()) ?></td></tr>
            <tr><td>VRAM Frequency</td><td><?= htmlspecialchars($graphicsCard->getVramFrequency()) ?></td></tr>
            <tr><td>VRAM Type</td><td><?= htmlspecialchars($graphicsCard->getVramType()) ?></td></tr>
            <tr><td>VRAM Bandwidth</td><td><?= htmlspecialchars($graphicsCard->getVramBandwidth()) ?></td></tr>
            <tr><td>VRAM Bus Width</td><td><?= htmlspecialchars($graphicsCard->getVramBusWidth()) ?></td></tr>
            <tr><td>Render Output Units</td><td><?= htmlspecialchars($graphicsCard->getRenderOutputUnitCount()) ?></td></tr>
            <tr><td>Texture Mapping Units</td><td><?= htmlspecialchars($graphicsCard->getTextureMappingUnitCount()) ?></td></tr>
            <tr><td>Die Size</td><td><?= htmlspecialchars($graphicsCard->getDieSize()) ?></td></tr>
            <tr><td>TDP</td><td><?= htmlspecialchars($graphicsCard->getTdp()) ?></td></tr>
            <tr><td>GPU</td><td><?= htmlspecialchars($graphicsCard->getGpu()) ?></td></tr>
            <tr><td>GPU Variant</td><td><?= htmlspecialchars($graphicsCard->getGpuVariant()) ?></td></tr>
            <tr><td>GPU Model</td><td><?= htmlspecialchars($graphicsCard->getGpuModel()) ?></td></tr>
            <tr><td>HLSL Shader Model</td><td><?= htmlspecialchars($graphicsCard->getHlslShaderModel()) ?></td></tr>
            <tr><td>GPU Boost Frequency</td><td><?= htmlspecialchars($graphicsCard->getGpuBoostFrequency()) ?></td></tr>
            <tr><td>FP32 Compute</td><td><?= htmlspecialchars($graphicsCard->getFp32Compute()) ?></td></tr>
            <tr><td>FP64 Compute</td><td><?= htmlspecialchars($graphicsCard->getFp64Compute()) ?></td></tr>
            <tr><td>Slot Width</td><td><?= htmlspecialchars($graphicsCard->getSlotWidth()) ?></td></tr>
            <tr><td>Outputs</td><td><?= htmlspecialchars(implode(', ', iterator_to_array($graphicsCard->getOutputs()))) ?></td></tr>
            <tr><td>Power Connectors</td><td><?= htmlspecialchars(implode(', ', iterator_to_array($graphicsCard->getPowerConnectors()))) ?></td></tr>
            <tr><td>Length</td><td><?= htmlspecialchars($graphicsCard->getLength()) ?></td></tr>
            <tr><td>Height</td><td><?= htmlspecialchars($graphicsCard->getHeight()) ?></td></tr>
            <tr><td>Width</td><td><?= htmlspecialchars($graphicsCard->getWidth()) ?></td></tr>
            <tr><td>Ray Tracing Cores</td><td><?= htmlspecialchars($graphicsCard->getRayTracingCores()) ?></td></tr>
            <tr><td>Tensor Cores</td><td><?= htmlspecialchars($graphicsCard->getTensorCores()) ?></td></tr>
            <tr><td>Hardware Accelerated Encoding</td><td><?= htmlspecialchars(implode(', ', iterator_to_array($graphicsCard->getHardwareAcceleratedEncoding()))) ?></td></tr>
            <tr><td>Hardware Accelerated Decoding</td><td><?= htmlspecialchars(implode(', ', iterator_to_array($graphicsCard->getHardwareAcceleratedDecoding()))) ?></td></tr>
            <tr><td>Module Count</td><td><?= htmlspecialchars($graphicsCard->getModuleCount()) ?></td></tr>
            <tr><td>Pixel Shaders</td><td><?= htmlspecialchars($graphicsCard->getPixelShaders()) ?></td></tr>
            <tr><td>Maximum VRAM Capacity</td><td><?= htmlspecialchars($graphicsCard->getMaximumVramCapacity()) ?></td></tr>
            <tr><td>Max Displays</td><td><?= htmlspecialchars($graphicsCard->getMaxDisplays()) ?></td></tr>
            <tr><td>Crossfire Support</td><td><?= htmlspecialchars($graphicsCard->getCrossfireSupport()) ?></td></tr>
            <tr><td>FreeSync Support</td><td><?= htmlspecialchars($graphicsCard->getFreeSyncSupport()) ?></td></tr>
        </tbody>
    </table> <?php
}

function printApuDetails(Apu $apu) {
    ?>
    <table style="width:100%; border-collapse: collapse; font-family: Arial, sans-serif;">
        <thead>
            <tr style="background-color: #f4f4f4;">
                <th>Property</th>
                <th>Value</th>
            </tr>
        </thead>
        <tbody>
            <tr><td style="">Core Count</td><td><?= htmlspecialchars($apu->getCoreCount()) ?></td></tr>
            <tr><td>Thread Count</td><td><?= htmlspecialchars($apu->getThreadCount()) ?></td></tr>
            <tr><td>Base Frequency</td><td><?= htmlspecialchars($apu->getBaseFrequency()) ?></td></tr>
            <tr><td>TDP</td><td><?= htmlspecialchars($apu->getTdp()) ?></td></tr>
            <tr><td>Boost Frequency</td><td><?= htmlspecialchars($apu->getBoostFrequency()) ?></td></tr>
            <tr><td>XFR Frequency</td><td><?= htmlspecialchars($apu->getXfrFrequency()) ?></td></tr>
            <tr><td>Socket</td><td><?= htmlspecialchars($apu->getSocket()) ?></td></tr>
            <tr><td>Stepping</td><td><?= htmlspecialchars($apu->getStepping()) ?></td></tr>
            <tr><td>L1 Cache Data</td><td><?= htmlspecialchars($apu->getL1CacheData()) ?></td></tr>
            <tr><td>L1 Cache Instruction</td><td><?= htmlspecialchars($apu->getL1CacheInstruction()) ?></td></tr>
            <tr><td>L2 Cache Total</td><td><?= htmlspecialchars($apu->getL2CacheTotal()) ?></td></tr>
            <tr><td>L3 Cache Total</td><td><?= htmlspecialchars($apu->getL3CacheTotal()) ?></td></tr>
            <tr><td>Memory Type</td><td><?= htmlspecialchars($apu->getMemoryType()) ?></td></tr>
            <tr><td>PCIe 5.0 Lanes</td><td><?= htmlspecialchars($apu->getPcie50Lanes()) ?></td></tr>
            <tr><td>PCIe 4.0 Lanes</td><td><?= htmlspecialchars($apu->getPcie40Lanes()) ?></td></tr>
            <tr><td>PCIe 3.0 Lanes</td><td><?= htmlspecialchars($apu->getPcie30Lanes()) ?></td></tr>
            <tr><td>PCIe 2.0 Lanes</td><td><?= htmlspecialchars($apu->getPcie20Lanes()) ?></td></tr>
            <tr><td>PCIe 1.0 Lanes</td><td><?= htmlspecialchars($apu->getPcie10Lanes()) ?></td></tr>
            <tr><td>AVX/SSE/MMX</td><td><?= htmlspecialchars($apu->getAvxSseMmx()) ?></td></tr>
            <tr><td>FMA4</td><td><?= htmlspecialchars($apu->getFma4()) ?></td></tr>
            <tr><td>FMA3</td><td><?= htmlspecialchars($apu->getFma3()) ?></td></tr>
            <tr><td>BMI</td><td><?= htmlspecialchars($apu->getBmi()) ?></td></tr>
            <tr><td>AES</td><td><?= htmlspecialchars($apu->getAes()) ?></td></tr>
            <tr><td>SHA</td><td><?= htmlspecialchars($apu->getSha()) ?></td></tr>
            <tr><td>Other Extensions</td><td><?= htmlspecialchars(implode(', ', iterator_to_array($apu->getOtherExtensions()))) ?></td></tr>
            <tr><td>Unlocked</td><td><?= htmlspecialchars($apu->getUnlocked()) ?></td></tr>
            <tr><td>XFR Support</td><td><?= htmlspecialchars($apu->getXfrSupport()) ?></td></tr>
            <tr><td>Max Memory Channels</td><td><?= htmlspecialchars($apu->getMaxMemoryChannels()) ?></td></tr>
            <tr><td>Max Memory Frequency</td><td><?= htmlspecialchars($apu->getMaxMemoryFrequency()) ?></td></tr>
            <tr><td>Compatible Chipsets</td><td><?= htmlspecialchars(implode(', ', iterator_to_array($apu->getCompatableChipsets()))) ?></td></tr>
            <tr><td>Performance Core Base Frequency</td><td><?= htmlspecialchars($apu->getPerformanceCoreBaseFrequency()) ?></td></tr>
            <tr><td>Efficient Core Base Frequency</td><td><?= htmlspecialchars($apu->getEfficientCoreBaseFrequency()) ?></td></tr>
            <tr><td>Performance Core Boost Frequency</td><td><?= htmlspecialchars($apu->getPerformanceCoreBoostFrequency()) ?></td></tr>
            <tr><td>Efficient Core Boost Frequency</td><td><?= htmlspecialchars($apu->getEfficientCoreBoostFrequency()) ?></td></tr>
            <tr><td>Performance Core Count</td><td><?= htmlspecialchars($apu->getPerformanceCoreCount()) ?></td></tr>
            <tr><td>Efficient Core Count</td><td><?= htmlspecialchars($apu->getEfficientCoreCount()) ?></td></tr>
            <tr><td>Performance Thread Count</td><td><?= htmlspecialchars($apu->getPerformanceThreadCount()) ?></td></tr>
            <tr><td>Efficient Thread Count</td><td><?= htmlspecialchars($apu->getEfficientThreadCount()) ?></td></tr>
            <tr><td>CTDP Support</td><td><?= htmlspecialchars($apu->getCtdpSupport()) ?></td></tr>
            <tr><td>Efficient Core Architecture</td><td><?= htmlspecialchars($apu->getEfficientCoreArchitecture()) ?></td></tr>
            <tr><td>Manufacturer</td><td><?= htmlspecialchars($apu->getManufacturer()) ?></td></tr>
            <tr><td>Market</td><td><?= htmlspecialchars(implode(', ', iterator_to_array($apu->getMarket()))) ?></td></tr>
            <tr><td>Architecture</td><td><?= htmlspecialchars($apu->getArchitecture()) ?></td></tr>
            <tr><td>Lithography</td><td><?= htmlspecialchars($apu->getLithography()) ?></td></tr>
            <tr><td>Release Date</td><td><?= htmlspecialchars($apu->getReleaseDate()) ?></td></tr>
            <tr><td>Shader Processor Count</td><td><?= htmlspecialchars($apu->getShaderProcessorCount()) ?></td></tr>
            <tr><td>GPU Base Frequency</td><td><?= htmlspecialchars($apu->getGpuBaseFrequency()) ?></td></tr>
            <tr><td>Manufacturer</td><td><?= htmlspecialchars($apu->getManufacturer()) ?></td></tr>
            <tr><td>Market</td><td><?= htmlspecialchars(implode(', ', iterator_to_array($apu->getMarket()))) ?></td></tr>
            <tr><td>Architecture</td><td><?= htmlspecialchars($apu->getArchitecture()) ?></td></tr>
            <tr><td>Lithography</td><td><?= htmlspecialchars($apu->getLithography()) ?></td></tr>
            <tr><td>Release Date</td><td><?= htmlspecialchars($apu->getReleaseDate()) ?></td></tr>
            <tr><td>DirectX Support</td><td><?= htmlspecialchars($apu->getDirectXSupport()) ?></td></tr>
            <tr><td>OpenGL Support</td><td><?= htmlspecialchars($apu->getOpenGlSupport()) ?></td></tr>
            <tr><td>OpenCL Support</td><td><?= htmlspecialchars($apu->getOpenClSupport()) ?></td></tr>
            <tr><td>Vulkan Support</td><td><?= htmlspecialchars($apu->getVulkanSupport()) ?></td></tr>
            <tr><td>VRAM Type</td><td><?= htmlspecialchars($apu->getVramType()) ?></td></tr>
            <tr><td>Render Output Units</td><td><?= htmlspecialchars($apu->getRenderOutputUnitCount()) ?></td></tr>
            <tr><td>Texture Mapping Units</td><td><?= htmlspecialchars($apu->getTextureMappingUnitCount()) ?></td></tr>
            <tr><td>TDP</td><td><?= htmlspecialchars($apu->getTdp()) ?></td></tr>
            <tr><td>GPU Model</td><td><?= htmlspecialchars($apu->getGpuModel()) ?></td></tr>
            <tr><td>HLSL Shader Model</td><td><?= htmlspecialchars($apu->getHlslShaderModel()) ?></td></tr>
            <tr><td>GPU Boost Frequency</td><td><?= htmlspecialchars($apu->getGpuBoostFrequency()) ?></td></tr>
            <tr><td>Ray Tracing Cores</td><td><?= htmlspecialchars($apu->getRayTracingCores()) ?></td></tr>
            <tr><td>Tensor Cores</td><td><?= htmlspecialchars($apu->getTensorCores()) ?></td></tr>
            <tr><td>Hardware Accelerated Encoding</td><td><?= htmlspecialchars(implode(', ', iterator_to_array($apu->getHardwareAcceleratedEncoding()))) ?></td></tr>
            <tr><td>Hardware Accelerated Decoding</td><td><?= htmlspecialchars(implode(', ', iterator_to_array($apu->getHardwareAcceleratedDecoding()))) ?></td></tr>
            <tr><td>Module Count</td><td><?= htmlspecialchars($apu->getModuleCount()) ?></td></tr>
            <tr><td>Pixel Shaders</td><td><?= htmlspecialchars($apu->getPixelShaders()) ?></td></tr>
            <tr><td>Max Displays</td><td><?= htmlspecialchars($apu->getMaxDisplays()) ?></td></tr>
            <tr><td>Crossfire Support</td><td><?= htmlspecialchars($apu->getCrossfireSupport()) ?></td></tr>
            <tr><td>FreeSync Support</td><td><?= htmlspecialchars($apu->getFreeSyncSupport()) ?></td></tr>
        </tbody>
    </table> <?php
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
        printCpuDetails($cpu);
    } elseif ($result->getSpecType() === SpecType::SPEC_TYPE_GRAPHICS_CARD) {
        $graphicsCard = getGraphicsCardDetails($result->getName());
        printGraphicsCardDetails($graphicsCard);
    } elseif ($result->getSpecType() === SpecType::SPEC_TYPE_APU) {
        $apu = getApuDetails($result->getName());
        printApuDetails($apu);
    }
    
    echo '<hr>';
}






?>
<style>
  td,tr {
    border: 1px solid #ccc; padding: 8px;
  }  
</style>
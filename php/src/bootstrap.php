<?php
require __DIR__ . '/../vendor/autoload.php';

include(__DIR__ . '/getFunctions.php');
include(__DIR__ . '/printFunctions.php');

?>

<form>
    <input type="text" name="query" placeholder="Enter your query..." required>
    <button type="submit">Submit</button>
</form>

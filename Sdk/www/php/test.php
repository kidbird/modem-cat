<?php

    if (isset($_POST['testbtn']) and $_POST['testbtn'] == '测试') {
        $host = $_POST['host'];
        $tif = $_POST['tif'];
        showTestParm();
        pingAddress($host, $tif);
    } else {
        showTestParm();
    }

?>

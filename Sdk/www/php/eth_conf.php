<?php

    echo '<div class="page-header"><h1>LAN配置</h1></div>';

    if (isset($_POST['applyethconf'])) {
        $method = $_POST['optradio'];
        switch ($method) {
            case 'ECM':
                $returnJson = shell_exec('quec_app usbnet set ECM');
                $returnList = json_decode($returnJson, true);
                if ($returnList['status'] == 1) {
                    echo '<script>alert("切换成功");</script>';
                } else {
                    echo '<script>alert("切换失败");</script>';
                }
                break;
            case 'RNDIS':
                $returnJson = shell_exec('quec_app usbnet set RNDIS');
                $returnList = json_decode($returnJson, true);
                if ($returnList['status'] == 1) {
                    echo '<script>alert("切换成功");</script>';
                } else {
                    echo '<script>alert("切换失败");</script>';
                }
                break;
            case 'NCM':
                $returnJson = shell_exec('quec_app usbnet set NCM');
                $returnList = json_decode($returnJson, true);
                if ($returnList['status'] == 1) {
                    echo '<script>alert("切换成功");</script>';
                } else {
                    echo '<script>alert("切换失败");</script>';
                }
                break;
        }
    }
    SwitchEtherInfo($method);
?>
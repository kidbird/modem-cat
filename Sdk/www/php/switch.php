<?php

    echo '<div class="page-header"><h1>连接方式</h1></div>';

    if (isset($_POST['apply'])) {
        $method = $_POST['optradio'];
        $usbnetMethod = $_POST['usbnetoptradio'];

        $resultMessage = '';

        $changeSuccess = false;

        switch ($method) {
            case 'nat0':
                $returnJson = shell_exec('quec_app conn set nat0');
                $returnList = json_decode($returnJson, true);
                if ($returnList['status'] == 1) {
                    $changeSuccess = true;
                    $resultMessage .= '切换连接/拨号方式成功';
                } else {
                    $resultMessage .= '切换连接/拨号方式失败';
                }
                break;
            case 'nat1':
                $returnJson = shell_exec('quec_app conn set nat1');
                $returnList = json_decode($returnJson, true);
                if ($returnList['status'] == 1) {
                    $changeSuccess = true;
                    $resultMessage .= '切换连接/拨号方式成功';
                } else {
                    $resultMessage .= '切换连接/拨号方式失败';
                }
                break;
            case 'nat2':
                $returnJson = shell_exec('quec_app conn set nat2');
                $returnList = json_decode($returnJson, true);
                if ($returnList['status'] == 1) {
                    $changeSuccess = true;
                    $resultMessage .= '切换连接/拨号方式成功';
                } else {
                    $resultMessage .= '切换连接/拨号方式失败';
                }
                break;
        }

        switch ($usbnetMethod) {
            case 'ECM':
                $returnJson = shell_exec('quec_app usbnet set ECM');
                $returnList = json_decode($returnJson, true);
                if ($returnList['status'] == 1) {
                    $changeSuccess = true;
                    $resultMessage .= ', 切换连接/拨号方式成功';
                } else {
                    $resultMessage .= ', 切换连接/拨号方式失败';
                }
                break;
            case 'RNDIS':
                $returnJson = shell_exec('quec_app usbnet set RNDIS');
                $returnList = json_decode($returnJson, true);
                if ($returnList['status'] == 1) {
                    $changeSuccess = true;
                    $resultMessage .= ', 切换连接/拨号方式成功';
                } else {
                    $resultMessage .= ', 切换连接/拨号方式失败';
                }
                break;
            case 'NCM':
                $returnJson = shell_exec('quec_app usbnet set NCM');
                $returnList = json_decode($returnJson, true);
                if ($returnList['status'] == 1) {
                    $changeSuccess = true;
                    $resultMessage .= ', 切换连接/拨号方式成功';
                } else {
                    $resultMessage .= ', 切换连接/拨号方式失败';
                }
                break;
        }

        echo '<script>alert("'.$resultMessage.', 设备将自动重启");</script>';

        if ($changeSuccess) {
            shell_exec('reboot');
        }
    }

    ShowSwitch($method, $usbnetMethod);
    //SwitchEtherInfo($method);
?>

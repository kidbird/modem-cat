<?php

    echo '<div class="page-header"><h1>云服务</h1></div>';

    if (isset($_POST['applymqttinfo'])) {
        $enableMqtt = $_POST['enableMqtt'];

        $docObj = GetMqttXMLDoc();
        $doc = $docObj['doc'];
        $file = $docObj['file'];
        $root = $doc->documentElement;

        $mqttInfo = CheckAndCreateNode($doc, $root->getElementsByTagName('mqtt')->item(0), $root, 'mqtt');

        $arixolinkInfo = CheckAndCreateNode($doc, $root->getElementsByTagName('arixolink')->item(0), $root, 'arixolink');

        $clientIdNode = CheckAndCreateNode($doc, $mqttInfo->getElementsByTagName('client_id')->item(0), $mqttInfo, 'client_id');
        $clientIdNode->nodeValue = $_POST['clientId'];

        $mqttIpNode = CheckAndCreateNode($doc, $mqttInfo->getElementsByTagName('mqtt_ip')->item(0), $mqttInfo, 'mqtt_ip');
        $mqttIpNode->nodeValue = $_POST['mqttIp'];

        $mqttPortNode = CheckAndCreateNode($doc, $mqttInfo->getElementsByTagName('mqtt_port')->item(0), $mqttInfo, 'mqtt_port');
        $mqttPortNode->nodeValue = $_POST['mqttPort'];

        $usernameNode = CheckAndCreateNode($doc, $mqttInfo->getElementsByTagName('mqtt_user')->item(0), $mqttInfo, 'mqtt_user');
        $usernameNode->nodeValue = $_POST['username'];

        $passwordNode = CheckAndCreateNode($doc, $mqttInfo->getElementsByTagName('mqtt_pwd')->item(0), $mqttInfo, 'mqtt_pwd');
        $passwordNode->nodeValue = $_POST['password'];

        $keepaliveNode = CheckAndCreateNode($doc, $mqttInfo->getElementsByTagName('keepalive')->item(0), $mqttInfo, 'keepalive');
        $keepaliveNode->nodeValue = $_POST['keepalive'];

        $autoReconnectNode = CheckAndCreateNode($doc, $mqttInfo->getElementsByTagName('auto_reconnect')->item(0), $mqttInfo, 'auto_reconnect');
        $autoReconnectNode->nodeValue = $_POST['autoReconnect'];

        $clearSessionNode = CheckAndCreateNode($doc, $mqttInfo->getElementsByTagName('clear_session')->item(0), $mqttInfo, 'clear_session');
        $clearSessionNode->nodeValue = $_POST['clearSession'];

        if ($enableMqtt == 1) {
            $arixolinkInfo->setAttribute('enable', 0);
        }

        $mqttInfo->setAttribute('enable', $enableMqtt);

        for ($i=0; $i<=10; $i++) {
            if (isset($_POST['topic'.$i])) {
                $publishNode = $mqttInfo->getElementsByTagName('publish')->item($i);
                $publishNode->setAttribute('topic', $_POST['topic'.$i]);
                $publishNode->setAttribute('period', $_POST['period'.$i]);
                $publishNode->setAttribute('qos', $_POST['qos'.$i]);

                $supportFunctions = GetCloudMqttSupportFunctions();

                foreach ($supportFunctions as $itemName => $itemValue) {
                    $items = $publishNode->getElementsByTagName('item');
                    $hasItem = false;
                    $modifyItem = array();
                    foreach ($items as $item) {
                        if ($item->nodeValue == $itemName) {
                            $hasItem = true;
                            $modifyItem = $item;
                            break;
                        }
                    }

                    if (isset($_POST[($itemName.$i).'']) && !$hasItem) {
                        $newCurrentNetType = $doc->createElement('item');
                        $newCurrentNetType->nodeValue = $itemName;
                        $publishNode->appendChild($newCurrentNetType);
                    } elseif (!isset($_POST[($itemName.$i).'']) && $hasItem) {
                        $publishNode->removeChild($modifyItem);
                    }
                }
            }
        } 

        $doc->save($file);
        shell_exec('arixo_cmd mqtt restart &');
        echo '<script>alert("配置成功");</script>';
    } elseif (isset($_POST['removeMqttPublish'])) {
        $removeIndex = $_POST['removeMqttPublish'];
        $docObj = GetMqttXMLDoc();
        $doc = $docObj['doc'];
        $file = $docObj['file'];
        $root = $doc->documentElement;
        $mqttInfo = $root->getElementsByTagName('mqtt')->item(0);

        $mqttInfo->removeChild($mqttInfo->getElementsByTagName('publish')->item($removeIndex));

        $doc->save($file);
        shell_exec('arixo_cmd mqtt restart');
        echo '<script>alert("配置成功");</script>';
    } elseif (isset($_POST['applyarixolinkinfo'])) {
        $enableArixoLink = $_POST['enableArixoLink'];
        
        $docObj = GetMqttXMLDoc();
        $doc = $docObj['doc'];
        $file = $docObj['file'];
        $root = $doc->documentElement;
        $mqttInfo = $root->getElementsByTagName('mqtt')->item(0);
        $arixolinkInfo = $root->getElementsByTagName('arixolink')->item(0);

        $mqttIpNode = CheckAndCreateNode($doc, $arixolinkInfo->getElementsByTagName('mqtt_ip')->item(0), $arixolinkInfo, 'mqtt_ip');
        $mqttIpNode->nodeValue = $_POST['mqttIp'];

        $mqttPortNode = CheckAndCreateNode($doc, $arixolinkInfo->getElementsByTagName('mqtt_port')->item(0), $arixolinkInfo, 'mqtt_port');
        $mqttPortNode->nodeValue = $_POST['mqttPort'];

        $usernameNode = CheckAndCreateNode($doc, $arixolinkInfo->getElementsByTagName('mqtt_user')->item(0), $arixolinkInfo, 'mqtt_user');
        $usernameNode->nodeValue = $_POST['username'];

        $passwordNode = CheckAndCreateNode($doc, $arixolinkInfo->getElementsByTagName('mqtt_pwd')->item(0), $arixolinkInfo, 'mqtt_pwd');
        $passwordNode->nodeValue = $_POST['password'];

        $companyKeyNode = CheckAndCreateNode($doc, $arixolinkInfo->getElementsByTagName('company')->item(0), $arixolinkInfo, 'company');
        $companyKeyNode->nodeValue = $_POST['companykey'];

        $productKeyNode = CheckAndCreateNode($doc, $arixolinkInfo->getElementsByTagName('product')->item(0), $arixolinkInfo, 'product');
        $productKeyNode->nodeValue = $_POST['productkey'];

        if ($enableArixoLink == 1) {
            $mqttInfo->setAttribute('enable', 0);
        }

        $arixolinkInfo->setAttribute('enable', $enableArixoLink);

        $doc->save($file);
        shell_exec('arixo_cmd mqtt restart');
        echo '<script>alert("配置成功");</script>';

    } elseif (isset($_POST['restartMqtt'])) {
        shell_exec('arixo_cmd mqtt restart');
        echo '<script>alert("重启成功");</script>';
    } elseif (isset($_POST['applybypassinfo'])) {
        $byPassConfJson = file_get_contents('/home/user/config/byPass.conf');
        $byPassConfig = json_decode($byPassConfJson, true);

        $oldEnableByPass = $byPassConfig['enable'];

        $enableByPass = $_POST['enableByPass'];
        $ipAddr = $_POST['ipAddr'];
        $port = $_POST['port'];

        $byPassConfig['enable'] = $enableByPass;
        $byPassConfig['ip'] = $ipAddr;
        $byPassConfig['port'] = $port;

        file_put_contents('/home/user/config/byPass.conf', json_encode($byPassConfig));
        exec('arixo_cmd SPTT close');

        if ($enableByPass == '1') {
            exec('arixo_cmd SPTT open');
        }
    }

    $cloudType = $_GET['type'];
    if (!isset($cloudType)) {
        $cloudType = 'arixolink';
    }

    ShowCloudManage($cloudType);

?>

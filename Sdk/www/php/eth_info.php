<?php

    exec('ifconfig |grep eth', $return);
    echo '<div class="page-header"><h1>以太网信息</h1></div>';

    if (isset($_POST['applylaninfo'])) {
        $lanIp = $_POST['lanip'];
        if (isset($lanIp) && $lanIp != '') {
            if (!isIP($lanIp)) {
                echo '<script>alert("IP格式错误");</script>';
            } else {
                if (ChangeLanIp($lanIp)) {
                    echo '<script>alert("设置成功, 设备将在1s后重启。");</script>';
                } else {
                    echo '<script>alert("设置失败");</script>';
                }
            }
        }
    } elseif (isset($_POST['applyfirewall'])) {
        $dmzHost = $_POST['dmzHost'];

        $firwallDataJson = file_get_contents('/home/user/config/arixo_firewall.conf');
        $firwallData = json_decode($firwallDataJson, true);
        $oldDmzHost = $firwallData['DMZ'];

        $firwallData['DMZ'] = $dmzHost;
        file_put_contents('/home/user/config/arixo_firewall.conf', json_encode($firwallData));
        exec('arixo_cmd firewall dmz');

    } elseif (isset($_POST['enablePortMapping'])) {
        $portMappingJson = file_get_contents('/home/user/config/arixo_firewall.conf');
        $portMappingConfig = json_decode($portMappingJson, true);
        $portMappingData = $portMappingConfig['UPnP'];
        $enablePortMapping = $portMappingData['enable'];

        if ($enablePortMapping == NULL || $enablePortMapping == '0') {
            $portMappingData['enable'] = '1';
        } else {
            $portMappingData['enable'] = '0';
        }

        $portMappingConfig['UPnP'] = $portMappingData;

        file_put_contents('/home/user/config/arixo_firewall.conf', json_encode($portMappingConfig));
        exec('arixo_cmd firewall upnp');

    } elseif (isset($_POST['updatePortMapping'])) {
        $portMappingJson = file_get_contents('/home/user/config/arixo_firewall.conf');
        $portMappingConfig = json_decode($portMappingJson, true);

        $portMappingData = $portMappingConfig['UPnP'];

        $updateIndex = $_POST['updatePortMapping'];
        $mappingName = $_POST['mappingName'.$updateIndex];
        $sourceIp = $_POST['sourceIp'.$updateIndex];
        $portRangeStart = $_POST['portRangeStart'.$updateIndex];
        $portRangeEnd = $_POST['portRangeEnd'.$updateIndex];
        $destIp = $_POST['destIp'.$updateIndex];
        $destPort = $_POST['destPort'.$updateIndex];
        $protocol = $_POST['protocol'.$updateIndex];

        if ($mappingName == '' || $sourceIp == '' || $portRangeStart == '' || $destIp == '' || $destPort == '' || $protocol == '') {
            echo '<script>alert("请输入正确的端口映射信息");</script>';
        } else {

            if ($portRangeEnd < 0) {
                $portRangeEnd = 0;
            }

            if ($portRangeStart < 0) {
                $portRangeStart = 0;
            }

            if ($portRangeEnd == '') {
                $portRangeEnd = $portRangeStart;
            } elseif ($portRangeEnd < $portRangeStart) {
                $tempStart = $portRangeStart;
                $portRangeStart = $portRangeEnd;
                $portRangeEnd = $tempStart;
            }
            $portMappingList = $portMappingData['mappingList'];
            $portMappingList[$updateIndex]['name'] = $mappingName;
            $portMappingList[$updateIndex]['sourceIp'] = $sourceIp;
            $portMappingList[$updateIndex]['portRange'] = $portRangeStart . ':' . $portRangeEnd;
            $portMappingList[$updateIndex]['destIp'] = $destIp;
            $portMappingList[$updateIndex]['destPort'] = $destPort;
            $portMappingList[$updateIndex]['protocol'] = $protocol;

            $portMappingData['mappingList'] = $portMappingList;

            $portMappingConfig['UPnP'] = $portMappingData;

            file_put_contents('/home/user/config/arixo_firewall.conf', json_encode($portMappingConfig));
            exec('arixo_cmd firewall upnp');
        }
    } elseif (isset($_POST['removePortMapping'])) {
        $portMappingJson = file_get_contents('/home/user/config/arixo_firewall.conf');
        $portMappingConfig = json_decode($portMappingJson, true);

        $portMappingData = $portMappingConfig['UPnP'];

        $removeIndex = $_POST['removePortMapping'];

        $newMappingList = json_decode("[]", true);
        $portMappingList = $portMappingData['mappingList'];
        $addedIndex = 0;
        for ($i = 0; $i < sizeof($portMappingList); $i++) {
            if ($i != $removeIndex) {
                $newMappingList[$addedIndex] = $portMappingList[$i];
                $addedIndex++;
            }
        }

        $portMappingData['mappingList'] = $newMappingList;
        $portMappingConfig['UPnP'] = $portMappingData;

        file_put_contents('/home/user/config/arixo_firewall.conf', json_encode($portMappingConfig));
        exec('arixo_cmd firewall upnp');
    }

    $ethArray = array();
    foreach($return as $ethInfo) {
        $arrInfo = explode(' ', $ethInfo);
        array_push($ethArray, trim($arrInfo[0]));
    }
    $names = array();
    for ($index = 0; $index < 1; $index++) {
        $expected = 'eth'.$index;
        if (in_array($expected, $ethArray)) {
            array_push($names, $expected);
        } else {
            echo '<script>alert("网络端口'.$expected.'未找到");</script>';
        }
    }

    $configType = $_GET['config'];
    if (!isset($configType)) {
        $configType = 'lan';
    }

    GetLanPageInfo($configType);

?>

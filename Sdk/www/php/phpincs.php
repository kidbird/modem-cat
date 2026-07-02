<?php

function GetDistString($input,$string,$offset,$separator) {
    $string = substr($input,strpos($input,$string)+$offset,strpos(substr($input,strpos($input,$string)+$offset),$separator));
    return $string;
}

function ParseConfig($arrConfig) {
    $config = array();
    foreach($arrConfig as $line) {
        if($line[0] != "#") {
            $arrLine = explode("=",$line);
            $config[$arrLine[0]] = $arrLine[1];
        }
    }
    return $config;
}

function ConvertToChannel($freq) {
    $base = 2412;
    $channel = 1;
    for($x = 0; $x < 13; $x++) {
        if($freq != $base) {
            $base = $base + 5;
            $channel++;
        } else {
            return $channel;
        }
    }
    return "Invalid Channel";
}

function ConvertToSecurity($security) {
    switch($security) {
        case "[WPA2-PSK-CCMP][ESS]":
            return "WPA2-PSK (AES)";
        break;
        case "[WPA2-PSK-TKIP][ESS]":
            return "WPA2-PSK (TKIP)";
        break;
        case "[WPA-PSK-TKIP+CCMP][WPS][ESS]":
            return "WPA-PSK (TKIP/AES) with WPS";
        break;
        case "[WPA-PSK-TKIP+CCMP][WPA2-PSK-TKIP+CCMP][ESS]":
            return "WPA/WPA2-PSK (TKIP/AES)";
        break;
        case "[WPA-PSK-TKIP][ESS]":
            return "WPA-PSK (TKIP)";
        break;
        case "[WEP][ESS]":
            return "WEP";
        break;
    }
}

function ShowWPAConfHead() {
    $wlanJson = shell_exec('quec_app wifi list');
    $wlanList = json_decode($wlanJson, true);
    $ssids = array();
    if ($wlanList != null) {
        foreach ($wlanList as $wlanInfo) {
            $ssids[] = $wlanInfo['SSID'];
        }
    }
    $ssidNum = count($ssids);
    $state = getWlanState();
    //echo $state;
    if ($state == 0) {
        $statestr = '关闭中';
        $color = 'red';
    } elseif ($state == 1) {
        $statestr = '已关闭';
        $color = 'red';
    } elseif ($state == 2) {
        $statestr = '开启中';
        $color = 'green';
    } elseif ($state == 3) {
        $statestr = '已开启';
        $color = 'green';
    } else {
        $statestr = '未知';
        $color = 'red';
    }
    $output = '
        <div class="page-header"><h1>无线网配置</h1></div>
        <form method="POST" action="/?page=wpa_conf" id="wpa_conf_form">
            <div class="panel panel-default">
                <div class="panel-heading">无线网配置</div>
                <div class="panel-body">
                    <div style="color:'.$color.'">当前状态：'.$statestr.'</div>
                    <br />
                    <input type="submit" class="btn btn-default" value="打开WIFI" name="Enable">
                    <input type="submit" class="btn btn-default" value="关闭WIFI" name="Disable">
                    <input type="submit" class="btn btn-default" value="搜索网络" name="Scan" />
                    <input type="button" class="btn btn-default" value="添加网络" onClick="AddNetwork();" />
                    <input type="submit" class="btn btn-default" value="重连当前网络" name="Reconnect">
                    <input type="submit" class="btn btn-default" value="断开连接" name="Disconnect">
                </div>
            </div>
            <div class="network" id="networkbox"></div>
            <div class="panel panel-default">
                <div class="panel-heading">已添加的网络</div>
                <div class="panel-body row">';
    for($num = 0; $num < $ssidNum; $num++) {
        $ssid = str_replace('"', '', $ssids[$num]);
        $output .= '
                    <div class="col-sm-2">ssid: '.$ssid.'</div>
                    <button type="submit" class="btn btn-default col-sm-1" value="'.$ssid.'" name="Connect" >连接网络</button>
                    <button type="submit" class="btn btn-default col-sm-1" value="'.$ssid.'" name="Delete" >删除网络</button>
        ';
    }

    $output .= '
                </div>
            </div>
        </form>

    <script src="../js/functions.js">UpdateNetworks();</script>
    ';

    echo $output;
}
function ConnectWlan($ssid) {
    $id = -1;
    $wlanJson = shell_exec('quec_app wifi list');
    $wlanList = json_decode($wlanJson, true);
    foreach ($wlanList as $wlanInfo) {
        if ($wlanInfo['SSID'] == '"'.$ssid.'"') {
            $id = $wlanInfo['networkId'];
        }
    }
    if ($id == -1) {
        echo '<script>alert("未找到该网络");</script>';
        return;
    } else {
        exec('quec_app wifi connect '.$id);
    }
    sleep(3);
    if (isCompleted($id)) {
        echo '<script>alert("连接成功");</script>';
    }
}
function EnableWlan() {
    exec('/etc/init.d/quec_wlan.init start');
    $state = getWlanState();
    $failTimes = 3;
    if ($state == 2) {
        for ($try = 0; $try < $failTimes; $try++) {
            sleep(1);
            $state = getWlanState();
            if ($state == 3) {
                echo '<script>alert("开启成功");</script>';
                return ;
            }
        }
        echo '<script>alert("开启超时");</script>';
    } elseif($state == 3) {
        echo '<script>alert("开启成功");</script>';
    } else {
        echo '<script>alert("开启失败");</script>';
    }
}
function DisableWlan() {
    exec('/etc/init.d/quec_wlan.init stop');
    $state = getWlanState();
    $failTimes = 3;
    if ($state == 0) {
        for ($try = 0; $try < $failTimes; $try++) {
            sleep(1);
            $state = getWlanState();
            if ($state == 1) {
                echo '<script>alert("关闭成功");</script>';
                return ;
            }
        }
        echo '<script>alert("关闭超时");</script>';
    } elseif($state == 1) {
        echo '<script>alert("关闭成功");</script>';
    } else {
        echo '<script>alert("关闭失败");</script>';
    }

}
function ReconnectWlan() {
    exec('quec_app wifi reconect');
    $status = getWlanStatus();
    $wlanJson = shell_exec('quec_app wifi info');
    $wlanInfo = json_decode($wlanJson, true);
    $id = $wlanInfo['mNetworkId'];
    $wlanJson = shell_exec('quec_app wifi list');
    $wlanList = json_decode($wlanJson, true);
    foreach ($wlanList as $wlanInfo) {
        if ($wlanInfo['networkId'] == $id){
            $ssid = $wlanInfo['SSID'];
            break;
        }
    }
    if ($status == 'COMPLETED') {
        echo '<script>alert("重连成功");</script>';
    } elseif($id == -1) {
        echo '<script>alert("获取当前wlan信息失败");</script>';
    } else {
        echo '<script>alert("当前连接网络SSID:'.$ssid.'")</script>';
    }
}
function DisconnectWlan() {
    $wlanJson = shell_exec('quec_app wifi info');
    $wlanInfo = json_decode($wlanJson, true);
    $id = $wlanInfo['mNetworkId'];
    $returnval = shell_exec('quec_app wifi disconnect '.$id);
    $status = getWlanStatus();
    $wlanJson = shell_exec('quec_app wifi list');
    $wlanList = json_decode($wlanJson, true);
    foreach ($wlanList as $wlanInfo) {
        if ($wlanInfo['networkId'] == $id){
            $ssid = $wlanInfo['SSID'];
            break;
        }
    }
    if ($status != 'INACTIVE' or $id == -1) {
        echo '<script>alert("断开成功");</script>';
    } else {
        echo '<script>alert("当前连接网络SSID:'.$ssid.'")</script>';
    }
}
function DeleteWlan($ssid) {
        $return = shell_exec('quec_app wifi remove \"'.$ssid.'\"');
        $returnVal = json_decode($return, true);
        if ($returnVal["status"] == 1) {
                echo '<script>alert("删除成功");</script>';
        } else {
                echo '<script>alert("删除失败");</script>';
        }
}
function ShowScan() {
    exec('wpa_cli scan',$return);
    sleep(5);
    exec('wpa_cli scan_results',$return);
    for($shift = 0; $shift < 4; $shift++ ) {
        array_shift($return);
    }
    $output =  '
        <div class="panel panel-default">
            <div class="panel-heading">找到的网络</div>
            <div class="panel-body row">
        ';
    foreach($return as $network) {
        $arrNetwork = preg_split("/[\t]+/",$network);
        $bssid = $arrNetwork[0];
        $channel = ConvertToChannel($arrNetwork[1]);
        $signal = $arrNetwork[2] . " dBm";
        $security = $arrNetwork[3];
        $ssid = $arrNetwork[4];
        $output .= '
                <div class="col-sm-6">
                    <input class="btn btn-default" type="button" value="添加该网络" onClick="AddScanned(\''.$ssid.'\')" />'.$ssid.'<br />
                </div>
                ';
    }
    $output .= '
            </div>
        </div>
        ';
    echo $output;
}
function SaveWlan($ssid, $psk) {
    $ssid = trim($ssid);
    $ssid = urlencode($ssid);
    $jsonString = '{
        "SSID" : \'"'.$ssid.'"\',
        "preSharedKey" : \'"'.$psk.'"\',
        "mHiddenSSID" : true
    }';
    $jsonString = preg_replace('/\s+/', '', $jsonString);
    $jsonString = str_replace(" ", "", $jsonString);
    $returnval = shell_exec('quec_app wifi upsert '.$jsonString);
    $returnList = json_decode($returnval, true);
    if ($returnList['status'] == '1') {
        echo '<script>alert("储存成功");</script>';
    } else {
        echo '<script>alert("储存失败,原因:'.$returnList['reason'].'");</script>';
    }
}
function isCompleted($id) {
    $wlanJson = shell_exec('quec_app wifi info');
    $wlanInfo = json_decode($wlanJson, true);
    if ($wlanInfo['mNetworkId'] == $id and $wlanInfo['mSupplicantState'] == 'COMPLETED') {
        return true;
    } elseif($wlanInfo['mSupplicantState'] != 'COMPLETED') {
        echo '<script>alert("当前状态:'.$wlanInfo['mSupplicantState'].'")</script>';
        return false;
    } else {
        echo '<script>alert("当前连接网络SSID:'.$wlanInfo['SSID'].'")</script>';
        return false;
    }
}
function getWlanStatus() {
    $wlanJson = shell_exec('quec_app wifi info');
    $wlanInfo = json_decode($wlanJson, true);
    return $wlanInfo['mSupplicantState'];
}
function getWlanState() {
    $wlanJson = shell_exec('quec_app wifi state');
    $wlanInfo = json_decode($wlanJson, true);
    return $wlanInfo['state'];
}
function ShowEthernetConfHead($ethname, $status) {
    $output = '
        <div class="col-md-4 panel panel-default">
        <div class="intheader panel-heading">'. $ethname . ' 配置</div>';
    if ($status == 'UP') {
    $output .= '
        <form action="/?page=eth_conf" method="POST">
        <div class="panel-body">
            <div class="checkbox">
                <label><input type="checkbox" value="primary" id="checkbox_'.$ethname.'" name="checkbox_'.$ethname.'" onchange="checkboxChange(\''.$ethname.'\')">互联网网口</label>
            </div>
            <div class="btn-group" data-toggle="buttons">
                <label class="btn btn-primary active" onclick="DHCPClick(\''.$ethname.'\');">
                    <input type="radio" name="modegroup_'.$ethname.'" value="DHCP" checked/>自动获取IP
                </label>
                <label class="btn btn-primary" onclick="ManualClick(\''.$ethname.'\');">
                    <input type="radio" name="modegroup_'.$ethname.'" value="Manual"/>手动配置IP
                </label>
            </div>
            <div class="ethconf" id="ethconfbox_'.$ethname.'" style="display:none">
                <div class="form-group row">
                    <label class="col-sm-4 form-control-label" for="ip_'.$ethname.'">IP :</label>
                    <div class="col-sm-8">
                        <input class="form-control" type="text" id="ip_'.$ethname.'" name="ip_'.$ethname.'" placeholder="IP">
                    </div>
                </div>
                <div class="form-group row">
                    <label class="col-sm-4 form-control-label" for="netmask_'.$ethname.'">子网掩码 :</label>
                    <div class="col-sm-8">
                        <input class="form-control" type="text" id="netmask_'.$ethname.'" name="netmask_'.$ethname.'" placeholder="Netmask">
                    </div>
                </div>
                <div id="primary_'.$ethname.'" style="display:none">
                    <div class="form-group row">
                        <label class="col-sm-4 form control-label" for="gateway_'.$ethname.'">网关 :</label>
                        <div class="col-sm-8">
                            <input class="form-control" type="text" id="gateway_'.$ethname.'" name="gateway_'.$ethname.'" placeholder="Gateway">
                        </div>
                    </div>
                    <div class="form-group row">
                        <label class="col-sm-4 form-control-label" for="dns_'.$ethname.'">DNS :</label>
                        <div class="col-sm-8">
                            <input class="form-control" type="text" id="dns_'.$ethname.'" name="dns_'.$ethname.'" placeholder="DNS">
                        </div>
                    </div>
                </div>
            </div> <!-- manual part-->';

    $output .= '
        <center>
            <input class="btn btn-default" type="submit" value="应用" name="Apply_'.$ethname.'" />
            <input class="btn btn-default" type="submit" value="关闭网口" name="ifdown_' . $ethname . '" />
        </center>
        </div> <!-- panel body -->
        </form>
        ';

    } else if ($status == 'DOWN') {
        $output .= '
            <div class="panel-body">
                <h3>网口'.$ethname.'处于关闭状态</h3>
                <form action="/?page=eth_conf" method="POST">
                    <input class="btn btn-default" type="submit" value="打开网口" name="ifup_' . $ethname . '" />
                </form>
            </div> <!-- panel body -->
            ';
    }
    $output .= '</div> <!-- panel -->';
    echo $output;
}
function GetLanInfo() {
    $returnJson = shell_exec('quec_app conn info');
    $returnList = json_decode($returnJson, true);
    $currentMethod = $returnList['mTypeName'];
    $ethInfo = '[{"dev_name": "';
    if ($currentMethod == 'nat2') {
        $ethInfo .= 'tether';
    } else {
        $ethInfo .= 'eth0';
    }
    $ethInfo .= '", "enable": true, "primary": true, "hwaddr": "';

    $hwAddr = shell_exec("ifconfig eth0 | grep HWaddr | awk '{print $5}'");

    $ethInfo .= trim($hwAddr).'"}]';

    return $ethInfo;
}
function GetEthInfo($ethobj) {
    $ethname = $ethobj['dev_name'];
    if($ethobj['enable']) {
        $strStatus = '<span style="color:green">启动</span>';
    } else {
        $strStatus = '<span style="color:red">未启动</span>';
    }

    $primaryContent = '';
    if ($ethobj['primary']) {
        $strPrimary = '是';
        $dns = getDNS();
        $dnsInfo = '';
        $count = 0;
        foreach($dns as $ele) {
            $count++;
            $dnsInfo .= '<label style="font: size 15px;">DNS'.$count.'</label>
                        <input type="text" name="dns'.$count.'" class="form-control" id="dns'.$count.'" style="height: 3%" value='.$ele.'>';
        }
        $gateway = getGateway();
        $gwInfo = '';
        $count = 0;
        foreach ($gateway as $ele) {
            $count++;
            $gwInfo .= '<label style="width: 100%; font: size 15px;">网关'.$count.': '.$ele.'</label>';
        }
        $primaryContent = $dnsInfo.$gwInfo;
    } else {
        $strPrimary = '否';
    }


    $linkedDevicesJson = shell_exec('arixo_cmd dofunc linkip');
    $linkedDevicesInfo = json_decode($linkedDevicesJson, true);
    $linkedDevices = $linkedDevicesInfo['linkIp'];

    $output = '
        <form action="/?page=eth_info" method="POST">
            <div class="col-md-12">
                <div id="ethintinfo" class="panel panel-default">
                    <div class="intheader panel-heading" style="font-size: 16px">接口信息</div>
                    <div class="panel-body">
                        <div class="form-group">
                            <div class="form-inline row" style="margin: 0 auto">
                                <div class="col-md-6">
                                    <div style="display: flex">
                                        <span style="flex: 5; text-align: end">接口名称: </span>
                                        <span style="flex: 5; padding-left: 10px">LAN</span>
                                    </div>
                                    <div style="display: flex; margin-top: 10px">
                                        <span style="flex: 5; text-align: end">接口状态: </span>
                                        <span style="flex: 5; padding-left: 10px">'.$strStatus.'</span>
                                    </div>
                                    <div style="display: flex; margin-top: 10px">
                                        <span style="flex: 5; text-align: end">IP地址: </span>
                                        <span style="flex: 5; padding-left: 10px">
                                            <input type="text" name="lanip" class="form-control" id="lanip" style="min-width: 145px; width: 40%;height: 80%" value=' . getIp($ethname) . '>
                                        </span>
                                    </div>
                                </div>
                                <div class="col-md-6">
                                    <div style="display: flex">
                                        <span style="flex: 5; text-align: end">MAC地址: </span>
                                        <span style="flex: 5; padding-left: 10px">' . $ethobj['hwaddr'] . '</span>
                                    </div>
                                    <div style="display: flex; margin-top: 10px">
                                        <span style="flex: 5; text-align: end">子网掩码: </span>
                                        <span style="flex: 5; padding-left: 10px">' . getNetmask($ethname) . '</span>
                                    </div>
                                </div>
                            </div>
                            <div class="col-md-12" style="margin-top: 12px; text-align: center;">
                                <input style="width: 100px;" class="btn btn-primary" type="submit" value="应 用" name="applylaninfo" id="applylaninfo" />
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <div class = "col-md-12" style="margin-top: 12px;">
                <div class="panel panel-default">
                    <div class="intheader panel-heading" style="font-size: 16px;">连接设备</div>
                    <div class="panel-body">
                        <div class="form-group" style="font-size 14px; text-align: center">
                            <div class="form-inline row" style="margin-top: 12px; display: flex;">
                                        <span style="flex: 5; text-align: end">连接设备数量: </span>
                                        <span style="flex: 5; text-align: start; padding-left: 10px">' . $linkedDevicesInfo['linkCount'] . '</span>
                            </div>
                            <div class="form-inline row" style="margin-top: 12px;">
                                <div style="border: 2px solid lightgrey; border-radius: 5px;">
                                    <div class="form-inline row" style="display: flex;">
                                        <span style="border: 1px solid lightgrey; border-top: none; border-bottom: none; border-left: none; flex: 1; line-height: 30px; height: 30px;">序号: </span>
                                        <span style="border: 1px solid lightgrey; border-top: none; border-bottom: none; border-left: none; flex: 5; line-height: 30px; height: 30px;">IP地址: </span>
                                        <span style="border: 1px solid lightgrey; border-top: none; border-bottom: none; border-left: none; border-right: none; flex: 5; line-height: 30px; height: 30px;">MAC地址: </span>
                                    </div>';


    for ($i = 0; $i < sizeof($linkedDevices); $i++) {
        $deviceInfo = $linkedDevices[$i];
        $ip = $deviceInfo['ip'];
        $mac = $deviceInfo['mac'];
        $output .= '
                                    <div class="form-inline row" style="display: flex;">
                                        <span style="border: 1px solid lightgrey; border-bottom: none; border-left: none; flex: 1; line-height: 30px; height: 30px;">'.($i+1).'</span>
                                        <span style="border: 1px solid lightgrey; border-bottom: none; border-left: none; flex: 5; line-height: 30px; height: 30px;">'.$ip.'</span>
                                        <span style="border: 1px solid lightgrey; border-bottom: none; border-left: none; border-right: none; flex: 5; line-height: 30px; height: 30px;">'.$mac.'</span>
                                    </div>
        ';
    }

                                        
    $output .= '
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </form>';
    return $output;
}

function GetFirewallSettings() {
    $portMappingJson = file_get_contents('/home/user/config/arixo_firewall.conf');
    $firewallData = json_decode($portMappingJson, true);
    $portMappingConfig = $firewallData['UPnP'];
    $dmzHost = $firewallData['DMZ'];

    $portMappingNew = '{"name": "", "sourceIp": "", "portRange": "", "destIp": "", "destPort": "", "protocol": ""}';

    $enablePortMapping = $portMappingConfig['enable'];
    if ($enablePortMapping == NULL) {
        $enablePortMapping = '0';
    }
    $portMappingList = $portMappingConfig['mappingList'];

    if ($portMappingList == NULL || sizeof($portMappingList) == 0) {
        $portMappingList[0] = json_decode($portMappingNew, true);
    } else {
        $lastItem = $portMappingList[sizeof($portMappingList) - 1];
        $name = $lastItem['name'];
        $sourceIp = $lastItem['sourceIp'];
        $portRange = $lastItem['portRange'];
        $destIp = $lastItem['destIp'];
        $destPort = $lastItem['destPort'];
        $protocol = $lastItem['protocol'];
        if ($name != '' && $sourceIp != '' && $portRange != '' && $destIp != '' && $destPort != '' && $protocol != '') {
            $portMappingList[sizeof($portMappingList)] = json_decode($portMappingNew, true);
        }
    }

    $output = '
        <form action="/?page=eth_info&config=firewall" method="POST">
            <div class = "col-md-12" style="height: 120px">
                <div class="panel panel-default" style="height: 100%;">
                    <div class="intheader panel-heading" style="font-size: 16px;">隔离区(DMZ)</div>
                    <div class="panel-body">
                        <div class="form-group">
                            <div class="form-inline row" style="margin: 10px auto; font-size 14px; display: flex">
                                <span style="flex: 1; text-align: end; line-height: 25px; height: 25px;">内网主机IP地址: </span>
                                <span style="flex: 1; padding-left: 10px">
                                    <input type="text" name="dmzHost" class="form-control" id="dmzHost" style="min-width: 145px; line-height: 25px; height: 25px;" value=' . $dmzHost . '>
                                </span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            <div class = "col-md-12" style="margin-top: 12px;">
                <div class="panel panel-default">
                    <div class="intheader panel-heading" style="font-size: 16px;">端口转发(UPnP)</div>
                    <div class="panel-body">
                        <div class="form-group" style="font-size 14px;">
                            <div class="form-inline row" style="margin: 10px auto; display: flex">
                                <span style="flex: 1; text-align: end; line-height: 30px; height: 30px;">启用端口映射: </span>
                                <span style="flex: 1; padding-left: 10px; height: 30px; line-height: 30px">
                                    <button class="btn btn-default" id="enablePortMapping" value="切换" style="width: 80px;" name="enablePortMapping">' . ($enablePortMapping=='0' ? '启 用' : '关 闭') . '</button>
                                </span>
                            </div>';
    if ($enablePortMapping == '1') {
        $output .= '
                            <div style="margin-top: 20px; border: 2px solid lightgrey; border-radius: 5px; padding: 0 10px;">
                                <div class="form-inline row" style="margin: 10px auto; display: flex;">
                                    <span style="flex: 4; line-height: 30px; height: 30px;">服务名称: </span>
                                    <span style="flex: 4; line-height: 30px; height: 30px;">源IP地址: </span>
                                    <span style="flex: 4; line-height: 30px; height: 30px;">端口范围: </span>
                                    <span style="flex: 4; line-height: 30px; height: 30px;">内网IP地址: </span>
                                    <span style="flex: 4; line-height: 30px; height: 30px;">本地端口: </span>
                                    <span style="flex: 4; line-height: 30px; height: 30px;">协议: </span>
                                    <span style="flex: 2; line-height: 30px; height: 30px;"></span>
                                </div>
        ';
        for ($i = 0; $i < sizeof($portMappingList); $i++) {
            $portMapping = $portMappingList[$i];
            $name = $portMapping['name'];
            $sourceIp = $portMapping['sourceIp'];
            $portRange = $portMapping['portRange'];
            $destIp = $portMapping['destIp'];
            $destPort = $portMapping['destPort'];
            $portRangeList = array();
            if ($portRange != '') {
                $portRangeList = explode(':', $portRange);
            }
            if (sizeof($portRangeList) == 0) {
                $portRangeList[0] = '';
                $portRangeList[1] = '';
            }
            $protocol = $portMapping['protocol'];

            $output .= '
                                <div class="form-inline row" style="margin: 10px auto; display: flex">
                                    <span style="flex: 4;">
                                        <input type="text" name="mappingName'.$i.'" class="form-control" id="mappingName'.$i.'" style="width: 95%; line-height: 30px; height: 30px;" value=' . $name . '>
                                    </span>
                                    <span style="flex: 4;">
                                        <input type="text" name="sourceIp'.$i.'" class="form-control" id="sourceIp'.$i.'" style="width: 95%; line-height: 30px; height: 30px;" value=' . $sourceIp . '>
                                    </span>
                                    <span style="flex: 4;">
                                        <input type="text" name="portRangeStart'.$i.'" class="form-control" id="portRangeStart'.$i.'" style="width: 44%; line-height: 30px; height: 30px;" value=' . $portRangeList[0] . '>
                                        <span>~</span>
                                        <input type="text" name="portRangeEnd'.$i.'" class="form-control" id="portRangeEnd'.$i.'" style="width: 44%; line-height: 30px; height: 30px;" value=' . $portRangeList[1] . '>
                                    </span>
                                    <span style="flex: 4;">
                                        <input type="text" name="destIp'.$i.'" class="form-control" id="destIp'.$i.'" style="width: 95%; line-height: 30px; height: 30px;" value=' . $destIp . '>
                                    </span>
                                    <span style="flex: 4;">
                                        <input type="text" name="destPort'.$i.'" class="form-control" id="destPort'.$i.'" style="width: 95%; line-height: 30px; height: 30px;" value=' . $destPort . '>
                                    </span>
                                    <span style="flex: 4;">
                                        <select name="protocol'.$i.'" id="protocol'.$i.'" class="form-control" style="width: 95%; line-height: 30px; height: 30px;">
                                                <option value="tcp" '.($protocol=='tcp' ? 'selected' : '').'>TCP</option>
                                                <option value="udp" '.($protocol=='udp' ? 'selected' : '').'>UDP</option>
                                                <option value="all" '.($protocol=='all' ? 'selected' : '').'>Both</option>
                                        </select>
                                    </span>
                                    <span style="flex: 2">';
            if ($name == '' && $sourceIp == '' && $portRange == '' && $destIp == '' && $destPort == '' && $protocol == '') {
                $output .= '
                                        <button class="btn btn-success" id="updatePortMapping" value="'.$i.'" style="width: 50px; line-height: 30px; height: 30px; padding-top: 0px; text-align: center" name="updatePortMapping">✚
                                        </button>';
            } else {
                $output .= '
                                        <button class="btn btn-success" id="updatePortMapping" value="'.$i.'" style="width: 50px; line-height: 30px; height: 30px; padding: 0px; text-align: center" name="updatePortMapping">✔
                                        </button>
                                        <button class="btn btn-danger" id="removePortMapping" value="'.$i.'" style="width: 50px; line-height: 30px; height: 30px; padding: 0px; text-align: center" name="removePortMapping">🞬
                                        </button>';
            }
            $output .= '
                                    </span>
                                </div>
            ';
        }
        $output .= '         </div>';
    }

    $output .= '
                        </div>
                    </div>
                </div>
            </div>
            <div class = "col-md-12" style="text-align: center">
                <input class="btn btn-primary" style="width: 100px;" type="submit" value="应用配置" name="applyfirewall" id="applyfirewall" />
            </div>
        </form>';
    return $output;
}

function GetATPageInfo() {
    $output = '
        <div class = "col-md-12" style="height: 500px">
            <div class="form-group">
                <div class="form-inline row" style="margin: 10px auto; font-size 14px; display: flex">
                    <textarea id="atDisplayBox" name="atDisplayBox" class="form-control" rows="20" style="width: 100%; resize: vertical; background-color: rgba(0,0,0,0.03);" readOnly></textarea>
                </div>
                <div class="form-inline row" style="margin: 10px auto; font-size 14px; display: flex">
                    <span style="flex: 9">
                        <input type="text" name="atCmd" class="form-control" id="atCmd" style="width: 100%">
                    </span>
                    <span style="flex: 1; padding-left: 10px">
                        <input style="width: 100px;" type="button" class="btn btn-primary" value="发 送" name="sendAt" id="sendAt" onClick="SendAT()" />
                    </span>
                    <script type="text/javascript">
                        $("#atCmd").keypress(function(e) {
                            if (e.keyCode == 13) {
                                var atValue = document.getElementById("atCmd").value;
                                if (atValue != "") {
                                    SendAT();
                                }
                            }
                            })
                    </script>
                 </div>
            </div>

    ';
    return $output;
}

function GetLanPageInfo($configType) {
    $output = '
        <div class="tab-div">
            <div class="tab-div-nav">
                <div class="col-md-3">
                    <a href="index.php?page=eth_info&config=lan"'.($configType == "lan" ? 'style="color:#3D84C5;border-bottom: 3px solid #3D84C5"' : '').'>LAN配置</a>
                </div>
                <div class="col-md-3">
                    <a href="index.php?page=eth_info&config=firewall"'.($configType == "firewall" ? 'style="color:#3D84C5;border-bottom: 3px solid #3D84C5"' : '').'>防火墙</a>
                </div>
                <div class="col-md-3">
                    <a href="index.php?page=eth_info&config=at"'.($configType == "at" ? 'style="color:#3D84C5;border-bottom: 3px solid #3D84C5"' : '').'>AT调试</a>
                </div>
            </div>
            <div style="height:3px;z-index:-1;width:100%;margin-top:-3px;margin-bottom: 10px;background-color:lightgrey"></div>
    ';

    if ($configType == 'lan') {
        $returnJson = GetLanInfo();
        $returnList = json_decode($returnJson, true);
        //$ethList = $returnList['values'];
        foreach($returnList as $ethobj) {
            $output .= GetEthInfo($ethobj);
        }
    } elseif ($configType == 'firewall') {
        $output .= GetFirewallSettings();
    } else {
        $output .= GetATPageInfo();
    }

    $output .= '</div>';

    echo $output;
}

function GetIPSecVPNInfo() {
    $fileConfigInfo = file_get_contents('/home/root/ip_work/ip_work.conf');

    $configInfo = json_decode('{}', true);

    $currentConfigInfo = explode("\n", $fileConfigInfo);

    $totalSize = sizeof($currentConfigInfo);
    $itemTotal = 0;
    for ($i = 0; $i < $totalSize; $i++) {
        $configItem = $currentConfigInfo[$i];

        if (strpos($configItem, '#') === 0 || (strpos($configItem, 'set ') !== 0 && strpos($configItem, 'add ') !== 0)) {
            continue;
        }

        if (strpos($configItem, 'forward') != 0) {
            $configInfo['forward'] = explode('=', $configItem)[1];
        } elseif (strpos($configItem, 'local') != 0) {
            $localIpInfo = explode(' to ', explode('=', $configItem)[1]);
            $configInfo['local'] = $localIpInfo;
        } else if (strpos($configItem, 'channel=') != 0) {
            if (strpos($configItem, 'encap=') != 0) {
                $channelNum = explode(' encap=', explode('channel=', $configItem)[1])[0];
                $configInfo['channel'][$channelNum]['encap'] = explode('encap=', $configItem)[1];
            } elseif (strpos($configItem, 'symm_alg_type=') != 0) {
                $channelNum = explode(' symm_alg_type=', explode('channel=', $configItem)[1])[0];
                $configInfo['channel'][$channelNum]['symm_alg_type'] = explode('symm_alg_type=', $configItem)[1];
            } elseif (strpos($configItem, 'sa_life_1=') != 0) {
                $channelNum = explode(' sa_life_1=', explode('channel=', $configItem)[1])[0];
                $configInfo['channel'][$channelNum]['sa_life_1'] = explode('sa_life_1=', $configItem)[1];
            } elseif (strpos($configItem, 'auth_alg=') != 0) {
                $channelNum = explode(' auth_alg=', explode('channel=', $configItem)[1])[0];
                $configInfo['channel'][$channelNum]['auth_alg'] = explode('auth_alg=', $configItem)[1];
            } elseif (strpos($configItem, 'dpd_interval=') != 0) {
                $channelNum = explode(' dpd_interval=', explode('channel=', $configItem)[1])[0];
                $configInfo['channel'][$channelNum]['dpd_interval'] = explode('dpd_interval=', $configItem)[1];
            } elseif (strpos($configItem, 'sa_life_2=') != 0) {
                $channelNum = explode(' sa_life_2=', explode('channel=', $configItem)[1])[0];
                $configInfo['channel'][$channelNum]['sa_life_2'] = explode('sa_life_2=', $configItem)[1];
            } elseif (strpos($configItem, 'anti-replay=') != 0) {
                $channelNum = explode(' anti-replay=', explode('channel=', $configItem)[1])[0];
                $configInfo['channel'][$channelNum]['anti_replay'] = explode('anti-replay=', $configItem)[1];
            } elseif (strpos($configItem, 'replaywindowsize=') != 0) {
                $channelNum = explode(' replaywindowsize=', explode('channel=', $configItem)[1])[0];
                $configInfo['channel'][$channelNum]['replaywindowsize'] = explode('replaywindowsize=', $configItem)[1];
            } elseif (strpos($configItem, 'remote=') != 0) {
                $channelNum = explode(' remote=', explode('channel=', $configItem)[1])[0];
                $configInfo['channel'][$channelNum]['remote'] = explode(' to ', explode('remote=', $configItem)[1]);
            } elseif (strpos($configItem, 'peer=') != 0) {
                $channelNum = explode(' peer=', explode('channel=', $configItem)[1])[0];
                $configInfo['channel'][$channelNum]['peer'] = explode('peer=', $configItem)[1];
            } elseif (strpos($configItem, 'reset=') != 0) {
                $channelNum = explode(' reset=', explode('channel=', $configItem)[1])[0];
                $configInfo['channel'][$channelNum]['reset'] = explode('reset=', $configItem)[1];
            }
        }
    }

    return $configInfo;
}

function SaveIPSecVPNInfo($configInfo) {
    $filename = '/home/root/ip_work/ip_work.conf';
    file_put_contents($filename, '');

    foreach ($configInfo as $key => $value) {
        $lineValue = '';

        if ($key == 'local') {
            $lineValue = 'add local=' .  $value[0] . ' to ' . $value[1];
            file_put_contents($filename, $lineValue . PHP_EOL, FILE_APPEND);
        } elseif ($key == 'forward') {
            $lineValue = 'set forward=' . $value;
            file_put_contents($filename, $lineValue . PHP_EOL, FILE_APPEND);
        } elseif ($key == 'channel') {
            foreach ($value as $channelNum => $channelValues) {
                foreach ($channelValues as $dataKey => $dataValue) {
                    if ($dataKey == 'remote') {
                        $lineValue = 'add channel=' . $channelNum . ' ' . $dataKey . '=' . $dataValue[0] . ' to ' . $dataValue[1];
                    } elseif ($dataKey == 'anti_replay') {
                        $lineValue = 'set channel=' . $channelNum . ' anti-replay=' . $dataValue;
                    } else {
                        $lineValue = 'set channel=' . $channelNum . ' ' . $dataKey . '=' . $dataValue;
                    }
                    file_put_contents($filename, $lineValue . PHP_EOL, FILE_APPEND);
                }
            }
        }
    }
}

function GetWifiConfigInfo($configType) {
    $wifiConfigInfo = file_get_contents('/mnt/data/' . ($configType == '2_4G' ? 'hostapd-wlan0.conf' : 'hostapd-wlan1.conf'));
    $configInfo = '{';

    $wifiInfo = explode("\n", $wifiConfigInfo);

    for ($i = 0; $i < sizeof($wifiInfo); $i++) {
        $lineValue = explode('=', $wifiInfo[$i]);
        $configInfo .= '"' . $lineValue[0] . '":"' . $lineValue[1] . '"';
        if ($i + 1 != sizeof($wifiInfo)) {
            $configInfo .= ',';
        }
    }

    $configInfo .= '}';

    return $configInfo;
}

function SaveWifiConfig($ssid, $pwd, $authType, $channel, $configType) {
    $filename = '/mnt/data/' . ($configType == '2_4G' ? 'hostapd-wlan0.conf' : 'hostapd-wlan1.conf');

    $currentConfigInfo = json_decode(GetWifiConfigInfo($configType), true);

    file_put_contents($filename, '');

    foreach($currentConfigInfo as $key => $value) {
        $lineValue = '';
        if ($key == 'ssid') {
            $lineValue = $key . '=' . $ssid;
        } elseif ($key == 'channel') {
            $lineValue = $key . '='. $channel;
        } elseif ($key == '#auth_algs' || $key == 'auth_algs') {
            if ($authType == '0') {
                $lineValue = '#auth_algs' . '=' . $value;
            } else{
                $lineValue = 'auth_algs' . '=' . $value;
            }
        } elseif ($key == '#wpa' || $key == 'wpa') {
            if ($authType == '0') {
                $lineValue = '#wpa' . '=' . $value;
            } else{
                $lineValue = 'wpa' . '=' . $value;
            }
        } elseif ($key == '#wpa_key_mgmt' || $key == 'wpa_key_mgmt') {
            if ($authType == '0') {
                $lineValue = '#wpa_key_mgmt' . '=' . $value;
            } else{
                $lineValue = 'wpa_key_mgmt' . '=' . $value;
            }
        } elseif ($key == '#wpa_pairwise' || $key == 'wpa_pairwise') {
            if ($authType == '0') {
                $lineValue = '#wpa_pairwise' . '=' . $value;
                } else{
                $lineValue = 'wpa_pairwise' . '=' . $value;
            }
        } elseif ($key == '#rsn_pairwise' || $key == 'rsn_pairwise') {
            if ($authType == '0') {
                $lineValue = '#rsn_pairwise' . '=' . $value;
            } else{
                $lineValue = 'rsn_pairwise' . '=' . $value;
            }
        } elseif ($key == '#wpa_passphrase' || $key == 'wpa_passphrase') {
            if ($authType == '0') {
                $lineValue = '#wpa_passphrase' . '=' . $pwd;
            } else{
                $lineValue = 'wpa_passphrase' . '=' . $pwd;
            }
        } else {
            $lineValue = $key . '='. $value;
        }
        if ($lineValue != '=') {
            file_put_contents($filename, $lineValue . PHP_EOL, FILE_APPEND);            
        }
    }
}

function GetWiFiPageInfo() {

    // $output = '
    //     <div class="tab-div">
    //         <div class="tab-div-nav">
    //             <div class="col-md-3">
    //                 <a href="index.php?page=wpa_conf&config=2_4G"'.($wifiType == "2_4G" ? 'style="color:#3D84C5;border-bottom: 3px solid #3D84C5"' : '').'>2.4G无线配置</a>
    //             </div>
    //             <div class="col-md-3">
    //                 <a href="index.php?page=wpa_conf&config=5_8G"'.($wifiType == "5_8G" ? 'style="color:#3D84C5;border-bottom: 3px solid #3D84C5"' : '').'>5.8G无线配置</a>
    //             </div>
    //         </div>
    //         <div style="height:3px;z-index:-1;width:100%;margin-top:-3px;margin-bottom: 10px;background-color:lightgrey"></div>
    // ';
    $output = '';

    $wlan0Info = shell_exec('ifconfig wlan0');
    $wlan0InfoList = explode(' ', $wlan0Info);
    $wifi2GUp = false;
    for ($i = 0; $i < sizeof($wlan0InfoList); $i++) {
        if ($wlan0InfoList[$i] == 'UP') {
            $wifi2GUp = true;
            break;
        }
    }

    $configInfo2G = json_decode(GetWifiConfigInfo('2_4G'), true);

    $authType2G = isset($configInfo2G['#wpa_key_mgmt']) ? '0' : '1'; // 0: NONE, 1: WPA-PSK/WPA2-PSK
    $pwd2G = $authType2G == '0' ? $configInfo2G['#wpa_passphrase'] : $configInfo2G['wpa_passphrase'];
    $channel2G = $configInfo2G['channel'];

    $wlan1Info = shell_exec('ifconfig wlan1');
    $wlan1InfoList = explode(' ', $wlan1Info);
    $wifi5GUp = false;
    for ($i = 0; $i < sizeof($wlan1InfoList); $i++) {
        if ($wlan1InfoList[$i] == 'UP') {
            $wifi5GUp = true;
            break;
        }
    }

    $configInfo5G = json_decode(GetWifiConfigInfo('5_8G'), true);


    $authType5G = isset($configInfo5G['#wpa_key_mgmt']) ? '0' : '1'; // 0: NONE, 1: WPA-PSK/WPA2-PSK
    $pwd5G = $authType5G == '0' ? $configInfo2G['#wpa_passphrase'] : $configInfo5G['wpa_passphrase'];
    $channel5G = $configInfo5G['channel'];

    $output .= '
            <form action="/?page=wpa_conf" method="POST">
                <div class="col-md-6">
                    <div id="24GWifiConfigInfo" class="panel panel-default">
                        <div class="intheader panel-heading" style="font-size: 16px">2.4G无线网络设置</div>
                        <div class="panel-body">
                            <div class="form-group">
                                <div class="form-inline row" style="margin: 0 auto">
                                    <div class="col-md-12">
                                        <div style="display: flex; height: 30px; align-items: center">
                                            <span style="flex: 4; text-align: end">无线网络状态: </span>
                                            <span style="flex: 6; padding-left: 10px;">
                                                <label class="switch" style="margin-top: 2.5px">
                                                    <input name="enable2G" id="enable2G" type="checkbox" ' . ($wifi2GUp ? 'checked' : '') . '>
                                                    <div class="slider round"></div>
                                                </label>
                                            </span>
                                        </div>
                                        <div style="display: flex; height: 30px;  margin-top: 10px; align-items: center">
                                            <span style="flex: 4; text-align: end">无线网络名称（SSID）: </span>
                                            <span style="flex: 6; padding-left: 10px">
                                                <input type="text" name="ssid2G" class="form-control" id="ssid2G" style="min-width: 200px; width: 60%;height: 80%" value=' . $configInfo2G['ssid'] . '>
                                            </span>
                                        </div>
                                        <div style="display: flex; height: 30px;  margin-top: 10px; align-items: center">
                                            <span style="flex: 4; text-align: end">加密方式: </span>
                                            <span style="flex: 6; padding-left: 10px">
                                                <select name="authType2G" id="authType2G" class="form-control" style="min-width: 200px; width: 60%;height: 80%; padding: 0 12px" onChange="WiFiAuthTypeChange()">
                                                    <option value="0" '.($authType2G=='0' ? 'selected' : '').'>不设密码</option>
                                                    <option value="1" '.($authType2G=='1' ? 'selected' : '').'>WPA-PSK/WPA2-PSK</option>
                                                </select>
                                            </span>
                                        </div>
                                        <div style="display: ' . ($authType2G == '0' ? 'none' : 'flex') . '; height: 30px;  margin-top: 10px; align-items: center" id="pwd2GHolder">
                                            <span style="flex: 4; text-align: end">无线密码: </span>
                                            <span style="flex: 6; padding-left: 10px">
                                                <input type="text" name="pwd2G" class="form-control" id="pwd2G" style="min-width: 200px; width: 60%;height: 80%" value="' . $pwd2G .'">
                                            </span>
                                        </div>
                                        <div style="display: flex; height: 30px;  margin-top: 10px; align-items: center">
                                            <span style="flex: 4; text-align: end">信道: </span>
                                            <span style="flex: 6; padding-left: 10px">
                                                <select name="channel2G" id="channel2G" class="form-control" style="min-width: 200px; width: 60%;height: 80%; padding: 0 12px">
                                                    <option value="0" '.($channel2G=='0' ? 'selected' : '').'>自动</option>
                                                    <option value="1" '.($channel2G=='1' ? 'selected' : '').'>1</option>
                                                    <option value="2" '.($channel2G=='2' ? 'selected' : '').'>2</option>
                                                    <option value="3" '.($channel2G=='3' ? 'selected' : '').'>3</option>
                                                    <option value="4" '.($channel2G=='4' ? 'selected' : '').'>4</option>
                                                    <option value="5" '.($channel2G=='5' ? 'selected' : '').'>5</option>
                                                    <option value="6" '.($channel2G=='6' ? 'selected' : '').'>6</option>
                                                    <option value="7" '.($channel2G=='7' ? 'selected' : '').'>7</option>
                                                    <option value="8" '.($channel2G=='8' ? 'selected' : '').'>8</option>
                                                    <option value="9" '.($channel2G=='9' ? 'selected' : '').'>9</option>
                                                    <option value="10" '.($channel2G=='10' ? 'selected' : '').'>10</option>
                                                    <option value="11" '.($channel2G=='11' ? 'selected' : '').'>11</option>
                                                    <option value="12" '.($channel2G=='12' ? 'selected' : '').'>12</option>
                                                    <option value="13" '.($channel2G=='13' ? 'selected' : '').'>13</option>
                                                </select>
                                            </span>
                                        </div>
                                    </div>
                                </div>
                                <div class="col-md-12" style="margin-top: 12px; text-align: center;">
                                    <input style="width: 100px;" class="btn btn-primary" type="submit" value="应 用" name="apply2Ginfo" id="apply2Ginfo" />
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="col-md-6">
                    <div id="58GWifiConfigInfo" class="panel panel-default">
                        <div class="intheader panel-heading" style="font-size: 16px">5G无线网络设置</div>
                        <div class="panel-body">
                            <div class="form-group">
                                <div class="form-inline row" style="margin: 0 auto">
                                    <div class="col-md-12">
                                        <div style="display: flex; height: 30px; align-items: center">
                                            <span style="flex: 4; text-align: end">无线网络状态: </span>
                                            <span style="flex: 6; padding-left: 10px;">
                                                <label class="switch" style="margin-top: 2.5px">
                                                    <input name="enable5G" id="enable5G" type="checkbox" ' . ($wifi5GUp ? 'checked' : '') . '>
                                                    <div class="slider round"></div>
                                                </label>
                                            </span>
                                        </div>
                                        <div style="display: flex; height: 30px;  margin-top: 10px; align-items: center">
                                            <span style="flex: 4; text-align: end">无线网络名称（SSID）: </span>
                                            <span style="flex: 6; padding-left: 10px">
                                                <input type="text" name="ssid5G" class="form-control" id="ssid5G" style="min-width: 200px; width: 60%;height: 80%" value=' . $configInfo5G['ssid'] . '>
                                        </div>
                                        <div style="display: flex; height: 30px;  margin-top: 10px; align-items: center">
                                            <span style="flex: 4; text-align: end">加密方式: </span>
                                            <span style="flex: 6; padding-left: 10px">
                                                <select name="authType5G" id="authType5G" class="form-control" style="min-width: 200px; width: 60%;height: 80%; padding: 0 12px" onChange="WiFiAuthTypeChange()">
                                                    <option value="0" '.($authType5G=='0' ? 'selected' : '').'>不设密码</option>
                                                    <option value="1" '.($authType5G=='1' ? 'selected' : '').'>WPA-PSK/WPA2-PSK</option>
                                                </select>
                                            </span>
                                        </div>
                                        <div style="display: ' . ($authType5G == '0' ? 'none' : 'flex') . '; height: 30px;  margin-top: 10px; align-items: center" id="pwd5GHolder">
                                            <span style="flex: 4; text-align: end">无线密码: </span>
                                            <span style="flex: 6; padding-left: 10px">
                                                <input type="text" name="pwd5G" class="form-control" id="pwd5G" style="min-width: 200px; width: 60%;height: 80%" value="' . $pwd5G .'">
                                            </span>
                                        </div>
                                        <div style="display: flex; height: 30px;  margin-top: 10px; align-items: center">
                                            <span style="flex: 4; text-align: end">信道: </span>
                                            <span style="flex: 6; padding-left: 10px">
                                                <select name="channel5G" id="channel5G" class="form-control" style="min-width: 200px; width: 60%;height: 80%; padding: 0 12px">
                                                    <option value="0" '.($channel5G=='0' ? 'selected' : '').'>自动</option>
                                                    <option value="36" '.($channel5G=='36' ? 'selected' : '').'>36 (80MHz)</option>
                                                    <option value="40" '.($channel5G=='40' ? 'selected' : '').'>40 (20MHz)</option>
                                                    <option value="44" '.($channel5G=='44' ? 'selected' : '').'>44 (80MHz)</option>
                                                    <option value="48" '.($channel5G=='48' ? 'selected' : '').'>48 (20MHz)</option>
                                                    <option value="149" '.($channel5G=='149' ? 'selected' : '').'>149 (80MHz)</option>
                                                    <option value="153" '.($channel5G=='153' ? 'selected' : '').'>153 (20MHz)</option>
                                                    <option value="157" '.($channel5G=='157' ? 'selected' : '').'>157 (80MHz)</option>
                                                    <option value="161" '.($channel5G=='161' ? 'selected' : '').'>161 (20MHz)</option>
                                                    <option value="165" '.($channel5G=='165' ? 'selected' : '').'>165 (20MHz)</option>
                                                </select>
                                            </span>
                                        </div>
                                    </div>
                                </div>
                                <div class="col-md-12" style="margin-top: 12px; text-align: center;">
                                    <input style="width: 100px;" class="btn btn-primary" type="submit" value="应 用" name="apply5Ginfo" id="apply5Ginfo" />
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </form>
    ';


    // $output .= '</div>';
    echo $output;
}

function GetEthStatus($ethname) {
    $returnJson = shell_exec('quec_app ether ifaces');
    $returnList = json_decode($returnJson, true);
    foreach ($returnList['values'] as $ele) {
        if ($ele['dev_name'] == $ethname) {
            return $ele['enable'];
        }
    }
    return false;
}
function showTestParm() {
    exec('netcfg', $return);
    $regex = '/(usb|sipa_usb|sipa_eth)([0-9]+)/i';
    $str = implode(" ", $return);
    $ifnames = array();
    preg_match_all($regex,$str,$ifnames);
    $names = array();
    foreach ($ifnames[0] as $if) {
        if (trim($if) == 'usb0' or trim($if) == 'sipa_usb0' or
            trim($if) == 'sipa_usb1' or trim($if) == 'sipa_eth0' or
            trim($if) == 'sipa_eth1')
          array_push($names, trim($if));
    }
    $ifnames = array();
    if (in_array('sipa_usb0', $names)) {
        array_push($ifnames, 'sipa_usb0');
    } else {
        echo '<script>alert("sipa_usb0接口信息获取失败");</script>';
    }
    if (in_array('sipa_eth0', $names)) {
        array_push($ifnames, 'sipa_eth0');
    } else {
        echo '<script>alert("sipa_eth0接口信息获取失败");</script>';
    }

    $output = '
        <div class="page-header"><h1>网络测试</h1></div>
            <div class="col-md-6 panel panel-default">
                <div class="panel-body">
                    <form action="/?page=test" method="POST">
                    <div class="form-group row">
                        <label class="col-sm-4 form-control-label" for="host">测试地址 :</label>
                        <div class="col-sm-8">
                            <input class="form-control" type="text" id="host" name="host" placeholder="HOST">
                        </div>
                    </div>
                    <div class="form-group row">
                        <label class="col-sm-4 form-control-label" for="tif">测试网口 :</label>
                        <div class="col-sm-8">
                            <select class="selectpicker" data-style="btn-primary" name="tif">
                                <option id="default">默认</option>';
    foreach ($ifnames as $if) {
        $output .= '
                    <option id="'.$if.'">'.$if.'</option>
            ';
    }


    $output .='
                            </select>
                        </div>
                    </div>
                    <input class="btn btn-default" type="submit" value="测试" name="testbtn"/>
                    </form>
                </div>
            </div>
        </div>';

    echo $output;
    echo '<script>bootstrapSelect();</script>';
}
function getIP($netname) {
    $returnStr = shell_exec("ifconfig ".$netname." | grep 'inet addr:' | awk '{print $2}'");
    $returnArray = explode(':', $returnStr);
    return $returnArray[1];
}
function getDNS() {
    exec('getprop | grep net.dns', $info);
    $dns = array();
    foreach($info as $ele) {
        $array = explode(': ', $ele);
        $ele = substr($array[1], 1, strlen($array[1])-2);
        if (isIP($ele)) {
            array_push($dns, $ele);
        }
    }
    return $dns;
}
function getGateway() {
    exec("route -n | grep eth | awk '{print $2}'", $info);
    $gateway = array();
    foreach ($info as $ele) {
        if ($ele != '0.0.0.0' and isIP($ele)) {
            array_push($gateway, $ele);
        }
    }
    return $gateway;
}
function getNetmask($netname) {
    $returnStr = shell_exec("ifconfig ".$netname." | grep Mask | awk '{print $4}'");
    $returnArray = explode(':', $returnStr);
    $netmask = $returnArray[1];
    return  $netmask;
}
function pingAddress($host, $tif) {
    if ($tif == "默认") {
        $ifstr = '';
    } else {
        $ifstr = ' -I '.$tif.'';
    }
    $shell = "ping ".$host.$ifstr.' -w 3';
    //echo $shell;
    $pingresult = exec($shell, $outcome, $strStatus);
    if ($strStatus == 0) {
        $status = "能够连通";
    } else {
        $status = "无法连通";
    }
    echo '<div>接口'.$tif.$status.$host.'</div><br />';
    $output = '<div>The outcome is :<br />';
    foreach ($outcome as $ele) {
        $output .= $ele.'<br />';
    }
    $output .= '</div><br />';
    echo $output;
}
function isIP($ip) {
    $array = explode('.', $ip);
    if (count($array) != 4) {
        return false;
    }
    foreach($array as $ele) {
        if (!is_numeric($ele)) {
            return false;
        } else if ($ele > 255 or $ele < 0){
            return false;
        }
    }
    return true;
}

function showLog() {
    // $returnval = shell_exec('netstat -tupln | grep 8081 | wc -l');
    // if ($returnval == 0) {
    //     echo '<a href="/?page=display&open=true" class="btn btn-default">打开</a> <hr>';
    // } else {
    //     echo '';
    // }
    $output = '
        <div class="tabbable">
            <ul class="nav nav-tabs" id="tabs">
            </ul>
            <div class="tab-content" id="contents">
            </div>
        </div>
    ';

    echo $output;
}

function ShowIPSecVPNPage() {
    $returnJson = exec('quec_app APN info');
    $returnList = json_decode($returnJson, true);
    $strIpAddr = $returnList['mIpAddr'];

    $configInfo = GetIPSecVPNInfo();
    $remoteConfigCount = sizeof($configInfo['channel']);
    $output = ' 
        <form action="/?page=ipsec_vpn" method="POST">
            <div class="col-md-12">
                <div id="ethintinfo" class="panel panel-default">
                    <div class="intheader panel-heading" style="font-size: 16px">本地配置</div>
                    <div class="panel-body">
                        <div class="form-group">
                            <div class="form-inline row" style="margin: 0 auto;">
                                <div class="col-md-6 col-lg-3" style="margin-top: 10px">
                                    <div style="display: flex; align-items: center; height: 30px">
                                        <span style="flex: 3; text-align: end">本机IP: </span>
                                        <span style="flex: 6; padding-left: 10px">'. $strIpAddr .'</span>
                                    </div>
                                </div>
                                <div class="col-md-6 col-lg-3" style="margin-top: 10px">
                                    <div style="display: flex; align-items: center;">
                                        <span style="flex: 3; text-align: end">转发策略: </span>
                                        <span style="flex: 6; padding-left: 10px">
                                            <select name="forwardType" id="forwardType" class="form-control" style="min-width: 120px; max-width: 160px; width: 100%; height: 30px; padding: 0 12px;">
                                                <option value="allblock" '.($configInfo['forward']=='allblock' ? 'selected' : '').'>allblock</option>
                                                <option value="block" '.($configInfo['forward']=='block' ? 'selected' : '').'>block</option>
                                                <option value="pass" '.($configInfo['forward']=='pass' ? 'selected' : '').'>pass</option>
                                                <option value="allpass" '.($configInfo['forward']=='allpass' ? 'selected' : '').'>allpass</option>
                                            </select>
                                        </span>
                                    </div>
                                </div>
                                <div class="col-md-6 col-lg-3" style="margin-top: 10px">
                                    <div style="display: flex; align-items: center;">
                                        <span style="flex: 3; text-align: end">起始IP: </span>
                                        <span style="flex: 6; padding-left: 10px">
                                            <input type="text" name="localIpStart" class="form-control" id="localIpStart" style="min-width: 120px; max-width: 160px; width: 100%;height: 80%" value="' . $configInfo['local'][0] . '" />
                                        </span>
                                    </div>
                                </div>
                                <div class="col-md-6 col-lg-3" style="margin-top: 10px">
                                    <div style="display: flex; align-items: center;">
                                        <span style="flex: 3; text-align: end">终止IP: </span>
                                        <span style="flex: 6; padding-left: 10px">
                                            <input type="text" name="localIpEnd" class="form-control" id="localIpEnd" style="min-width: 120px; max-width: 160px; width: 100%;height: 80%" value="' . $configInfo['local'][1] . '" />
                                        </span>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            <div class="col-md-12">
                <div id="ethintinfo" class="panel panel-default">
                    <div class="intheader panel-heading" style="font-size: 16px">
                        远端配置(<span id="totalRemoteCount"></span>)
                    </div>
                    <div class="panel-body">
                        <div class="form-group">
                            <div class="form-inline row" style="margin: 0 auto;">
                                <input style="display: none" type="number" name="totalRemoteConfigCount" id="totalRemoteConfigCount" />
                                <div id="remoteConfigHolder">
                                </div>
                                <div class="col-md-12" style="text-align: center; margin-top: 12px">
                                    <input type="button" style="width: 100px;" class="btn btn-success" value="添 加" name="addConfig" id="addConfig" onClick="AddIPSecVPNRemoteConfig()" />
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
            <div class="col-md-12" style="margin-top: 12px; text-align: center;">
                <input style="width: 100px;" class="btn btn-primary" type="submit" value="应 用" name="applyipsecinfo" id="applyipsecinfo" />
            </div>
        </form>
        <script type="text/javascript">
            var remoteConfigCount = '. $remoteConfigCount.';
            var checkStatusId = setInterval(function() {
                updateStatus();
            }, 10000);
            window.onload = function() {
                document.getElementById("totalRemoteCount").innerHTML = remoteConfigCount;
                document.getElementById("totalRemoteConfigCount").value = remoteConfigCount;';
    if ($remoteConfigCount > 0) {
        foreach ($configInfo['channel'] as $channelNum => $channelValues) {
            $output .= 'GetIPSecVPNRemoteInfo('.$channelNum.', '. json_encode($channelValues) .');';
        }
    }

    $output .= '
            };
        </script>
        ';
    echo $output;
}

function ShowSwitch($method, $usbnetMethod) {
    $returnJson = shell_exec('quec_app conn info');
    $returnList = json_decode($returnJson, true);

    $usbnetReturnJson = shell_exec('quec_app usbnet info');
    $usbnetReturnList = json_decode($usbnetReturnJson, true);

    echo '<script type="text/javascript">
        var count = 0;
        var intId = setInterval(function(){
            $.getJSON("index.php?page=ajax&data=conn", function(data){
                if (data != "") {
                    $("#method").text(data);
                }
            });
            count++;
            if (count > 3) {
                clearInterval(intId);
            }
        }, 2000);

        var usbnetCount = 0;
        var usbnetIntId = setInterval(function(){
            $.getJSON("index.php?page=ajax&data=usbnet", function(data){
                if (data != "") {
                    $("#usbnetMethod").text(data);
                }
            });
            usbnetCount++;
            if (usbnetCount > 3) {
                clearInterval(usbnetIntId);
            }
        }, 2000);
    </script>';
    // $output = '
    //     <div class="col-md-12 alert alert-info">
    //         当前连接方式: <span id="method">'.$returnList['mTypeName'].'</span>
    //     </div>
    // ';
    $method = $returnList['mTypeName'];
    // $output = '
    //     <div class="col-md-12 alert alert-info">
    //         当前网卡拨号方式: <span id="method">'.$returnList['mTypeName'].'</span>
    //     </div>
    // ';
    $usbnetMethod = $usbnetReturnList['mTypeName'];

    //echo $output;
    $output = '
        <form action="/?page=switch" method="POST">
            <div class="col-md-6">
                <div class="panel panel-default">
                    <div class="intheader panel-heading" style="font-size: 16px">连接配置</div>
                        <div class="panel-body" style="font-size: 14px;">
                            <div class="form-group">
                                <div class="col-md-6">
                                    <div class="form-inline row" style="display: flex">
                                        <span style="flex: 3; text-align: end">连接方式: </span>
                                        <span style="flex: 7; padding-left: 10px">
                                            <div class="col-md-12" style="padding-left: 0px"><label class="radio-inline"><input type="radio" name="optradio" value="nat0" '.($method=='nat0' ? 'checked' : '').'>网卡模式</label></div>
                                            <div class="col-md-12" style="padding-left: 0px"><label class="radio-inline"><input type="radio" name="optradio" value="nat1" '.($method=='nat1' ? 'checked' : '').'>路由模式</label></div>
                                            <div class="col-md-12" style="padding-left: 0px"><label class="radio-inline"><input type="radio" name="optradio" value="nat2" '.($method=='nat2' ? 'checked' : '').'>网桥模式</label></div>
                                        </span>
                                    </div>
                                </div>
                                <div class="col-md-6">
                                    <div class="form-inline row" style="display: flex">
                                        <span style="flex: 3; text-align: end">拨号方式: </span>                           
                                        <span style="flex: 7; padding-left: 10px">
                                            <div class="col-md-12" style="padding-left: 0px"><label class="radio-inline"><input type="radio" name="usbnetoptradio" value="ECM" '.($usbnetMethod=='ECM' ? 'checked' : '').'>ECM</label></div>
                                            <div class="col-md-12" style="padding-left: 0px"><label class="radio-inline"><input type="radio" name="usbnetoptradio" value="RNDIS" '.($usbnetMethod=='RNDIS' ? 'checked' : '').'>RNDIS</label></div>
                                            <div class="col-md-12" style="padding-left: 0px"><label class="radio-inline"><input type="radio" name="usbnetoptradio" value="NCM" '.($usbnetMethod=='NCM' ? 'checked' : '').'>NCM</label></div>
                                        </span>
                                    </div>
                                </div>
                            </div>
                            <div class="col-md-12" style="margin-top: 12px; text-align: center">
                                <input style="width: 100px;" class="btn btn-primary" type="submit" value="应用并重启" name="apply" id="apply" />
                                <input style="width: 100px; margin-left: 12px;" class="btn btn-warning" type="button" value="重启" name="reboot" id="reboot" />
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </form>
        <script type="text/javascript">
            $("#reboot").click(function(){
                $("#reboot").prop("disabled", true);
                $("#apply").prop("disabled", true);
                $.get("index.php?page=ajax&data=reboot");
                alert("正在重启");
            });
        </script>
    ';

    echo $output;
}

function SwitchEtherInfo($method) {
    $returnJson = shell_exec('quec_app usbnet info');
    $returnList = json_decode($returnJson, true);
    echo '<script type="text/javascript">
        var count = 0;
        var intId = setInterval(function(){
            $.getJSON("index.php?page=ajax&data=usbnet", function(data){
                if (data != "") {
                    $("#method").text(data);
                }
            });
            count++;
            if (count > 3) {
                clearInterval(intId);
            }
        }, 2000);
    </script>';
    // $output = '
    //     <div class="col-md-12 alert alert-info">
    //         当前网卡拨号方式: <span id="method">'.$returnList['mTypeName'].'</span>
    //     </div>
    // ';
    $method = $returnList['mTypeName'];
    // echo $output;
    $output = '
        <form action="/?page=switch" method="POST">
            <div class="col-md-6">
                <div class="panel panel-default">
                    <div class="intheader panel-heading">拨号方式</div>
                        <div class="panel-body">
                            <label class="radio-inline"><input type="radio" name="optradio" value="ECM" '.($method=='ECM' ? 'checked' : '').'>ECM</label>
                            <label class="radio-inline"><input type="radio" name="optradio" value="RNDIS" '.($method=='RNDIS' ? 'checked' : '').'>RNDIS</label>
                            <label class="radio-inline"><input type="radio" name="optradio" value="NCM" '.($method=='NCM' ? 'checked' : '').'>NCM</label>
                            <input class="btn btn-default" type="submit" value="应用" name="applyethconf" id="applyethconf" />
                            <input class="btn btn-default" type="button" value="重启" name="reboot" id="reboot" />
                        </div>
                    </div>
                </div>
            </div>
        </form>
        <script type="text/javascript">
            $("#reboot").click(function(){
                $("#reboot").prop("disabled", true);
                $("#applyethconf").prop("disabled", true);
                $.get("index.php?page=ajax&data=reboot");
                alert("正在重启");
            });
        </script>
    ';

    echo $output;
}

function GetCFUNStatus() {
    $cfunStatusJson = json_decode(exec('arixo_cmd atty AT+CFUN?'), true);
    return strpos($cfunStatusJson['atAck'][0], 'CFUN: ') != 0 ? explode(': ', $cfunStatusJson['atAck'][0])[1] : '0';
}

function SwitchNetworkType($method) {
    $dataJson = shell_exec('quec_app network type');
    $dataInfo = json_decode($dataJson, true);
    $networktype = $dataInfo['mNetworkType'];
    $networkstatus = $dataInfo['mNetworkStatus'];
    $dataJson = shell_exec('quec_app network nwsignalstrength');
    $dataInfo = json_decode($dataJson, true);
    $nwsignalstrenght = $dataInfo['mSigStrength'];
    $nwsignalstrenght2 = str_replace('\n', ',', $nwsignalstrenght);
    $nwsignal = explode(',', $nwsignalstrenght2);

    $returnJson = exec('quec_app APN info');
    $returnList = json_decode($returnJson, true);
    $strIpAddr = $returnList['mIpAddr'];
    $strGateway = $returnList['mGateway'];
    $strDnsAddr = $returnList['mDnsAddr'];

    $simStatusJson = exec('arixo_cmd atask sim');
    $simCardStatus = json_decode($simStatusJson, true);
    $simStatus = $simCardStatus['simStatus'];

    $registerStatusJson = exec('arixo_cmd atask register');
    $registeredStatus = json_decode($registerStatusJson, true);
    $registerStatus = $registeredStatus['registerStatus'];

    $netCarrierJson = exec('arixo_cmd atask plmn');
    $netCarrier = json_decode($netCarrierJson, true);
    $networkCarrier = $netCarrier['plmn'];

    switch ($networkCarrier) {
        case '41004':
            $networkCarrier = '中国移动香港';
            break;
        case 'CHINA MOBILE':
        case 'CHN-MOBILE':
        case '46000':
        case '46002':
        case '46007':
        case '46008':
            $networkCarrier = '中国移动';
            break;
        case 'CHINA UNICOM':
        case 'CHN-UNICOM':
        case '46001':
        case '46006':
        case '46009':
        case '46010':
            $networkCarrier = '中国联通';
            break;
        case 'CHINA TELECOM':
        case 'CHN-TELECOM':
        case '46003':
        case '46005':
        case '46011':
            $networkCarrier = '中国电信';
            break;
        default:
            break;
    }

    $cellInfoJson = exec('arixo_cmd atask servCell');
    $cellInfo = json_decode($cellInfoJson, true);
    $cellId = $cellInfo['cellId'];
    $pci = $cellInfo['pci'];
    $band = $cellInfo['band'];
    $arfcn = $cellInfo['arfcn'];

    $imsJson = exec('arixo_cmd atask ims');
    $imsInfo = json_decode($imsJson, true);
    $enableIMS = $imsInfo['ims'];

    $lockedBand = json_decode(exec('arixo_cmd atask 5glband'), true)['nr5g_band'];
    // $lockedBand = explode(',', json_decode(exec('arixo_cmd atty AT+QNWPREFCFG=\"nr5g_band\"'), true)['atAck'][0])[1];
    $lockBandList = array();
    if ($lockedBand) {
        $lockBandList = explode(':', $lockedBand);
    }

    $bandWidthInfo = json_decode(exec('arixo_cmd atask c5gqos'), true);
    $uplinkBW = '--';
    $downlinkBW = '--';

    if ($bandWidthInfo) {
        $uplinkBW = intval($bandWidthInfo['ul_sambr']);
        $downlinkBW = intval($bandWidthInfo['dl_sambr']);
        if ($uplinkBW > 1000) {
            $uplinkBW /= 1000;
            $uplinkBW .= 'Mbit/s';
        } else {
            $uplinkBW .= 'kbit/s';
        }

        if ($downlinkBW > 1000) {
            $downlinkBW /= 1000;
            $downlinkBW .= 'Mbit/s';
        } else {
            $downlinkBW .= 'kbit/s';
        }
    }

    $cfunStaus = GetCFUNStatus();

    $arxioInitConfigJson = file_get_contents('/home/user/config/arixo_init_config.conf');
    $arxioInitConfig = json_decode($arxioInitConfigJson, true);
    
    $output = '
        <form action="/?page=cellular_network" method="POST">
            <div class = "col-md-12">
                <div class="panel panel-default">
                    
                    <div class="intheader panel-heading" style="font-size: 16px;">网络制式</div>
                    <div class="panel-body" style="font-size: 14px;">
                        <div class = "col-md-6">
                            <form role="form">
                                <div class="form-group">
                                    <div class="form-inline row" style="display: flex; line-height: 32px; height: 32px; align-items: center">
                                        <span style="flex: 3; text-align: end">制式选择: </span>
                                        <span style="flex: 7; padding-left: 10px">
                                            <select name="networksel" id="networksel" class="form-control" style="min-width: 70px; width: 45%;height: 80%; padding: 0 12px;">
                                                <option value="AUTO" '.($networktype=='AUTO' ? 'selected' : '').'>AUTO</option>
                                                <option value="NR5G" '.($networktype=='NR5G' ? 'selected' : '').'>NR5G</option>
                                                <option value="NR5G-NSA" '.($networktype=='NR5G-NSA' ? 'selected' : '').'>NR5G-NSA</option>
                                                <option value="LTE" '.($networktype=='LTE' ? 'selected' : '').'>LTE</option>
                                                <option value="WCDMA" '.($networktype=='WCDMA' ? 'selected' : '').'>WCDMA</option>
                                            </select>
                                        </span>
                                    </div>
                                </div>
                            </form>
                            <div class="form-inline row" style="margin: -5px 0 10px 0; display: flex; align-items: center">
                                <span style="flex: 3; text-align: end">IMS: </span>
                                <span style="flex: 7; padding-left: 10px;">
                                    <label class="switch" style="margin-bottom: 0px">
                                        <input name="enableIMS" id="enableIMS" type="checkbox" ' . ($enableIMS == '1' ? 'checked' : '') . '/>
                                        <div class="slider round"></div>
                                    </label>
                                </span>
                            </div>
                            <div class="form-inline row" style="margin: 0 auto; display: flex; align-items: center">
                                <span style="flex: 3; text-align: end">当前网络: </span>
                                <span style="flex: 7; padding-left: 10px">' . $networkstatus . '</span>
                            </div>
                            <div class="form-inline row" style="margin: 10px auto; display: flex; align-items: center">
                                <span style="flex: 3; text-align: end">运营商: </span>
                                <span style="flex: 7; padding-left: 10px">' . $networkCarrier . '</span>
                            </div>
                            <div class="form-inline row" style="margin: 10px auto; display: flex; align-items: center">
                                <span style="flex: 3; text-align: end">上行带宽: </span>
                                <span style="flex: 7; padding-left: 10px">' . $uplinkBW . '</span>
                            </div>
                            <div class="form-inline row" style="margin: 10px auto; display: flex; align-items: center">
                                <span style="flex: 3; text-align: end">下行带宽: </span>
                                <span style="flex: 7; padding-left: 10px">' . $downlinkBW . '</span>
                            </div>
                            <div class="form-inline row" style="margin: 10px auto; display: flex; align-items: center">
                                <span style="flex: 3; text-align: end">小区ID: </span>
                                <span style="flex: 7; padding-left: 10px">' . $cellId . '</span>
                            </div>
                            <div class="form-inline row" style="margin: 10px auto; display: flex; align-items: center">
                                <span style="flex: 3; text-align: end">PCI: </span>
                                <span style="flex: 7; padding-left: 10px">';
                                    
    if ($networkstatus == 'NR5G') {
        
        $output .= '                
                                    <input type="text" name="pci" class="form-control" id="pci" style="padding: 3px 12px; width: 90px; height: 80%" value="' . $pci .'">

                                    <label class="switch" style="margin-bottom: 0px;" >
                                        <input name="pciToggle" id="pciToggle" type="checkbox" ' . ($arxioInitConfig['qnwlock']['action'] == '1' ? 'checked' : '') . ' />
                                        <span class="slider round" style="color: white;">
                                            <span id="pciToggleTextOn" name="qnwlockToggleTextOn" style="display: ' . ($arxioInitConfig['qnwlock']['action'] == '1' ? 'block': 'none') . '; padding: 2px 0 0 5px; color: white">锁定</span>
                                            <span id="pciToggleTextOff" name="qnwlockToggleTextOff" style="display: ' . ($arxioInitConfig['qnwlock']['action'] == '1' ? 'none': 'block') . '; padding: 2px 0 0 25px; color: white">解锁</span>    
                                        </span>
                                        <span>Switch</span>
                                        <script type="text/javascript">
                                            $("#pciToggle").change(function() {
                                                var toggleChecked = document.getElementById("pciToggle").checked;
                                                $("#arfcnToggle").prop("checked", toggleChecked);
                                                setTimeout(function() {
                                                    if (toggleChecked) {
                                                        document.getElementsByName("qnwlockToggleTextOn")[0].style.display = "block";
                                                        document.getElementsByName("qnwlockToggleTextOff")[0].style.display = "none";
                                                        document.getElementsByName("qnwlockToggleTextOn")[1].style.display = "block";
                                                        document.getElementsByName("qnwlockToggleTextOff")[1].style.display = "none";
                                                    } else {
                                                        document.getElementsByName("qnwlockToggleTextOn")[0].style.display = "none";
                                                        document.getElementsByName("qnwlockToggleTextOff")[0].style.display = "block";
                                                        document.getElementsByName("qnwlockToggleTextOn")[1].style.display = "none";
                                                        document.getElementsByName("qnwlockToggleTextOff")[1].style.display = "block";
                                                    }
                                                }, 100);
                                                console.log("button Clicked " + toggleChecked);
                                            });
                                        </script>
                                    </label>';
    } else {
        $output .= '                <span>' . $pci . '</span>';
    }
                                     
    
    $output .=  '               </span>
                            </div>
                            <div class="form-inline row" style="margin: 10px auto; display: flex; align-items: center">
                                <span style="flex: 3; text-align: end">ARFCN: </span>
                                <span style="flex: 7; padding-left: 10px">';
    if ($networkstatus == 'NR5G') {
        
        $output .= '                
                                    <input type="text" name="arfcn" class="form-control" id="arfcn" style="padding: 3px 12px; width: 90px; height: 80%" value="' . $arfcn .'">

                                    <label class="switch" style="margin-bottom: 0px;" >
                                        <input name="arfcnToggle" id="arfcnToggle" type="checkbox" ' . ($arxioInitConfig['qnwlock']['action'] == '1' ? 'checked' : '') . ' />
                                        <span class="slider round" style="color: white;">
                                            <span id="arfcnToggleTextOn" name="qnwlockToggleTextOn" style="display: ' . ($arxioInitConfig['qnwlock']['action'] == '1' ? 'block': 'none') . '; padding: 2px 0 0 5px; color: white">锁定</span>
                                            <span id="arfcnToggleTextOff" name="qnwlockToggleTextOff" style="display: ' . ($arxioInitConfig['qnwlock']['action'] == '1' ? 'none': 'block') . '; padding: 2px 0 0 25px; color: white">解锁</span>    
                                        </span>
                                        <span>Switch</span>
                                        <script type="text/javascript">
                                            $("#arfcnToggle").change(function() {
                                                var toggleChecked = document.getElementById("arfcnToggle").checked;
                                                $("#pciToggle").prop("checked", toggleChecked);
                                                setTimeout(function() {
                                                    if (toggleChecked) {
                                                        document.getElementsByName("qnwlockToggleTextOn")[0].style.display = "block";
                                                        document.getElementsByName("qnwlockToggleTextOff")[0].style.display = "none";
                                                        document.getElementsByName("qnwlockToggleTextOn")[1].style.display = "block";
                                                        document.getElementsByName("qnwlockToggleTextOff")[1].style.display = "none";
                                                    } else {
                                                        document.getElementsByName("qnwlockToggleTextOn")[0].style.display = "none";
                                                        document.getElementsByName("qnwlockToggleTextOff")[0].style.display = "block";
                                                        document.getElementsByName("qnwlockToggleTextOn")[1].style.display = "none";
                                                        document.getElementsByName("qnwlockToggleTextOff")[1].style.display = "block";
                                                    }
                                                }, 100);
                                                console.log("button Clicked " + toggleChecked);
                                            });
                                        </script>
                                    </label>';
    } else {
        $output .= '                <span>' . $arfcn . '</span>';
    }
    $output .=  '                </span>
                            </div>
                            <div id="bandDisplayHolder" class="form-inline row" style="display: flex; align-items: center">
                                <span style="flex: 3; text-align: end">BAND: </span>
                                <span style="flex: 7; padding-left: 10px;">
                                    <span style="width: 90px; height: 80%; display: inline-block;">' . $band . '</span>';
    if ($networkstatus == 'NR5G') {
        
        $output .= '                

                                    <label class="switch" style="margin-bottom: -10px;" >
                                        <input name="bandToggle" id="bandToggle" type="checkbox" ' . ((sizeof($lockBandList) > 0 && sizeof($lockBandList) < 6) ? 'checked' : '') . ' />
                                        <span class="slider round" style="color: white;">
                                            <span id="bandToggleTextOn" style="display: ' . ((sizeof($lockBandList) > 0 && sizeof($lockBandList) < 6) ? 'block': 'none') . '; padding: 2px 0 0 5px; color: white">锁定</span>
                                            <span id="bandToggleTextOff" style="display: ' . ((sizeof($lockBandList) > 0 && sizeof($lockBandList) < 6) ? 'none': 'block') . '; padding: 2px 0 0 25px; color: white">解锁</span>    
                                        </span>
                                        <script type="text/javascript">
                                            $("#bandToggle").change(function() {
                                                var toggleChecked = document.getElementById("bandToggle").checked;
                                                setTimeout(function() {
                                                    if (toggleChecked) {
                                                        document.getElementById("bandDisplayHolder").style.alignItems = "start";
                                                        document.getElementById("bandListHolder").style.display = "block";
                                                        document.getElementById("bandToggleTextOn").style.display = "block";
                                                        document.getElementById("bandToggleTextOff").style.display = "none";
                                                    } else {
                                                        document.getElementById("bandDisplayHolder").style.alignItems = "center";
                                                        document.getElementById("bandListHolder").style.display = "none";
                                                        document.getElementById("bandToggleTextOn").style.display = "none";
                                                        document.getElementById("bandToggleTextOff").style.display = "block";
                                                    }
                                                }, 100);
                                                console.log("button Clicked " + toggleChecked);
                                            });
                                        </script>
                                    </label>
                                    <div id="bandListHolder" style="display: ' . ((sizeof($lockBandList) > 0 && sizeof($lockBandList) < 6) ? 'block' : 'none') . '; margin-top: 10px; align-items: start;">
                                        <div class="row">
                                            <span class="col-md-2" style="line-height: 20px; padding: 0"><input type="checkbox" name="band1" id="band1" '. (in_array('1', $lockBandList, true) ? 'checked' : '') . '/> 1</span>
                                            <span class="col-md-2" style="line-height: 20px; padding: 0"><input type="checkbox" name="band28" id="band28" '. (in_array('28', $lockBandList, true) ? 'checked' : '') . '/> 28</span>
                                            <span class="col-md-2" style="line-height: 20px; padding: 0"><input type="checkbox" name="band41" id="band41" '. (in_array('41', $lockBandList, true) ? 'checked' : '') . '/> 41</span>
                                        </div>
                                        <div class="row">
                                            <span class="col-md-2" style="line-height: 20px; padding: 0"><input type="checkbox" name="band77" id="band77" '. (in_array('77', $lockBandList, true) ? 'checked' : '') . '/> 77</span>
                                            <span class="col-md-2" style="line-height: 20px; padding: 0"><input type="checkbox" name="band78" id="band78" '. (in_array('78', $lockBandList, true) ? 'checked' : '') . '/> 78</span>
                                            <span class="col-md-2" style="line-height: 20px; padding: 0"><input type="checkbox" name="band79" id="band79" '. (in_array('79', $lockBandList, true) ? 'checked' : '') . '/> 79</span>
                                        </div>
                                    </div>
                                    ';
    }
    $output .=  '               </span>
                            </div>
                        </div>
                        <div class = "col-md-6">

                            <div class="form-inline row" style="margin: 5px 0 10px 0; display: flex; align-items: center">
                                <span style="flex: 3; text-align: end">射频状态: </span>
                                <span style="flex: 7; padding-left: 10px;">
                                    <label class="switch" style="margin-bottom: 0px">
                                        <input name="enableCFUN" id="enableCFUN" type="checkbox" ' . ($cfunStaus == '1' ? 'checked' : '') . '/>
                                        <div class="slider round"></div>
                                    </label>
                                </span>
                            </div>
                            <div class="form-inline row" style="margin: 10px auto; display: flex; align-items: center">
                                <span style="flex: 3; text-align: end">SIM卡状态: </span>
                                <span style="flex: 7; padding-left: 10px">' . ($simStatus == 'Ready' ? '正常' : '异常') . '</span>
                            </div>
                            <div class="form-inline row" style="margin: 10px auto; display: flex; align-items: center">
                                <span style="flex: 3; text-align: end">注册状态: </span>
                                <span style="flex: 7; padding-left: 10px">' . ($registerStatus == 'Registered' ? '已注册' : '未注册') . '</span>
                            </div>
                            <div class="form-inline row" style="margin: 10px auto; display: flex; align-items: center">
                                <span style="flex: 3; text-align: end">IP地址: </span>
                                <span style="flex: 7; padding-left: 10px">' . $strIpAddr . '</span>
                            </div>
                            <div class="form-inline row" style="margin: 10px auto; display: flex; align-items: center">
                                <span style="flex: 3; text-align: end">网关地址: </span>
                                <span style="flex: 7; padding-left: 10px">' . $strGateway . '</span>
                            </div>
                            <div class="form-inline row" style="margin: 10px auto; display: flex; align-items: center">
                                <span style="flex: 3; text-align: end">DNS: </span>
                                <span style="flex: 7; padding-left: 10px">' . $strDnsAddr . '</span>
                            </div>
                            <div class="form-inline row" style="margin: 10px auto; display: flex">
                                <span style="flex: 3; text-align: end">信号质量: </span>
                                <span style="flex: 7; padding-left: 10px">';
    foreach ($nwsignal as $signalValue) {
        $signalValueInfo = explode(':', $signalValue);
        if (is_numeric(stripos($signalValueInfo[0], 'rssi')) || is_numeric(stripos($signalValueInfo[0], 'rsrp'))) {
            $signalValue = strtoupper($signalValueInfo[0]) . ': ' . $signalValueInfo[1] . 'dBm';
        } else if (is_numeric(stripos($signalValueInfo[0], 'rsrq')) || is_numeric(stripos($signalValueInfo[0], 'sinr'))) {
            $signalValue = strtoupper($signalValueInfo[0]) . ': ' . $signalValueInfo[1] . 'dB';
        }
        $output .= '<span>' . $signalValue . '</span><br />';
    }
    $output .= '
                                </span>
                            </div>
                        </div>
                        <div class = "col-md-12" style="margin-top: 12px; text-align: center">
                            <input style="width: 120px;" class="btn btn-primary"  type="submit" value="应用" name="applynetworktype" id="applynetworktype" />
                        </div>
                    </div>
                </div>
            </div>
        </form>
    ';
    echo $output;
}

function ShowAPNinfo($method) {

    $apnConfigJson = file_get_contents('/home/user/config/apnConfig.conf');
    $apnConfig = json_decode($apnConfigJson, true);


    $returnJson = exec('quec_app APN info');
    $returnList = json_decode($returnJson, true);
    $strIpType = $returnList['mIpType'];
    $strApnName = $returnList['mApnType'];
    $strIpAddr = $returnList['mIpAddr'];
    $strGateway = $returnList['mGateway'];
    $strDnsAddr = $returnList['mDnsAddr'];
    $strApnUsrname=$returnList['mApnUsrname'];
    $strApnPasswd=$returnList['mApnPasswd'];
    $strApnAuthtype = $returnList['mApnAuthtype'];

    $apnType = $apnConfig['apnType'];
    $apnList = $apnConfig['apnList'];
    if ($apnType == 1 && sizeof($apnList) < 4) {
        for ($i = 0; $i < 4; $i++) {
            $apnList[$i]['enable'] = 0;
            $apnList[$i]['ipType'] = '3';
            $apnList[$i]['apnName'] = '';
            $apnList[$i]['apnUsername'] = '';
            $apnList[$i]['apnPasswd'] = '';
            $apnList[$i]['apnAuthtype'] = '0';
        }
        $apnConfig['apnList'] = $apnList;
        file_put_contents('/home/user/config/apnConfig.conf', json_encode($apnConfig));
    }

    $output = '
        <form action="/?page=cellular_network" method="POST">
        <div class = "col-md-12">
        <div class="panel panel-default">
            <div class="intheader panel-heading" style="font-size:16px;">APN信息</div>
                <div class="panel-body" style="font-size: 14px;">
                    <div class = "col-md-12">
                        <div class="form-inline row" style="display: flex;
                        line-height: 35px; height: 35px;">
                            <span style="flex: 1; text-align: end">APN配置: </span>
                            <span style="flex: 6; padding-left: 10px">
                                <button class="btn btn-info" id="switchApnType" value="切换" name="switchApnType">' . ($apnType==0 ? '切换手动配置' : '切换自动配置') . '</button>
                            </span>
                        </div>
                    </div>
                    <div class="form-group">
                        <div class="form-inline row">';
    if ($apnType == 0) {
        $output .= '
                            <div class = "col-md-12" style="margin: 12px 0; padding: 5px 0; border: 2px solid lightgrey; border-radius: 5px;">
                                <div class = "col-md-6">
                                    <div style="display: flex">
                                        <span style="flex: 3; text-align: end">IP类型: </span>
                                        <span style="flex: 7; padding-left: 10px">' . $strIpType . '</span>
                                    </div>
                                    <div style="display: flex; margin-top: 12px;">
                                        <span style="flex: 3; text-align: end">APN: </span>
                                        <span style="flex: 7; padding-left: 10px">' . $strApnName . '</span>
                                    </div>
                                    <div style="display: flex; margin-top: 12px;">
                                        <span style="flex: 3; text-align: end">认证方式: </span>';
        $authTypeStr = 'NONE';
        if ($strApnAuthtype == '0') {
            $authTypeStr = 'NONE';
        } else if ($strApnAuthtype == '1') {
            $authTypeStr = 'PAP';
        } else if ($strApnAuthtype == '2') {
            $authTypeStr = 'CHAP';
        } else if ($strApnAuthtype == '3') {
            $authTypeStr = 'PAP/CHAP';
        }
        $output .= '            
                                        <span style="flex: 7; padding-left: 10px">' . $authTypeStr . '</span>
                                    </div>
                                </div>
                                <div class = "col-md-6">
                                    <div style="display: flex">
                                        <span style="flex: 3; text-align: end">用户名: </span>
                                        <span style="flex: 7; padding-left: 10px">' . $strApnUsrname . '</span>
                                    </div>
                                    <div style="display: flex; margin-top: 12px;">
                                        <span style="flex: 3; text-align: end">密码: </span>
                                        <span style="flex: 7; padding-left: 10px">' . $strApnPasswd . '</span>
                                    </div>
                                </div>
                            </div>';
    } else {
        for ($i = 0; $i < sizeof($apnList); $i++) {
            $config = $apnList[$i];

            $enabled = $config['enable'];
            $ipType = $config['ipType'];
            $apnName = $config['apnName'];
            $apnUsername = $config['apnUsername'];
            $apnPasswd = $config['apnPasswd'];
            $apnAuthtype = $config['apnAuthtype'];

            if ($enabled == 1) {
                $output .= '
                            <div class = "col-md-12" style="margin: 12px 0; padding: 10px 0; border: 2px solid lightgreen; border-radius: 5px;">';
            } else {
                $output .= '
                            <div class = "col-md-12" style="margin: 12px 0; padding: 10px 0; border: 2px solid lightgrey; border-radius: 5px;">';
            }
            $output .= '

                                <div class = "col-md-6">
                                    <div style="display: flex">
                                        <span style="flex: 3; text-align: end">IP类型: </span>
                                        <span style="flex: 7; padding-left: 10px">
                                            <select name="apninfoiptype'.$i.'" id="apninfoiptype'.$i.'" class="form-control" style="width: 25%; min-width: 95px; height: 90%">
                                                <option value="1" '.($ipType=='1' ? 'selected' : '').'>IPV4</option>
                                                <option value="2" '.($ipType=='2' ? 'selected' : '').'>IPV6</option>
                                                <option value="3" '.($ipType=='3' ? 'selected' : '').'>IPV4V6</option>
                                            </select>
                                        </span>
                                    </div>
                                    <div style="height: 12px"></div>
                                    <div style="display: flex">
                                        <span style="flex: 3; text-align: end">APN: </span>
                                        <span style="flex: 7; padding-left: 10px">
                                            <input type="text" name="apninfoapn'.$i.'" class="form-control" id="apninfoapn'.$i.'" style="min-width: 145px; width: 40%;height: 80%" value=' . $apnName . '>
                                        </span>
                                    </div>
                                    <div style="height: 12px"></div>
                                    <div style="display: flex">
                                        <span style="flex: 3; text-align: end">认证方式: </span>
                                        <span style="flex: 7; padding-left: 10px">
                                            <select name="authtype'.$i.'" id="authtype'.$i.'" class="form-control" style="width: 25%; min-width: 95px; height: 90%">
                                                <option value="0" '.($apnAuthtype=='0' ? 'selected' : '').'>NONE</option>
                                                <option value="1" '.($apnAuthtype=='1' ? 'selected' : '').'>PAP</option>
                                                <option value="2" '.($apnAuthtype=='2' ? 'selected' : '').'>CHAP</option>
                                                <option value="3" '.($apnAuthtype=='3' ? 'selected' : '').'>PAP/CHAP</option>
                                            </select>
                                        </span>
                                    </div>
                                </div>
                                <div class = "col-md-6">
                                    <div style="display: flex">
                                        <span style="flex: 3; text-align: end">用户名: </span>
                                        <span style="flex: 7; padding-left: 10px">
                                            <input type="text" name="apninfousrname'.$i.'" class="form-control" id="apninfousrname'.$i.'" style="min-width: 145px; width: 40%;height: 80%" value=' . $apnUsername . '>
                                        </span>
                                    </div>
                                    <div style="height: 12px"></div>
                                    <div style="display: flex">
                                        <span style="flex: 3; text-align: end">密码: </span>
                                        <span style="flex: 7; padding-left: 10px">
                                            <input type="text" name="apninfopasswd'.$i.'" class="form-control" id="apninfopasswd'.$i.'" style="min-width: 145px; width: 40%;height: 80%" value=' . $apnPasswd . '>
                                        </span>
                                    </div>
                                </div>
                                <div class = "col-md-12" style="height: 12px"></div>
                                <div class = "col-md-12" style="text-align: center;">';
            if ($enabled == 1) {
                $output .= '
                                    <button style="width: 100px" class="btn btn-primary" type="submit" value="'.$i.'" name="applyapninfo" id="applyapninfo">保存并应用</button>';
            } else {
                $output .= '
                                    <button style="width: 100px" class="btn btn-success" type="submit" value="'.$i.'" name="saveapninfo" id="saveapninfo">保存</button>
                                    <button style="margin-left: 12px; width: 100px;" class="btn btn-primary" type="submit" value="'.$i.'" name="applyapninfo" id="applyapninfo" >应用此配置</button>';
            }
            $output .= '
                                </div>
                            </div>';           
        }
    }
    $output .= '        </div>
                    </div>    
                </div>
            </div>
        </div>
        </form>';

    echo $output;
}

function ShowFOTAInfo() {
    $firmwareVersion = GetFirmwareVersion();

    $output = '
        <form action="/?page=dev_manage" method="post" enctype="multipart/form-data">
            <div class = "col-md-6" style="height: 300px">
                <div class="panel panel-default" style="height: 100%">
                    <div style="font-size: 16px" class="intheader panel-heading">FOTA升级</div>
                    <div class="panel-body">
                        <span style="font-size: 15px">固件版本: </span>
                        <span>' . $firmwareVersion . '</span>
                        <div class="form-group" style="margin-top: 10px">
                            <span style="font-size: 15px">本地升级: </span>
                            <div style="margin-left: 12px">
                                <span for="file">文件名：</span>
                                <input type="file" name="file" id="file"><br>
                                <input style="width: 100px" class="btn btn-primary" type="submit" name="fotaLocal" value="提 交">
                            </div>
                            <div style="margin-top: 12px;">
                                <span style="font-size: 15px">在线升级: </span><br/>
                                <input style="width: 100px; margin-left: 12px" class="btn btn-primary" style="margin-left: 12px;" type="submit" name="fotaOnline" value="检测并更新" />
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </form>';

    echo $output;
}

function ShowDeviceManage() {
    $returnvalimei = shell_exec('quec_app get IMEI');
    $imeiValue = preg_split('/[\s,：]+/', $returnvalimei);
    $returnvalfw = shell_exec('quec_app get firmware');

    $tempJson = exec('arixo_cmd atask temper');
    $temperatureJson = json_decode($tempJson, true);
    $temperature = $temperatureJson['temperature'];

    $mdTypeJson = exec('arixo_cmd atask mdType');
    $moduleTypeJson = json_decode($mdTypeJson, true);
    $moduleType = $moduleTypeJson['moduleType'];

    $vJson = exec('arixo_cmd atask vendor');
    $vendorJson = json_decode($vJson, true);
    $vendor = $vendorJson['vendor'];
    
    $output = '
        <form action="/?page=dev_manage" method="POST">
            <div class = "col-md-6" style="height: 300px">
                <div class="panel panel-default" style="height: 100%">
                    <div class="intheader panel-heading" style="font-size: 16px">设备信息</div>
                    <div class="panel-body" style="line-height: 40px">
                        <div class="form-group">
                            <div class="form-group">
                                <div style="font-size: 14px; display: flex">
                                    <span style="flex: 5; text-align: end">IMEI： </span>
                                    <span style="flex: 5">'.$imeiValue[1].'</span>
                                </div>
                                <div style="font-size: 14px; display: flex">
                                    <span style="flex: 5; text-align: end">模组型号： </span>
                                    <span style="flex: 5">'.$moduleType.'</span>
                                </div>
                                <div style="font-size: 14px; display: flex">
                                    <span style="flex: 5; text-align: end">模组厂商： </span>
                                    <span style="flex: 5">'.$vendor.'</span>
                                </div>
                                <div style="font-size: 14px; display: flex">
                                    <span style="flex: 5; text-align: end">模组温度： </span>
                                    <span style="flex: 5">'.$temperature.'℃</span>
                                </div>
                            </div>
                            <div class="form-group" style="text-align: center">
                                <button  style="width: 110px;" class="btn btn-warning" id="reboot">重启设备</button>
                                <script type="text/javascript">
                                $("#reboot").click(function(){
                                    $("#reboot").prop("disabled", true);
                                    $("#poweroff").prop("disabled", true);
                                    $.get("index.php?page=ajax&data=reboot");
                                    alert("正在重启");
                                });</script>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </form>';

    echo $output;
}

function ModLoginPasswd() {
    $loginusername = $_SESSION['username'];
    $output = '
        <form action="/?page=dev_manage" method="POST">
        <div class = "col-md-6" style="height: 260px; margin-top: 12px">
            <div class="panel panel-default" style="height: 100%">
                <div class="intheader panel-heading" style="font-size: 16px">修改登录密码</div>
                <div class="panel-body">
                    <div class="form-group">
                        <div class="form-group">
                            <input type="text" class="form-control" id="inputmodusername" name="username" placeholder='.$loginusername.' readonly="true">
                            <i class="fa fa-user"></i>
                        </div>
                        <div class="form-group help">
                            <input type="password" class="form-control" id="inputnewPassword" name="inputnewPassword" placeholder="新密码" onblur="CheckNewPasswd(this)">
                            <i class="fa fa-lock"></i>
                            <a href="#" class="fa fa-question-circle"></a>
                        </div>
                        <div class="form-group help">
                            <input type="password" class="form-control" id="inputComfirePassword" name="inputComfirePassword" placeholder="确认密码" onblur="CheckComfirmPasswd(this)">
                            <i class="fa fa-lock"></i>
                            <a href="#" class="fa fa-question-circle"></a>
                        </div>
                        <div style="text-align: center">
                            <input style="width: 100px" class="btn btn-primary" type="submit" value="应 用" name="applymodloginpasswd" id="applymodloginpasswd" />
                        </div>
                    </div>
                </div>
            </div>
        </div>
        </form>';

    echo $output;
}

function ShowNTPInfo() {
    $firewallJson = file_get_contents('/home/user/config/arixo_firewall.conf');
    $firewallData = json_decode($firewallJson, true);

    $ntpInfo = $firewallData['NTP'];
    $serverAddr1 = $ntpInfo['server1'];
    $serverAddr2 = $ntpInfo['server2'];

    $output = '<form action="/?page=dev_manage" method="POST">
                    <div class="col-md-6" style="height: 260px; margin-top: 12px">
                        <div class="panel panel-default" style="height: 100%;">
                            <div class="intheader panel-heading" style="font-size: 16px;">系统时间</div>
                            <div class="panel-body">
                                <div class="form-group">
                                    <div class="form-inline row" style="margin: 25px auto; font-size 14px; display: flex">
                                        <span style="flex: 4; text-align: end; line-height: 30px; height: 30px;">NTP服务器1:</span>
                                        <span style="flex: 6; padding-left: 10px">
                                            <input type="text" name="serverAddr1" class="form-control" id="serverAddr1" style="min-width: 145px; width: 60%; line-height: 30px; height: 30px;" value=' . $serverAddr1 . '>
                                        </span>
                                    </div>
                                    <div class="form-inline row" style="margin: 20px auto; font-size 14px; display: flex">
                                        <span style="flex: 4; text-align: end; line-height: 30px; height: 30px;">NTP服务器2:</span>
                                        <span style="flex: 6; padding-left: 10px">
                                            <input type="text" name="serverAddr2" class="form-control" id="serverAddr2" style="min-width: 145px; width: 60%; line-height: 30px; height: 30px;" value=' . $serverAddr2 . '>
                                        </span>
                                    </div>
                                    <div class = "col-md-12" style="margin-top: 12px; text-align: center">
                                        <input style="width: 100px" class="btn btn-primary" type="submit" value="应用" name="applyntpinfo" id="applyntpinfo" />
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </form>';

    echo $output;
}

function ShowDeviceOfflineCheck() {
    $offlineCheckJson = file_get_contents('/home/user/config/arixo_doserver.conf');
    $offlineCheckConfig = json_decode($offlineCheckJson, true);

    $netCheckConfig = $offlineCheckConfig['netcheck'];


    $output = '<form action="/?page=sys_setting" method="POST">
                    <div class="col-md-12" style="height: 420px; margin-top: 12px" id="offlineCheckContainer">
                        <div class="panel panel-default" style="height: 100%;">
                            <div class="intheader panel-heading" style="font-size: 16px;">离线检测</div>
                            <div class="panel-body">
                                <div class="form-group">
                                    <div class="form-inline row" style="margin: 10px auto; font-size 14px; display: flex">
                                        <span style="flex: 4; text-align: end; line-height: 30px; height: 30px;">启用离线检测:</span>
                                        <span style="flex: 6; padding-left: 10px">
                                            <label class="switch" style="margin-top: 2.5px">
                                                <input name="enableOfflineCheck" id="enableOfflineCheck" type="checkbox" ' . ($netCheckConfig['enable'] == '1' ? 'checked' : '') . '>
                                                <div class="slider round"></div>
                                            </label>
                                        </span>
                                    </div>
                                        <div class="form-inline row" style="margin-top: -5px; font-size 14px; display: flex">
                                            <span style="flex: 4; text-align: end; line-height: 30px; height: 30px;">检测策略:</span>
                                            <span style="flex: 6; padding: 4px 0 0 10px;">
                                                <label class="radio-inline"><input type="radio" name="checkType" id="" value="icmp" '.($netCheckConfig['checkType']=='icmp' ? 'checked' : '').'>Ping</label>
                                                <label class="radio-inline"><input type="radio" name="checkType" value="http" '.($netCheckConfig['checkType']=='http' ? 'checked' : '').'>HTTP</label>
                                            </span>
                                        </div>
                                        <div class="form-inline row" style="margin: 10px auto; font-size 14px; display: flex">
                                            <span style="flex: 4; text-align: end; line-height: 30px; height: 30px;">服务器地址/IP:</span>
                                            <span style="flex: 6; padding-left: 10px">
                                                <input type="text" name="serverAddr" class="form-control" id="serverAddr" style="min-width: 90px; width: 40%; line-height: 30px; height: 30px;" value=' . $netCheckConfig['addr'] . '>
                                            </span>
                                        </div>
                                        <div class="form-inline row" style="margin: 10px auto; font-size 14px; display: flex">
                                            <span style="flex: 4; text-align: end; line-height: 30px; height: 30px;">端口:</span>
                                            <span style="flex: 6; padding-left: 10px">
                                                <input type="number" name="serverPort" class="form-control" id="serverPort" style="min-width: 90px; width: 40%; line-height: 30px; height: 30px;" min=0 max=65535 value="' . $netCheckConfig['port'] . '" /><span style="margin-left: 12px; color: rgba(0,0,0,0.5)">0-65535，0为不使用端口</span>
                                            </span>
                                        </div>
                                        <div class="form-inline row" style="margin: 10px auto; font-size 14px; display: flex">
                                            <span style="flex: 4; text-align: end; line-height: 30px; height: 30px;">检测时间间隔:</span>
                                            <span style="flex: 6; padding-left: 10px">
                                                <input type="number" name="checkPeriod" class="form-control" id="checkPeriod" style="min-width: 90px; width: 40%; line-height: 30px; height: 30px;" min=10 max=600 value="' . $netCheckConfig['period'] . '" /><span style="margin-left: 12px; color: rgba(0,0,0,0.5)">10秒-600秒</span>
                                            </span>
                                        </div>
                                        <div class="form-inline row" style="margin: 10px auto; font-size 14px; display: flex">
                                            <span style="flex: 4; text-align: end; line-height: 30px; height: 30px;">检测重试次数:</span>
                                            <span style="flex: 6; padding-left: 10px">
                                                <input type="number" name="tryCount" class="form-control" id="tryCount" style="min-width: 90px; width: 40%; line-height: 30px; height: 30px;" min=0 value="' . $netCheckConfig['tryCount'] . '" />
                                            </span>
                                        </div>
                                        <div class="form-inline row" style="font-size 14px; display: flex">
                                            <span style="flex: 4; text-align: end; line-height: 30px; height: 30px;">触发策略:</span>
                                            <span style="flex: 6; padding: 4px 0 0 10px;">
                                                <label class="radio-inline"><input type="radio" name="doAction" value="cfun" '.($netCheckConfig['doAction']=='cfun' ? 'checked' : '').'>重新注册网络</label>
                                                <label class="radio-inline"><input type="radio" name="doAction" value="reboot" '.($netCheckConfig['doAction']=='reboot' ? 'checked' : '').'>重启设备</label>
                                            </span>
                                        </div>
                                        <div class = "col-md-12" style="margin-top: 12px; text-align: center">
                                            <input style="width: 100px" class="btn btn-primary" type="submit" value="应用" name="applyOfflineCheck" id="applyOfflineCheck" />
                                        </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </form>';

    echo $output;
}

function GetItemCheckbox($type, $checked, $name) {
    return '<span class="col-md-4" style="margin-top: 8px;"><input type="checkbox" name="'.$type.'" value="'.$type.'" '.($checked ? 'checked' : '').' style="margin-left: 12px;" /> '.$name.'</span>';
}

function GetCloudMqttSupportFunctions() {
    return array(
        'currentNetType' => '当前网络制式',
        'rssi' => 'RSSI/RSRP/RSRQ/SINR',
        'netType' => '制式选择',
        'ipType' => 'IP类型',
        //'apnAuthType' => '认证方式',
        'apn' => 'APN',
        //'apnUsername' => 'APN用户名',
        //'apnPwd' => 'APN密码',
        'ip' => 'IP',
        'gateway' => '网关地址',
        'dns' => 'DNS',
        //'connectionType' => '连接方式',
        //'dialType' => '网卡拨号模式',
        'firmware' => '固件版本',
        'imei' => 'IMEI',
        'temperature' => '模组温度',
        'cellId' => '小区ID',
        'pci' => 'PCI',
        'arfcn' => 'ARFCN',
        'networkCarrier' => '运营商'
    );
}

function GetArixoLinkShowInfo($arixolinkInfo) {
    $enableArixoLink = $arixolinkInfo->getAttribute('enable');
    $mqttIp = $arixolinkInfo->getElementsByTagName('mqtt_ip')->item(0)->nodeValue;
    $mqttPort = $arixolinkInfo->getElementsByTagName('mqtt_port')->item(0)->nodeValue;
    $username = $arixolinkInfo->getElementsByTagName('mqtt_user')->item(0)->nodeValue;
    $password = $arixolinkInfo->getElementsByTagName('mqtt_pwd')->item(0)->nodeValue;
    $companyKey = $arixolinkInfo->getElementsByTagName('company')->item(0)->nodeValue;
    $productKey = $arixolinkInfo->getElementsByTagName('product')->item(0)->nodeValue;

    $output = '
                <form action="/?page=cloud_manage&type=arixolink" method="POST">
                    <div class = "col-md-6" style="height: 230px">
                        <div class="panel panel-default" style="height: 100%;">
                            <div class="intheader panel-heading" style="font-size: 16px;">基本设置</div>
                            <div class="panel-body">
                                <div class="form-group">
                                    <div class="form-inline row" style="margin: 10px auto; font-size 14px; display: flex">
                                        <span style="flex: 3; text-align: end">启用ArixoLink : </span>
                                        <span style="flex: 7; padding-left: 10px">
                                            <label class="radio-inline"><input type="radio" name="enableArixoLink" value="1" '.($enableArixoLink=='1' ? 'checked' : '').'>开启</label>
                                            <label class="radio-inline"><input type="radio" name="enableArixoLink" value="0" '.($enableArixoLink=='0' ? 'checked' : '').'>关闭</label>
                                        </span>
                                    </div>
                                    <div class="form-inline row" style="margin: 10px auto; font-size 14px; display: flex">
                                        <span style="flex: 3; text-align: end">服务器地址:</span>
                                        <span style="flex: 7; padding-left: 10px">
                                            <input type="text" name="mqttIp" class="form-control" id="mqttIp" style="min-width: 145px; width: 40%;height: 80%" value=' . $mqttIp . '>
                                        </span>
                                    </div>
                                    <div class="form-inline row" style="margin: 10px auto; font-size 14px; display: flex">
                                        <span style="flex: 3; text-align: end">端口:</span>
                                        <span style="flex: 7; padding-left: 10px">
                                            <input type="text" name="mqttPort" class="form-control" id="mqttPort" style="width: 20%; min-width: 80px; height: 80%" value=' . $mqttPort . '>
                                        </span>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class = "col-md-6" style="height: 230px">
                        <div class="panel panel-default" style="height: 100%">
                            <div class="intheader panel-heading" style="font-size: 16px;">认证</div>
                            <div class="panel-body">
                                <div class="form-group">
                                    <div class="form-inline row" style="margin: 10px auto; font-size 14px; display: flex">
                                        <span style="flex: 3; text-align: end">用户名:</span>
                                        <span style="flex: 7; padding-left: 10px">
                                            <input type="text" name="username" class="form-control" id="username" style="min-width: 145px; width: 40%;height: 80%" value=' . $username . '>
                                        </span>
                                    </div>
                                    <div class="form-inline row" style="margin: 10px auto; font-size 14px; display: flex">
                                        <span style="flex: 3; text-align: end">密 码:</span>
                                        <span style="flex: 7; padding-left: 10px">
                                            <input type="text" name="password" class="form-control" id="password" style="min-width: 145px; width: 40%;height: 80%" value=' . $password . '>
                                        </span>
                                    </div>
                                    <div class="form-inline row" style="margin: 10px auto; font-size 14px; display: flex">
                                        <span style="flex: 3; text-align: end">Company Key:</span>
                                        <span style="flex: 7; padding-left: 10px">
                                            <input type="text" name="companykey" class="form-control" id="companykey" style="min-width: 145px; width: 40%;height: 80%" value=' . $companyKey . '>
                                        </span>
                                    </div>
                                    <div class="form-inline row" style="margin: 10px auto; font-size 14px; display: flex">
                                        <span style="flex: 3; text-align: end">Product Key:</span>
                                        <span style="flex: 7; padding-left: 10px">
                                            <input type="text" name="productkey" class="form-control" id="productkey" style="min-width: 145px; width: 40%;height: 80%" value=' . $productKey . '>
                                        </span>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                <div class = "col-md-12" style="margin-top: 12px;">
                    <input class="btn btn-primary" type="submit" value="应用并重启" name="applyarixolinkinfo" id="applyarixolinkinfo" />
                    <input class="btn btn-warning" style="margin-left: 12px;" type="submit" value="重启" name="restartMqtt" id="restartMqtt" />
                </div>
            </form>';
    return $output;
}

function GetMqttShowInfo($mqttInfo) {
    $enableMqtt = $mqttInfo->getAttribute('enable');
    $returnvalimei = shell_exec('quec_app get IMEI');
    $clientId = $mqttInfo->getElementsByTagName('client_id')->item(0)->nodeValue;
    if ($clientId == '0123456789ABCDEFG') {
        $clientId = substr($returnvalimei, 6);
    }
    $mqttIp = $mqttInfo->getElementsByTagName('mqtt_ip')->item(0)->nodeValue;
    $mqttPort = $mqttInfo->getElementsByTagName('mqtt_port')->item(0)->nodeValue;
    $username = $mqttInfo->getElementsByTagName('mqtt_user')->item(0)->nodeValue;
    $password = $mqttInfo->getElementsByTagName('mqtt_pwd')->item(0)->nodeValue;
    $keepalive = $mqttInfo->getElementsByTagName('keepalive')->item(0)->nodeValue;
    $autoReconnect = $mqttInfo->getElementsByTagName('auto_reconnect')->item(0)->nodeValue;
    $clearSession = $mqttInfo->getElementsByTagName('clear_session')->item(0)->nodeValue;


    $returnMqttStatus = shell_exec('arixo_cmd mqtt getsta');
    $mqttStatus = json_decode($returnMqttStatus, true);

    $output = '
                <form action="/?page=cloud_manage&type=mqtt" method="POST">
                    <div class = "col-md-6" style="height: 240px;">
                        <div class="panel panel-default" style="height: 100%">
                            <div class="intheader panel-heading" style="font-size: 16px;">基本设置</div>
                            <div class="panel-body">
                                <div class="form-group">
                                    <div class="form-inline row" style="font-size 14px; display: flex">
                                        <span style="flex: 3; text-align: end">状态: </span>
                                        <span style="flex: 7; padding-left: 10px">
                                            '.($mqttStatus['result'] == "ON" ? '运行中' : '未启动').'
                                        </span>
                                    </div>
                                    <div class="form-inline row" style="margin: 10px auto; font-size 14px; display: flex">
                                        <span style="flex: 3; text-align: end">启用MQTT: </span>
                                        <span style="flex: 7; padding-left: 10px">
                                            <label class="radio-inline"><input type="radio" name="enableMqtt" value="1" '.($enableMqtt=='1' ? 'checked' : '').'>开启</label>
                                            <label class="radio-inline"><input type="radio" name="enableMqtt" value="0" '.($enableMqtt=='0' ? 'checked' : '').'>关闭</label>
                                        </span>
                                    </div>
                                    <div class="form-inline row" style="margin: 10px auto; font-size 14px; display: flex">
                                        <span style="flex: 3; text-align: end">ClientId: </span>
                                        <span style="flex: 7; padding-left: 10px">
                                            <input type="text" name="clientId" class="form-control" id="clientId" style="width: 80%; min-width: 150px; height: 80%" value=' . $clientId . '>
                                        </span>
                                    </div>
                                    <div class="form-inline row" style="margin: 10px auto; font-size 14px; display: flex">
                                        <span style="flex: 3; text-align: end">服务器地址: </span>
                                        <span style="flex: 7; padding-left: 10px">
                                            <input type="text" name="mqttIp" class="form-control" id="mqttIp"style="min-width: 145px; width: 40%;height: 80%" value=' . $mqttIp . '>
                                        </span>
                                    </div>
                                    <div class="form-inline row" style="margin: 10px auto; font-size 14px; display: flex">
                                        <span style="flex: 3; text-align: end">端口: </span>
                                        <span style="flex: 7; padding-left: 10px">
                                            <input type="text" name="mqttPort" class="form-control" id="mqttPort" style="width: 20%; min-width: 80px; height: 80%" value=' . $mqttPort . '>
                                        </span>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class = "col-md-6" style="height: 240px;">
                        <div class="panel panel-default" style="height: 100%;">
                            <div class="intheader panel-heading" style="font-size: 16px;">认证</div>
                            <div class="panel-body">
                                <div class="form-group">
                                    <div class="form-inline row" style="margin: 0 auto;">
                                        <div class="form-inline row" style="margin: 10px auto; font-size 14px; display: flex">
                                            <span style="flex: 3; text-align: end">用户名:</span>
                                            <span style="flex: 7; padding-left: 10px">
                                                <input type="text" name="username" class="form-control" id="username" style="min-width: 145px; width: 40%;height: 80%" value=' . $username . '>
                                            </span>
                                        </div>
                                        <div class="form-inline row" style="margin: 10px auto; font-size 14px; display: flex">
                                            <span style="flex: 3; text-align: end">密 码:</span>
                                            <span style="flex: 7; padding-left: 10px">
                                                <input type="text" name="password" class="form-control" id="password" style="min-width: 145px; width: 40%;height: 80%" value=' . $password . '>
                                            </span>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class = "col-md-12" style="margin-top: 12px;">
                        <div class="panel panel-default">
                            <div class="intheader panel-heading" style="font-size: 16px;">高级配置</div>
                            <div class="panel-body">
                                <div class="form-group">
                                    <div class="form-inline row" style="margin: 0 auto; font-size: 14px;">
                                        <div class = "col-md-4" style="display: flex">
                                            <span style="flex: 3; text-align: end">心跳(秒): </span>
                                            <span style="flex: 7; padding-left: 10px">
                                                <input type="text" name="keepalive" class="form-control" id="keepalive" style="min-width: 100px; width: 40%;height: 80%"  value=' . $keepalive . '>
                                            </span>
                                        </div>
                                        <div class = "col-md-4" style="display: flex">
                                            <span style="flex: 3; text-align: end">自动重连: </span>
                                            <span style="flex: 7; padding-left: 10px">
                                                <label class="radio-inline"><input type="radio" name="autoReconnect" value="1" '.($autoReconnect=='1' ? 'checked' : '').'>开启</label>
                                                <label class="radio-inline"><input type="radio" name="autoReconnect" value="0" '.($autoReconnect=='0' ? 'checked' : '').'>关闭</label>
                                            </span>
                                            
                                        </div>
                                        <div class = "col-md-4" style="display: flex">
                                            <span style="flex: 3; text-align: end">清除会话: </span>
                                            <span style="flex: 7; padding-left: 10px">
                                                <label class="radio-inline"><input type="radio" name="clearSession" value="1" '.($clearSession=='1' ? 'checked' : '').'>开启</label>
                                                <label class="radio-inline"><input type="radio" name="clearSession" value="0" '.($clearSession=='0' ? 'checked' : '').'>关闭</label>
                                            </span>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>';

    $publishList = $mqttInfo->getElementsByTagName('publish');
    $output .= '
                        <div class = "col-md-12">
                            <div class="panel panel-default">
                                <div class="intheader panel-heading" style="font-size: 16px;">消息发布(最多10条)</div>
                                <div class="panel-body">
                                    <div class="form-group">';
    $publishIndex = 0;
    foreach ($publishList as $ele) {
        $topic = $ele->getAttribute('topic');
        $period = $ele->getAttribute('period');
        $qos = $ele->getAttribute('qos');
        $output .= '
                                        <div class="form-inline row" style="margin: 12px 0; padding: 5px 0; border: 2px solid lightgrey; border-radius: 5px; font-size: 14px;">
                                            <div class = "col-md-4" style="display: flex">
                                                <span style="flex: 3; text-align: end">Topic: </span>
                                                <span style="flex: 7; padding-left: 10px">
                                                    <input type="text" name="topic'.$publishIndex.'" maxlength="64" class="form-control" id="topic'.$publishIndex.'" style="min-width: 150px; width: 40%;height: 80%" value=' . $topic . '>
                                                    <span style="font-size: 8px;">(最多64个字节)</label>
                                                </span>
                                            </div>
                                            <div class = "col-md-4" style="display: flex; line-height: 35px; height: 35px;">
                                                <span style="flex: 3; text-align: end">QoS: </span>
                                                <span style="flex: 7; padding-left: 10px">
                                                    <select name="qos'.$publishIndex.'" id="qos'.$publishIndex.'" class="form-control" style="min-width: 55px; width: 20%;height: 90%">
                                                        <option value="0" '.($qos=='0' ? 'selected' : '').'>0</option>
                                                        <option value="1" '.($qos=='1' ? 'selected' : '').'>1</option>
                                                        <option value="2" '.($qos=='2' ? 'selected' : '').'>2</option>
                                                    </select>
                                                </span>
                                            </div>
                                            <div class = "col-md-4" style="display: flex">
                                                <span style="flex: 3; text-align: end">Period(秒): </span>
                                                <span style="flex: 7; padding-left: 10px">
                                                    <input type="text" name="period'.$publishIndex.'" class="form-control" id="period'.$publishIndex.'" style="min-width: 80px; width: 20%; height: 80%" value=' . $period . '>
                                                    <span style="font-size: 8px;">(最小为0, 0为开机上报一次)</label>
                                                </span>
                                            </div>
                                            <div class = "col-md-12" style="margin-top: 12px; display: flex;">
                                                <span style="flex: 1; text-align: end">上报信息: </span>
                                                <div style="flex: 10">';
            $items = $ele->getElementsByTagName('item');
            $checkboxItems = '';
            $supportFunctions = GetCloudMqttSupportFunctions();
            foreach ($supportFunctions as $itemName => $itemValue) {
                $hasItem = false;
                foreach ($items as $itemTag) {
                    $nodeItemValue = $itemTag->nodeValue;
                    if ($itemName == $nodeItemValue) {
                        $hasItem = true;
                        $output .= GetItemCheckbox($itemName.$publishIndex, true, ($itemValue . '(' . $itemName . ')' ));
                        break;
                    }
                }
                if (!$hasItem) {
                    $output .= GetItemCheckbox($itemName.$publishIndex, false, ($itemValue . '(' . $itemName . ')' ));
                }
            }

            $output .= '                        
                                                </div> <!-- col-md-10 -->
                                            </div> <!-- col-md-12 上报信息 -->
                                            <div class = "col-md-12" style="text-align: end; margin-top: 12px;">
                                                <button style="width: 80px" class="btn btn-danger" id="removePublish" value="'.$publishIndex.'" name="removeMqttPublish" id="removeMqttPublish">删 除</button>
                                            </div>
                                        </div> <!-- form-inline row -->';
            $publishIndex++;
    }

    if ($publishList->length < 10) {
            $output .= '                <div class="form-inline row" style="margin: 10px 0; padding: 5px 0;">
                                            <div class = "col-md-12" style="text-align: center">
                                                <button class="btn btn-success" style="width: 150px" id="addPublish">添     加</button>
                                                <script type="text/javascript">
                                                $("#addPublish").click(function(){
                                                    $.get("index.php?page=ajax&data=addmqttpublish");
                                                });</script>
                                            </div>
                                        </div>';
    }
        $output .='                 </div>
                                </div>
                            </div>
                        </div>
                        
                        <div class = "col-md-12" style="padding-bottom: 12px;">
                            <input class="btn btn-primary" type="submit" value="应用并重启" name="applymqttinfo" id="applymqttinfo" />
                            <input class="btn btn-warning" style="margin-left: 12px;" type="submit" value="重启" name="restartMqtt" id="restartMqtt" />
                        </div>
                    </form>';
    return $output;
}

function GetByPassInfo() {
    $byPassConfJson = file_get_contents('/home/user/config/byPass.conf');
    $byPassConfig = json_decode($byPassConfJson, true);

    $enableByPass = $byPassConfig['enable'];
    $ipAddr = $byPassConfig['ip'];
    $port = $byPassConfig['port'];

    $byPassConnectResultJson = exec('arixo_cmd SPTT getconn');
    $byPassConnectResult = json_decode($byPassConnectResultJson, true);
    $connectStatus = $byPassConnectResult['socketConnection'] == 'Connected' ? '已连接' : '未连接';

    $output = '<form action="/?page=cloud_manage&type=bypass" method="POST">
                    <div class = "col-md-12" style="height: 180px">
                        <div class="panel panel-default" style="height: 100%;">
                            <div class="intheader panel-heading" style="font-size: 16px;">基本设置</div>
                            <div class="panel-body">
                                <div class="form-group">
                                    <div class="col-md-6">
                                        <div class="form-inline row" style="margin: 10px auto; font-size 14px; display: flex">
                                            <span style="flex: 3; text-align: end">启用串口透传: </span>
                                            <span style="flex: 7; padding-left: 10px">
                                                <label class="radio-inline"><input type="radio" name="enableByPass" value="1" '.($enableByPass=='1' ? 'checked' : '').'>开启</label>
                                                <label class="radio-inline"><input type="radio" name="enableByPass" value="0" '.($enableByPass=='0' ? 'checked' : '').'>关闭</label>
                                            </span>
                                        </div>
                                        <div class="form-inline row" style="margin: 10px auto; font-size 14px; display: flex">
                                            <span style="flex: 3; text-align: end">连接状态: </span>
                                            <span style="flex: 7; padding-left: 10px">'.$connectStatus.'</span>
                                        </div>
                                    </div>
                                    <div class="col-md-6">
                                        <div class="form-inline row" style="margin: 10px auto; font-size 14px; display: flex;">
                                            <span style="flex: 3; text-align: end; line-height: 25px; height: 25px;">服务器地址:</span>
                                            <span style="flex: 7; padding-left: 10px">
                                                <input type="text" name="ipAddr" class="form-control" id="ipAddr" style="min-width: 145px; line-height: 25px; height: 25px;" value=' . $ipAddr . '>
                                            </span>
                                        </div>
                                        <div class="form-inline row" style="margin: 10px auto; font-size 14px; display: flex;">
                                            <span style="flex: 3; text-align: end; line-height: 25px; height: 25px;">服务器端口:</span>
                                            <span style="flex: 7; padding-left: 10px">
                                                <input type="text" name="port" class="form-control" id="port" style="width: 20%; min-width: 80px; line-height: 25px; height: 25px;" value=' . $port . '>
                                            </span>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class = "col-md-12" style="margin-top: 12px;">
                        <input class="btn btn-primary" type="submit" value="应用" name="applybypassinfo" id="applybypassinfo" />
                    </div>
                </form>';

    return $output;
}

function ShowCloudManage($cloudType) {

    $docObj = GetMqttXMLDoc();
    $doc = $docObj['doc'];
    $file = $docObj['file'];
    $arixolink = $doc->documentElement->getElementsByTagName('arixolink')->item(0);
    $mqttInfo = $doc->documentElement->getElementsByTagName('mqtt')->item(0);

    $output = '
        <div class="tab-div">
            <div class="tab-div-nav">
                <div class="col-md-2">
                    <a href="index.php?page=cloud_manage&type=arixolink"'.($cloudType == "arixolink" ? 'style="color:#3D84C5;border-bottom: 3px solid #3D84C5"' : '').'>ArixoLink</a>
                </div>
                <div class="col-md-2">
                    <a href="index.php?page=cloud_manage&type=mqtt"'.($cloudType == "mqtt" ? 'style="color:#3D84C5;border-bottom: 3px solid #3D84C5"' : '').'>MQTT</a>
                </div>
                <div class="col-md-2">
                    <a href="index.php?page=cloud_manage&type=bypass"'.($cloudType == "bypass" ? 'style="color:#3D84C5;border-bottom: 3px solid #3D84C5"' : '').'>透传服务</a>
                </div>
            </div>
            <div style="height:3px;z-index:-1;width:100%;margin-top:-3px;margin-bottom: 10px;background-color:lightgrey"></div>
        ';
    if ($cloudType == 'mqtt') {
        $output .= GetMqttShowInfo($mqttInfo);        
    } else if ($cloudType == 'arixolink') {
        $output .= GetArixoLinkShowInfo($arixolink);
    } else if ($cloudType == 'bypass') {
        $output .= GetByPassInfo();
    }

    $output .= '</div>';

    echo $output;
}

function CheckAndCreateNode($doc, $node, $parentNode, $nodeName) {
    if (!isset($node)) {
        $newNode = $doc->createElement($nodeName);
        $newNode->nodeValue = '';
        $parentNode->appendChild($newNode);
        return $newNode;
    }
    return $node;
}

function GetMqttXMLDoc() {
    $doc = new DOMDocument;
    $doc->formatOutput = true;
    $doc->preserveWhiteSpace = false;
    //$doc->load('/etc/quectel/arixo_iot5g_config.xml');
    $file = '/home/user/config/arixo_iot5g_config.xml';
    $doc->load($file);
    $returnObj = array(
        'doc' => $doc,
        'file' => $file);
    return $returnObj;
}

function SendAt($atcmd) {
    $retJson = shell_exec('arixo_cmd atty '.$atcmd);
    $atRet = json_decode($retJson, true);
    return $atRet['atAck'];
}

function GetFirmwareVersion() {
    $retJson = shell_exec('arixo_cmd dofunc getver');
    $versionResult = json_decode($retJson, true);
    $resultVer = '';
    if ($versionResult['status'] == 'OK') {
        $resultVer = $versionResult['result'];
    }
    return $resultVer;
}

function ChangeLanIp($newIp) {
    $retJson = shell_exec('arixo_cmd dofunc lanip='.$newIp);
    $changeResult = json_decode($retJson, true);
    return $changeResult['status'] == 'OK';
}

function BytesFormat($num) {
    $p = 0;
    $format = 'bytes';
    if( $num > 0 && $num < 1024 ) {
        $p = 0;
        return number_format($num) . ' ' . $format;
    }
    if( $num >= 1024 && $num < pow(1024, 2) ){
        $p = 1;
        $format = 'KB';
    }
   if ( $num >= pow(1024, 2) && $num < pow(1024, 3) ) {
        $p = 2;
        $format = 'MB';
    }
    if ( $num >= pow(1024, 3) && $num < pow(1024, 4) ) {
        $p = 3;
        $format = 'GB';
    }
    if ( $num >= pow(1024, 4) && $num < pow(1024, 5) ) {
        $p = 3;
        $format = 'TB';
    }
    $num /= pow(1024, $p);
    return number_format($num, 3) . ' ' . $format;
}

?>

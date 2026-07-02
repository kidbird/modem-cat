<?php

    exec('ifconfig wlan0',$return);
    $strWlan0 = implode(" ",$return);
    $strWlan0 = preg_replace('/\s\s+/', ' ', $strWlan0);
    preg_match('/HWaddr ([0-9a-f:]+)/i',$strWlan0,$result);
    $strHWAddress = $result[1];
    preg_match('/inet addr:([0-9.]+)/i',$strWlan0,$result);
    $strIPAddress = $result[1];
    preg_match('/Mask:([0-9.]+)/i',$strWlan0,$result);
    $strNetMask = $result[1];
    preg_match('/RX packets:(\d+)/',$strWlan0,$result);
    $strRxPackets = $result[1];
    preg_match('/TX packets:(\d+)/',$strWlan0,$result);
    $strTxPackets = $result[1];
    preg_match('/RX bytes:(\d+ \(\d+.\d+ [K|M|G]iB\))/i',$strWlan0,$result);
    $strRxBytes = $result[1];
    preg_match('/TX bytes:(\d+ \(\d+.\d+ [K|M|G]iB\))/i',$strWlan0,$result);
    $strTxBytes = $result[1];
    //preg_match('/ESSID:\"([\-\.a-zA-Z0-9\s]+)\"/i',$strWlan0,$result);
    //$strSSID = str_replace('"','',$result[1]);
    //preg_match('/Access Point: ([0-9a-f:]+)/i',$strWlan0,$result);
    //$strBSSID = $result[1];
    //preg_match('/Bit Rate:([0-9]+ Mb\/s)/i',$strWlan0,$result);
    //$strBitrate = $result[1];
    preg_match('/Tx-Power=([0-9]+ dBm)/i',$strWlan0,$result);
    $strTxPower = $result[1];
    //preg_match('/Link Quality=([0-9]+\/[0-9]+)/i',$strWlan0,$result);
    //$strLinkQuality = $result[1];
    //preg_match('/Signal level=([0-9]+\/[0-9]+)/i',$strWlan0,$result);
    //$strSignalLevel = $result[1];

    $wlanJson = shell_exec('quec_app wifi info');
    $wlanInfo = json_decode($wlanJson, true);
    $id = $wlanInfo['mNetworkId'];
    $BSSID = $wlanInfo['mBSSID'];
    $status = $wlanInfo['mSupplicantState'];
    $rssi = $wlanInfo['mRssi'];
    $linkSpeed = $wlanInfo['mLinkSpeed'];
    $wlanJson = shell_exec('quec_app wifi list');
    $wlanList = json_decode($wlanJson, true);
    if ($wlanList != null) {
        foreach ($wlanList as $wlanInfo) {
            if ($wlanInfo['networkId'] == $id){
                $SSID = $wlanInfo['SSID'];
                break;
            }
        }
    }
    $state = getWlanState();
    //if(strpos($strWlan0, "UP") !== false) {
    if ($state == 3) {
        $strStatus = '<span style="color:green">WiFi打开</span>';
    } else {
        $strStatus = '<span style="color:red">WiFi关闭</span>';
    }
    echo '
        <div class="page-header"><h1>WiFi信息</h1></div>
        <div class = "col-md-4">
            <div class="panel panel-default">
                <div class="intheader panel-heading">接口信息</div>
                <div class="panel-body">
                接口名称 : wlan0 <br />
                接口状态 : ' . $strStatus . ' <br />
                IP地址 : ' . $strIPAddress . ' <br />
                子网掩码 : ' . $strNetMask .' <br />
                MAC地址 : ' . $strHWAddress . ' <br />
                </div>
            </div>
        </div>
        <div class = "col-md-4">
            <div class="panel panel-default">
                <div class="intheader panel-heading">接口数据</div>
                <div class="panel-body">
                接收数据包 : ' . $strRxPackets . ' <br />
                接收字节 : ' . $strRxBytes . ' <br />
                转移数据包 : ' . $strTxPackets . ' <br />
                转移字节 : ' . $strTxBytes .' <br />
                </div>
            </div>
        </div>
        <div class = "col-md-4">
            <div class="panel panel-default">
                <div class="intheader panel-heading">WiFi信息</div>
                <div class="panel-body">
                连接到 : ' . substr($SSID,1,-1) . ' <br />
                BSSID : ' . $BSSID . ' <br />
                状态 : ' . $status . ' <br />
                连接速率 : ' . $linkSpeed .' <br />
                信号强度(Rssi) : ' . $rssi .' <br />
                </div>
            </div>
        </div>
        ';

?>

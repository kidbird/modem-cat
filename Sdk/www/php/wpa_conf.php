<?php

  //   if(isset($_POST['SaveWPAPSKSettings'])) {
  //       $ssid = $_POST['ssid'];
  //       $psk = $_POST['psk'];
  //       SaveWlan($ssid, $psk);
  //       ShowWPAConfHead();
  //   }
  //   elseif(isset($_POST['Scan'])) {
  //       ShowWPAConfHead();
		// ShowScan();
  //   } elseif(isset($_POST['Connect'])) {
  //       $ssid = $_POST['Connect'];
  //       ConnectWlan($ssid);
  //       ShowWPAConfHead();
  //   } elseif(isset($_POST['Delete'])) {
  //       $ssid = $_POST['Delete'];
  //       DeleteWlan($ssid);
  //       ShowWPAConfHead();
  //   } elseif(isset($_POST['Enable'])) {
  //       EnableWlan();
  //       ShowWPAConfHead();
  //   } elseif(isset($_POST['Disable'])) {
  //       DisableWlan();
  //       ShowWPAConfHead();
  //   } elseif(isset($_POST['Reconnect'])) {
  //       ReconnectWlan();
  //       ShowWPAConfHead();
  //   } elseif(isset($_POST['Disconnect'])) {
  //       DisconnectWlan();
  //       ShowWPAConfHead();
  //   } else {
  //       GetWiFiPageInfo('2_4G');
  //   }

    echo '<div class="page-header"><h1>WLAN配置信息</h1></div>';

    // $wifiType = $_GET['config'];
    // if (!isset($wifiType)) {
    //     $wifiType = '2_4G';
    // }

    if (isset($_POST['apply2Ginfo'])) {
        $enable2G = isset($_POST['enable2G']);
        $ssid2G = $_POST['ssid2G'];
        $pwd2G = $_POST['pwd2G'];
        $authType2G = $_POST['authType2G'];
        $channel2G = $_POST['channel2G'];

        SaveWifiConfig($ssid2G, $pwd2G, $authType2G, $channel2G, '2_4G');

        if (!$enable2G) {
            exec('/etc/init.d/quec_wlan.init stop_2G');
        } else {
            exec('/etc/init.d/quec_wlan.init restart_2G');
        }
    } elseif (isset($_POST['apply5Ginfo'])) {
        $enable5G = isset($_POST['enable5G']);
        $ssid5G = $_POST['ssid5G'];
        $pwd5G = $_POST['pwd5G'];
        $authType5G = $_POST['authType5G'];
        $channel5G = $_POST['channel5G'];

        SaveWifiConfig($ssid5G, $pwd5G, $authType5G, $channel5G, '5_8G');

        if (!$enable5G) {
            exec('/etc/init.d/quec_wlan.init stop_5G');
        } else {
            exec('/etc/init.d/quec_wlan.init restart_5G');
        }
    }
    GetWiFiPageInfo();

?>

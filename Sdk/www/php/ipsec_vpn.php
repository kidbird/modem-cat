<?php

    echo '<div class="page-header"><h1>IPSec VPN</h1></div>';

    if (isset($_POST['applyipsecinfo'])) {
        $totalCount = $_POST['totalRemoteConfigCount'];
        $configInfo = json_decode('{}', true);
        $configInfo['local'] = array();
        $configInfo['local'][0] = $_POST['localIpStart'];
        $configInfo['local'][1] = $_POST['localIpEnd'];
        $configInfo['forward'] = $_POST['forwardType'];
        if (isset($totalCount) && $totalCount > 0) {
            $configInfo['channel'] = array();
            for ($i=1; $i < $totalCount + 1; $i++) { 
                if (isset($_POST['encapType'.$i])) {
                   $configInfo['channel'][''.$i]['encap'] = $_POST['encapType'.$i];
                   $configInfo['channel'][''.$i]['symm_alg_type'] = $_POST['symmAlg'.$i];
                   $configInfo['channel'][''.$i]['sa_life_1'] = $_POST['mainSaLife'.$i];
                   $configInfo['channel'][''.$i]['auth_alg'] = 'SM2';
                   $configInfo['channel'][''.$i]['dpd_interval'] = $_POST['dpdInterval'.$i];
                   $configInfo['channel'][''.$i]['sa_life_2'] = $_POST['fastSaLife'.$i];
                   $configInfo['channel'][''.$i]['anti_replay'] = (isset($_POST['enableESP'.$i]) ? '1' : '0');
                   $configInfo['channel'][''.$i]['replaywindowsize'] = $_POST['replayWindowSize'.$i];
                   $configInfo['channel'][''.$i]['peer'] = $_POST['remoteIp'.$i];
                   $configInfo['channel'][''.$i]['remote'] = array();
                   $configInfo['channel'][''.$i]['remote'][0] = $_POST['remoteIpStart'.$i];
                   $configInfo['channel'][''.$i]['remote'][1] = $_POST['remoteIpEnd'.$i];
                   $configInfo['channel'][''.$i]['reset'] = (isset($_POST['resetStatus'.$i]) ? $_POST['resetStatus'.$i] : '0');
                }
            }
        }
        SaveIPSecVPNInfo($configInfo);
    } elseif (isset($_POST['deleteRemoteConfig'])) {
        $currentInfo = GetIPSecVPNInfo();

        unset($currentInfo['channel'][$_POST['deleteRemoteConfig']]);
        SaveIPSecVPNInfo($currentInfo);
    } elseif (isset($_POST['resetChannel'])) {
        $currentInfo = GetIPSecVPNInfo();
        $currentInfo['channel'][$_POST['resetChannel']]['reset'] = '1';
        SaveIPSecVPNInfo($currentInfo);
    }

    ShowIPSecVPNPage();
?>

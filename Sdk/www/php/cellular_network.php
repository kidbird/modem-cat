<?php

    echo '<div class="page-header"><h1>蜂窝网络</h1></div>';

    if (isset($_POST['applynetworktype'])) {
        $method = $_POST['networksel'];
        $enableIMS = isset($_POST['enableIMS']) ? 1 : 0;
        $enableCFUN = isset($_POST['enableCFUN']) ? 1 : 0;
        $returnJson = shell_exec('quec_app networktype set '.$method.'');
        $returnList = json_decode($returnJson, true);

        $enableQNWLock = (isset($_POST['pciToggle']) || isset($_POST['arfcnToggle'])) ? '1' : '0';

        if (isset($_POST['bandToggle']) && !isset($_POST['band1']) && !isset($_POST['band28']) &&
                !isset($_POST['band41']) && !isset($_POST['band77']) && !isset($_POST['band78']) && 
                !isset($_POST['band79'])) {
            echo '<script>alert("请选择BAND");</script>';
            header('refresh:0; url=index.php?page=cellular_network');
            exit;
        }

        if (isset($_POST['pciToggle']) && (!isset($_POST['pci']) || $_POST['pci'] == "")) {
            echo '<script>alert("请输入PCI");</script>';
            header('refresh:0; url=index.php?page=cellular_network');
            exit;
        } 

        if (isset($_POST['arfcnToggle']) && (!isset($_POST['arfcn']) || $_POST['arfcn'] == "")) {
            echo '<script>alert("请输入ARFCN");</script>';
            header('refresh:0; url=index.php?page=cellular_network');
            exit;
        } 

        $imsJson = exec('arixo_cmd atask ims');
        $imsInfo = json_decode($imsJson, true);
        $oldEnableIMS = $imsInfo['ims'];

        if ($enableIMS != $oldEnableIMS) {
            $setResult = shell_exec('arixo_cmd atset ims '.$enableIMS.'');
        }

        $initConfig = json_decode(file_get_contents('/home/user/config/arixo_init_config.conf'), true);

        $lastQNWLockEnable = $initConfig['qnwlock']['action'];
        $initConfig['qnwlock']['action'] = $enableQNWLock;
        $initConfig['qnwlock']['enable'] = $enableQNWLock;
        $initConfig['qnwlock']['standard'] = 'common/5g';
        if (isset($_POST['arfcn'])) {
           $initConfig['qnwlock']['freq'] = $_POST['arfcn'];
        }

        if (isset($_POST['pci'])) {
            $initConfig['qnwlock']['pci'] = $_POST['pci'];
        }

        file_put_contents('/home/user/config/arixo_init_config.conf', json_encode($initConfig, JSON_UNESCAPED_SLASHES));

        if ($lastQNWLockEnable != $enableQNWLock) {
            shell_exec('/etc/init.d/start_arixo_init_config start_config');
        }

        if ($enableCFUN != GetCFUNStatus()) {
            shell_exec('arixo_cmd atty AT+CFUN=' . $enableCFUN);
        }

        if (isset($_POST['bandToggle'])) {
            $bandList = '';
            if ($_POST['band1']) {
                $bandList .= '1:';
            }
            if ($_POST['band28']) {
                $bandList .= '28:';
            }
            if ($_POST['band41']) {
                $bandList .= '41:';
            }
            if ($_POST['band77']) {
                $bandList .= '77:';
            }
            if ($_POST['band78']) {
                $bandList .= '78:';
            }
            if ($_POST['band79']) {
                $bandList .= '79';
            }

            if ((strrpos($bandList, ':') + 1) == strlen($bandList)) {
                $bandList = substr($bandList, 0, strlen($bandList) -1);
            }
            shell_exec('arixo_cmd atset 5glband ' . $bandList);
            // shell_exec('arixo_cmd atty AT+QNWPREFCFG=\"nr5g_band\",'.$bandList);
        } else {
            shell_exec('arixo_cmd atty AT+QNWPREFCFG=\"all_band_reset\"');
        }

        if ($returnList['status'] == 1) {
            echo '<script>alert("设置成功");</script>';
        } else {
            echo '<script>alert("设置失败");</script>';
        }
    }
        
    if (isset($_POST['applyapninfo'])) {
        $applyapnIndex = $_POST['applyapninfo'];

        $apnConfigJson = file_get_contents('/home/user/config/apnConfig.conf');
        $apnConfig = json_decode($apnConfigJson, true);
        $apnList = $apnConfig['apnList'];

        for ($i = 0; $i < 4; $i++) {
            $apnList[$i]['enable'] = 0;
        }

        $iptype = $_POST['apninfoiptype'.$applyapnIndex];
        $apnparam = $_POST['apninfoapn'.$applyapnIndex];
        $apnusrname = $_POST['apninfousrname'.$applyapnIndex];
        $apnpasswd = $_POST['apninfopasswd'.$applyapnIndex];
        $apnauthtype = $_POST['authtype'.$applyapnIndex];

        $apnList[$applyapnIndex]['ipType'] = $iptype;
        $apnList[$applyapnIndex]['apnName'] = $apnparam;
        $apnList[$applyapnIndex]['apnUsername'] = $apnusrname;
        $apnList[$applyapnIndex]['apnPasswd'] = $apnpasswd;
        $apnList[$applyapnIndex]['apnAuthtype'] = $apnauthtype;

        $apnList[$applyapnIndex]['enable'] = 1;

        $apnConfig['apnList'] = $apnList;
        file_put_contents('/home/user/config/apnConfig.conf', json_encode($apnConfig));

        $returnJson = exec('arixo_cmd atty AT+QICSGP=1,'.$iptype.',\"'.$apnparam.'\",\"'.$apnusrname.'\",\"'.$apnpasswd.'\",' .$apnauthtype);
        // $returnJson = shell_exec('quec_app apn set '.$iptype.' '.$apnparam.' '.$apnusrname.' '.$apnpasswd.' ' .$apnauthtype. '');
        $returnList = json_decode($returnJson, true);
        if ($returnList['status'] == 'OK' && $returnList['atAck'][0] == 'OK') {
            echo '<script>alert("设置成功");</script>';
        } else {
            echo '<script>alert("设置失败");</script>';
        }
    } 
    if (isset($_POST['saveapninfo'])) {
        $saveApnIndex = $_POST['saveapninfo'];

        $apnConfigJson = file_get_contents('/home/user/config/apnConfig.conf');
        $apnConfig = json_decode($apnConfigJson, true);
        $apnList = $apnConfig['apnList'];

        for ($i = 0; $i < 4; $i++) {
            $apnList[$i]['enable'] = 0;
        }

        $apnList[$saveApnIndex]['ipType'] = $_POST['apninfoiptype'.$saveApnIndex];
        $apnList[$saveApnIndex]['apnName'] = $_POST['apninfoapn'.$saveApnIndex];
        $apnList[$saveApnIndex]['apnUsername'] = $_POST['apninfousrname'.$saveApnIndex];
        $apnList[$saveApnIndex]['apnPasswd'] = $_POST['apninfopasswd'.$saveApnIndex];
        $apnList[$saveApnIndex]['apnAuthtype'] = $_POST['authtype'.$saveApnIndex];

        $apnConfig['apnList'] = $apnList;
        file_put_contents('/home/user/config/apnConfig.conf', json_encode($apnConfig));
        echo '<script>alert("保存成功");</script>';
    }

    if (isset($_POST['switchApnType'])) {
        $apnConfigJson = file_get_contents('/home/user/config/apnConfig.conf');
        $apnConfig = json_decode($apnConfigJson, true);

        $apnType = $apnConfig['apnType'];

        if ($apnType == 0) {
            $apnConfig['apnType'] = 1;
            $apnList = $apnConfig['apnList'];

            for ($i = 0; $i < 4; $i++) {
                $apnList[$i]['enable'] = 0;
            }

            $apnConfig['apnList'] = $apnList;
        } else {
            $apnConfig['apnType'] = 0;
            $resetApnResult = exec('arixo_cmd atty AT+QICSGP=1,3,\"\",\"\",\"\",0');
        }

        file_put_contents('/home/user/config/apnConfig.conf', json_encode($apnConfig));
    }
    
    SwitchNetworkType($method);
    ShowAPNinfo($method);

?>

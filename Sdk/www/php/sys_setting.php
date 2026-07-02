<?php 
    
    echo '<div class="page-header"><h1>系统设置</h1></div>';
    if (isset($_POST['applymodloginpasswd'])) {
        $usrname = $_SESSION['username'];
        $newpasswd = $_POST['inputnewPassword'];

        class MyDB extends SQLite3{
            function __construct(){
                $this->open('/mnt/data/quec_web.db');
            }
        }
        $db = new MyDB();
        if(!$db){
            echo $db->lastErrorMsg();
        } else {
            //echo "Opened database successfully\n";
        }
        
        $sql="UPDATE tbl_usrname set PASSWD='$newpasswd' where USERNAME='$usrname';";
        //echo "$sql";
        $ret = $db->query($sql);
        //echo "$ret";
        $db->close();
        echo '<script>alert("修改成功,请重新登录");</script>';
        
        $output ='
            <script type="text/javascript">
            window.location.href="login.html"; 
            </script>';
        echo $output;
        
        exit;

    } elseif (isset($_POST['applyntpinfo'])) {
        $firewallJson = file_get_contents('/home/user/config/arixo_firewall.conf');
        $firewallData = json_decode($firewallJson, true);

        $ntpInfo = $firewallData['NTP'];

        $serverAddr1 = $_POST['serverAddr1'];
        $serverAddr2 = $_POST['serverAddr2'];

        $ntpInfo['server1'] = $serverAddr1;
        $ntpInfo['server2'] = $serverAddr2;

        $firewallData['NTP'] = $ntpInfo;
        file_put_contents('/home/user/config/arixo_firewall.conf', json_encode($firewallData));
        
        exec('arixo_cmd dofunc ntp'); 
        echo '<script>alert("配置成功");</script>';
    } elseif (isset($_POST['applyOfflineCheck'])) {
        $offlineCheckJson = file_get_contents('/home/user/config/arixo_doserver.conf');
        $offlineCheckConfig = json_decode($offlineCheckJson, true);


        if ($_POST['enableOfflineCheck']) {
            $offlineCheckConfig['netcheck']['checkType'] = $_POST['checkType'];
            $offlineCheckConfig['netcheck']['addr'] = $_POST['serverAddr'];
            $offlineCheckConfig['netcheck']['port'] = $_POST['serverPort'];
            $offlineCheckConfig['netcheck']['period'] = $_POST['checkPeriod'];
            $offlineCheckConfig['netcheck']['tryCount'] = $_POST['tryCount'];
            $offlineCheckConfig['netcheck']['doAction'] = $_POST['doAction'];
        }

        $offlineCheckConfig['netcheck']['enable'] = isset($_POST['enableOfflineCheck']) ? '1' : '0';
        file_put_contents('/home/user/config/arixo_doserver.conf', json_encode($offlineCheckConfig));
        exec('/etc/init.d/start_arixo_init_config start_server');
        echo '<script>alert("配置成功");</script>';
    }
    
    ShowNTPInfo();
    ModLoginPasswd();
    ShowDeviceOfflineCheck();

?>

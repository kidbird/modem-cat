<?php

    header('Content-type:text/html');
    // 开启Session
    session_start();if (isset($_GET['action']) && $_GET['action'] == 'showlogo') {
        $projInfo = json_decode(shell_exec('arixo_cmd dofunc showlogo'), true);
        echo (isset($projInfo['result']) && $projInfo['result'] == '1') ? 1 : 0;
        exit;
    } 
    
    // 首先判断Cookie是否记住了用户信息
    if (isset($_COOKIE['username'])) {
        # 若记住了用户信息,则直接传给Session
        $_SESSION['username'] = $_COOKIE['username'];
        $_SESSION['islogin'] = 1;
    }
    if (isset($_SESSION['islogin'])) {
        // 若已经登录
        //echo "你好! ".$_SESSION['username'].' ,已登录!<br>';
        //echo "<a href='logout.php'>注销</a>";
    } else {
        // 若没有登录
        //echo "您还没有登录,请<a href='login.html'>登录</a>";
        header('location:login.html');
        exit;
    }

    echo '<link rel="icon" href="images/favicon.ico" type="image/x-icon" />';
    echo '<link rel="Bookmark" href="images/favicon.ico" type="image/x-icon" />';
    echo '<link rel="shortcut icon" href="images/favicon.ico" type="image/x-icon" />';
    
    include('./php/phpincs.php');
    $output = $return = 0;

    $page = isset($_GET['page']) ? trim(strtolower($_GET['page'])) : "home";
    
    switch($_SESSION['username']){
        case 'quectel':
            if ($page == 'ajax') {
                $data = trim(strtolower($_GET['data']));
                switch($data){
                    case 'conn':
                        $returnJson = shell_exec('quec_app conn info');
                        $returnList = json_decode($returnJson, true);
                        if (!empty($returnList['mTypeName'])) {
                            echo json_encode($returnList['mTypeName']);
                        }
                        break;
                    case 'usbnet':
                        $returnJson = shell_exec('quec_app usbnet info');
                        $returnList = json_decode($returnJson, true);
                        if (!empty($returnList['mTypeName'])) {
                            echo json_encode($returnList['mTypeName']);
                        }
                        break;
                    case 'reboot':
                        shell_exec('reboot');
                        break;
                    case 'poweroff':
                        shell_exec('poweroff');
                        break;
                    case 'pdpactive':
                        shell_exec('quec_app conn on');
                        break;
                    case 'pdpdeactive':
                        shell_exec('quec_app conn off');
                        break;
                    case 'getimeiinfo':
                        shell_exec('quec_app get IMEI');
                        break;
                    default:
                        break;
                }
                
                return;
            }
        
            include('./php/header.php');
        
            $allowedPages = array(
                'home' => './php/home.php',
                'switch' => './php/switch.php',
                'cellular_network' => './php/cellular_network.php',
                'eth_conf' => './php/eth_conf.php',
                'eth_info' => './php/eth_info.php',
                'wpa_conf' => './php/wpa_conf.php',
                'wlan0_info' => './php/wlan0_info.php',
                'dhcpd_conf' => './php/dhcpd_conf.php',
                'dev_manage' => './php/dev_manage.php',
                'config' => './php/hidden/zapdos_config.php',
            );
            include( isset($allowedPages[$page]) ? $allowedPages[$page] : $allowedPages["home"] );
            break;
        case 'admin':
            
            if ($page == 'ajax') {
                $data = trim(strtolower($_GET['data']));
                switch($data){
                    case 'conn':
                        $returnJson = shell_exec('quec_app conn info');
                        $returnList = json_decode($returnJson, true);
                        if (!empty($returnList['mTypeName'])) {
                            echo json_encode($returnList['mTypeName']);
                        }
                        break;
                    case 'usbnet':
                        $returnJson = shell_exec('quec_app usbnet info');
                        $returnList = json_decode($returnJson, true);
                        if (!empty($returnList['mTypeName'])) {
                            echo json_encode($returnList['mTypeName']);
                        }
                        break;
                    case 'pdpactive':
                        shell_exec('quec_app conn on');
                        break;
                    case 'pdpdeactive':
                        shell_exec('quec_app conn off');
                        break;
                    case 'reboot':
                        shell_exec('reboot');
                        break;
                    case 'poweroff':
                        shell_exec('poweroff');
                        break;
                    case 'getimeiinfo':
                        shell_exec('quec_app get IMEI');
                        break;
                    case 'addmqttpublish':
                        $docObj = GetMqttXMLDoc();
                        $doc = $docObj['doc'];
                        $file = $docObj['file'];
                        $mqttInfo = $doc->documentElement->getElementsByTagName('mqtt')->item(0);
                        $newPublish = $doc->createElement('publish');
                        $newPublish->setAttribute('topic', '');
                        $newPublish->setAttribute('period', '0');
                        $newPublish->setAttribute('qos', '1');
                        $newPublish->nodeValue = '';
                        $mqttInfo->appendChild($newPublish);
                        $doc->save($file);
                        break;
                    case 'sendat':
                        $atcmd = $_POST['atCmd'];
                        $atResult = shell_exec('arixo_cmd atty ' . str_replace('"', '\"', $atcmd));
                        echo $atResult;
                        break;
                    case 'checkipsecstatus':
                        $result = shell_exec('cat /proc/ipsec');
                        $data = json_decode('{}', true);
                        $data['data'] = $result;
                        echo json_encode($data);
                        break;
                    default:
                        break;
                }
                
                return;
            }
        
            include('./php/headerex.php');
        
            $allowedPages = array(
                'home' => './php/home.php',
                'switch' => './php/switch.php',
                'cellular_network' => './php/cellular_network.php',
                'eth_conf' => './php/eth_conf.php',
                'eth_info' => './php/eth_info.php',
                'wpa_conf' => './php/wpa_conf.php',
                'wlan0_info' => './php/wlan0_info.php',
                'dhcpd_conf' => './php/dhcpd_conf.php',
                'dev_manage' => './php/dev_manage.php',
                'config' => './php/hidden/zapdos_config.php',
                'cloud_manage' => './php/cloud_manage.php',
                'sys_setting' => './php/sys_setting.php',
                'ipsec_vpn' => './php/ipsec_vpn.php',
            );

            include( isset($allowedPages[$page]) ? $allowedPages[$page] : $allowedPages["home"] );
            break;
        default:
            break;
    }

    include("./php/footer.php");
?>

<?php

    $supportFuncs = json_decode(exec('arixo_cmd dofunc webswh'), true);
    $needShowLogo = json_decode(exec('arixo_cmd dofunc showlogo'), true);
    $output = '
    <html lang="en">
        <head>
            <meta charset="utf-8">
            <meta http-equiv="X-UA-Compatible" content="IE=edge">
            <meta name="viewport" content="width=device-width, initial-scale=1">

            <link href="../css/styles.css" rel="stylesheet" type="text/css">
            <link href="../css/jsoneditor.css" rel="stylesheet" type="text/css">
            <link href="../css/bootstrap.min.css" rel="stylesheet">
            <link href="../css/bootstrap-theme.min.css" rel="stylesheet">
            <link href="../css/bootstrap-select.min.css" rel="stylesheet">
            <link href="../css/img/jsoneditor-icons.svg">


            <script src="../js/jquery.min.js"></script>
            <script src="../js/socket.io.js"> </script>
            <script src="../js/bootstrap.min.js"></script>
            <script src="../js/bootstrap-select.min.js"></script>
            <script src="../js/jsoneditor.js"></script>
            <script src="../js/functions.js"></script>';
    if ($needShowLogo['result'] == '1') {
        $output .= '<title>ArixoLink通信模块配置</title>';
    } else {
        $output .= '<title>通信模块配置</title>';
    }
    $output .= '
        </head>
        <body>
            <div class="container" style="height:100%">
                <div class="row" style="height:100%">
                <div class="col-md-2" style="height:100%; position: fixed">
                    <div>
                    <nav class="navbar navbar-default" role="navigation" style="height:100%;">
                        <div class="navbar-header">
                            <button type="button" class="navbar-toggle" data-toggle="collapse" data-target="#bs-example-navbar-collapse-1">
                                <span class="sr-only">Toggle navigation</span><span class="icon-bar"></span><span class="icon-bar"></span><span class="icon-bar"></span>
                            </button>';
    if ($needShowLogo['result'] == '1') {
        $output .= '        <a href="index.php"><img src="images/logo.jpg" style="width: 90%"></a>';
    }
    $output .= '
                        </div>

                        <div class="collapse navbar-collapse" id="navbar" style="height:100%; text-align: center;">
                            <ul class="nav navbar-nav" style="height:85%">';
    if (!isset($needShowLogo['result']) || $needShowLogo['result'] == '0') {
        $output .= '
                                <li class="navli">
                                    <a href="index.php">首页</a>
                                </li>';
    }

    if ($supportFuncs['connect']) {
        $output .= '
                                <li class="navli">
                                    <a href="index.php?page=switch">连接方式</a>
                                </li>';
    }
    if ($supportFuncs['cellular']) {
        $output .= '
                                <li class="navli">
                                    <a href="index.php?page=cellular_network">蜂窝网络</a>
                                </li>';
    }
    if ($supportFuncs['lan']) {
        $output .= '
                                <li class="navli">
                                    <a href="index.php?page=eth_info">LAN配置</a>
                                </li>';
    }
    if ($supportFuncs['wlan']) {
        $output .= '
                                <li>
                                    <a href="index.php?page=wpa_conf">WLAN配置</a>
                                </li>';
    }
    if ($supportFuncs['ipsecvpn']) {
        $output .= '
                                <li class="navli">
                                    <a href="index.php?page=ipsec_vpn">IPSec VPN</a>
                                </li>';
    }
    if ($supportFuncs['cloud']) {
        $output .= '
                                <li class="navli">
                                    <a href="index.php?page=cloud_manage">云服务</a>
                                </li>';
    }
    $output .= '
                                <li class="navli">
                                    <a href="index.php?page=dev_manage">设备信息</a>
                                </li>
                                <li class="navli">
                                    <a href="index.php?page=sys_setting">系统设置</a>
                                </li>';
    if (isset($_SESSION['islogin'])) {
        // 若已经登录
        $output .= '
                                <li class="navli">
                                    <a href="logout.php">注销</a>
                                </li>
        ';
    }
    $output .= '
                            </ul>';
    if ($needShowLogo['result'] == '1') {
        $output .= '        <div style="display: inline-block; width: 100%">
                                <span>Copyright ©️ 2021 北京零矩科技有限公司 ALL Rights Reserved</span>
                            </div>';
    }
    $output .= '        </div>
                    </nav>
                    </div>
                </div>
                <div class="col-md-10 col-sm-12" style="left: 16%">
    ';
    echo $output;

?>

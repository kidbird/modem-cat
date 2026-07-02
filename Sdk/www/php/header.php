<?php

    echo '
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
            <script src="../js/functions.js"></script>
            <script src="../js/bootstrap-select.min.js"></script>
            <script src="../js/jsoneditor.js"></script>

            <title>ArixoLink通信模块配置</title>
        </head>
        <body>
            <div class="container">
                <div class="row">
                <div class="col-md-12">
                    <nav class="navbar navbar-default" role="navigation">
                        <div class="navbar-header">
                            <button type="button" class="navbar-toggle" data-toggle="collapse" data-target="#bs-example-navbar-collapse-1">
						        <span class="sr-only">Toggle navigation</span><span class="icon-bar"></span><span class="icon-bar"></span><span class="icon-bar"></span>
                            </button>
                           <a href="index.php"><img src="images/logo.jpg"></a>                           

                        </div>

                        <div class="collapse navbar-collapse" id="navbar">
                            <ul class="nav navbar-nav">
                                <li class="navli">
                                    <a href="index.php?page=switch">连接方式</a>
                                </li>
                                <li class="navli">
                                    <a href="index.php?page=cellular_network">蜂窝网络</a>
                                </li>
                                <li>
                                    <a href="index.php?page=eth_conf">LAN配置</a>
                                </li>
                                <li>
                                    <a href="index.php?page=eth_info">LAN信息</a>
                                </li>
                                <li>
                                    <a href="index.php?page=wpa_conf">WiFi配置</a>
                                </li>
                                <li>
                                    <a href="index.php?page=wlan0_info">WiFi信息</a>
                                </li>
                                <li>
                                    <a href="index.php?page=dev_manage">设备信息</a>
                                </li>';
    if (isset($_SESSION['islogin'])) {
        // 若已经登录
        echo '
                                <li>
                                    <a href="logout.php">注销</a>
                                </li>
        ';
    }
    echo '
                            </ul>
                        </div>
                    </nav>
    ';

?>

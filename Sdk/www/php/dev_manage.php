<?php 
    
    echo '<div class="page-header"><h1>设备信息</h1></div>';
    if (isset($_POST['fotaOnline'])) {

        $firmwareVersion = GetFirmwareVersion();

        $versionArray = explode('_', $firmwareVersion);

        $versionPath = str_replace('.', '_', $versionArray[3]);

        $returnJson = shell_exec('arixo_cmd fota https://fota.matrixz.cn/MW600-GQ/update/'.$versionPath.'/newest/update.zip');
        $returnList = json_decode($returnJson, true);

        if ($returnList['status'] == 'OK') {
            echo '<script>alert("正在升级请勿断电或做任何操作, 设备将会自动重启");</script>';
        } else {
            echo '<script>alert("未检测到新版本，无需升级");</script>';
        }
    } elseif (isset($_POST['fotaLocal'])) {

        $allowedExts = array("zip");
        $temp = explode(".", $_FILES["file"]["name"]);
        $extension = end($temp);     // 获取文件后缀名

        if ($_FILES["file"]["error"] > 0) {
            echo '<script>alert("上传文件失败，请检查文件并重新上传。");</script>';
            echo "错误：" . $_FILES["file"]["error"] . "<br>";
        } else {
            if (!is_null($_FILES["file"]) && $temp != '') {
                if ($_FILES["file"]["type"] == "application/x-zip-compressed" && $_FILES["file"]["size"] < 102400000 && in_array($extension, $allowedExts)) {
                    //echo "上传文件名: " . $_FILES["file"]["name"] . "<br>";
                    //echo "文件类型: " . $_FILES["file"]["type"] . "<br>";
                    //echo "文件大小: " . ($_FILES["file"]["size"] / 1024) . " kB<br>";
                    //echo "文件临时存储的位置: " . $_FILES["file"]["tmp_name"];

                    move_uploaded_file($_FILES["file"]["tmp_name"], "/home/user/update.zip");
                    $returnJson = shell_exec('arixo_cmd fota /home/user/update.zip');
                    $returnList = json_decode($returnJson, true);
                    if ($returnList['status'] == 'OK') {
                        echo '<script>alert("上传成功，正在升级请勿断电或做任何操作, 设备将会自动重启");</script>';
                    } else{
                        echo '<script>alert("升级失败，请检查文件是否正确并重试");</script>';
                    }
                    //echo "文件存储在: " . "/var/tmp/" . $_FILES["file"]["name"];
                } else {
                    echo '<script>alert("文件格式错误，请上传正确文件。");</script>';
                }
            }
        }
    }
    

    ShowFOTAInfo();
    ShowDeviceManage();
    
        //showLog();
        //var_dump($returnval);
    // if (isset($_GET['open'])) {
    //     //do something to open
    //     shell_exec('am broadcast -a com.maxtropy.zapdos.socket.START');
    //     sleep(2);
    //     $returnval = shell_exec('netstat -tupln | grep 8081 | wc -l');
    //     if ($returnval == 1) {
    //         showLog();
    //         echo '<script>listen()</script>';
    //     } else {
    //         echo '<script>alert("打开失败");</script>';
    //         showLog();
    //     }
    // } elseif (isset($_GET['close'])) {
    //     //do something to close
    //     shell_exec('am broadcast -a com.maxtropy.zapdos.socket.END');
    //     sleep(2);
    //     $returnval = shell_exec('netstat -tupln | grep 8081 | wc -l');
    //     if ($returnval == 0) {
    //         showLog();
    //     } else {
    //         echo '<script>alert("关闭失败");</script>';
    //         showLog();
    //     }
    // } else {
        
    // }

?>

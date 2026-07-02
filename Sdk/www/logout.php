<?php
    header('Content-type:text/html');
    header('refresh:1; url=login.html');
    // 注销后的操作
    session_start();
    // 清除Session
    $username = $_SESSION['username']; //用于后面的提示信息
    $_SESSION = array();
    session_destroy();
  
    // 清除Cookie
    setcookie($username, '', time()-99);
    echo '<link rel="icon" href="images/favicon.ico" type="image/x-icon" />';
    echo '<link rel="Bookmark" href="images/favicon.ico" type="image/x-icon" />';
    echo '<link rel="shortcut icon" href="images/favicon.ico" type="image/x-icon" />';
    // 提示信息
    // echo "请重新登录, ".$username.'<br>';
    // echo "<a href='login.html'>重新登录</a>";
    
 ?>
<?php 
    header('Content-type:text/html');
    // 开启Session
    session_start();

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

    // 处理用户登录信息
    if(isset($_POST['login'])){
        # 接收用户的登录信息
        $username = trim($_POST['username']);
        $password = trim($_POST['password']);

        $sql="SELECT * from tbl_usrname where USERNAME='$username';";
        //echo "$sql";
        $ret = $db->query($sql);
        $row = $ret->fetchArray(SQLITE3_ASSOC);
        // while($row = $ret->fetchArray(SQLITE3_ASSOC)){
        //     echo "USERNAME = ".$row['USERNAME']."\n";
        //     echo "PASSWD = ".$row['PASSWD']."\n";
        // }
        //echo "Operation done successfully\n";
        $db->close();
        // 判断提交的登录信息
        if(($username == '') || ($password == '')){
            // 若为空,视为未填写,提示错误,并3秒后返回登录界面
            header('refresh:3; url=login.html');
            echo "用户名或密码不能为空,系统将在3秒后跳转到登录界面,请重新填写登录信息!";
            exit;
        }elseif(($username != $row['USERNAME']) || ($password != $row['PASSWD'])){
            # 用户名或密码错误,同空的处理方式
            header('refresh:3; url=login.html');
            echo "用户名或密码错误,系统将在3秒后跳转到登录界面,请重新填写登录信息!";
            exit;
        }elseif(($username == $row['USERNAME']) && ($password == $row['PASSWD'])){
            # 用户名和密码都正确,将用户信息存到Session中
            $_SESSION['username'] = $username;
            $_SESSION['islogin'] = 1;
    
            // 没有勾选则删除Cookie
            setcookie($username, '', time()-999);
            //setcookie('root', '', time()-999);
            //setcookie('code', '', time()-999);
            // 处理完附加项后跳转到登录成功的首页
            header('location:index.php');
        }
    }
 ?>
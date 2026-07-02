<?php

    #echo '<img src="images/ie.jpg" style="margin-top: 50px; width: 40%">'; 

    $retJsonpdp = exec('quec_app conn pdpstatus');
    $returnList = json_decode($retJsonpdp, true);
    $retpdpstatus = $returnList['mPdpStatus'];
    $temppdpactive=1;
    $temppdpdeactive=0;
    echo '<img src="images/ie.jpg" style="top: 10%;width: 30%;margin:auto;" class="img-responsive">';
    echo '<div class="page-header"><h1></h1></div>';
    echo '<div class="row" align="center">
			<input class="btn btn-lg btn-info" type="button" style="margin-right:12%;" value="拨号上网"  name="gogo" id="pdpactive" />
			<script type="text/javascript">
				if('.$retpdpstatus.' == '.$temppdpactive.'){
					$("#pdpactive").prop("disabled", true);
				}else{
					$("#pdpactive").prop("disabled", false);
				}
				$("#pdpactive").click(function(){
						$.get("index.php?page=ajax&data=pdpactive");
						$("#pdpactive").prop("disabled", true);
						alert("操作成功");
				});
			</script> 
			<input class="btn btn-lg btn-info" type="button" value="断开网络"  name="gogo" id="pdpdeactive" />
			<script type="text/javascript">
				if('.$retpdpstatus.' == '.$temppdpdeactive.'){
					$("#pdpdeactive").prop("disabled", true);
				}else{
					$("#pdpdeactive").prop("disabled", false);
				}
				$("#pdpdeactive").click(function(){
						$.get("index.php?page=ajax&data=pdpdeactive");
						$("#pdpdeactive").prop("disabled", true);
						alert("操作成功");
				});
			</script>
		</div>';
	$uptimeInfo = exec('cat /proc/uptime');
	$uptimeInfoDetail = explode(' ', $uptimeInfo);
	$uptime = $uptimeInfoDetail[0];

    $output = '
    	<div class="row" style="margin-top: 24px">
    		<div style="font-size: 16px; display: flex">
            	<span style="flex: 5; text-align: end">运行时间： </span>
            	<span id="runtime" style="flex: 5">';

    $d = floor($uptime / (3600*24));
	$h = floor(($uptime % (3600*24)) / 3600);
	$m = floor((($uptime % (3600*24)) % 3600) / 60);
	$s = floor((($uptime % (3600*24)) % 3600) % 60);

	$runtime = $d.'天 '.$h.'小时 '.$m.'分 ' .$s. '秒';

	$output .= $runtime;

	$dFlowJson = shell_exec('arixo_cmd dofunc dflow');
	$dFlow = json_decode($dFlowJson, true);

	$rxBytes = BytesFormat($dFlow['rxBytes']);
	$txBytes = BytesFormat($dFlow['txBytes']);

    $output .= '
    			</span>
    		</div>
    		<div style="font-size: 16px; display: flex">
            	<span style="flex: 5; text-align: end">已发送： </span>
            	<span id="txBytes" style="flex: 5">'.$txBytes.'</span>
            </div>
    		<div style="font-size: 16px; display: flex">
            	<span style="flex: 5; text-align: end">已接收： </span>
            	<span id="rxBytes" style="flex: 5">'.$rxBytes.'</span>
            </div>
    	</div>';
    echo $output;
?>

# AT 指令功能映射

## 1. 基础信息查询

| 功能 | AT 指令 | 解析函数 | 返回数据结构 |
|------|---------|----------|-------------|
| SIM 状态 | `AT+CPIN?` | `parse_cpin()` | `String` (READY/UNKNOWN) |
| IMEI | `AT+CGSN` | `parse_cgsn()` | `String` |
| ICCID | `AT+CCID` | `parse_ccid()` | `String` |
| 备用 ICCID | `AT+QCCID` | `parse_ccid()` | `String` |
| 模组信息 | `ATI` | `parse_ati()` | `(manufacturer, model, firmware)` |
| 型号 | `AT+CGMM` | `parse_cgmm()` | `String` |
| 厂商 | `AT+CGMI` | `parse_cgmi()` | `String` |
| 固件版本 | `AT+GMR` | `parse_gmr()` | `String` |
| AP 基线 | `AT+QBASELINE` | `parse_qbaseline()` | `String` |
| CP 基线 | `AT+QBASELINE` | `parse_qbaseline()` | `String` |
| SOC 温度 | `AT+QTEMP` | `parse_qtemp()` | `String` (如 "48°C") |
| PA 温度 | `AT+QTEMP` | `parse_qtemp()` | `String` |

## 2. 网络状态查询

| 功能 | AT 指令 | 解析函数 | 返回数据结构 |
|------|---------|----------|-------------|
| 服务小区 | `AT+QENG="servingcell"` | `parse_qeng_servingcell()` | `ServingCellInfo` |
| 运营商 | `AT+COPS?` | `parse_cops()` | `(operator_name, act)` |
| 天线信号 | `AT+QANTRSSI?` | `parse_qantrssi()` | `[String; 4]` |
| 连接状态 | `AT+CGACT?` | `parse_cgact()` | `Vec<(cid, status)>` |
| 注册状态 | `AT+CEREG?` | `parse_cereg()` | `(status, tac, ci)` |
| 网络模式 | `AT+QNWPREFCFG="mode_pref"` | `parse_qnwprefcfg_mode()` | `String` |

## 3. IP 与数据连接

| 功能 | AT 指令 | 解析函数 | 返回数据结构 |
|------|---------|----------|-------------|
| IP 信息 | `AT+QNETDEVSTATUS=<cid>` | `parse_qnetdevstatus()` | `(ipv4, mask, gw, dns, ipv6)` |
| APN 列表 | `AT+QICSGP?` | `parse_qicsgp()` | `Vec<ApnEntry>` |
| 连接数据 | `AT+QNETDEVCTL=<cid>,3,1` | `is_ok()` | `()` |
| 断开数据 | `AT+QNETDEVCTL=<cid>,2,0` | `is_ok()` | `()` |

## 4. 邻区信息

| 功能 | AT 指令 | 解析函数 | 返回数据结构 |
|------|---------|----------|-------------|
| 邻区列表 | `AT+QENG="neighbourcell"` | `parse_qeng_neighbourcell()` | `(lte_cells, nr_cells)` |

## 5. 频段配置

| 功能 | AT 指令 | 解析函数 | 返回数据结构 |
|------|---------|----------|-------------|
| 支持频段 | `AT+QNWPREFCFG=?` | `parse_qnwprefcfg_supported()` | `(lte, nr)` |
| LTE 锁定频段 | `AT+QNWPREFCFG="lte_band"` | `parse_qnwprefcfg_bands()` | `Vec<String>` |
| NR 锁定频段 | `AT+QNWPREFCFG="nr5g_band"` | `parse_qnwprefcfg_bands()` | `Vec<String>` |
| 设置 LTE 频段 | `AT+QNWPREFCFG="lte_band","B1:B3:B5"` | `is_ok()` | `()` |
| 设置 NR 频段 | `AT+QNWPREFCFG="nr5g_band","n1:n3:n5"` | `is_ok()` | `()` |
| 重置频段 | `AT+QNWPREFCFG="all_band_reset"` | `is_ok()` | `()` |

## 6. QoS 与流量

| 功能 | AT 指令 | 解析函数 | 返回数据结构 |
|------|---------|----------|-------------|
| QoS 信息 | `AT+C5GQOSRDP=<cid>` | `parse_c5gqosrdp()` | `(cqi, ul_bw, dl_bw)` |
| 流量统计 | `AT+QGDCNT?` | `parse_qgdcnt()` | `(ul_bytes, dl_bytes)` |

## 7. 功能开关

| 功能 | AT 指令 | 解析函数 | 返回数据结构 |
|------|---------|----------|-------------|
| PCIe 模式 | `AT+QCFG="pcie/mode"` | `parse_qcfg_int()` | `bool` |
| 以太网 | `AT+QCFG="ethernet"` | `parse_qcfg_int()` | `bool` |
| ETH AT | `AT+QCFG="eth_at"` | `parse_qcfg_int()` | `bool` |
| UART AT | `AT+QCFG="uartat"` | `parse_qcfg_int()` | `bool` |
| USB 配置 | `AT+QCFG="usbcfg"` | `parse_qcfg_usbcfg_adb()` | `bool` |
| USB 网卡模式 | `AT+QCFG="usbnet"` | `parse_qcfg_usbnet()` | `i32` |
| Proxy ARP | `AT+QCFG="proxyarp"` | `parse_qcfg_int()` | `bool` |

## 8. 控制命令

| 功能 | AT 指令 | 解析函数 | 备注 |
|------|---------|----------|------|
| 设置 APN | `AT+QICSGP=<cid>,<type>,"<apn>"` | `is_ok()` | |
| 删除 APN | `AT+CGDCONT=<cid>` | `is_ok()` | |
| 设置网络模式 | `AT+QNWPREFCFG="mode_pref","AUTO"` | `is_ok()` | |
| 重启模组 | `AT+CRESET` | `is_ok()` | |
| 恢复出厂 | `AT+QFACT=0` | `is_ok()` | |
| 设置功能开关 | `AT+QCFG="<key>",<value>` | `is_ok()` | |
| 设置 USB 网卡模式 | `AT+QCFG="usbnet",<mode>` | `is_ok()` | |

## 9. 解析辅助函数

| 函数 | 功能 |
|------|------|
| `is_ok()` | 检查响应是否以 OK 结尾 |
| `extract_data_lines()` | 提取数据行（过滤回显、OK、ERROR） |
| `format_rsrp()` | RSRP 格式化（-94 dBm） |
| `format_rsrq()` | RSRQ 格式化（-4 dB） |
| `format_bw()` | 带宽格式化（100 MHz） |
| `format_bandwidth_bps()` | 带宽 bps 格式化（1000 Mbps） |
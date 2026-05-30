# AT 指令功能映射

平台划分：
- **UniSoc（展锐）**：RM500U / RG200U / RG500U
- **Qualcomm（高通）**：RM500Q / RG500Q / RM520N / RG520N / RM551E / RM530F / RM530N / RG530F / RG525F / RM501Q

---

## 1. 基础信息查询（通用）

| 功能 | AT 指令 | 解析函数 | 备注 |
|------|---------|----------|------|
| SIM 状态 | `AT+CPIN?` | `parse_cpin()` | |
| IMEI | `AT+CGSN` | `parse_cgsn()` | |
| ICCID（UniSoc） | `AT+CCID` | `parse_iccid()` | UniSoc 首选 |
| ICCID（Qualcomm） | `AT+ICCID` | `parse_iccid()` | Qualcomm 首选 |
| ICCID（备用） | `AT+QCCID` | `parse_iccid()` | 两平台均可 fallback |
| 型号 | `AT+CGMM` | `parse_cgmm()` | 用于厂商检测 |
| 厂商 | `AT+CGMI` | `parse_cgmm()` | |
| 固件版本 | `AT+GMR` | `parse_gmr()` | |
| AP/CP 基线 | `AT+QBASELINE` | `parse_qbaseline()` | |
| 温度 | `AT+QTEMP` | `parse_qtemp()` | SOC 温度 / PA 温度 |

## 2. 网络状态查询（通用）

| 功能 | AT 指令 | 解析函数 | 备注 |
|------|---------|----------|------|
| 服务小区 | `AT+QENG="servingcell"` | `parse_qeng_serving_cell()` | 带宽字段 Qualcomm/UniSoc 格式不同 |
| 邻区列表 | `AT+QENG="neighbourcell"` | `parse_qeng_neighbour_cells()` | |
| 运营商 | `AT+COPS?` | `parse_cops_with_act()` | |
| 注册状态 | `AT+CEREG?` | `parse_cereg()` | |
| PDP 激活状态 | `AT+CGACT?` | `parse_cgact()` | |
| 网络模式 | `AT+QNWPREFCFG="mode_pref"` | `parse_qnwprefcfg_mode()` | |

## 3. IP 与数据连接（平台差异）

### UniSoc（RM500U / RG200U / RG500U）

| 功能 | AT 指令 | 备注 |
|------|---------|------|
| 连接数据 | `AT+QNETDEVCTL=1,{cid},1` | |
| 断开数据 | `AT+QNETDEVCTL=0,{cid},1` | |
| 查询 IP | `AT+QNETDEVSTATUS={cid}` | 响应格式见下方说明 |
| 流量统计 | `AT+QGDCNT?` | `parse_qgdcnt()` |
| 重置流量 | `AT+QGDCNT=0` | |
| 天线信号 | `AT+QANTRSSI?` | `parse_qantrssi()` → `[ANT0, ANT1, ANT2, ANT3]` |

`AT+QNETDEVSTATUS` 响应格式（无 status 前缀字段）：
```
<ipv4>,<mask>,<gw>,,<dns1>,<dns2>,<ipv6>,,,,<v6dns1>,<v6dns2>
  [0]   [1]   [2] [3] [4]   [5]   [6]  [7][8][9] [10]    [11]
```

### Qualcomm（RM500Q / RG500Q / RM520N / RG520N / RM551E / RM530F 等）

| 功能 | AT 指令 | 备注 |
|------|---------|------|
| 连接数据 | `AT+QMAP="connect",{cid}` | |
| 断开数据 | `AT+QMAP="disconnect",{cid}` | |
| 查询 IP | `AT+QMAP="WWAN"` | fallback: `AT+CGPADDR` |
| 流量统计 | `AT+QGDNRCNT?` | `parse_qgdnrcnt()` |
| 重置流量 | `AT+QGDNRCNT=0` | |
| 天线信号 | `AT+QRSRP` | `parse_qrsrp()` → `[ANT0, ANT1, ANT2, ANT3]` |

## 4. 频段配置（通用）

| 功能 | AT 指令 | 解析函数 |
|------|---------|----------|
| 支持频段 | `AT+QNWPREFCFG=?` | `parse_qnwprefcfg_supported()` |
| LTE 锁定频段 | `AT+QNWPREFCFG="lte_band"` | `parse_qnwprefcfg_bands()` |
| NR 锁定频段 | `AT+QNWPREFCFG="nr5g_band"` | `parse_qnwprefcfg_bands()` |
| 设置 LTE 频段 | `AT+QNWPREFCFG="lte_band","B1:B3:B5"` | |
| 设置 NR 频段 | `AT+QNWPREFCFG="nr5g_band","n1:n3:n5"` | |
| 重置频段 | `AT+QNWPREFCFG="all_band_reset"` | |

## 5. QoS 信息（通用）

| 功能 | AT 指令 | 解析函数 |
|------|---------|----------|
| QoS 查询 | `AT+C5GQOSRDP={cid}` | `parse_c5gqosrdp()` → `(cqi, ul_bw, dl_bw)` |

## 6. 功能开关（通用）

| 功能 | 查询指令 | 设置指令 | 备注 |
|------|---------|---------|------|
| PCIe 模式 | `AT+QCFG="pcie/mode"` | `AT+QCFG="pcie/mode",{0/1}` | |
| 以太网 | `AT+QCFG="ethernet"` | `AT+QCFG="ethernet",{0/1}` | |
| Proxy ARP | `AT+QCFG="proxyarp"` | `AT+QCFG="proxyarp",{0/1}` | |
| UART AT | `AT+QCFG="uartat"` | `AT+QCFG="uartat",{0/1}` | |
| ETH AT | `AT+QCFG="eth_at"` | `AT+QCFG="eth_at",{0/1}` | |
| ADB | `AT+QCFG="usbcfg"` | `AT+QCFG="usbcfg",{...}` | 修改倒数第二个字段 |
| NAPT | `AT+QCFG="napt"` | `AT+QCFG="napt",{0/1}` | |
| Netmask | `AT+QCFG="netmask"` | `AT+QCFG="netmask",{0/1}` | |
| USB 网卡模式 | `AT+QCFG="usbnet"` | `AT+QCFG="usbnet",{mode}` | |

## 7. 高通专用配置（仅 Qualcomm 平台）

| 功能 | AT 指令 | 备注 |
|------|---------|------|
| 数据接口 | `AT+QCFG="data_interface"` | |
| USB 速度 | `AT+QCFG="usbspeed"` | |
| 以太网驱动 | `AT+QETH="eth_driver"` | |

## 8. 控制命令（通用）

| 功能 | AT 指令 | 备注 |
|------|---------|------|
| 射频开启 | `AT+CFUN=1` | |
| 射频关闭 | `AT+CFUN=0` | |
| 查询射频状态 | `AT+CFUN?` | 响应 `+CFUN: N` |
| 重启模组 | `AT+CFUN=1,1` | |
| 恢复出厂 | `AT+QFACT=0` | |
| SIM 卡槽查询 | `AT+QUIMSLOT?` | |
| SIM 卡槽切换 | `AT+QUIMSLOT={1/2}` | |
| LAN IP 查询 | `AT+QCFG="lanip_ex"` | 响应 `+QCFG: "lanip_ex","<gw>","<start>","<end>"` |
| LAN IP 设置 | `AT+QCFG="lanip_ex","<gw>","<start>","<end>"` | |
| 设置 APN | `AT+QICSGP={cid},{type},"<apn>","<user>","<pass>",{auth}` | |
| 删除 APN | `AT+CGDCONT={cid}` | |
| 激活/停用 PDP | `AT+CGACT={0/1},{cid}` | |
| 5GLAN 查询 | `AT+QCFG="5glan"` | |
| 5GLAN 设置 | `AT+QCFG="5glan",{cid},{0/1},1` | |
| 小区锁定 | `AT+QNWLOCK="common/5g",1,{arfcn},{pci}` | |
| 频点锁定 | `AT+QNWLOCKFREQ="common/5g",1,{arfcn}` | |
| 清除锁定 | `AT+QNWLOCK="common/5g",0` | |
| PLMN 锁定 | `AT+QSIMLOCK="PN","{password}",2,"{plmn}"` | |
| PLMN 解锁 | `AT+QSIMLOCK="PN","{password}"` | |

## 9. 解析辅助函数

| 函数 | 功能 |
|------|------|
| `is_ok()` | 响应是否以 OK 结尾 |
| `extract_data_lines()` | 提取数据行（过滤回显/OK/ERROR） |
| `parse_qcfg_int()` | 通用 QCFG 整数字段提取 |
| `format_rsrp()` | RSRP 格式化（如 `-94 dBm`） |
| `format_rsrq()` | RSRQ 格式化（如 `-4 dB`） |
| `format_bw()` | 带宽格式化（如 `100 MHz`） |
